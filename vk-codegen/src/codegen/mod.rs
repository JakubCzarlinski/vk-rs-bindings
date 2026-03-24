//! Code generator: IR -> Rust source files for `vk-rs-bindings`.
mod cargo_toml;
mod command_buffer_rs;
mod command_pool_rs;
mod commands_rs;
mod consts_rs;
mod device_rs;
mod dot;
mod entry_rs;
mod enums_rs;
mod instance_rs;
mod lib_rs;
mod physical_device_rs;
mod queue_rs;
mod types_rs;
mod utils;
mod validation_rs;

use crate::codegen::cargo_toml::gen_cargo_toml;
use crate::codegen::command_buffer_rs::gen_command_buffer_rs;
use crate::codegen::command_pool_rs::gen_command_pool_rs;
use crate::codegen::commands_rs::gen_commands_rs;
use crate::codegen::consts_rs::gen_consts_rs;
use crate::codegen::device_rs::gen_device_rs;
use crate::codegen::dot::gen_dot_graph;
use crate::codegen::entry_rs::gen_entry_rs;
use crate::codegen::enums_rs::gen_enums_rs;
use crate::codegen::instance_rs::gen_instance_rs;
use crate::codegen::lib_rs::gen_lib_rs;
use crate::codegen::physical_device_rs::gen_physical_device_rs;
use crate::codegen::queue_rs::gen_queue_rs;
use crate::codegen::types_rs::gen_types_rs;
use crate::codegen::utils::{build_handle_type_set, build_result_cfg_map};
use crate::codegen::validation_rs::gen_validation_rs;
use crate::ir::{DeprecationInfo, Registry};
use proc_macro2::TokenStream;
use quote::quote;

pub struct GeneratedFiles {
    pub cargo_toml: String,
    pub lib_rs: String,
    pub types_rs: String,
    pub enums_rs: String,
    pub consts_rs: String,
    pub commands_rs: String,
    pub validation_rs: String,
    pub entry_rs: String,
    pub instance_rs: String,
    pub physical_device_rs: String,
    pub device_rs: String,
    pub queue_rs: String,
    pub command_pool_rs: String,
    pub command_buffer_rs: String,
    pub dot_graph: String,
}

pub fn generate(reg: &Registry) -> GeneratedFiles {
    let result_cfgs = build_result_cfg_map(reg);
    let handle_types = build_handle_type_set(reg);
    GeneratedFiles {
        cargo_toml: gen_cargo_toml(reg),
        lib_rs: gen_lib_rs(),
        types_rs: gen_types_rs(reg),
        enums_rs: gen_enums_rs(reg),
        consts_rs: gen_consts_rs(reg),
        commands_rs: gen_commands_rs(reg),
        validation_rs: gen_validation_rs(reg),
        entry_rs: gen_entry_rs(reg, &result_cfgs, &handle_types),
        instance_rs: gen_instance_rs(reg, &result_cfgs, &handle_types),
        physical_device_rs: gen_physical_device_rs(reg, &result_cfgs, &handle_types),
        device_rs: gen_device_rs(reg, &result_cfgs, &handle_types),
        queue_rs: gen_queue_rs(reg, &result_cfgs, &handle_types),
        command_pool_rs: gen_command_pool_rs(reg, &result_cfgs, &handle_types),
        command_buffer_rs: gen_command_buffer_rs(reg, &result_cfgs, &handle_types),
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

/// Vulkan refpage URL for a named symbol.
/// Format: https://docs.vulkan.org/refpages/latest/refpages/source/<name>.html
pub fn refpage_url(name: &str) -> String {
    let name = name.replace("FlagBits", "Flags");
    format!("https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html")
}

// Token helpers
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
