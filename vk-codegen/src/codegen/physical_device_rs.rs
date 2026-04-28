use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::handles_rs::HandleMeta;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, ctype_to_tokens, enabled_set, kw_escape, safe_method,
};
use crate::ir::{Command, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashSet};

pub fn gen_physical_device_rs(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_physical_device_dispatch_table(reg));
    ts.extend(gen_physical_device(reg, handle_types, handle_meta));
    pretty(&ts)
}

fn preamble() -> TokenStream {
    quote! {
        //! PhysicalDevice-tier dispatch table and safe wrapper.
        #![allow(non_snake_case, unused_imports, clippy::too_many_arguments, clippy::missing_safety_doc)]
        use core::ffi::{c_char, c_void};
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use crate::instance::Instance;
    }
}

fn gen_physical_device_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::PhysicalDevice, &skip, &enabled);
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
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Debug, Clone)]
        pub struct PhysicalDeviceDispatchTable { #fields_ts }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl PhysicalDeviceDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };
            pub fn load<F>(loader: F) -> Self where F: Fn(*const c_char) -> Option<unsafe extern "system" fn()> {
                Self {
                    #init_ts
                }
            }
        }
    }
}

fn gen_physical_device(
    reg: &Registry,
    handle_types: &HashSet<String>,
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::PhysicalDevice, &skip, &enabled);
    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if name == "vkCreateDevice" {
                methods_ts.extend(gen_create_device(cmd, providers, handle_meta));
            } else {
                methods_ts.extend(safe_method(
                    cmd,
                    name,
                    providers,
                    "VkPhysicalDevice",
                    quote! { self.raw },
                    quote! { self.table },
                    handle_types,
                    None,
                    quote! {},
                    quote! { self.instance() },
                ));
            }
        }
    }
    quote! {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct PhysicalDevice<'inst> {
            pub(crate) raw: VkPhysicalDevice,
            pub(crate) instance: &'inst Instance<'inst>,
            pub(crate) table: &'inst PhysicalDeviceDispatchTable,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'inst> Send for PhysicalDevice<'inst> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        unsafe impl<'inst> Sync for PhysicalDevice<'inst> {}

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> PhysicalDevice<'inst> {
            #[inline] pub fn raw(&self) -> VkPhysicalDevice { self.raw }
            #[inline] pub fn instance(&self) -> &Instance<'inst> { self.instance }
            #methods_ts
        }
    }
}

fn gen_create_device(
    cmd: &Command,
    providers: &[String],
    handle_meta: &BTreeMap<String, HandleMeta>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    // remove parameter [0] since it gets replaced by `self.raw`
    let sig_params: Vec<_> = crate::codegen::utils::strip_first_param(&cmd.params)
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

    let mut tb_load = TokenStream::new();
    let mut tb_args = TokenStream::new();
    for m in handle_meta.values() {
        let name = format_ident!("{}_table", m.mod_name);
        let tb = format_ident!("{}", m.table_name);
        let md = format_ident!("{}", m.mod_name);
        let cfg = cfg_any(&m.providers);
        tb_load.extend(quote! {
            #cfg
            let #name = crate::#md::#tb::load(|name| unsafe { gdpa(raw, name) });
        });
        tb_args.extend(quote! {
            #cfg
            #name,
        });
    }

    quote! {
        #cfg
        #[inline]
        pub fn vkCreateDevice(
            &self,
            #(#p_defs,)*
        ) -> Result<crate::device::Device<'inst>, VkResult> {
            use crate::device::{Device, DeviceDispatchTable};
            let fp  = unsafe { self.table.vkCreateDevice.unwrap_unchecked() };
            let mut raw = VkDevice::NULL;
            let r = unsafe { fp(self.raw, #(#p_fwd,)* &mut raw) };
            if r < VkResult::VK_SUCCESS { return Err(r); }
            let gdpa  = unsafe { self.instance.table.vkGetDeviceProcAddr.unwrap_unchecked() };
            let table = DeviceDispatchTable::load(|name| unsafe { gdpa(raw, name) });
            #tb_load
            Ok(unsafe { Device::from_raw(raw, self.instance, table, #tb_args) })
        }
    }
}
