use crate::cfggen::cfg_any;
use crate::codegen::pretty;
use crate::ir::{CType, Command, Member, Optional, Registry, TypedefKind};
use crate::types::c_type_to_rust;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn gen_loader_rs(reg: &Registry) -> String {
    // VkResult variant -> cfg guard, built once and shared by all result checks.
    let result_cfgs = build_result_cfg_map(reg);

    // Set of type names that are actual Vulkan handle types (TypedefKind::Handle).
    // Used by classify_return to distinguish opaque handles (which can be
    // initialised with null_mut cast) from structs (which cannot).
    let handle_types = build_handle_type_set(reg);

    let mut ts = TokenStream::new();
    ts.extend(gen_preamble());
    ts.extend(gen_vulkan_lib());
    ts.extend(gen_entry_table(reg));
    ts.extend(gen_dispatch_table(reg, Tier::Instance));
    ts.extend(gen_dispatch_table(reg, Tier::Device));
    ts.extend(gen_instance_wrapper(reg, &result_cfgs, &handle_types));
    ts.extend(gen_device_wrapper(reg, &result_cfgs, &handle_types));
    ts.extend(gen_command_buffer_wrapper(reg, &result_cfgs, &handle_types));
    pretty(ts)
}

#[derive(Clone, Copy, PartialEq)]
enum Tier {
    Instance,
    Device,
}

impl Tier {
    fn kind(self) -> &'static str {
        match self {
            Tier::Instance => "Instance",
            Tier::Device => "Device",
        }
    }
    fn vk_handle(self) -> proc_macro2::Ident {
        format_ident!("Vk{}", self.kind())
    }
    fn table_ty(self) -> proc_macro2::Ident {
        format_ident!("{}DispatchTable", self.kind())
    }
    fn get_proc(self) -> &'static str {
        match self {
            Tier::Instance => "vkGetInstanceProcAddr",
            Tier::Device => "vkGetDeviceProcAddr",
        }
    }
}

/// Describes what a generated safe wrapper method should return.
enum WrapperReturn<'a> {
    /// `void` return -> `()`
    Unit,
    /// Non-`VkResult` return -> forward the raw type
    Raw(&'a CType),
    /// `VkResult` + single `*mut VkFoo` out-param at end -> `Result<VkFoo, VkResult>`
    ResultHandle { handle_ty: &'a CType },
    /// Two-call enumerate pattern -> `Result<Vec<T>, VkResult>`
    ///
    /// Detected when the command has:
    ///   - a `*mut u32` count param (the `pXxxCount` sentinel)
    ///   - a `*mut T` array param that is `Optional::TrueTrue` (null on first call)
    ///   - returns `VkResult`
    Enumerate {
        item_ty: &'a CType,
        count_idx: usize,
        array_idx: usize,
    },
    /// Everything else fallible -> `Result<VkResult, VkResult>`
    ResultRaw,
}

/// Classify a command's return shape for safe wrapper generation.
///
/// `handle_types` is the set of names that are `TypedefKind::Handle` in the
/// registry - used to distinguish opaque handle out-params (lifted to
/// `Result<Handle, VkResult>`) from struct out-params (kept as `ResultRaw`).
fn classify_return<'a>(cmd: &'a Command, handle_types: &HashSet<String>) -> WrapperReturn<'a> {
    let ret = &cmd.return_type;

    if ret.base.is_empty() || ret.base == "void" {
        return WrapperReturn::Unit;
    }
    if ret.base != "VkResult" {
        return WrapperReturn::Raw(ret);
    }

    // Enumerate pattern detection
    let count_idx = cmd
        .params
        .iter()
        .position(|m| m.ty.base == "uint32_t" && m.ty.pointer_depth == 1 && !m.ty.is_const);
    if let Some(ci) = count_idx {
        let array_idx = cmd.params.iter().enumerate().position(|(i, m)| {
            i > ci
                && m.ty.pointer_depth == 1
                && !m.ty.is_const
                && !m.ty.base.is_empty()
                && matches!(m.optional, Optional::TrueTrue)
        });
        if let Some(ai) = array_idx {
            return WrapperReturn::Enumerate {
                item_ty: &cmd.params[ai].ty,
                count_idx: ci,
                array_idx: ai,
            };
        }
    }

    // Single out-handle
    // Criteria (all must hold):
    //   - pointer_depth == 1, not const, no array  -> writable out-param
    //   - base type is in handle_types             -> genuine opaque handle,
    //                                                NOT a struct or bitmask
    //   - last parameter                           -> Vulkan convention
    //   - Optional::False                          -> required output
    //
    // The handle_types check is the critical gate: without it,
    // `*mut VkImageFormatProperties` (a struct) would be misidentified as a
    // handle and `null_mut::<c_void>() as VkImageFormatProperties` would fail
    // to compile because structs are not pointer-sized.
    let out_params: Vec<(usize, &Member)> = cmd
        .params
        .iter()
        .enumerate()
        .filter(|(_, m)| {
            m.ty.pointer_depth == 1
                && !m.ty.is_const
                && m.ty.is_array.is_none()
                && matches!(m.optional, Optional::False)
                && handle_types.contains(&m.ty.base)
        })
        .collect();

    if out_params.len() == 1 {
        let (idx, m) = out_params[0];
        if idx == cmd.params.len() - 1 {
            return WrapperReturn::ResultHandle { handle_ty: &m.ty };
        }
    }

    WrapperReturn::ResultRaw
}

// Per-command VkResult -> Result conversion

/// Build a map from VkResult variant name -> cfg guard `TokenStream` by
/// consulting `reg.enums["VkResult"]`.  Variants with no `provided_by` get an
/// empty (always-true) guard; extension-gated variants get `cfg_any(providers)`.
///
/// This is computed once and threaded through every call to `result_check_arms`.
fn build_result_cfg_map(reg: &Registry) -> HashMap<String, TokenStream> {
    let mut map = HashMap::new();
    let Some(variants) = reg.enums.get("VkResult") else {
        return map;
    };
    for enum_def in variants {
        for variant in &enum_def.variants {
            let cfg = if variant.provided_by.is_empty() {
                quote! {}
            } else {
                cfg_any(&variant.provided_by)
            };
            // Use the canonical name; aliases point to the same providers.
            map.entry(variant.name.clone()).or_insert(cfg);
        }
    }
    map
}

/// Build the set of all Vulkan handle type names from the registry.
///
/// Only `TypedefKind::Handle` entries are included - structs, bitmasks,
/// enums, and basetypes are excluded.  This is used by `classify_return` to
/// avoid misidentifying a `*mut VkSomeStruct` out-param as a returnable handle.
///
/// A `*mut VkFoo` out-param is only lifted to `Result<VkFoo, VkResult>` when
/// `VkFoo` is a genuine opaque handle; otherwise the command falls through to
/// `ResultRaw` and the caller passes the pointer themselves.
fn build_handle_type_set(reg: &Registry) -> HashSet<String> {
    let mut set = HashSet::new();
    for (name, variants) in &reg.typedefs {
        if variants
            .iter()
            .any(|t| matches!(t.kind, TypedefKind::Handle { .. }))
        {
            set.insert(name.clone());
        }
    }
    set
}

/// Emit a `match r { ... }` expression that converts `r: VkResult` into
/// `Ok(r)` / `Err(r)` using the command's exact success/error code lists.
///
/// Each arm is individually `#[cfg(...)]`-gated on the feature that introduces
/// that result code, so that extension error/success variants only appear when
/// the relevant feature is enabled.
///
/// The catch-all `_` arm uses `r >= VkResult::VK_SUCCESS` - `VkResult` is
/// `#[repr(i32)]` so this is equivalent to `>= 0` but requires no cast.
fn vk_result_check(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    result_check_arms(&cmd.success_codes, &cmd.error_codes, cfg_map)
}

/// Error-only predicate for the enumerate first call.
///
/// Returns a bool expression that is `true` when `r` is a definite error.
/// Non-error non-success codes (`VK_INCOMPLETE`, `VK_TIMEOUT`, ...) must not
/// be treated as errors here - they just mean "continue to the second call".
fn vk_result_is_err(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    if cmd.error_codes.is_empty() {
        // No explicit error list: negative codes are errors by Vulkan spec convention.
        // VkResult is repr(i32); VK_SUCCESS has discriminant 0.
        quote! { r < VkResult::VK_SUCCESS }
    } else {
        let arms: Vec<TokenStream> = cmd
            .error_codes
            .iter()
            .map(|s| {
                let id = format_ident!("{}", s);
                let cfg = cfg_map.get(s).cloned().unwrap_or_default();
                quote! { #cfg VkResult::#id }
            })
            .collect();
        // Sign fallback for unknown future negative extension codes.
        quote! {
            matches!(r, #(#arms)|*) || r < VkResult::VK_SUCCESS
        }
    }
}

fn result_check_arms(
    success_codes: &[String],
    error_codes: &[String],
    cfg_map: &HashMap<String, TokenStream>,
) -> TokenStream {
    // Each pattern in a match arm must share the same cfg gate for the arm to
    // compile under all feature combinations.  We therefore group codes by
    // their cfg guard and emit one arm per unique guard.
    //
    // e.g.:
    //   #[cfg(feature = "VK_BASE_VERSION_1_0")]
    //   VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
    //   #[cfg(feature = "VK_KHR_swapchain")]
    //   VkResult::VK_SUBOPTIMAL_KHR => Ok(r),

    let ok_arms = cfg_grouped_arms(success_codes, cfg_map, true);
    let err_arms = cfg_grouped_arms(error_codes, cfg_map, false);

    // Catch-all: sign comparison with no cast.
    // `VkResult` is `#[repr(i32)]` and `VK_SUCCESS` has discriminant 0, so
    // `r >= VkResult::VK_SUCCESS` is identical to `(r as i32) >= 0` but
    // avoids the `as` cast entirely.
    quote! {
        match r {
            #(#ok_arms)*
            #(#err_arms)*
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) },
        }
    }
}

/// Group a list of `VkResult` code names by their cfg guard and emit one
/// match arm per unique guard, returning `Ok(r)` (is_ok=true) or `Err(r)`.
fn cfg_grouped_arms(
    codes: &[String],
    cfg_map: &HashMap<String, TokenStream>,
    is_ok: bool,
) -> Vec<TokenStream> {
    if codes.is_empty() {
        return vec![];
    }

    // Group by the string representation of the cfg so we can merge codes that
    // share the same guard into a single `A | B | C => ...` arm.
    let mut by_cfg: BTreeMap<String, (TokenStream, Vec<TokenStream>)> = BTreeMap::new();

    for s in codes {
        let id = format_ident!("{}", s);
        let cfg = cfg_map.get(s).cloned().unwrap_or_default();
        // Use the cfg token string as the grouping key.
        let key = cfg.to_string();
        by_cfg
            .entry(key)
            .or_insert_with(|| (cfg, Vec::new()))
            .1
            .push(quote! { VkResult::#id });
    }

    by_cfg
        .into_values()
        .map(|(cfg, pats)| {
            let result = if is_ok {
                quote! { Ok(r)  }
            } else {
                quote! { Err(r) }
            };
            quote! { #cfg #(#pats)|* => #result, }
        })
        .collect()
}

// Preamble

fn gen_preamble() -> TokenStream {
    quote! {
        //! Vulkan dispatch tables and safe wrappers.
        //!
        //! # Architecture
        //!
        //! ```text
        //! VulkanLib                    - OS shared library handle (libloading)
        //!   |- EntryDispatchTable      - pre-instance commands
        //!       |- InstanceDispatchTable / Instance<'table>
        //!           |- DeviceDispatchTable  / Device<'inst>
        //!               |- CommandBuffer<'dev>
        //! ```
        //!
        //! **Raw tables** (`*DispatchTable`) hold `Option<PFN_*>` fields and
        //! `unsafe` thin-wrapper methods with original Vulkan names.  Zero overhead.
        //!
        //! **Safe wrappers** (`Instance`, `Device`, `CommandBuffer`) provide:
        //! - `Result<T, VkResult>` for fallible commands
        //! - `Result<Vec<T>, VkResult>` for two-call enumerate commands
        //! - `Option<&VkAllocationCallbacks>` for allocator params
        //! - Compile-time lifetime enforcement (Device can't outlive Instance, etc.)
        //! - Explicit `destroy_*` methods that consume `self` (no implicit Drop)
        //!
        //! # Multi-GPU
        //!
        //! Create one `Device<'inst>` per `VkDevice`; tables are fully independent.

        #![allow(
            non_snake_case,
            unused_imports,
            clippy::too_many_arguments,
            clippy::missing_safety_doc,
        )]

        use core::ffi::{c_char, c_void};
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use std::boxed::Box;
    }
}

// VulkanLib

fn gen_vulkan_lib() -> TokenStream {
    quote! {
        #[cfg(target_os = "windows")]
        const VULKAN_LIB_NAMES: &[&str] = &["vulkan-1.dll"];

        // macOS: system Vulkan loader only; MoltenVK surfaces via ICD JSON discovery.
        // Set VK_ICD_FILENAMES or DYLD_LIBRARY_PATH to override.
        #[cfg(target_os = "macos")]
        const VULKAN_LIB_NAMES: &[&str] = &["libMoltenVK.dylib", "libvulkan.dylib", "libvulkan.1.dylib"];

        // iOS: no system Vulkan loader; MoltenVK is the only implementation.
        #[cfg(target_os = "ios")]
        const VULKAN_LIB_NAMES: &[&str] = &["libMoltenVK.dylib"];

        #[cfg(target_os = "android")]
        const VULKAN_LIB_NAMES: &[&str] = &["libvulkan.so"];

        #[cfg(all(
            unix,
            not(target_os = "macos"),
            not(target_os = "ios"),
            not(target_os = "android"),
        ))]
        const VULKAN_LIB_NAMES: &[&str] = &["libvulkan.so.1", "libvulkan.so"];

        /// Error from [`VulkanLib::load`].
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum LoadError {
            /// No platform Vulkan library could be opened.
            LibraryNotFound,
            /// Library opened but `vkGetInstanceProcAddr` was missing.
            MissingGetInstanceProcAddr,
        }

        impl core::fmt::Display for LoadError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    LoadError::LibraryNotFound =>
                        write!(f, "Vulkan loader not found (tried: {:?})", VULKAN_LIB_NAMES),
                    LoadError::MissingGetInstanceProcAddr =>
                        write!(f, "Vulkan library missing `vkGetInstanceProcAddr`"),
                }
            }
        }

        impl std::error::Error for LoadError {}

        /// Handle to the Vulkan shared library.
        ///
        /// Must outlive all dispatch tables derived from it.
        pub struct VulkanLib {
            _lib: libloading::Library,
            get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
        }

        impl VulkanLib {
            /// Open the platform Vulkan loader library.
            pub fn load() -> Result<Self, LoadError> {
                for &name in VULKAN_LIB_NAMES {
                    let Ok(lib) = (unsafe { libloading::Library::new(name) }) else {
                        continue;
                    };
                    let sym = unsafe {
                        lib.get::<PFN_vkGetInstanceProcAddr>(c"vkGetInstanceProcAddr")
                    };
                    let sym = match sym {
                        Ok(s) => *s,
                        Err(_) => return Err(LoadError::MissingGetInstanceProcAddr),
                    };
                    return Ok(VulkanLib {
                        _lib: lib,
                        get_instance_proc_addr: sym,
                    });
                }
                Err(LoadError::LibraryNotFound)
            }

            /// Call `vkGetInstanceProcAddr(instance, name)`.
            ///
            /// Pass a zeroed `VkInstance` to resolve pre-instance commands.
            #[inline(always)]
            pub fn get_instance_proc_addr(
                &self,
                instance: VkInstance,
                name: *const c_char,
            ) -> Option<unsafe extern "system" fn()> {
                unsafe { (self.get_instance_proc_addr)(instance, name) }
            }

            /// Build the pre-instance [`EntryDispatchTable`].
            #[inline]
            pub fn entry_table(&self) -> EntryDispatchTable {
                EntryDispatchTable::load(|name| {
                    self.get_instance_proc_addr(VkInstance::NULL, name)
                })
            }
        }
    }
}

// EntryDispatchTable

/// The canonical pre-instance commands, in declaration order.
///
/// These are the commands resolvable via `vkGetInstanceProcAddr(NULL, name)`.
/// We keep them in a fixed list so that:
///   - `gen_device_wrapper` / `gen_instance_wrapper` can skip them, and
///   - the entry table stays a well-known minimal struct rather than absorbing
///     every command that happens to have no instance param in some future spec.
const ENTRY_CMD_NAMES: &[&str] = &[
    "vkCreateInstance",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
];

fn entry_set() -> HashSet<&'static str> {
    ENTRY_CMD_NAMES.iter().copied().collect()
}

fn gen_entry_table(reg: &Registry) -> TokenStream {
    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    let mut methods_ts = TokenStream::new();

    for &raw in ENTRY_CMD_NAMES {
        let Some(variants) = reg.commands.get(raw) else {
            continue;
        };
        let cmd = variants.last().unwrap();

        // Collect all providers from every variant so the cfg guard matches
        // whatever features actually expose this command.
        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();

        // If there are no providers the command is implicitly always available
        // (shouldn't happen for well-formed registry data, but be safe).
        let cfg = if providers.is_empty() {
            quote! {}
        } else {
            cfg_any(&providers)
        };

        let fname = format_ident!("{}", raw);
        let pfn = format_ident!("PFN_{}", raw);
        let clit = c_str_lit(raw);
        let miss = format!("entry point not loaded: {}", raw);
        let (p_defs, p_fwd) = params_to_tokens(&cmd.params);
        let ret = ctype_to_tokens(&cmd.return_type);

        fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
        empty_ts.extend(quote! { #cfg #fname: None, });
        load_ts.extend(quote! {
            #cfg {
                table.#fname = loader(#clit.as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
            }
        });
        methods_ts.extend(quote! {
            #cfg
            #[inline(always)]
            pub unsafe fn #fname(&self, #(#p_defs),*) -> #ret {
                unsafe { (self.#fname.expect(#miss))(#(#p_fwd),*) }
            }
        });
    }

    quote! {
        /// Pre-instance Vulkan entry points.  Obtain via [`VulkanLib::entry_table`].
        ///
        /// Fields are cfg-gated on the Vulkan version or extension that introduced
        /// them.  For example, `vkEnumerateInstanceVersion` requires
        /// `feature = "VK_VERSION_1_1"`.
        #[derive(Clone)]
        pub struct EntryDispatchTable { #fields_ts }

        impl EntryDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            #methods_ts
        }
    }
}

// Raw dispatch tables
fn gen_dispatch_table(reg: &Registry, tier: Tier) -> TokenStream {
    let table_ty = tier.table_ty();
    let vk_handle = tier.vk_handle();
    let kind = tier.kind();
    let kind_low = kind.to_lowercase();
    let get_proc = tier.get_proc();

    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, tier, &skip, &enabled);

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    let mut methods_ts = TokenStream::new();

    for (feature, cmds) in &groups {
        let sec = format!(" `{}`", feature);
        fields_ts.extend(quote! { #[doc = #sec] });

        for (name, providers, cmd) in cmds {
            let cfg = cfg_any(providers);
            let fname = format_ident!("{}", name);
            let pfn = format_ident!("PFN_{}", name);
            let clit = c_str_lit(name);
            let core_fn = is_core(providers);

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! { #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg {
                    table.#fname = loader(#clit.as_ptr())
                        .map(|f| unsafe { core::mem::transmute(f) });
                }
            });
            methods_ts.extend(raw_dispatch_method(cmd, name, providers, core_fn));
        }
    }

    let table_doc = format!(
        " Raw dispatch table for `Vk{k}` commands.\n\
         \n\
         All fields are `Option<PFN_*>`.  `None` means the command was absent at\n\
         load time.  For the ergonomic safe API see [`{k}`].",
        k = kind
    );
    let load_doc = format!(" Resolve all `{}` commands via `{}`.", kind_low, get_proc);

    quote! {
        #[doc = #table_doc]
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Clone)]
        pub struct #table_ty { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl #table_ty {
            pub const EMPTY: Self = Self { #empty_ts };

            #[doc = #load_doc]
            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            /// Capture `handle` in the `vkGet*ProcAddr` closure.
            pub fn load_with_handle<F>(handle: #vk_handle, mut get_proc: F) -> Self
            where F: FnMut(#vk_handle, *const c_char) -> Option<unsafe extern "system" fn()> {
                Self::load(|name| get_proc(handle, name))
            }

            #methods_ts
        }
    }
}

fn raw_dispatch_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    core_fn: bool,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", name);
    let miss = format!("command not loaded: {}", name);
    let (handle_def, handle_fwd) = first_param_tokens(cmd);
    let rest = cmd.params.get(1..).unwrap_or(&[]);
    let (p_defs, p_fwd) = params_to_tokens(rest);
    let ret = ctype_to_tokens(&cmd.return_type);
    let fp = if core_fn {
        quote! { self.#fname.unwrap_unchecked() }
    } else {
        quote! { self.#fname.expect(#miss) }
    };

    quote! {
        #cfg
        #[inline(always)]
        pub unsafe fn #fname(&self, #handle_def, #(#p_defs),*) -> #ret {
            unsafe { (#fp)(#handle_fwd, #(#p_fwd),*) }
        }
    }
}

// Instance<'table> safe wrapper
fn gen_instance_wrapper(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Instance, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if name == "vkCreateDevice" {
                methods_ts.extend(gen_create_device_method(cmd, providers, result_cfgs));
            } else {
                methods_ts.extend(safe_instance_method(
                    cmd,
                    name,
                    providers,
                    result_cfgs,
                    handle_types,
                ));
            }
        }
    }

    quote! {
        /// Safe `VkInstance` wrapper.
        ///
        /// Borrows the [`InstanceDispatchTable`] it was created with.
        ///
        /// **No implicit `Drop`** - call [`Instance::vkDestroyInstance`] explicitly,
        /// after all child `Device`s have been destroyed.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Instance<'table> {
            raw:   VkInstance,
            table: &'table InstanceDispatchTable,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'table> Instance<'table> {
            /// Wrap a `VkInstance` with a reference to its dispatch table.
            ///
            /// # Safety
            /// `raw` must be valid for the lifetime of `table`.
            #[inline]
            pub unsafe fn new(raw: VkInstance, table: &'table InstanceDispatchTable) -> Self {
                Self { raw, table }
            }

            /// The raw `VkInstance` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkInstance { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &InstanceDispatchTable { self.table }

            #methods_ts
        }
    }
}

fn safe_instance_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    safe_method_body(
        cmd,
        name,
        providers,
        Tier::Instance,
        quote! { self.raw },
        quote! { &self.table },
        result_cfgs,
        handle_types,
    )
}

// Instance::vkCreateDevice  (special-cased to return Device<'table>)
fn gen_create_device_method(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);

    // Build the per-command result check from the IR's success/error code lists.
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);

    // Params: physicalDevice, pCreateInfo, pAllocator, pDevice
    // Exposed: physicalDevice (kept), pCreateInfo (kept), pAllocator -> Option<&...>, pDevice -> stripped (returned)
    let phys_param = cmd.params.first().map(|m| {
        let n = format_ident!("{}", kw_escape(&m.name));
        let t = ctype_to_tokens(&m.ty);
        (quote! { #n: #t }, {
            let n2 = n.clone();
            quote! { #n2 }
        })
    });
    let (phys_def, phys_fwd) = phys_param.unwrap_or_else(|| {
        (
            quote! { physical_device: VkPhysicalDevice },
            quote! { physical_device },
        )
    });

    // Middle params: skip phys (idx 0), alloc, and the *mut VkDevice output
    let mid_params: Vec<&Member> = cmd
        .params
        .iter()
        .skip(1)
        .filter(|m| {
            m.ty.base != "VkAllocationCallbacks"
                && !(m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base == "VkDevice")
        })
        .collect();
    let (mid_defs, mid_fwds): (Vec<_>, Vec<_>) = mid_params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        })
        .unzip();

    quote! {
        #cfg
        /// Create a logical device.
        ///
        /// The returned [`Device`] is lifetime-tied to this `Instance`; it cannot
        /// outlive it.  The device dispatch table is loaded immediately via
        /// `vkGetDeviceProcAddr`, bypassing the loader intercept layer.
        ///
        /// # Safety
        /// `physicalDevice` must have been enumerated from this instance.
        #[inline]
        pub unsafe fn vkCreateDevice(
            &self,
            #phys_def,
            #(#mid_defs,)*
            allocator: Option<&VkAllocationCallbacks>,
        ) -> Result<Device<'table>, VkResult> {
            let fp       = unsafe { self.table.vkCreateDevice.unwrap_unchecked() };
            let alloc    = allocator.map_or(core::ptr::null(), |a| a as *const _);
            let mut dev = core::ptr::null_mut::<core::ffi::c_void>() as VkDevice;
            let r = unsafe { fp(#phys_fwd, #(#mid_fwds,)* alloc, &mut dev) };
            // Use the exact success/error codes from the Vulkan spec for this command.
            // On error, return immediately before touching the (invalid) dev handle.
            if let Err(e) = { #result_check } { return Err(e); }
            // Load device table via vkGetDeviceProcAddr - skips the loader
            // dispatch layer, giving the driver's function pointers directly.
            let gdpa      = unsafe { self.table.vkGetDeviceProcAddr.unwrap_unchecked() };
            let dev_table = DeviceDispatchTable::load(|name| unsafe { gdpa(dev, name) });
            Ok(Device::new_owned(dev, dev_table))
        }
    }
}

// Device<'inst> safe wrapper
fn gen_device_wrapper(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if is_cmd_buf_cmd(cmd) {
                continue;
            }
            methods_ts.extend(safe_device_method(
                cmd,
                name,
                providers,
                result_cfgs,
                handle_types,
            ));
        }
    }

    quote! {
        /// Safe `VkDevice` wrapper.
        ///
        /// `'inst` ties this device to the [`Instance`] that created it.
        ///
        /// **No implicit `Drop`** - call [`Device::vkDestroyDevice`] explicitly,
        /// after destroying all child resources (buffers, pipelines, ...).
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Device<'inst> {
            raw:   VkDevice,
            /// Heap-allocated so that moving a `Device` doesn't relocate ~1500 fn ptrs.
            table: Box<DeviceDispatchTable>,
            _inst: core::marker::PhantomData<&'inst Instance<'inst>>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> Device<'inst> {
            /// Internal constructor called from `Instance::vkCreateDevice`.
            #[inline]
            pub(crate) unsafe fn new_owned(raw: VkDevice, table: DeviceDispatchTable) -> Self {
                Self { raw, table: Box::new(table), _inst: core::marker::PhantomData }
            }

            /// Wrap a raw `VkDevice` with a pre-loaded table.
            ///
            /// # Safety
            /// `raw` must be valid for `'inst` and the caller must ensure the
            /// device does not outlive its instance.
            #[inline]
            pub unsafe fn from_raw(raw: VkDevice, table: DeviceDispatchTable) -> Self {
                Self { raw, table: Box::new(table), _inst: core::marker::PhantomData }
            }

            /// The raw `VkDevice` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkDevice { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &DeviceDispatchTable { &self.table }

            /// Borrow a [`CommandBuffer`] wrapper for recording.
            ///
            /// The `CommandBuffer` borrows this `Device` so it cannot outlive it.
            #[inline(always)]
            pub fn command_buffer<'dev>(&'dev self, cmd_buf: VkCommandBuffer) -> CommandBuffer<'dev, 'inst> {
                CommandBuffer { raw: cmd_buf, device: self }
            }

            #methods_ts
        }
    }
}

fn safe_device_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    safe_method_body(
        cmd,
        name,
        providers,
        Tier::Device,
        quote! { self.raw },
        quote! { &self.table },
        result_cfgs,
        handle_types,
    )
}

// CommandBuffer<'dev, 'inst> wrapper
fn gen_command_buffer_wrapper(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if !is_cmd_buf_cmd(cmd) {
                continue;
            }
            methods_ts.extend(safe_cmd_buf_method(
                cmd,
                name,
                providers,
                result_cfgs,
                handle_types,
            ));
        }
    }

    quote! {
        /// Borrowed `VkCommandBuffer` wrapper.
        ///
        /// Borrows the [`Device`] it was obtained from, so it cannot outlive it.
        /// Obtain via [`Device::command_buffer`].
        ///
        /// All `vkCmd*` commands are methods on this type.  The first Vulkan
        /// parameter (`commandBuffer`) is supplied from `self.raw`.
        ///
        /// Recording, submission, and lifecycle management (allocation, reset, free)
        /// remain the caller's responsibility.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct CommandBuffer<'dev, 'inst: 'dev> {
            raw:    VkCommandBuffer,
            device: &'dev Device<'inst>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'dev, 'inst: 'dev> CommandBuffer<'dev, 'inst> {
            /// The raw `VkCommandBuffer` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkCommandBuffer { self.raw }

            /// The parent [`Device`].
            #[inline(always)]
            pub fn device(&self) -> &Device<'inst> { self.device }

            #methods_ts
        }
    }
}

fn is_cmd_buf_cmd(cmd: &Command) -> bool {
    cmd.params
        .first()
        .map(|m| m.ty.base == "VkCommandBuffer")
        .unwrap_or(false)
}

fn safe_cmd_buf_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    safe_method_body(
        cmd,
        name,
        providers,
        Tier::Device,
        quote! { self.raw },
        quote! { &self.device.table },
        result_cfgs,
        handle_types,
    )
}

// Shared safe method body generator
//
// Used by Instance, Device, and CommandBuffer - they differ only in how
// `self_handle` (the first Vulkan param value) and `table_expr` are obtained.
fn safe_method_body(
    cmd: &Command,
    name: &str,
    providers: &[String],
    tier: Tier,
    self_handle: TokenStream,
    table_expr: TokenStream,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", name);
    let miss = format!("command not loaded: {}", name);

    let core_fn = is_core(providers);
    let fp = if core_fn {
        quote! { (#table_expr).#fname.unwrap_unchecked() }
    } else {
        quote! { (#table_expr).#fname.expect(#miss) }
    };

    let first_param = cmd.params.first();
    let matches_tier = first_param.is_some_and(|p| match tier {
        Tier::Instance => p.ty.base == "VkInstance",
        Tier::Device => p.ty.base == "VkDevice",
    });

    let (params_for_sig, _alloc_idx_for_sig) = if matches_tier {
        strip_allocator(cmd.params.get(1..).unwrap_or(&[]))
    } else {
        strip_allocator(cmd.params.as_slice())
    };

    let (clean_params_full, alloc_idx_full) = strip_allocator(cmd.params.as_slice());

    let has_alloc = _alloc_idx_for_sig.is_some();
    let alloc_param = if has_alloc {
        quote! { allocator: Option<&VkAllocationCallbacks>, }
    } else {
        quote! {}
    };
    let alloc_expr = quote! { allocator.map_or(core::ptr::null(), |a| a as *const _) };

    let mut fwd_args = build_fwd_args(&clean_params_full, alloc_idx_full, &alloc_expr);
    if matches_tier {
        fwd_args[0] = quote! { #self_handle };
    }

    match classify_return(cmd, handle_types) {
        WrapperReturn::Unit => {
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(&self, #alloc_param #(#p_defs),*) {
                    unsafe { (#fp)(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens(ret_ty);
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(&self, #alloc_param #(#p_defs),*) -> #ret {
                    unsafe { (#fp)(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let inner = strip_out_param(&params_for_sig);
            let (p_defs, _) = params_to_tokens(&inner);
            let check = vk_result_check(cmd, result_cfgs);

            let last_idx = fwd_args.len().saturating_sub(1);
            if !fwd_args.is_empty() {
                fwd_args[last_idx] = quote! { &mut handle };
            }

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<#h_ty, VkResult> {
                    let mut handle = #h_ty::NULL;
                    let r = unsafe { (#fp)(#(#fwd_args),*) };
                    #check .map(|_| handle)
                }
            }
        }

        WrapperReturn::Enumerate {
            item_ty,
            count_idx,
            array_idx,
        } => {
            let elem_ty = deref_ctype(item_ty);
            let ci = count_idx;
            let ai = array_idx;

            let keep_params: Vec<Member> = params_for_sig
                .iter()
                .enumerate()
                .filter(|(i, _m)| {
                    let actual_idx = if matches_tier { *i + 1 } else { *i };
                    actual_idx != ci && actual_idx != ai
                })
                .map(|(_, m)| m.clone())
                .collect();

            let (p_defs, _) = params_to_tokens(&keep_params);
            let is_err = vk_result_is_err(cmd, result_cfgs);
            let check2 = vk_result_check(cmd, result_cfgs);

            let mut fwd_first = fwd_args.clone();
            fwd_first[ci] = quote! { &mut count };
            fwd_first[ai] = quote! { core::ptr::null_mut() };

            let mut fwd_second = fwd_args.clone();
            fwd_second[ci] = quote! { &mut count };
            fwd_second[ai] = quote! { out.as_mut_ptr() };

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<Vec<#elem_ty>, VkResult> {
                    let mut count: u32 = 0;
                    let r = unsafe { (#fp)(#(#fwd_first),*) };
                    if #is_err { return Err(r); }
                    if count == 0 { return Ok(Vec::new()); }

                    let mut out: Vec<#elem_ty> = Vec::with_capacity(count as usize);
                    let r = unsafe { (#fp)(#(#fwd_second),*) };
                    #check2 .map(|_| { unsafe { out.set_len(count as usize) }; out })
                }
            }
        }

        WrapperReturn::ResultRaw => {
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            let check = vk_result_check(cmd, result_cfgs);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<VkResult, VkResult> {
                    let r = unsafe { (#fp)(#(#fwd_args),*) };
                    #check
                }
            }
        }
    }
}

// Command grouping
type Groups = BTreeMap<String, Vec<(String, Vec<String>, Command)>>;

fn collect_groups(
    reg: &Registry,
    tier: Tier,
    skip: &HashSet<&str>,
    enabled: &HashSet<String>,
) -> Groups {
    let mut groups: Groups = BTreeMap::new();

    for (name, variants) in &reg.commands {
        if skip.contains(name.as_str()) {
            continue;
        }

        let matches = match tier {
            Tier::Instance => variants.iter().any(is_instance_cmd),
            Tier::Device => !variants.iter().any(is_instance_cmd),
        };
        if !matches {
            continue;
        }

        let cmd_raw = variants.last().unwrap();
        if !cmd_raw.api.vulkan && !cmd_raw.api.vulkanbase {
            continue;
        }

        let cmd = resolve_alias(cmd_raw, reg);
        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();
        if providers.is_empty() {
            continue;
        }

        let primary = pick_primary(&providers, enabled);
        groups
            .entry(primary)
            .or_default()
            .push((name.clone(), providers, cmd));
    }

    for cmds in groups.values_mut() {
        cmds.sort_by(|a, b| a.0.cmp(&b.0));
    }
    groups
}

/// Walk the alias chain of a command to find the canonical definition.
///
/// We walk at most 10 steps to avoid infinite loops on malformed data.
fn resolve_alias<'r>(cmd: &'r Command, reg: &'r Registry) -> Command {
    let mut current = cmd;
    for i in 0..10 {
        // A command needs resolution if it has an alias and no params.
        // If it has params it is already canonical (or a valid independent entry).
        if !current.params.is_empty() {
            break;
        }
        let Some(alias_name) = &current.alias else {
            break;
        };
        let Some(alias_variants) = reg.commands.get(alias_name) else {
            break;
        };
        let Some(alias_cmd) = alias_variants.last() else {
            break;
        };
        current = alias_cmd;
        if i == 9 {
            panic!("alias chain too long resolving {}", cmd.name);
        }
    }
    // Clone the resolved command but keep the aliased command's success/error
    // codes if the alias entry has them and the canonical doesn't - some
    // aliases carry additional result codes (e.g. extension-specific errors).
    let mut resolved = current.clone();
    if resolved.success_codes.is_empty() && !cmd.success_codes.is_empty() {
        resolved.success_codes = cmd.success_codes.clone();
    }
    if resolved.error_codes.is_empty() && !cmd.error_codes.is_empty() {
        resolved.error_codes = cmd.error_codes.clone();
    }
    resolved
}

// Helpers

fn enabled_set(reg: &Registry) -> HashSet<String> {
    reg.features
        .iter()
        .map(|f| f.name.clone())
        .chain(
            reg.extensions
                .iter()
                .filter(|e| !e.is_disabled())
                .map(|e| e.name.clone()),
        )
        .collect()
}

fn pick_primary(providers: &[String], enabled: &HashSet<String>) -> String {
    providers
        .iter()
        .find(|f| f.starts_with("VK_BASE_VERSION_") || f.starts_with("VK_VERSION_"))
        .or_else(|| providers.iter().find(|f| f.starts_with("VK_KHR_")))
        .or_else(|| providers.iter().find(|f| f.starts_with("VK_EXT_")))
        .or_else(|| providers.iter().find(|f| enabled.contains(*f)))
        .cloned()
        .unwrap_or_else(|| providers[0].clone())
}

fn is_core(providers: &[String]) -> bool {
    providers
        .iter()
        .any(|f| f.starts_with("VK_BASE_VERSION_") || f.starts_with("VK_VERSION_"))
}

fn is_instance_cmd(cmd: &Command) -> bool {
    match cmd.params.first() {
        Some(m) => m.ty.base == "VkInstance" || m.ty.base == "VkPhysicalDevice",
        None => true,
    }
}

/// Build (definition, forward) tokens for param[0] (the raw dispatchable handle).
fn first_param_tokens(cmd: &Command) -> (TokenStream, TokenStream) {
    match cmd.params.first() {
        Some(m) => {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        }
        None => (quote! { _handle: *mut c_void }, quote! { _handle }),
    }
}

/// Build (definition, forward) token lists from a `Member` slice.
fn params_to_tokens(params: &[Member]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        })
        .unzip()
}

/// Split params into (user-visible params, allocator index if present).
///
/// Returns `(params_without_allocator, Option<original_index_in_slice>)`.
/// The index is into the *input slice* (i.e. already offset past param[0]).
/// We keep the index so the forwarding call can insert the allocator expression
/// at the correct position rather than appending it at the end.
fn strip_allocator(params: &[Member]) -> (Vec<Member>, Option<usize>) {
    match params
        .iter()
        .position(|m| m.ty.base == "VkAllocationCallbacks")
    {
        Some(i) => {
            let mut out = params.to_vec();
            out.remove(i);
            (out, Some(i))
        }
        None => (params.to_vec(), None),
    }
}

/// Build forwarding argument tokens for the full parameter list (excluding
/// param[0] which is always `self_handle`), re-inserting the allocator
/// expression at its original position.
///
/// `clean_params` has the allocator removed; `alloc_idx` is where to splice
/// it back in (index into the original post-handle slice).
fn build_fwd_args(
    clean_params: &[Member],
    alloc_idx: Option<usize>,
    alloc_expr: &TokenStream,
) -> Vec<TokenStream> {
    // Generate forwarding idents for the user-visible params.
    let fwd: Vec<TokenStream> = clean_params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            quote! { #n }
        })
        .collect();

    match alloc_idx {
        None => fwd,
        Some(idx) => {
            // Splice the allocator expression back at its original position.
            let mut result = Vec::with_capacity(fwd.len() + 1);
            result.extend_from_slice(&fwd[..idx]);
            result.push(alloc_expr.clone());
            result.extend_from_slice(&fwd[idx..]);
            result
        }
    }
}

fn strip_out_param(params: &[Member]) -> Vec<Member> {
    let mut out = params.to_vec();
    if out
        .last()
        .map(|m| m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base.starts_with("Vk"))
        .unwrap_or(false)
    {
        out.pop();
    }
    out
}

/// `*mut VkFoo` -> `VkFoo` as a token stream (strips one pointer level).
fn deref_ctype(ty: &CType) -> TokenStream {
    // Use the same mapping as ctype_to_tokens so C primitive bases resolve
    // correctly (e.g. uint32_t -> u32).
    base_type_tokens(&ty.base)
}

/// Translate a `CType` into a Rust type `TokenStream`.
///
/// The output must exactly match the corresponding `PFN_*` typedef so that
/// the generated wrapper can be called through the stored function pointer
/// without any implicit coercions.
fn ctype_to_tokens(ty: &CType) -> TokenStream {
    if (ty.base.is_empty() || ty.base == "void") && ty.pointer_depth == 0 && ty.is_array.is_none() {
        return quote! { () };
    }

    let base = base_type_tokens(&ty.base);
    let mut ts = if let Some(ref size) = ty.is_array {
        let size_ts: TokenStream = if size.parse::<u64>().is_ok() {
            size.parse().unwrap()
        } else {
            format!("{} as usize", size).parse().unwrap()
        };
        quote! { [#base; #size_ts] }
    } else {
        base
    };

    // Apply const to every pointer level - *const *const T, not *const *mut T.
    // This matches the PFN typedefs generated from the Vulkan XML.
    for _ in 0..ty.pointer_depth {
        ts = if ty.is_const {
            quote! { *const #ts }
        } else {
            quote! { *mut #ts }
        };
    }
    ts
}

/// Resolve the base type name to a Rust `TokenStream`.
///
/// C primitives go through `c_type_to_rust`; Vulkan types (which already have
/// valid Rust idents in the generated crate) are passed through unchanged.
/// Paths like `core::ffi::c_void` are parsed rather than wrapped in
/// `format_ident!` because they contain `::` separators.
fn base_type_tokens(base: &str) -> TokenStream {
    let resolved = c_type_to_rust(base);
    let name = if resolved.is_empty() {
        // Vulkan type or unknown - pass through as-is.  Empty base means void.
        if base.is_empty() {
            "core::ffi::c_void"
        } else {
            base
        }
    } else {
        resolved
    };

    // Parse the resolved name as a token stream.  This handles both plain
    // identifiers (`u32`, `VkDevice`) and path expressions (`core::ffi::c_void`).
    name.parse::<TokenStream>()
        .unwrap_or_else(|_| format_ident!("{}", name).into_token_stream())
}

fn c_str_lit(name: &str) -> TokenStream {
    format!("c\"{}\"", name).parse().unwrap()
}

fn kw_escape(name: &str) -> &str {
    match name {
        "type" => "ty",
        "ref" => "reference",
        "mod" => "module",
        "in" => "input",
        "out" => "output",
        "use" => "usage",
        "loop" => "loop_",
        "match" => "match_",
        "where" => "where_",
        "return" => "return_",
        other => other,
    }
}
