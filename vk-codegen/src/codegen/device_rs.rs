use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::handles_rs::HandleMeta;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, create_doc, enabled_set, is_cmd_buf_cmd, params_to_tokens,
    safe_method, safe_method_unit_with_overrides, strip_first_param,
};
use crate::ir::{Command, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashSet};

pub fn gen_device_rs(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_device_dispatch_table(reg));
    ts.extend(gen_device(reg, handle_types, handle_meta));
    pretty(&ts)
}

// Preamble
fn preamble() -> TokenStream {
    quote! {
        //! Device-tier dispatch table and safe [`Device`] wrapper.

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
        use crate::instance::Instance;
    }
}

// DeviceDispatchTable
fn gen_device_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut init_ts = TokenStream::new();

    for cmds in groups.values() {
        for (name, providers, _cmd) in cmds {
            let cfg = cfg_any(providers);
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
    }

    quote! {
        /// Raw device-tier function pointer table.
        ///
        /// Fields are `Option<PFN_*>`; `None` means absent at load time.
        /// Use [`Device`] for the safe API and [`CommandBuffer`] for `vkCmd*`.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Debug, Clone)]
        pub struct DeviceDispatchTable { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl DeviceDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all device commands from the given loader closure.
            pub fn load<F>(loader: F) -> Self
            where F: Fn(*const c_char) -> Option<unsafe extern "system" fn()> {
                Self {
                    #init_ts
                }
            }

            /// Resolve all device commands via `vkGetDeviceProcAddr(device, …)`.
            pub fn load_for_device<F>(device: VkDevice, get_proc: F) -> Self
            where F: Fn(VkDevice, *const c_char) -> Option<unsafe extern "system" fn()> {
                Self::load(|name| get_proc(device, name))
            }
        }
    }
}

// Device<'inst>
//
// Lifetime 'inst ties this device to the Instance that created it.  Holds
// DeviceDispatchTable by value (no Box).  All vkCmd* commands are excluded
// here and live on CommandBuffer instead.
fn gen_device(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if is_cmd_buf_cmd(cmd) {
                continue;
            }
            if name == "vkGetDeviceQueue" {
                methods_ts.extend(gen_get_device_queue(cmd, providers));
            } else if name == "vkDestroyDevice" {
                methods_ts.extend(safe_method_unit_with_overrides(
                    cmd,
                    name,
                    providers,
                    "VkDevice", // strip param[0] = VkDevice
                    quote! { self.raw },
                    quote! { &self.table },
                    handle_types,
                    Some(handle_meta),
                    quote! { self },
                    quote! { self.instance() },
                    quote! { &mut self },
                    quote! {
                        if self.raw.0.is_null() {
                            return;
                        }
                    },
                    quote! {
                        self.raw = VkDevice::NULL;
                    },
                ));
            } else if name == "vkCreateCommandPool" {
                methods_ts.extend(gen_create_command_pool(cmd, providers));
            } else if name.starts_with("vkCreate") && name.ends_with("Pipelines") {
                methods_ts.extend(gen_create_pipelines(cmd, providers));
            } else {
                methods_ts.extend(safe_method(
                    cmd,
                    name,
                    providers,
                    "VkDevice", // strip param[0] = VkDevice
                    quote! { self.raw },
                    quote! { &self.table },
                    handle_types,
                    Some(handle_meta),
                    quote! { self },
                    quote! { self.instance() },
                ));
            }
        }
    }

    let mut handle_fields = TokenStream::new();
    let mut handle_args = TokenStream::new();
    let mut handle_init = TokenStream::new();
    for m in handle_meta.values() {
        let field_name = format_ident!("{}", m.table_field);
        let md = format_ident!("{}", m.mod_name);
        let tb = format_ident!("{}", m.table_name);
        let cfg = cfg_any(&m.providers);
        handle_fields.extend(quote! {
            #cfg
            pub(crate) #field_name: crate::#md::#tb,
        });
        handle_args.extend(quote! {
            #cfg
            #field_name: crate::#md::#tb,
        });
        handle_init.extend(quote! {
            #cfg
            #field_name,
        });
    }

    quote! {
        /// Safe `VkDevice` wrapper.
        ///
        /// Lifetime `'inst` ties this device to the [`Instance`] that created it;
        /// it cannot outlive it.
        ///
        /// Holds [`DeviceDispatchTable`] by value.
        ///
        /// # Cleanup
        /// On drop, if the raw `VkDevice` is non-null and `vkDestroyDevice` is
        /// present in the dispatch table, it is called with `self.raw` and
        /// `pAllocator = null`.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Device<'inst> {
            pub(crate) raw:   VkDevice,
            pub(crate) instance: &'inst Instance<'inst>,
            pub(crate) table: DeviceDispatchTable,
            #handle_fields
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'inst> Send for Device<'inst> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'inst> Sync for Device<'inst> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> Device<'inst> {
            /// Wrap a raw `VkDevice` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkDevice` for `'inst`.
            #[inline]
            pub const unsafe fn from_raw(
                raw: VkDevice,
                instance: &'inst Instance<'inst>,
                table: DeviceDispatchTable,
                #handle_args
            ) -> Self {
                Self { raw, instance, table, #handle_init }
            }

            /// The raw `VkDevice` handle.
            #[inline(always)]
            pub const fn raw(&self) -> VkDevice { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub const fn table(&self) -> &DeviceDispatchTable { &self.table }

            /// The instance that created this device.
            #[inline(always)]
            pub const fn instance(&self) -> &'inst Instance<'inst> { self.instance }

            #methods_ts
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> Drop for Device<'inst> {
            fn drop(&mut self) {
                if self.raw.0.is_null() {
                    return;
                }
                unsafe {
                  self.table.vkDestroyDevice.unwrap_unchecked()(self.raw, core::ptr::null())
                };
            }
        }
    }
}

fn gen_get_device_queue(_cmd: &Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    let doc = create_doc(_cmd, providers);
    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    token_stream.extend(quote! {
        #cfg
        #[inline]
        pub fn vkGetDeviceQueue<'dev>(
            &'dev self,
            queueFamilyIndex: u32,
            queueIndex: u32,
        ) -> crate::queue::Queue<'dev> {
            let mut raw = VkQueue::NULL;
            let fp = unsafe { self.table.vkGetDeviceQueue.unwrap_unchecked() };
            unsafe { fp(self.raw, queueFamilyIndex, queueIndex, &mut raw) };
            crate::queue::Queue { raw, parent: self, table: &self.queue_table }
        }
    });
    token_stream
}

fn gen_create_command_pool(cmd: &Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    let doc = create_doc(cmd, providers);
    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    token_stream.extend(quote! {
        #cfg
        #[inline]
        pub fn vkCreateCommandPool<'dev>(
            &'dev self,
            pCreateInfo: &VkCommandPoolCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
        ) -> Result<crate::command_pool::CommandPool<'dev>, VkResult> {
            let mut raw = VkCommandPool::NULL;
            let fp = unsafe { self.table.vkCreateCommandPool.unwrap_unchecked() };
            let r = unsafe { fp(self.raw, pCreateInfo, pAllocator, &mut raw) };
            if r >= VkResult::VK_SUCCESS {
                Ok(crate::command_pool::CommandPool { raw, parent: self, table: &self.command_pool_table })
            } else {
                Err(r)
            }
        }
    });
    token_stream
}

fn gen_create_pipelines(cmd: &Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", cmd.name);
    let doc = create_doc(cmd, providers);
    let sig_params: Vec<_> = strip_first_param(&cmd.params)
        .iter()
        .filter(|m| m.ty.base != "VkPipeline")
        .cloned()
        .collect();
    let (p_defs, p_fwd) = params_to_tokens(&sig_params);

    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    token_stream.extend(quote! {
        #cfg
        #[inline]
        pub fn #fname<'dev>(
            &'dev self,
            #(#p_defs,)*
        ) -> Result<alloc::boxed::Box<[crate::pipeline::Pipeline<'dev>]>, VkResult> {
            let count = createInfoCount;
            let mut raw_pipelines = alloc::boxed::Box::<[VkPipeline]>::new_uninit_slice(count as usize);
            let fp = unsafe { self.table.#fname.unwrap_unchecked() };
            let r = unsafe { fp(self.raw, #(#p_fwd,)* raw_pipelines.as_mut_ptr().cast()) };
            if r < VkResult::VK_SUCCESS { return Err(r); }
            let raw_pipelines = unsafe { raw_pipelines.assume_init() };

            Ok(raw_pipelines.into_iter().map(|raw| crate::pipeline::Pipeline {
                raw,
                parent: self,
                table: &self.pipeline_table
            }).collect())
        }
    });
    token_stream
}
