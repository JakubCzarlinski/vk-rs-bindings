use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_set;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, ctype_to_tokens, enabled_set, is_core, kw_escape,
    raw_dispatch_method, result_check_arms, safe_method_body,
};
use crate::ir::{Command, Member, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

/// Generate `instance.rs` - the instance dispatch table and the `Instance`
/// safe wrapper.
///
/// # Generated types
///
/// - [`InstanceDispatchTable`] - raw `Option<PFN_*>` table for all instance-tier
///   commands, with `unsafe` thin-wrapper methods.
/// - [`Instance`] - ergonomic safe wrapper that owns or borrows the table.
///   Created directly from a [`VulkanLib`] via [`Instance::create`]; no need to
///   manually drive `entry_table → vkCreateInstance → load_table`.
pub fn gen_instance_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(instance_preamble());
    ts.extend(gen_instance_dispatch_table(reg));
    ts.extend(gen_instance_wrapper(reg, result_cfgs, handle_types));
    pretty(ts)
}

// ---------------------------------------------------------------------------
// Preamble
// ---------------------------------------------------------------------------

fn instance_preamble() -> TokenStream {
    quote! {
        //! Instance-tier dispatch table and safe wrapper.
        //!
        //! # Usage
        //!
        //! ```rust
        //! use vk::{VulkanLib, Instance};
        //!
        //! let lib = VulkanLib::load()?;
        //!
        //! // One call: create VkInstance + load all instance function pointers.
        //! let instance = Instance::create(&lib, &create_info, None)?;
        //!
        //! // Enumerate physical devices (two-call, returns Box<[VkPhysicalDevice]>).
        //! let gpus = instance.vkEnumeratePhysicalDevices()?;
        //! ```

        #![allow(
            non_snake_case,
            unused_imports,
            clippy::too_many_arguments,
            clippy::missing_safety_doc,
        )]

        use core::ffi::{c_char, c_void};
        use std::boxed::Box;
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use crate::entry::{VulkanLib, EntryDispatchTable};
    }
}

// ---------------------------------------------------------------------------
// InstanceDispatchTable
// ---------------------------------------------------------------------------

fn gen_instance_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Instance, &skip, &enabled);

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

    quote! {
        /// Raw dispatch table for `VkInstance` commands.
        ///
        /// All fields are `Option<PFN_*>`.  `None` means the command was absent at
        /// load time.  For the ergonomic safe API see [`Instance`].
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Clone)]
        pub struct InstanceDispatchTable { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl InstanceDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all instance commands via `vkGetInstanceProcAddr`.
            pub fn load<F>(mut loader: F) -> Self
            where
                F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
            {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            /// Capture `instance` in the `vkGetInstanceProcAddr` closure.
            pub fn load_with_handle<F>(instance: VkInstance, mut get_proc: F) -> Self
            where
                F: FnMut(VkInstance, *const c_char) -> Option<unsafe extern "system" fn()>,
            {
                Self::load(|name| get_proc(instance, name))
            }

            #methods_ts
        }
    }
}

// ---------------------------------------------------------------------------
// Instance safe wrapper
// ---------------------------------------------------------------------------

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
                methods_ts.extend(safe_method_body(
                    cmd,
                    name,
                    providers,
                    Tier::Instance,
                    quote! { self.raw },
                    quote! { &self.table },
                    result_cfgs,
                    handle_types,
                ));
            }
        }
    }

    quote! {
        /// Safe `VkInstance` wrapper with owned dispatch table.
        ///
        /// # Creation
        ///
        /// Use [`Instance::create`] for the one-call ergonomic path:
        ///
        /// ```rust
        /// let instance = Instance::create(&lib, &create_info, None)?;
        /// ```
        ///
        /// Or build manually if you need the raw handle split from the table:
        ///
        /// ```rust
        /// let entry = lib.entry_table();
        /// let raw   = entry.vkCreateInstance_safe(&info, None)?;
        /// let table = InstanceDispatchTable::load_with_handle(raw, |inst, name| {
        ///     lib.get_instance_proc_addr(inst, name)
        /// });
        /// let instance = unsafe { Instance::from_raw(raw, table) };
        /// ```
        ///
        /// # Cleanup
        ///
        /// No implicit `Drop`.  Call [`Instance::vkDestroyInstance`] explicitly
        /// after all child [`Device`]s have been destroyed.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Instance {
            raw:   VkInstance,
            /// Heap-allocated so that moving an `Instance` doesn't relocate hundreds
            /// of function pointers.
            table: Box<InstanceDispatchTable>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl Instance {
            // -------------------------------------------------------------------
            // Construction
            // -------------------------------------------------------------------

            /// **One-call ergonomic constructor.**
            ///
            /// Opens an [`EntryDispatchTable`] from `lib`, calls `vkCreateInstance`,
            /// then immediately loads the full [`InstanceDispatchTable`] via
            /// `vkGetInstanceProcAddr`.
            ///
            /// This is the recommended way to obtain an `Instance`.
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            #[inline]
            pub fn create(
                lib:        &VulkanLib,
                create_info: *const VkInstanceCreateInfo,
                allocator:  Option<&VkAllocationCallbacks>,
            ) -> Result<Self, VkResult> {
                let entry = lib.entry_table();
                let raw   = entry.vkCreateInstance_safe(create_info, allocator)?;
                let table = InstanceDispatchTable::load_with_handle(raw, |inst, name| {
                    lib.get_instance_proc_addr(inst, name)
                });
                Ok(Self { raw, table: Box::new(table) })
            }

            /// Wrap a `VkInstance` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkInstance` for the duration of use.
            #[inline]
            pub unsafe fn from_raw(raw: VkInstance, table: InstanceDispatchTable) -> Self {
                Self { raw, table: Box::new(table) }
            }

            // -------------------------------------------------------------------
            // Accessors
            // -------------------------------------------------------------------

            /// The raw `VkInstance` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkInstance { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &InstanceDispatchTable { &self.table }

            // -------------------------------------------------------------------
            // Safe methods + special-cased vkCreateDevice
            // -------------------------------------------------------------------
            #methods_ts
        }
    }
}

// ---------------------------------------------------------------------------
// vkCreateDevice - special-cased to return Device
// ---------------------------------------------------------------------------

fn gen_create_device_method(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);

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
        /// **Prefer [`Device::create`]** which wraps this call with table loading
        /// in a single step.
        ///
        /// Loads the device dispatch table via `vkGetDeviceProcAddr` immediately,
        /// bypassing the loader intercept layer for direct driver function pointers.
        ///
        /// # Safety
        /// `physicalDevice` must have been enumerated from this instance.
        #[inline]
        pub unsafe fn vkCreateDevice(
            &self,
            #phys_def,
            #(#mid_defs,)*
            allocator: Option<&VkAllocationCallbacks>,
        ) -> Result<crate::device::Device, VkResult> {
            let fp    = unsafe { self.table.vkCreateDevice.unwrap_unchecked() };
            let alloc = allocator.map_or(core::ptr::null(), |a| a as *const _);
            let mut dev = core::ptr::null_mut::<core::ffi::c_void>() as VkDevice;
            let r = unsafe { fp(#phys_fwd, #(#mid_fwds,)* alloc, &mut dev) };
            if let Err(e) = { #result_check } { return Err(e); }
            let gdpa      = unsafe { self.table.vkGetDeviceProcAddr.unwrap_unchecked() };
            let dev_table = crate::device::DeviceDispatchTable::load(|name| {
                unsafe { gdpa(dev, name) }
            });
            Ok(crate::device::Device::new_owned(dev, dev_table))
        }
    }
}
