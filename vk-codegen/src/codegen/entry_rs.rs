use crate::cfggen::cfg_any;
use crate::codegen::pretty;
use crate::codegen::utils::{c_str_lit, create_doc, params_to_tokens, safe_method};
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

// Entry command names (pre-instance, resolved via vkGetInstanceProcAddr(NULL))
const ENTRY_CMDS: &[&str] = &[
    "vkCreateInstance",
    "vkEnumerateInstanceExtensionProperties",
    "vkEnumerateInstanceLayerProperties",
    "vkEnumerateInstanceVersion",
];

pub fn entry_cmd_set() -> HashSet<&'static str> {
    ENTRY_CMDS.iter().copied().collect()
}

// Top-level generator
pub fn gen_entry_rs(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &std::collections::BTreeMap<String, crate::codegen::handles_rs::HandleMeta>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_vulkan_lib());
    ts.extend(gen_entry_dispatch_table(reg));
    ts.extend(gen_entry(reg, handle_types, handle_meta));
    pretty(&ts)
}

fn preamble() -> TokenStream {
    quote! {
        //! Vulkan library loader, pre-instance dispatch table, and `Entry` wrapper.
        //!
        //! # Hierarchy
        //!
        //! ```text
        //! VulkanLib
        //!   └── Entry<'lib>          (pre-instance commands)
        //!         └── Instance<'lib> (instance commands)
        //!               └── Device<'inst> (device commands)
        //!                     └── CommandBuffer<'dev>
        //! ```
        //!
        //! # Quick start
        //!
        //! ```rust,no_run
        //! let lib      = VulkanLib::load()?;
        //! let entry    = Entry::new(&lib);
        //! let instance = entry.vkCreateInstance(&info, ptr::null())?;
        //! let gpus     = instance.vkEnumeratePhysicalDevices()?;
        //! let device   = instance.vkCreateDevice(gpus[0], &dev_info, ptr::null())?;
        //! ```

        #![allow(
            non_snake_case,
            unused_imports,
            clippy::too_many_arguments,
            clippy::missing_safety_doc,
        )]

        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use core::ffi::{c_char, c_void};
    }
}

// VulkanLib
fn gen_vulkan_lib() -> TokenStream {
    quote! {
        #[cfg(target_os = "windows")]
        const VULKAN_LIB_NAMES: &[&str] = &["vulkan-1.dll"];

        #[cfg(target_os = "macos")]
        const VULKAN_LIB_NAMES: &[&str] = &[
            "libMoltenVK.dylib", "libvulkan.dylib", "libvulkan.1.dylib",
        ];

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


        /// Handle to the platform Vulkan shared library.
        ///
        /// Must outlive all [`Entry`], [`Instance`], and [`Device`] values
        /// derived from it - the lifetime chain is enforced by the borrow checker.
        ///
        /// # Cleanup
        /// No cleanup required. The library is closed when this value is dropped.
        pub struct VulkanLib {
            _lib: libloading::Library,
            pub(crate) get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
        }

        unsafe impl Send for VulkanLib {}
        unsafe impl Sync for VulkanLib {}

        impl VulkanLib {
            /// Open the platform Vulkan loader.
            pub fn load() -> Result<Self, LoadError> {
                for &name in VULKAN_LIB_NAMES {
                    let Ok(lib) = (unsafe { libloading::Library::new(name) }) else {
                        continue;
                    };
                    let sym = unsafe {
                        lib.get::<PFN_vkGetInstanceProcAddr>(c"vkGetInstanceProcAddr")
                    };
                    let sym = match sym {
                        Ok(s)  => *s,
                        Err(_) => return Err(LoadError::MissingGetInstanceProcAddr),
                    };
                    return Ok(Self { _lib: lib, get_instance_proc_addr: sym });
                }
                Err(LoadError::LibraryNotFound)
            }
        }
    }
}

// EntryDispatchTable
// Pure data: Option<PFN_*> fields + load constructor only.
fn gen_entry_dispatch_table(reg: &Registry) -> TokenStream {
    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut init_ts = TokenStream::new();

    for &name in ENTRY_CMDS {
        let Some(variants) = reg.commands.get(name) else {
            continue;
        };

        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();

        let cfg = if providers.is_empty() {
            quote! {}
        } else {
            cfg_any(&providers)
        };
        let fname = format_ident!("{}", name);
        let pfn = format_ident!("PFN_{}", name);
        let clit = c_str_lit(name);

        fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
        empty_ts.extend(quote! { #cfg #fname: None, });
        init_ts.extend(quote! {
            #cfg
            #fname: loader(#clit.as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
        });
    }

    quote! {
        /// Pre-instance function pointer table.
        ///
        /// Fields are `Option<PFN_*>`; `None` means absent at load time.
        /// Use [`Entry`] for the safe API.
        #[derive(Debug, Clone)]
        pub struct EntryDispatchTable { #fields_ts }

        impl EntryDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all pre-instance commands from the given loader closure.
            pub fn load<F>(loader: F) -> Self
            where F: Fn(*const c_char) -> Option<unsafe extern "system" fn()> {
                Self {
                    #init_ts
                }
            }
        }
    }
}

// Entry<'lib>
//
// Borrows VulkanLib for its lifetime.  Holds EntryDispatchTable by value.
// Exposes Vulkan-named safe methods; vkCreateInstance returns Instance<'lib>.
//
// Entry has no dispatchable param[0] to strip (pre-instance commands take no
// VkInstance), so handle_base is "" - all params appear in the signature as-is.
// The only transformation is VkResult -> Result<T, VkResult>.
fn gen_entry(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &std::collections::BTreeMap<String, crate::codegen::handles_rs::HandleMeta>,
) -> TokenStream {
    let mut methods_ts = TokenStream::new();

    for &name in ENTRY_CMDS {
        let Some(variants) = reg.commands.get(name) else {
            continue;
        };
        let cmd = variants.last().unwrap();

        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();

        if name == "vkCreateInstance" {
            methods_ts.extend(gen_create_instance(cmd, &providers, handle_meta));
        } else {
            // No param[0] stripping for entry commands (handle_base = "").
            methods_ts.extend(safe_method(
                cmd,
                name,
                &providers,
                "",
                quote! {}, // self_handle unused when handle_base is ""
                quote! { &self.table },
                handle_types,
                None,
                quote! {},
                quote! {},
            ));
        }
    }

    quote! {
        /// Pre-instance Vulkan entry point.
        ///
        /// Borrows the [`VulkanLib`] it was created from; cannot outlive it.
        ///
        /// Obtain via [`Entry::new`], then call [`Entry::vkCreateInstance`] to
        /// get an [`Instance`].
        ///
        /// # Cleanup
        /// No cleanup required.  Entry commands leave no Vulkan objects alive.
        pub struct Entry<'lib> {
            table: EntryDispatchTable,
            /// Retained so that `vkCreateInstance` can resolve the instance-tier
            /// table via `vkGetInstanceProcAddr` after the instance is created.
            lib:   &'lib VulkanLib,
        }

        unsafe impl<'lib> Send for Entry<'lib> {}
        unsafe impl<'lib> Sync for Entry<'lib> {}

        impl<'lib> Entry<'lib> {
            /// Create an `Entry` by loading all pre-instance commands from `lib`.
            #[inline]
            pub fn new(lib: &'lib VulkanLib) -> Self {
                let table = EntryDispatchTable::load(|name| unsafe {
                    (lib.get_instance_proc_addr)(VkInstance::NULL, name)
                });
                Self { table, lib }
            }

            #methods_ts
        }
    }
}

// vkCreateInstance - special-cased to return Instance<'lib>
fn gen_create_instance(
    cmd: &crate::ir::Command,
    providers: &[String],
    handle_meta: &std::collections::BTreeMap<String, crate::codegen::handles_rs::HandleMeta>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let doc = create_doc(cmd, providers);

    // All params except the output *mut VkInstance (returned as Instance).
    let sig_params: Vec<_> = cmd
        .params
        .iter()
        .filter(|m| !(m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base == "VkInstance"))
        .collect();
    let sig_params_owned: Vec<_> = sig_params.into_iter().cloned().collect();
    let (p_defs, p_fwd) = params_to_tokens(&sig_params_owned);

    let mut handle_load = TokenStream::new();
    let mut handle_args = TokenStream::new();
    for m in handle_meta
        .values()
        .filter(|m| m.root_vk_name == "VkInstance")
    {
        let name = format_ident!("{}_table", m.mod_name);
        let tb = format_ident!("{}", m.table_name);
        let md = format_ident!("{}", m.mod_name);
        let cfg = cfg_any(&m.providers);
        handle_load.extend(quote! {
            #cfg
            let #name = crate::#md::#tb::load(|name| unsafe {
                (self.lib.get_instance_proc_addr)(raw, name)
            });
        });
        handle_args.extend(quote! {
            #cfg
            #name,
        });
    }

    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    token_stream.extend(quote! {
        #cfg
        #[inline]
        pub fn vkCreateInstance(
            &self,
            #(#p_defs,)*
        ) -> Result<crate::instance::Instance<'lib>, VkResult> {
            use crate::instance::{Instance, InstanceDispatchTable};
            let fp  = unsafe { self.table.vkCreateInstance.unwrap_unchecked() };
            let mut raw = VkInstance::NULL;
            let r = unsafe { fp(#(#p_fwd,)* &mut raw) };
            if r < VkResult::VK_SUCCESS { return Err(r); }
            let table = InstanceDispatchTable::load(|name| unsafe {
                (self.lib.get_instance_proc_addr)(raw, name)
            });
            let pd_table = crate::physical_device::PhysicalDeviceDispatchTable::load(|name| unsafe {
                (self.lib.get_instance_proc_addr)(raw, name)
            });
            #handle_load
            Ok(unsafe { Instance::from_raw(raw, table, pd_table, #handle_args) })
        }
    });
    token_stream
}
