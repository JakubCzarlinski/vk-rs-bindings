use crate::ir::Registry;

pub fn gen_cargo_toml(reg: &Registry) -> String {
    let feature_deps = reg.feature_deps();
    // Build the complete set of known feature names so deps can be filtered
    let known_features: std::collections::HashSet<String> =
        reg.all_feature_names().into_iter().collect();

    let filter_deps = |deps: Vec<String>| -> Vec<String> {
        deps.into_iter()
            .filter(|d| known_features.contains(d))
            .collect()
    };

    let mut lines: Vec<String> = vec![
        "[package]".into(),
        "name = \"vk-rs-bindings\"".into(),
        "version = \"0.1.0\"".into(),
        "edition = \"2024\"".into(),
        "description = \"Auto-generated Vulkan FFI (vk-codegen)\"".into(),
        "".into(),
        "[dependencies]".into(),
        "".into(),
        "[features]".into(),
        "# Vulkan 1.0 enabled by default".into(),
        "default = [\"VK_VERSION_1_0\"]".into(),
        "".into(),
    ];

    lines.push("# Core Vulkan versions".into());
    for feat in &reg.features {
        let deps = filter_deps(feature_deps.get(&feat.name).cloned().unwrap_or_default());
        lines.push(format!("{} = [{}]", feat.name, toml_feat_list(&deps)));
    }
    lines.push("".into());

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
            .map(|d| d.common_dependencies())
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
