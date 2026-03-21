use crate::cfggen;
use crate::codegen::{pretty, refpage_url};
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen_validation_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Compile-time feature dependency validation.
        //!
        //! If you enable an extension feature without its required dependencies,
        //! a `compile_error!` will fire.  When using Cargo feature implication
        //! these checks are redundant but help with manual `cfg` usage.
    });

    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }
        let clauses = match &ext.depends {
            Some(d) => d.to_dnf_clauses(),
            None => continue,
        };
        if clauses.is_empty() {
            continue;
        }

        let feat = ext.name.as_str();
        let url = refpage_url(feat);
        let dnf_cfg = cfggen::cfg_expr_from_dnf(&clauses);

        let depends_desc = clauses
            .iter()
            .map(|c| c.join(" + "))
            .collect::<Vec<_>>()
            .join(" , ");

        let msg = format!(
            "Feature `{feat}` requires `{depends_desc}`.\nAdd the required features to Cargo.toml.\nSpec: {url}"
        );

        ts.extend(quote! {
            #[cfg(all(feature = #feat, not(#dnf_cfg)))]
            compile_error!(#msg);
        });
    }
    pretty(ts)
}
