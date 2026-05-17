use crate::cfggen::cfg_availability;
use crate::codegen::utils::{
    ExplicitImports, create_doc, ctype_to_tokens_for_registry, resolve_alias,
    rewrite_command_types_for_providers,
};
use crate::codegen::{deprecate_attr, pretty, sanitize_ident};
use crate::ir::{Command, Registry};
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

    let mut imports = ExplicitImports::default();
    imports.add_all_enum_names(reg);
    imports.add_all_type_names(reg);
    for cmds in groups.values() {
        for cmd in cmds {
            imports.add_ctype(reg, &cmd.return_type);
            for param in &cmd.params {
                imports.add_ctype(reg, &param.ty);
            }
        }
    }
    let imports = imports.to_tokens(reg);

    let mut token_stream = TokenStream::new();
    token_stream.extend(quote! {
        //! Vulkan command function pointer types (`PFN_vk*`).
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #imports
    });

    for (_name, cmds) in groups {
        // Merge all provided_by features
        let mut all_features = Vec::new();
        let mut availability = Vec::new();
        for cmd in &cmds {
            all_features.extend(cmd.provided_by.clone());
            availability.extend(cmd.availability.clone());
        }
        all_features.sort();
        all_features.dedup();

        // Use the first command for the definition (assuming signatures match)
        let cmd = cmds[0];
        let cfg = cfg_availability(&availability, &all_features, cmd.dep.as_ref());
        let pfn = format_ident!("PFN_{}", &cmd.name);

        let depr = deprecate_attr(&cmd.depr);

        let doc = create_doc(cmd, &all_features);
        for doc_lines in doc.lines() {
            token_stream.extend(quote! { #[doc = #doc_lines] });
        }
        let sig_cmd = if cmd.alias.is_some() {
            let mut resolved = resolve_alias(cmd, reg);
            resolved.provided_by = all_features.clone();
            resolved.availability = availability.clone();
            rewrite_command_types_for_providers(&mut resolved, reg, &all_features);
            resolved
        } else {
            cmd.clone()
        };

        let ret_ts: TokenStream =
            if sig_cmd.return_type.base == "void" || sig_cmd.return_type.base.is_empty() {
                quote! {}
            } else {
                let r = ctype_to_tokens_for_registry(&sig_cmd.return_type, reg, quote! { '_ });
                quote! { -> #r }
            };

        let mut params = TokenStream::new();
        for p in &sig_cmd.params {
            let pname = format_ident!("{}", sanitize_ident(&p.name));
            let pty: TokenStream = ctype_to_tokens_for_registry(&p.ty, reg, quote! { '_ });
            params.extend(quote! { #pname: #pty, });
        }

        token_stream.extend(quote! {
            #cfg #depr
            pub type #pfn = unsafe extern "system" fn(#params) #ret_ts;
        });
    }
    pretty(&token_stream)
}
