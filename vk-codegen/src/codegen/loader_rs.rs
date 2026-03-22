use crate::cfggen::cfg_any;
use crate::codegen::{feature_key, pretty};
use crate::ir::Registry;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub fn gen_loader_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan dispatch tables.
        //!
        //! `InstanceDispatchTable` and `DeviceDispatchTable` hold `Option<PFN_*>` for
        //! every Vulkan command.  Populate them via a `vkGetXxxProcAddr` closure.
        //!
        //! For runtime extension detection, use the `VK_*_EXTENSION_NAME` string
        //! constants in `crate::consts` and compare against the names returned by
        //! `vkEnumerateXxxExtensionProperties`.
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::commands::*;
        #[allow(unused_imports)] use crate::types::*;
    });

    ts.extend(gen_dispatch_table(reg, "Instance", is_instance_cmd));
    ts.extend(gen_dispatch_table(reg, "Device", |n| !is_instance_cmd(n)));

    pretty(ts)
}

fn gen_dispatch_table<F: Fn(&str) -> bool>(reg: &Registry, kind: &str, filter: F) -> TokenStream {
    let tname = format_ident!("{}DispatchTable", kind);
    let doc = format!(
        " Dispatch table for Vulkan {k} commands.",
        k = kind.to_lowercase()
    );

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();

    let mut groups: BTreeMap<Vec<String>, Vec<(&String, Vec<&crate::ir::Command>)>> =
        BTreeMap::new();
    for cmd in reg.commands.values().flatten() {
        if filter(&cmd.name) && !cmd.provided_by.is_empty() {
            groups
                .entry(feature_key(&cmd.provided_by))
                .or_default()
                .push((&cmd.name, vec![cmd]));
        }
    }

    for (feat, cmd_list) in groups {
        let cfg = cfg_any(&feat);
        for (name, _cmds) in cmd_list {
            let fname = format_ident!("{}", cmd_field_name(name));
            let pfn = format_ident!("PFN_{}", name);
            let clit = Literal::byte_string(format!("{}\0", name).as_bytes());

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! {  #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg {
                    let raw = loader(#clit.as_ptr() as *const c_char);
                    if !raw.is_null() {
                        table.#fname = Some(unsafe { core::mem::transmute(raw) });
                    }
                }
            });
        }
    }

    quote! {
        #[doc = #doc]
        #[cfg(feature = "VK_VERSION_1_0")]
        #[derive(Clone)]
        pub struct #tname { #fields_ts }

        #[cfg(feature = "VK_VERSION_1_0")]
        impl #tname {
            /// All entries set to `None`.
            pub const EMPTY: Self = Self { #empty_ts };

            /// Load commands via a `vkGetXxxProcAddr`-style loader function.
            ///
            /// `loader` receives a null-terminated command name and must return
            /// a raw function pointer cast to `*const c_void`, or null if unavailable.
            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> *const c_void
            {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }
        }
    }
}

fn is_instance_cmd(name: &str) -> bool {
    matches!(
        name,
        "vkCreateInstance"
            | "vkDestroyInstance"
            | "vkEnumeratePhysicalDevices"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceVersion"
            | "vkGetInstanceProcAddr"
    ) || name.starts_with("vkGetPhysicalDevice")
        || name.starts_with("vkEnumeratePhysicalDevice")
        || name.contains("Surface")
        || name.contains("Display")
}

fn cmd_field_name(cmd: &str) -> String {
    camel_to_snake(cmd.strip_prefix("vk").unwrap_or(cmd))
}

fn camel_to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + 8);
    let mut i = 0usize;
    while i < n {
        let c = chars[i];
        let not_first = i > 0;

        if !not_first {
            out.push(c.to_ascii_lowercase());
            i += 1;
            continue;
        }

        if c.is_uppercase() {
            let prev_lower = chars[i - 1].is_lowercase() || chars[i - 1].is_ascii_digit();
            let next_lower = i + 1 < n && chars[i + 1].is_lowercase();
            if prev_lower || next_lower {
                out.push('_');
            }
        }

        if (chars[i - 1].is_alphabetic() && c.is_ascii_digit())
            || (chars[i - 1].is_ascii_digit() && c.is_alphabetic())
        {
            out.push('_');
        }

        out.push(c.to_ascii_lowercase());
        i += 1;
    }
    let mut res = String::new();
    let mut prev = false;
    for c in out.chars() {
        if c == '_' {
            if !prev && !res.is_empty() {
                res.push('_');
            }
            prev = true;
        } else {
            res.push(c);
            prev = false;
        }
    }
    res.trim_matches('_').to_owned()
}
