use crate::codegen::pretty;
use crate::codegen::utils::{c_str_lit, ctype_to_tokens, entry_safe_method, params_to_tokens};
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};

/// Generate `entry.rs` - the Vulkan shared library loader and pre-instance
/// entry table.
///
/// # Generated types
///
/// - [`VulkanLib`] - OS shared library handle + `vkGetInstanceProcAddr`
/// - [`EntryDispatchTable`] - raw `Option<PFN_*>` fields for pre-instance commands
///   with both raw `unsafe` methods and safe `pub fn` wrappers that return
///   `Result<T, VkResult>` / `Result<Box<[T]>, VkResult>`.
pub fn gen_entry_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(entry_preamble());
    ts.extend(gen_vulkan_lib());
    ts.extend(gen_entry_table(reg, result_cfgs, handle_types));
    pretty(ts)
}

// ---------------------------------------------------------------------------
// Preamble
// ---------------------------------------------------------------------------

fn entry_preamble() -> TokenStream {
    quote! {
        //! Vulkan library loader and pre-instance entry points.
        //!
        //! # Usage
        //!
        //! ```rust
        //! use vk::{VulkanLib, EntryDispatchTable};
        //!
        //! // 1. Open the platform Vulkan loader.
        //! let lib = VulkanLib::load().expect("Vulkan not available");
        //!
        //! // 2. Get pre-instance entry points.
        //! let entry = lib.entry_table();
        //!
        //! // 3. Enumerate layers (two-call pattern, returns Box<[VkLayerProperties]>).
        //! let layers = entry.vkEnumerateInstanceLayerProperties_safe()
        //!     .expect("enumerate layers failed");
        //!
        //! // 4. Create an instance (returns VkInstance wrapped in Result).
        //! let instance_handle = entry.vkCreateInstance_safe(&create_info, None)
        //!     .expect("vkCreateInstance failed");
        //! ```

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
    }
}

// ---------------------------------------------------------------------------
// VulkanLib
// ---------------------------------------------------------------------------

fn gen_vulkan_lib() -> TokenStream {
    quote! {
        #[cfg(target_os = "windows")]
        const VULKAN_LIB_NAMES: &[&str] = &["vulkan-1.dll"];

        #[cfg(target_os = "macos")]
        const VULKAN_LIB_NAMES: &[&str] = &["libMoltenVK.dylib", "libvulkan.dylib", "libvulkan.1.dylib"];

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
        ///
        /// # Example
        ///
        /// ```rust
        /// let lib   = VulkanLib::load()?;
        /// let entry = lib.entry_table();
        /// let inst  = entry.vkCreateInstance_safe(&info, None)?;
        /// ```
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

// ---------------------------------------------------------------------------
// EntryDispatchTable
// ---------------------------------------------------------------------------

const ENTRY_CMD_NAMES: &[&str] = &[
    "vkCreateInstance",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
];

pub fn entry_set() -> std::collections::HashSet<&'static str> {
    ENTRY_CMD_NAMES.iter().copied().collect()
}

fn gen_entry_table(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    let mut raw_methods_ts = TokenStream::new();
    let mut safe_methods_ts = TokenStream::new();

    for &raw in ENTRY_CMD_NAMES {
        let Some(variants) = reg.commands.get(raw) else {
            continue;
        };
        let cmd = variants.last().unwrap();

        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();

        let cfg = if providers.is_empty() {
            quote! {}
        } else {
            crate::cfggen::cfg_any(&providers)
        };

        let fname = quote::format_ident!("{}", raw);
        let pfn = quote::format_ident!("PFN_{}", raw);
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

        // Raw unsafe forwarding method - original Vulkan signature, zero overhead.
        raw_methods_ts.extend(quote! {
            #cfg
            #[inline(always)]
            pub unsafe fn #fname(&self, #(#p_defs),*) -> #ret {
                unsafe { (self.#fname.expect(#miss))(#(#p_fwd),*) }
            }
        });

        // Safe method: allocator -> Option<&...>, VkResult -> Result<T, VkResult>,
        // enumerate -> Result<Box<[T]>, VkResult>.
        safe_methods_ts.extend(entry_safe_method(
            cmd,
            raw,
            &providers,
            result_cfgs,
            handle_types,
        ));
    }

    quote! {
        /// Pre-instance Vulkan entry points.  Obtain via [`VulkanLib::entry_table`].
        ///
        /// # Raw vs safe methods
        ///
        /// Each command exists in two forms:
        ///
        /// - `vkFoo(...)` - raw `unsafe` thin wrapper, original Vulkan types.
        /// - `vkFoo_safe(...)` - `pub fn`, `Option<&VkAllocationCallbacks>` for
        ///   allocators, `Result<T, VkResult>` for fallible commands, and
        ///   `Result<Box<[T]>, VkResult>` for enumerate commands.
        ///
        /// Fields are cfg-gated on the feature that introduced them; e.g.
        /// `vkEnumerateInstanceVersion` requires `feature = "VK_VERSION_1_1"`.
        #[derive(Clone)]
        pub struct EntryDispatchTable { #fields_ts }

        impl EntryDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all entry commands from the given loader closure.
            pub fn load<F>(mut loader: F) -> Self
            where
                F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
            {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            // ---------------------------------------------------------------
            // Raw unsafe methods
            // ---------------------------------------------------------------
            #raw_methods_ts
        }

        /// Safe entry-point wrappers.
        ///
        /// - Allocator parameters become `Option<&VkAllocationCallbacks>`.
        /// - Fallible commands return `Result<T, VkResult>`.
        /// - Enumerate commands return `Result<Box<[T]>, VkResult>` using the
        ///   two-call pattern; the `Box<[T]>` is allocated exactly once with the
        ///   right capacity.
        impl EntryDispatchTable {
            #safe_methods_ts
        }
    }
}
