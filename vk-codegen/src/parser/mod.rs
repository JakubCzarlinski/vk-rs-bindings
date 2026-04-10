//! Registry API and orchestration

pub mod commands;
pub mod enums;
pub mod features;
pub mod nodes;
pub mod types;

use crate::ir::Registry;
use crate::parser::commands::parse_commands_block;
use crate::parser::enums::parse_enums_block;
use crate::parser::features::parse_extensions;
use crate::parser::features::parse_feature;
use crate::parser::types::parse_types;
use roxmltree::Document;

/// Parses a registry from an XML string.
///
/// Arguments:
/// - `xml`: The XML input to parse.
///
/// Returns:
/// - `Registry`: The fully populated registry.
#[must_use] 
pub fn parse_registry(xml: &str) -> Registry {
    let doc = Document::parse(xml).expect("valid XML");
    let root = doc.root_element();
    let mut reg = Registry::default();
    for child in root.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "types" => parse_types(child, &mut reg),
            "enums" => parse_enums_block(child, &mut reg),
            "commands" => parse_commands_block(child, &mut reg),
            "feature" => parse_feature(child, &mut reg),
            "extensions" => parse_extensions(child, &mut reg),
            _ => {}
        }
    }
    reg
}

/// Merges an XML-based registry definition into an existing one.
///
/// Arguments:
/// - `base`: The base Registry to merge into.
/// - `xml`: The XML representing additional definitions.
pub fn merge_registry(base: &mut Registry, xml: &str) {
    let sec = parse_registry(xml);
    for (k, v) in sec.typedefs {
        base.typedefs.entry(k).or_insert(v);
    }
    for (k, v) in sec.structs {
        base.structs.entry(k).or_insert(v);
    }
    for (k, v) in sec.enums {
        base.enums.entry(k).or_insert(v);
    }
    for (k, v) in sec.commands {
        base.commands.entry(k).or_insert(v);
    }
    for (k, v) in sec.constants {
        base.constants.entry(k).or_insert(v);
    }
    base.features.extend(sec.features);
    base.extensions.extend(sec.extensions);
}

pub use crate::parser::features::apply_require_extensions;
