use crate::ir::Registry;

pub fn gen_dot_graph(reg: &Registry) -> String {
    let feature_deps = reg.feature_deps();
    let mut out = String::from("digraph vk_features {\n  rankdir=LR;\n");
    out.push_str("  node [shape=box fontname=\"Monospace\" fontsize=9];\n\n");

    out.push_str("  subgraph cluster_core {\n    label=\"Core Versions\"; style=dashed;\n");
    for f in &reg.features {
        let id = dot_id(&f.name);
        out.push_str(&format!("    {id} [label=\"{}\"];\n", f.name));
    }
    out.push_str("  }\n\n");

    out.push_str("  subgraph cluster_ext {\n    label=\"Extensions\"; style=dashed;\n");
    for e in &reg.extensions {
        if e.is_video_header() {
            continue;
        } // shown in separate cluster below
        let id = dot_id(&e.name);
        let color = if e.is_disabled() {
            "fillcolor=lightgray style=filled"
        } else if e.depr.is_any() {
            "fillcolor=lightyellow style=filled"
        } else if e.name.contains("_KHR_") {
            "fillcolor=lightblue style=filled"
        } else if e.name.contains("_EXT_") {
            "fillcolor=lightyellow style=filled"
        } else {
            ""
        };
        out.push_str(&format!("    {id} [label=\"{}\" {color}];\n", e.name));
    }
    out.push_str("  }\n\n");

    out.push_str(
        "  subgraph cluster_video {\n    label=\"Video codec headers (remapped)\"; style=dashed;\n",
    );
    for e in &reg.extensions {
        if !e.is_video_header() {
            continue;
        }
        let id = dot_id(&e.name);
        out.push_str(&format!(
            "    {id} [label=\"{}\" fillcolor=lightgreen style=filled];\n",
            e.name
        ));
    }
    out.push_str("  }\n\n");

    // Edges for enabled non-video features
    for (feat, deps) in &feature_deps {
        let from = dot_id(feat);
        for dep in deps {
            out.push_str(&format!("  {from} -> {};\n", dot_id(dep)));
        }
    }
    // Dashed edges for disabled extensions
    for e in &reg.extensions {
        if !e.is_disabled() || e.is_video_header() {
            continue;
        }
        if let Some(ref d) = e.depends {
            let from = dot_id(&e.name);
            for dep in d.atoms() {
                out.push_str(&format!(
                    "  {from} -> {} [style=dashed color=gray];\n",
                    dot_id(&dep)
                ));
            }
        }
    }

    out.push_str("}\n");
    out
}

fn dot_id(name: &str) -> String {
    name.replace(['-', '.'], "_")
}
