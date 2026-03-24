use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::pretty;
use crate::codegen::utils::*;
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

pub fn gen_physical_device_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_physical_device_dispatch_table(reg));
    ts.extend(gen_physical_device(reg, result_cfgs, handle_types));
    pretty(ts)
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
                #cfg { table.#fname = loader(#clit.as_ptr()).map(|f| unsafe { core::mem::transmute(f) }); }
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
            pub fn load<F>(mut loader: F) -> Self where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }
        }
    }
}

fn gen_physical_device(reg: &Registry, result_cfgs: &HashMap<String, TokenStream>, handle_types: &HashSet<String>) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::PhysicalDevice, &skip, &enabled);
    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            methods_ts.extend(safe_method(cmd, name, providers, "VkPhysicalDevice", quote!{ self.raw }, quote!{ &self.instance.physical_device_table }, result_cfgs, handle_types));
        }
    }
    quote! {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct PhysicalDevice<'inst> {
            pub(crate) raw: VkPhysicalDevice,
            pub(crate) instance: &'inst Instance<'inst>,
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'inst> PhysicalDevice<'inst> {
            #[inline] pub fn raw(&self) -> VkPhysicalDevice { self.raw }
            #[inline] pub fn instance(&self) -> &Instance<'inst> { self.instance }
            #methods_ts
        }
    }
}