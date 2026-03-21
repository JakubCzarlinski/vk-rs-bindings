//! Parser for Vulkan commands.

use crate::ir::{CType, Command, Registry, parse_dep_expr};
use crate::parser::nodes::{api_set, attr, child_name, depr_info, parse_c_type, parse_member};
use roxmltree::Node;

/// Parses a commands block from an XML node into the Registry.
///
/// Arguments:
///
/// - `node`: The XML node containing the commands block.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_commands_block(node: Node, reg: &mut Registry) {
    for cn in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "command")
    {
        let alias = attr(cn, "alias").map(str::to_owned);
        let name_attr = attr(cn, "name").map(str::to_owned);
        let aset = api_set(cn);
        let dep = attr(cn, "depends").map(parse_dep_expr);
        let depr = depr_info(cn);
        let comment = attr(cn, "comment").map(str::to_owned);

        if alias.is_some() {
            if let Some(name) = name_attr {
                reg.commands.insert(
                    name.clone(),
                    Command {
                        name,
                        alias,
                        return_type: CType::simple("VkResult"),
                        params: vec![],
                        api: aset,
                        comment,
                        dep,
                        provided_by: vec![],
                        depr,
                        success_codes: vec![],
                        error_codes: vec![],
                    },
                );
            }
            continue;
        }

        let proto = cn
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "proto");
        let (name, return_type) = if let Some(proto) = proto {
            (child_name(proto).unwrap_or_default(), parse_c_type(proto))
        } else {
            (name_attr.unwrap_or_default(), CType::simple("void"))
        };
        if name.is_empty() {
            continue;
        }

        let params = cn
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "param")
            .map(parse_member)
            .collect();
        reg.commands.insert(
            name.clone(),
            Command {
                name,
                alias,
                return_type,
                params,
                api: aset,
                comment,
                dep,
                provided_by: vec![],
                depr,
                success_codes: attr(cn, "successcodes")
                    .map(|s| s.split(',').map(str::to_owned).collect())
                    .unwrap_or_default(),
                error_codes: attr(cn, "errorcodes")
                    .map(|s| s.split(',').map(str::to_owned).collect())
                    .unwrap_or_default(),
            },
        );
    }
}
