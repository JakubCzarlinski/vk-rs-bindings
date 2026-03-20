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
        let dep_atoms: Vec<String> = ext.depends.as_ref().map(|d| d.atoms()).unwrap_or_default();
        if dep_atoms.is_empty() {
            continue;
        }

        let feat = ext.name.as_str();
        let url = refpage_url(feat);
        for dep in &dep_atoms {
            let dep = dep.as_str();
            let msg = format!(
                "Feature `{feat}` requires `{dep}`.\nAdd `\"{dep}\"` to [features] in Cargo.toml.\nSpec: {url}"
            );
            ts.extend(quote! {
                #[cfg(all(feature = #feat, not(feature = #dep)))]
                compile_error!(#msg);
            });
        }
    }
    pretty(ts)
}
