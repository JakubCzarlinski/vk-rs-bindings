use crate::cfggen::cfg_any;
use crate::codegen::{depr_attr, feat_key, pretty, refpage_url, sanitize_ident};
use crate::ir::{Command, Registry};
use crate::types::ctype_to_rust_str;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::BTreeMap;

pub fn gen_commands_rs(reg: &Registry) -> String {
    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();
    for cmd in reg.commands.values() {
        let ts = gen_command(cmd);
        if !ts.is_empty() {
            groups
                .entry(feat_key(&cmd.provided_by))
                .or_default()
                .extend(ts);
        }
    }
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan command function pointer types (`PFN_vk*`).
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::types::*;
        #[allow(unused_imports)] use crate::enums::*;
    });
    for (_, items) in groups {
        ts.extend(items);
    }
    pretty(ts)
}

fn gen_command(cmd: &Command) -> TokenStream {
    if cmd.provided_by.is_empty() {
        return quote! {};
    }
    let cfg = cfg_any(&cmd.provided_by);
    let pfn = format_ident!("PFN_{}", &cmd.name);
    let url = refpage_url(&cmd.name);
    let provided: String = cmd
        .provided_by
        .iter()
        .map(|f| format!(" - `{f}`"))
        .collect::<Vec<_>>()
        .join("\n");
    let doc = format!(" [`{n}`]({url})\n\n Provided by:\n{provided}", n = cmd.name);
    let depr = depr_attr(&cmd.depr);

    if let Some(ref alias) = cmd.alias {
        let a = format_ident!("PFN_{}", alias);
        return quote! { #[doc = #doc] #cfg #depr pub type #pfn = #a; };
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

    quote! {
        #[doc = #doc]
        #cfg #depr
        pub type #pfn = unsafe extern "system" fn(#params) #ret_ts;
    }
}
