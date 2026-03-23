use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::pretty;
use crate::codegen::utils::*;
use crate::ir::{Command, Member, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

pub fn gen_instance_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_instance_dispatch_table(reg));
    ts.extend(gen_instance(reg, result_cfgs, handle_types));
    pretty(ts)
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
        #[derive(Clone)]
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
) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Instance, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if name == "vkCreateDevice" {
                methods_ts.extend(gen_create_device(cmd, providers, result_cfgs));
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
                ));
            }
        }
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
            _lib: core::marker::PhantomData<&'lib VulkanLib>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'lib> Instance<'lib> {
            /// Wrap a raw `VkInstance` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkInstance` for `'lib`.
            #[inline]
            pub unsafe fn from_raw(raw: VkInstance, table: InstanceDispatchTable) -> Self {
                Self { raw, table, _lib: core::marker::PhantomData }
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
                    unsafe { destroy(self.raw, std::ptr::null()) };
                }
            }
        }
    }
}

// vkCreateDevice - special-cased to return Device<'inst>
fn gen_create_device(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);

    // Strip the VkDevice out-param (returned as Device) and build sig from the rest.
    let sig_params: Vec<&Member> = cmd
        .params
        .iter()
        .filter(|m| !(m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base == "VkDevice"))
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
        /// Create a logical device.
        ///
        /// On success returns a [`Device`] whose lifetime is tied to this
        /// `Instance`.  The device dispatch table is loaded immediately via
        /// `vkGetDeviceProcAddr`, bypassing the loader for direct driver ptrs.
        ///
        /// # Safety
        /// `physicalDevice` must have been enumerated from this instance.
        /// All pointer parameters must satisfy Vulkan validity rules.
        #[inline]
        pub fn vkCreateDevice(
            &self,
            #(#p_defs,)*
        ) -> Result<crate::device::Device<'_>, VkResult> {
            use crate::device::{Device, DeviceDispatchTable};
            let fp  = unsafe { self.table.vkCreateDevice.unwrap_unchecked() };
            let mut raw = VkDevice::NULL;
            let r = unsafe { fp(#(#p_fwd,)* &mut raw) };
            if let Err(e) = { #result_check } { return Err(e); }
            let gdpa  = unsafe { self.table.vkGetDeviceProcAddr.unwrap_unchecked() };
            let table = DeviceDispatchTable::load(|name| unsafe { gdpa(raw, name) });
            Ok(unsafe { Device::from_raw(raw, table) })
        }
    }
}
