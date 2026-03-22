use crate::cfggen::cfg_any;
use crate::codegen::pretty;
use crate::ir::Registry;
use proc_macro2::TokenStream;
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

    let enabled_features: std::collections::HashSet<String> = reg
        .features
        .iter()
        .map(|f| f.name.clone())
        .chain(
            reg.extensions
                .iter()
                .filter(|e| !e.is_disabled())
                .map(|e| e.name.clone()),
        )
        .collect();

    // Map primary feature name -> Vec<(command_name, all_providing_features)>
    let mut ext_groups: BTreeMap<String, Vec<(String, Vec<String>)>> = BTreeMap::new();

    for (name, cmds) in &reg.commands {
        if !filter(name) {
            continue;
        }

        let mut all_features: Vec<String> =
            cmds.iter().flat_map(|c| c.provided_by.clone()).collect();
        all_features.sort();
        all_features.dedup();

        // Skip commands that have no enabled providers.
        if all_features.is_empty() {
            continue;
        }

        // Grouping: Prefer core-ish versions, then any enabled feature.
        let primary = all_features
            .iter()
            .find(|f| {
                f.starts_with("VK_BASE_VERSION_")
                    || f.starts_with("VK_VERSION_")
                    || f.starts_with("VK_EXT_")
                    || f.starts_with("VK_KHR_")
            })
            .or_else(|| all_features.iter().find(|f| enabled_features.contains(*f)))
            .cloned()
            .unwrap_or_else(|| all_features[0].clone());

        ext_groups
            .entry(primary)
            .or_default()
            .push((name.clone(), all_features));
    }

    for (ext_name, cmds) in ext_groups {
        let ext_doc = format!(" Commands from {}", ext_name);
        fields_ts.extend(quote! { #[doc = #ext_doc] });

        for (name, features) in cmds {
            let cfg = cfg_any(&features);
            // Preserve Vulkan naming (minus 'vk' prefix).
            let fname = format_ident!("{}", name);
            let pfn = format_ident!("PFN_{}", &name);
            let clit = format!("c\"{}\"", name);
            let clit_ts: TokenStream = clit.parse().unwrap();

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! {  #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg {
                    let raw = loader(#clit_ts.as_ptr());
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
