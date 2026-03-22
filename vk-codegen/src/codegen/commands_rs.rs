use crate::cfggen::cfg_any;
use crate::codegen::{deprecate_attr, pretty, refpage_url, sanitize_ident};
use crate::ir::{Command, Registry};
use crate::types::ctype_to_rust_str;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub fn gen_commands_rs(reg: &Registry) -> String {
    let mut groups: BTreeMap<String, Vec<&Command>> = BTreeMap::new();
    for cmd in reg.commands.values().flatten() {
        if cmd.provided_by.is_empty() {
            continue;
        }
        groups.entry(cmd.name.clone()).or_default().push(cmd);
    }

    let mut token_stream = TokenStream::new();
    token_stream.extend(quote! {
        //! Vulkan command function pointer types (`PFN_vk*`).
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::types::*;
        #[allow(unused_imports)] use crate::enums::*;
    });

    for (_name, cmds) in groups {
        // Merge all provided_by features
        let mut all_features = Vec::new();
        for cmd in &cmds {
            all_features.extend(cmd.provided_by.clone());
        }
        all_features.sort();
        all_features.dedup();

        // Use the first command for the definition (assuming signatures match)
        let cmd = cmds[0];
        let cfg = cfg_any(&all_features);
        let pfn = format_ident!("PFN_{}", &cmd.name);
        let url = refpage_url(&cmd.name);
        let provided: String = all_features
            .iter()
            .map(|f| format!(" - `{f}`"))
            .collect::<Vec<_>>()
            .join("\n");
        let doc = format!(" [`{n}`]({url})\n\n Provided by:\n{provided}", n = cmd.name);
        let depr = deprecate_attr(&cmd.depr);

        if let Some(ref alias) = cmd.alias {
            let a = format_ident!("PFN_{}", alias);
            token_stream.extend(quote! { #[doc = #doc] #cfg #depr pub type #pfn = #a; });
            continue;
        }

        let ret_s = ctype_to_rust_str(&cmd.return_type);
        let ret_ts: TokenStream = if ret_s == "core::ffi::c_void" || ret_s == "void" {
            quote! {}
        } else {
            let r: TokenStream = ret_s.parse().unwrap_or_else(|_| quote! {()});
            quote! { -> #r }
        };

        let mut params = TokenStream::new();
        for p in &cmd.params {
            let pname = format_ident!("{}", sanitize_ident(&p.name));
            let pty: TokenStream = ctype_to_rust_str(&p.ty)
                .parse()
                .unwrap_or_else(|_| quote! { *mut core::ffi::c_void });
            params.extend(quote! { #pname: #pty, });
        }

        token_stream.extend(quote! {
            #[doc = #doc]
            #cfg #depr
            pub type #pfn = unsafe extern "system" fn(#params) #ret_ts;
        });
    }
    pretty(token_stream)
}
