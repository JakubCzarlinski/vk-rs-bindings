use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, enabled_set, is_cmd_buf_cmd, safe_method,
    safe_method_unit_with_overrides,
};
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

pub fn gen_device_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
    handle_meta: &std::collections::BTreeMap<String, crate::codegen::handles_rs::HandleMeta>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_device_dispatch_table(reg));
    ts.extend(gen_device(reg, result_cfgs, handle_types, handle_meta));
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
    let mut load_ts = TokenStream::new();

    for cmds in groups.values() {
        for (name, providers, _cmd) in cmds {
            let cfg = cfg_any(providers);
            let fname = format_ident!("{}", name);
            let pfn = format_ident!("PFN_{}", name);
            let clit = c_str_lit(name);

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! { #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg {
                    table.#fname = loader(#clit.as_ptr())
                        .map(|f| unsafe { core::mem::transmute(f) });
                }
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
            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            /// Resolve all device commands via `vkGetDeviceProcAddr(device, …)`.
            pub fn load_for_device<F>(device: VkDevice, mut get_proc: F) -> Self
            where F: FnMut(VkDevice, *const c_char) -> Option<unsafe extern "system" fn()> {
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
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
    handle_meta: &std::collections::BTreeMap<String, crate::codegen::handles_rs::HandleMeta>,
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
                    result_cfgs,
                    handle_types,
                    Some(handle_meta),
                    quote! { self },
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
                methods_ts.extend(gen_create_command_pool(cmd, providers, result_cfgs));
            } else if name.starts_with("vkCreate") && name.ends_with("Pipelines") {
                methods_ts.extend(gen_create_pipelines(cmd, providers, result_cfgs));
            } else {
                methods_ts.extend(safe_method(
                    cmd,
                    name,
                    providers,
                    "VkDevice", // strip param[0] = VkDevice
                    quote! { self.raw },
                    quote! { &self.table },
                    result_cfgs,
                    handle_types,
                    Some(handle_meta),
                    quote! { self },
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
            pub(crate) table: DeviceDispatchTable,
            #handle_fields
            _inst: core::marker::PhantomData<&'inst Instance<'inst>>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> Device<'inst> {
            /// Wrap a raw `VkDevice` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkDevice` for `'inst`.
            #[inline]
            pub unsafe fn from_raw(
                raw: VkDevice,
                table: DeviceDispatchTable,
                #handle_args
            ) -> Self {
                Self { raw, table, #handle_init _inst: core::marker::PhantomData }
            }

            /// The raw `VkDevice` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkDevice { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &DeviceDispatchTable { &self.table }

            #methods_ts
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> Drop for Device<'inst> {
            fn drop(&mut self) {
                if self.raw.0.is_null() {
                    return;
                } else if let Some(destroy) = self.table.vkDestroyDevice {
                    unsafe { destroy(self.raw, core::ptr::null()) };
                }
            }
        }
    }
}

fn gen_get_device_queue(_cmd: &crate::ir::Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    quote! {
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
    }
}

fn gen_create_command_pool(
    cmd: &crate::ir::Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check =
        crate::codegen::utils::result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);
    quote! {
        #cfg
        #[inline]
        pub fn vkCreateCommandPool<'dev>(
            &'dev self,
            pCreateInfo: *const VkCommandPoolCreateInfo,
            pAllocator: *const VkAllocationCallbacks,
        ) -> Result<crate::command_pool::CommandPool<'dev>, VkResult> {
            let mut raw = VkCommandPool::NULL;
            let fp = unsafe { self.table.vkCreateCommandPool.unwrap_unchecked() };
            let r = unsafe { fp(self.raw, pCreateInfo, pAllocator, &mut raw) };
            if let Err(e) = { #result_check } { return Err(e); }
            Ok(crate::command_pool::CommandPool { raw, parent: self, table: &self.command_pool_table })
        }
    }
}

fn gen_create_pipelines(
    cmd: &crate::ir::Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    use crate::codegen::utils::{ctype_to_tokens, kw_escape, strip_first_param};
    let cfg = cfg_any(providers);
    let result_check =
        crate::codegen::utils::result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);
    let fname = format_ident!("{}", cmd.name);
    let sig_params: Vec<_> = strip_first_param(&cmd.params)
        .iter()
        .filter(|m| m.ty.base != "VkPipeline")
        .cloned()
        .collect();
    let (p_defs, p_fwd): (Vec<_>, Vec<_>) = sig_params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        })
        .unzip();

    quote! {
        #cfg
        #[inline]
        pub fn #fname<'dev>(
            &'dev self,
            #(#p_defs,)*
        ) -> Result<alloc::vec::Vec<crate::pipeline::Pipeline<'dev>>, VkResult> {
            let count = createInfoCount;
            let mut raw_pipelines = alloc::vec::Vec::with_capacity(count as usize);
            let fp = unsafe { self.table.#fname.unwrap_unchecked() };
            let r = unsafe { fp(self.raw, #(#p_fwd,)* raw_pipelines.as_mut_ptr()) };
            if let Err(e) = { #result_check } { return Err(e); }
            unsafe { raw_pipelines.set_len(count as usize); }

            Ok(raw_pipelines.into_iter().map(|raw| crate::pipeline::Pipeline {
                raw,
                parent: self,
                table: &self.pipeline_table
            }).collect())
        }
    }
}
