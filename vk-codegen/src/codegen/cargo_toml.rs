use std::collections::HashSet;

use crate::ir::DepExpr;
use crate::ir::Registry;

pub fn gen_cargo_toml(reg: &Registry) -> String {
    let feature_deps = reg.feature_deps();
    // Build the complete set of known feature names so deps can be filtered
    let known_features: HashSet<String> = reg.all_feature_names().into_iter().collect();

    let filter_deps = |deps: Vec<String>| -> Vec<String> {
        deps.into_iter()
            .filter(|d| known_features.contains(d))
            .collect()
    };

    let mut lines: Vec<String> = vec![
        "[package]".into(),
        "name = \"vk-rs-bindings\"".into(),
        "version = \"0.1.1\"".into(),
        "edition = \"2024\"".into(),
        "license = \"MIT\"".into(),
        "repository = \"https://github.com/JakubCzarlinski/vk-rs-bindings\"".into(),
        "homepage = \"https://github.com/JakubCzarlinski/vk-rs-bindings\"".into(),
        "documentation = \"https://docs.rs/vk-rs-bindings\"".into(),
        "authors = [\"Jakub Czarlinski <jakubczarlinski@gmail.com>\"]".into(),
        "keywords = [\"vulkan\", \"ffi\", \"bindings\", \"graphics\"]".into(),
        "categories = [\"api-bindings\", \"external-ffi-bindings\", \"graphics\"]".into(),
        "rust-version = \"1.85\"".into(),
        "description = \"Auto-generated Vulkan FFI (vk-codegen)\"".into(),
        String::new(),
        "[dependencies]".into(),
        "libloading = \"0.9.0\"\n".into(),
        "[features]".into(),
        "# Vulkan 1.0 enabled by default".into(),
        "default = [\"VK_VERSION_1_0\"]".into(),
        String::new(),
    ];

    lines.push("# Core Vulkan versions".into());
    for feat in &reg.features {
        if let Some(ref comment) = feat.comment {
            lines.push(format!("# {comment}"));
        }
        lines.push(format!("# version: {}", feat.number));
        let deps = filter_deps(feature_deps.get(&feat.name).cloned().unwrap_or_default());
        lines.push(format!("{} = [{}]", feat.name, toml_feat_list(&deps)));
    }
    lines.push(String::new());

    lines.push("# Extensions".into());
    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }

        // Cargo features can't express OR dependencies.
        // We only include dependencies that appear in EVERY valid clause of the DNF.
        let mut common_deps = ext
            .depends
            .as_ref()
            .map(DepExpr::common_dependencies)
            .unwrap_or_default();

        // Filter out any dependencies that aren't actually known features
        common_deps.retain(|d| known_features.contains(d));

        if let Some(ref s) = ext.depr.superseded_by {
            lines.push(format!("# superseded by: {s}"));
        }
        if let Some(ref s) = ext.depr.obsoleted_by {
            lines.push(format!("# obsoleted by: {s}"));
        }
        if let Some(ref s) = ext.depr.promoted_to {
            lines.push(format!("# promoted to: {s}"));
        }
        if let Some(ref s) = ext.depr.deprecated {
            lines.push(format!("# deprecated: {s}"));
        }
        if let Some(ref s) = ext.requires_core {
            lines.push(format!("# requires core: {s}"));
        }
        if let Some(ref s) = ext.ext_type {
            lines.push(format!("# type: {s}"));
        }
        if let Some(ref s) = ext.comment {
            lines.push(format!("# {s}"));
        }
        lines.push(format!("{} = [{}]", ext.name, toml_feat_list(&common_deps)));
    }

    lines.join("\n")
}

fn toml_feat_list(deps: &[String]) -> String {
    deps.iter()
        .map(|d| format!("\"{d}\""))
        .collect::<Vec<_>>()
        .join(", ")
}
