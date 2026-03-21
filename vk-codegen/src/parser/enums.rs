//! Parser for Vulkan enums.

use crate::ir::{ApiSet, Constant, DeprecationInfo, Enum, EnumVariant, Registry};
use crate::parser::nodes::{attr, depr_info, parse_enum_value_node};
use roxmltree::Node;

/// Parses an enums block from an XML node into the Registry.
///
/// Arguments:
///
/// - `node`: The XML node containing the enums definition.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_enums_block(node: Node, reg: &mut Registry) {
    let name = match attr(node, "name") {
        Some(n) => n.to_owned(),
        None => return,
    };
    let is_bitmask = attr(node, "type") == Some("bitmask");
    let bit_width: u8 = attr(node, "bitwidth")
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let comment = attr(node, "comment").map(str::to_owned);

    if name == "API Constants" {
        for en in node
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "enum")
        {
            let ename = attr(en, "name").unwrap_or("").to_owned();
            if ename.is_empty() {
                continue;
            }
            let alias = attr(en, "alias").map(str::to_owned);
            let value = attr(en, "value").unwrap_or("0").to_owned();
            let ty = attr(en, "type").unwrap_or("u32").to_owned();
            let depr = depr_info(en);
            reg.constants.entry(ename.clone()).or_default().push(
                Constant {
                    name: ename,
                    value,
                    ty,
                    alias,
                    comment: attr(en, "comment").map(str::to_owned),
                    provided_by: vec![],
                    depr,
                },
            );
        }
        return;
    }

    let mut entries = reg.enums.entry(name.clone()).or_default();
    if entries.is_empty() {
        entries.push(Enum {
            name: name.clone(),
            alias: None,
            variants: vec![],
            is_bitmask,
            bit_width,
            api: ApiSet::all(),
            comment: comment.clone(),
            dep: None,
            provided_by: vec![],
            depr: DeprecationInfo::default(),
        });
    }
    let mut entry = entries.last_mut().unwrap();
    entry.is_bitmask = is_bitmask;
    entry.bit_width = bit_width;
    if entry.comment.is_none() {
        entry.comment = comment;
    }

    for en in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "enum")
    {
        let vname = attr(en, "name").unwrap_or("").to_owned();
        if vname.is_empty() {
            continue;
        }
        let value = parse_enum_value_node(en, None);
        entry.variants.push(EnumVariant {
            name: vname,
            value,
            alias: attr(en, "alias").map(str::to_owned),
            comment: attr(en, "comment").map(str::to_owned),
            api: attr(en, "api").map(ApiSet::parse),
            depr: depr_info(en),
            provided_by: vec![],
        });
    }
}
