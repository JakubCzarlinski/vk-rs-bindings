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
        let comment = cmd.comment.as_deref().unwrap_or("");
        let doc = format!(
            " [`{n}`]({url})\n\n Provided by:\n{provided}\n\n{comment}",
            n = cmd.name
        );
        let depr = deprecate_attr(&cmd.depr);

        let mut extra_doc = String::new();
        if !cmd.success_codes.is_empty() {
            extra_doc.push_str(&format!(
                "\n\n **Success Codes:**\n - {}",
                cmd.success_codes.join("\n - ")
            ));
        }
        if !cmd.error_codes.is_empty() {
            extra_doc.push_str(&format!(
                "\n\n **Error Codes:**\n - {}",
                cmd.error_codes.join("\n - ")
            ));
        }
        if !cmd.queues.is_empty() {
            let q_names: Vec<_> = cmd.queues.iter().map(|q| format!("{:?}", q)).collect();
            extra_doc.push_str(&format!("\n\n **Queues:** {}", q_names.join(", ")));
        }
        if let Some(ref rp) = cmd.render_pass {
            extra_doc.push_str(&format!("\n\n **Render Pass:** {:?}", rp));
        }
        if cmd.conditional_rendering {
            extra_doc.push_str("\n\n **Conditional Rendering:** Affected");
        }
        if cmd.allow_no_queues {
            extra_doc.push_str("\n\n **Allow No Queues:** True");
        }
        if !cmd.tasks.is_empty() {
            let t_names: Vec<_> = cmd.tasks.iter().map(|t| format!("{:?}", t)).collect();
            extra_doc.push_str(&format!("\n\n **Tasks:** {}", t_names.join(", ")));
        }
        if let Some(ref dep) = cmd.dep {
            extra_doc.push_str(&format!(
                "\n\n **Availability:** depends on `{}`",
                dep.atoms().join(" + ")
            ));
        }
        if !cmd.cmd_buffer_levels.is_empty() {
            let l_names: Vec<_> = cmd
                .cmd_buffer_levels
                .iter()
                .map(|l| format!("{:?}", l))
                .collect();
            extra_doc.push_str(&format!(
                "\n\n **Command Buffer Levels:** {}",
                l_names.join(", ")
            ));
        }
        if let Some(ref es) = cmd.extern_sync {
            extra_doc.push_str(&format!("\n\n **External Sync:** {}", es));
        }
        if !cmd.export.is_empty() {
            let e_names: Vec<_> = cmd.export.iter().map(|e| format!("{:?}", e)).collect();
            extra_doc.push_str(&format!("\n\n **Export Scopes:** {}", e_names.join(", ")));
        }
        if !cmd.export.is_empty() {
            let e_names: Vec<_> = cmd.export.iter().map(|e| format!("{:?}", e)).collect();
            extra_doc.push_str(&format!("\n\n **Export Scopes:** {}", e_names.join(", ")));
        }

        let mut param_docs = String::new();
        for p in &cmd.params {
            let mut p_meta = Vec::new();
            if p.optional != crate::ir::Optional::False {
                p_meta.push(format!("optional: {:?}", p.optional));
            }
            if let Some(ref len) = p.len {
                p_meta.push(format!("len: {}", len));
            }
            if let Some(ref vals) = p.values {
                p_meta.push(format!("values: {}", vals));
            }
            if let Some(ref ot) = p.object_type {
                p_meta.push(format!("object_type: {}", ot));
            }
            if !p_meta.is_empty() {
                param_docs.push_str(&format!("\n - `{}`: {}", p.name, p_meta.join(", ")));
            }
        }
        if !param_docs.is_empty() {
            extra_doc.push_str("\n\n **Parameter Requirements:**");
            extra_doc.push_str(&param_docs);
        }

        if let Some(ref alias) = cmd.alias {
            let a = format_ident!("PFN_{}", alias);
            let full_doc = format!("{doc}{extra_doc}");
            token_stream.extend(quote! { #[doc = #full_doc] #cfg #depr pub type #pfn = #a; });
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
            #[doc = #extra_doc]
            #cfg #depr
            pub type #pfn = unsafe extern "system" fn(#params) #ret_ts;
        });
    }
    pretty(token_stream)
}
