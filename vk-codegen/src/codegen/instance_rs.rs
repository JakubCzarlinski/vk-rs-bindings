use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::handles_rs::HandleMeta;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, enabled_set, result_check_arms, safe_method,
    safe_method_unit_with_overrides, vk_result_is_err,
};
use crate::ir::{Command, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn gen_instance_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_instance_dispatch_table(reg));
    ts.extend(gen_instance(reg, result_cfgs, handle_types, handle_meta));
    pretty(&ts)
}

// Preamble
fn preamble() -> TokenStream {
    quote! {
        //! Instance-tier dispatch table and safe [`Instance`] wrapper.

        #![allow(
            non_snake_case,
            unused_imports,
            clippy::too_many_arguments,
            clippy::missing_safety_doc,
        )]

        use core::ptr;
        use core::ffi::{c_char, c_void};
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use crate::entry::VulkanLib;
    }
}

// InstanceDispatchTable
// Pure data: Option<PFN_*> fields + load constructors only.
fn gen_instance_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Instance, &skip, &enabled);

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
        /// Raw instance-tier function pointer table.
        ///
        /// Fields are `Option<PFN_*>`; `None` means absent at load time.
        /// Use [`Instance`] for the safe API.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Debug, Clone)]
        pub struct InstanceDispatchTable { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl InstanceDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all instance commands from the given loader closure.
            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            /// Resolve all instance commands via `vkGetInstanceProcAddr(instance, …)`.
            pub fn load_for_instance<F>(instance: VkInstance, mut get_proc: F) -> Self
            where F: FnMut(VkInstance, *const c_char) -> Option<unsafe extern "system" fn()> {
                Self::load(|name| get_proc(instance, name))
            }
        }
    }
}

// Instance<'lib>
//
// Lifetime 'lib ties back to VulkanLib via Entry.  Holds InstanceDispatchTable
// by value (no Box).  Strips VkInstance from every method signature, supplying
// it from self.raw.  vkCreateDevice is special-cased to return Device<'inst>.
fn gen_instance(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Instance, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if name == "vkEnumeratePhysicalDevices" {
                methods_ts.extend(gen_enumerate_physical_devices(cmd, providers, result_cfgs));
            } else if name == "vkDestroyInstance" {
                methods_ts.extend(safe_method_unit_with_overrides(
                    cmd,
                    name,
                    providers,
                    "VkInstance", // strip param[0] = VkInstance
                    quote! { self.raw },
                    quote! { &self.table },
                    result_cfgs,
                    handle_types,
                    Some(handle_meta),
                    quote! {},
                    quote! { self },
                    quote! { &mut self },
                    quote! {
                        if self.raw.0.is_null() {
                            return;
                        }
                    },
                    quote! {
                        self.raw = VkInstance::NULL;
                    },
                ));
            } else {
                methods_ts.extend(safe_method(
                    cmd,
                    name,
                    providers,
                    "VkInstance", // strip param[0] = VkInstance
                    quote! { self.raw },
                    quote! { &self.table },
                    result_cfgs,
                    handle_types,
                    Some(handle_meta),
                    quote! {},
                    quote! { self },
                ));
            }
        }
    }

    let mut handle_fields = TokenStream::new();
    let mut handle_args = TokenStream::new();
    let mut handle_init = TokenStream::new();
    for m in handle_meta
        .values()
        .filter(|m| m.root_vk_name == "VkInstance")
    {
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
        /// Safe `VkInstance` wrapper.
        ///
        /// Lifetime `'lib` ties this instance to the [`VulkanLib`] it came from;
        /// it cannot outlive it.
        ///
        /// Holds [`InstanceDispatchTable`] by value.
        ///
        /// # Cleanup
        /// `Instance` automatically destroys itself on `Drop` if it has not already
        /// been destroyed by the user.  The user does not need to call `vkDestroyInstance`.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Instance<'lib> {
            pub(crate) raw:   VkInstance,
            pub(crate) table: InstanceDispatchTable,
            pub(crate) physical_device_table: crate::physical_device::PhysicalDeviceDispatchTable,
            #handle_fields
            _lib: core::marker::PhantomData<&'lib VulkanLib>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'lib> Send for Instance<'lib> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'lib> Sync for Instance<'lib> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'lib> Instance<'lib> {
            /// Wrap a raw `VkInstance` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkInstance` for `'lib`.
            #[inline]
            pub unsafe fn from_raw(
                raw: VkInstance,
                table: InstanceDispatchTable,
                physical_device_table: crate::physical_device::PhysicalDeviceDispatchTable,
                #handle_args
            ) -> Self {
                Self { raw, table, physical_device_table, #handle_init _lib: core::marker::PhantomData }
            }

            /// The raw `VkInstance` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkInstance { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &InstanceDispatchTable { &self.table }

            #methods_ts
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'lib> Drop for Instance<'lib> {
            #[inline]
            fn drop(&mut self) {
                // Enusre that destroy was not already called by the user.
                if self.raw.0.is_null() {
                    return;
                } else if let Some(destroy) = self.table.vkDestroyInstance {
                    unsafe { destroy(self.raw, core::ptr::null()) };
                }
            }
        }
    }
}

fn gen_enumerate_physical_devices(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);
    let is_err = vk_result_is_err(cmd, result_cfgs);

    quote! {
        #cfg
        #[inline]
        pub fn vkEnumeratePhysicalDevices<'inst>(
            &'inst self,
        ) -> Result<alloc::vec::Vec<crate::physical_device::PhysicalDevice<'inst>>, VkResult> {
            use crate::physical_device::PhysicalDevice;
            let fp = unsafe { self.table.vkEnumeratePhysicalDevices.unwrap_unchecked() };
            let mut count = 0;
            let r = unsafe { fp(self.raw, &mut count, core::ptr::null_mut()) };
            if #is_err { return Err(r); }
            if count == 0 { return Ok(alloc::vec::Vec::new()); }
            let mut raw_gpus = alloc::vec::Vec::with_capacity(count as usize);
            let r = unsafe { fp(self.raw, &mut count, raw_gpus.as_mut_ptr()) };
            if let Err(e) = { #result_check } { return Err(e); }
            unsafe { raw_gpus.set_len(count as usize); }

            Ok(raw_gpus.into_iter().map(|raw| PhysicalDevice {
                raw,
                instance: self,
                table: &self.physical_device_table,
            }).collect())
        }
    }
}
