//! Helper functions for parsing XML nodes.

use crate::ir::{ApiSet, CType, DeprecationInfo, EnumValue, Member};
use roxmltree::Node;

/// Parses an enum value node (e.g., bitpos, offset, or value) into an EnumValue.
///
/// Arguments:
///
/// - `node`: The XML node representing the enum value.
/// - `ext_number`: Optional extension number for calculating offsets.
///
/// Returns:
///
/// - `EnumValue`: The parsed enum value representation.
pub fn parse_enum_value_node(node: Node, ext_number: Option<u32>) -> EnumValue {
    if let Some(alias) = attr(node, "alias") {
        return EnumValue::Alias(alias.to_owned());
    }
    if let Some(bp) = attr(node, "bitpos") {
        return EnumValue::BitPos(bp.parse().unwrap_or(0));
    }
    if let Some(offset_str) = attr(node, "offset") {
        let offset: u32 = offset_str.parse().unwrap_or(0);
        let extnumber: u32 = attr(node, "extnumber")
            .and_then(|s| s.parse().ok())
            .or(ext_number)
            .unwrap_or(1);
        let negative = attr(node, "dir") == Some("-");
        return EnumValue::Offset {
            extnumber,
            offset,
            negative,
        };
    }
    if let Some(value) = attr(node, "value") {
        let v = value.trim();
        if v.starts_with("0x") || v.starts_with("0X") {
            let hex_part = v.trim_start_matches("0x").trim_start_matches("0X");
            if let Ok(n) = u64::from_str_radix(hex_part, 16) {
                return EnumValue::Hex(n);
            }
        }
        if v.starts_with('(') || v.starts_with('~') {
            return EnumValue::Expr(v.to_owned());
        }
        if let Ok(n) = v.trim_end_matches(['U', 'u', 'L', 'l']).parse::<i64>() {
            return EnumValue::Integer(n);
        }
        return EnumValue::Expr(v.to_owned());
    }
    EnumValue::Integer(0)
}

/// Retrieves an attribute value from a node.
///
/// Arguments:
/// - `node`: The XML node.
/// - `name`: The attribute name.
///
/// Returns:
/// - `Option<&str>`: The attribute value if present.
pub fn attr<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.attribute(name)
}

/// Retrieves the text content of a node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `String`: The trimmed text content.
pub fn text_of(node: Node) -> String {
    node.children()
        .filter(|n| n.is_text())
        .map(|n| n.text().unwrap_or(""))
        .collect::<String>()
        .trim()
        .to_owned()
}

/// Retrieves the text content of a specific child element.
///
/// Arguments:
/// - `node`: The XML node.
/// - `tag`: The child tag name.
///
/// Returns:
/// - `Option<String>`: The text content if the child exists.
pub fn child_text(node: Node, tag: &str) -> Option<String> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == tag)
        .map(text_of)
        .filter(|s| !s.is_empty())
}

/// Retrieves the name from a child node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `Option<String>`: The name if present.
pub fn child_name(node: Node) -> Option<String> {
    child_text(node, "name").or_else(|| child_text(node, "n"))
}

/// Retrieves the API set from a node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `ApiSet`: The API set.
pub fn api_set(node: Node) -> ApiSet {
    attr(node, "api")
        .map(ApiSet::parse)
        .unwrap_or_else(ApiSet::all)
}

/// Retrieves deprecation information from a node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `DeprecationInfo`: The deprecation info.
pub fn depr_info(node: Node) -> DeprecationInfo {
    DeprecationInfo {
        deprecated: attr(node, "deprecated").map(str::to_owned),
        superseded_by: attr(node, "supersededby").map(str::to_owned),
        obsoleted_by: attr(node, "obsoletedby").map(str::to_owned),
        promoted_to: attr(node, "promotedto").map(str::to_owned),
    }
}

/// Parses a member node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `Member`: The parsed member.
pub fn parse_member(node: Node) -> Member {
    Member {
        name: child_name(node).unwrap_or_default(),
        ty: parse_c_type(node),
        optional: attr(node, "optional").is_some_and(|v| v.contains("true")),
        len: attr(node, "len").map(str::to_owned),
        values: attr(node, "values").map(str::to_owned),
        api: attr(node, "api").map(ApiSet::parse),
        comment: attr(node, "comment").map(str::to_owned),
    }
}

/// Parses a C type from a node.
///
/// Arguments:
/// - `node`: The XML node.
///
/// Returns:
/// - `CType`: The parsed C type.
pub fn parse_c_type(node: Node) -> CType {
    let mut base = String::new();
    let mut is_const = false;
    let mut pointer_depth: u8 = 0;
    let mut is_array: Option<String> = None;
    let mut full_text = String::new();

    for child in node.children() {
        if child.is_text() {
            let t = child.text().unwrap_or("");
            full_text.push_str(t);
            if t.contains("const") {
                is_const = true;
            }
            pointer_depth += t.chars().filter(|&c| c == '*').count() as u8;
        } else if child.is_element() {
            match child.tag_name().name() {
                "type" => {
                    base = text_of(child);
                    full_text.push_str(&base);
                }
                "enum" => {
                    is_array = Some(text_of(child));
                }
                "name" | "n" => {}
                _ => {}
            }
        }
    }

    if base.is_empty() {
        base = full_text
            .split('*')
            .next()
            .unwrap_or("")
            .replace("const", "")
            .replace("struct", "")
            .trim()
            .to_owned();
    }

    if is_array.is_none()
        && full_text.contains('[')
        && let (Some(a), Some(b)) = (full_text.find('['), full_text.find(']'))
    {
        let sz = full_text[a + 1..b].trim().to_owned();
        if !sz.is_empty() {
            is_array = Some(sz);
        }
    }

    CType {
        is_const,
        base,
        pointer_depth,
        is_array,
    }
}
