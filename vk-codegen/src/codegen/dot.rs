use crate::ir::{Extension, Registry};
use std::fmt::Write;

pub fn gen_dot_graph(reg: &Registry) -> String {
    let feature_deps = reg.feature_deps();
    let mut out = String::new();

    // Graph header
    write!(
        out,
        r#"digraph vk_features {{
  rankdir=LR;
  node [shape=box fontname="Monospace" fontsize=9];

"#
    )
    .unwrap();

    // Core versions cluster
    write!(
        out,
        r#"  subgraph cluster_core {{
    label="Core Versions";
    style=dashed;
"#
    )
    .unwrap();

    for feature in &reg.features {
        let id = dot_id(&feature.name);
        writeln!(out, r#"    {id} [label="{}"];"#, feature.name).unwrap();
    }
    writeln!(out, "  }}\n").unwrap();

    // Extensions cluster
    write!(
        out,
        r#"  subgraph cluster_ext {{
    label="Extensions";
    style=dashed;
"#
    )
    .unwrap();

    for ext in &reg.extensions {
        if ext.is_video_header() {
            continue;
        }

        let id = dot_id(&ext.name);
        let style = extension_style(ext).unwrap_or("");
        writeln!(out, r#"    {id} [label="{}" {}];"#, ext.name, style).unwrap();
    }
    writeln!(out, "  }}\n").unwrap();

    // Video extensions cluster
    write!(
        out,
        r#"  subgraph cluster_video {{
    label="Video codec headers (remapped)";
    style=dashed;
"#
    )
    .unwrap();

    for ext in &reg.extensions {
        if !ext.is_video_header() {
            continue;
        }
        let id = dot_id(&ext.name);
        writeln!(
            out,
            r#"    {id} [label="{}" fillcolor=lightgreen style=filled];"#,
            ext.name
        )
        .unwrap();
    }
    writeln!(out, "  }}\n").unwrap();

    // Feature dependency edges
    for (features, deps) in &feature_deps {
        let from = dot_id(features);
        for dep in deps {
            writeln!(out, "  {from} -> {};", dot_id(dep)).unwrap();
        }
    }

    // Disabled extension edges (dashed)
    for ext in &reg.extensions {
        if !ext.is_disabled() || ext.is_video_header() {
            continue;
        }
        if let Some(ref dep) = ext.depends {
            let from = dot_id(&ext.name);
            for dep in dep.atoms() {
                writeln!(
                    out,
                    "  {from} -> {} [style=dashed color=gray];",
                    dot_id(&dep)
                )
                .unwrap();
            }
        }
    }

    writeln!(out, "}}").unwrap();
    out
}

fn dot_id(name: &str) -> String {
    name.replace(['-', '.'], "_")
}

fn extension_style(ext: &Extension) -> Option<&'static str> {
    if ext.is_disabled() {
        Some("fillcolor=lightgray style=filled")
    } else if ext.depr.is_any() {
        Some("fillcolor=lightyellow style=filled")
    } else if ext.name.contains("_KHR_") {
        Some("fillcolor=lightblue style=filled")
    } else if ext.name.contains("_EXT_") {
        Some("fillcolor=lightyellow style=filled")
    } else {
        None
    }
}
