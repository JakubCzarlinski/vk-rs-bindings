//! Code generator: IR -> Rust source files for `vk-sys`.
mod cargo_toml;
mod commands_rs;
mod consts_rs;
mod dot;
mod enums_rs;
mod lib_rs;
mod loader_rs;
mod types_rs;
mod validation_rs;

use crate::codegen::cargo_toml::gen_cargo_toml;
use crate::codegen::commands_rs::gen_commands_rs;
use crate::codegen::consts_rs::gen_consts_rs;
use crate::codegen::dot::gen_dot_graph;
use crate::codegen::enums_rs::gen_enums_rs;
use crate::codegen::lib_rs::gen_lib_rs;
use crate::codegen::loader_rs::gen_loader_rs;
use crate::codegen::types_rs::gen_types_rs;
use crate::codegen::validation_rs::gen_validation_rs;
use crate::ir::{DeprecationInfo, Registry};
use proc_macro2::TokenStream;
use quote::quote;

// -- Public API ----------------------------------------------------------------

pub struct GeneratedFiles {
    pub cargo_toml: String,
    pub lib_rs: String,
    pub types_rs: String,
    pub enums_rs: String,
    pub consts_rs: String,
    pub commands_rs: String,
    pub loader_rs: String,
    pub validation_rs: String,
    pub dot_graph: String,
}

pub fn generate(reg: &Registry) -> GeneratedFiles {
    GeneratedFiles {
        cargo_toml: gen_cargo_toml(reg),
        lib_rs: gen_lib_rs(),
        types_rs: gen_types_rs(reg),
        enums_rs: gen_enums_rs(reg),
        consts_rs: gen_consts_rs(reg),
        commands_rs: gen_commands_rs(reg),
        loader_rs: gen_loader_rs(reg),
        validation_rs: gen_validation_rs(reg),
        dot_graph: gen_dot_graph(reg),
    }
}

/// Canonical key for grouping items by their cfg gate.
/// Items with identical sorted provided_by sets share a group.
fn feature_key(provided_by: &[String]) -> Vec<String> {
    let mut v = provided_by.to_vec();
    v.sort();
    v
}

fn sanitize_ident(s: &str) -> &str {
    match s {
        "type" => "type_",
        "match" => "match_",
        other => other,
    }
}

// -- URL helpers ---------------------------------------------------------------

/// Vulkan refpage URL for a named symbol.
/// Format: https://docs.vulkan.org/refpages/latest/refpages/source/<name>.html
pub fn refpage_url(name: &str) -> String {
    format!("https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html")
}

// -- Token helpers -------------------------------------------------------------

fn deprecate_attr(d: &DeprecationInfo) -> TokenStream {
    if !d.is_any() {
        return quote! {};
    }
    let note = d.note();
    quote! { #[deprecated(note = #note)] }
}

fn pretty(ts: TokenStream) -> String {
    match syn::parse2::<syn::File>(ts.clone()) {
        Ok(f) => prettyplease::unparse(&f),
        Err(e) => {
            eprintln!("vk-codegen: prettyplease error: {e}");
            format!("// PARSE ERROR: {e}\n{ts}")
        }
    }
}
