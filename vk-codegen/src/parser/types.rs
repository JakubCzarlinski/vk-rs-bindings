//! Parser for Vulkan types.

use crate::ir::{
    ApiSet, DepExpr, DeprecationInfo, Enum, Registry, Struct, Typedef, TypedefKind, parse_dep_expr,
};
use crate::parser::nodes::{
    api_set, attr, child_name, child_text, depr_info, parse_c_type, parse_member, text_of,
    true_or_panic,
};
use crate::types::ctype_to_rust_str;
use roxmltree::Node;

/// Parses types from an XML node into the Registry.
///
/// Arguments:
/// - `node`: The XML node containing type definitions.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_types(node: Node, reg: &mut Registry) {
    for type_node in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "type")
    {
        let category = attr(type_node, "category").unwrap_or("");
        let alias = attr(type_node, "alias").map(str::to_owned);
        let comment = attr(type_node, "comment").map(str::to_owned);
        let dep = attr(type_node, "depends").map(parse_dep_expr);
        let depr = depr_info(type_node);
        let aset = api_set(type_node);
        let name: String = attr(type_node, "name")
            .map(str::to_owned)
            .or_else(|| child_name(type_node))
            .or_else(|| {
                type_node
                    .children()
                    .find(|n| n.is_element() && n.tag_name().name() == "proto")
                    .and_then(|proto| child_name(proto))
            })
            .unwrap_or_default();

        if name.is_empty() {
            continue;
        }

        match category {
            "struct" | "union" => parse_struct_union(
                type_node,
                reg,
                name,
                alias,
                comment,
                dep,
                aset,
                category == "union",
            ),
            "handle" => parse_handle(type_node, reg, name, alias, comment, dep, aset, depr),
            "enum" => parse_enum(reg, name, alias, comment, dep, aset, depr),
            "bitmask" => parse_bitmask(type_node, reg, name, alias, comment, dep, aset, depr),
            "basetype" => parse_basetype(type_node, reg, name, alias, comment, dep, aset, depr),
            "funcpointer" => {
                parse_funcpointer(type_node, reg, name, alias, comment, dep, aset, depr);
            }
            "define" => parse_define(type_node, reg, name, alias, comment, dep, aset, depr),
            _ => {
                if alias.is_some() {
                    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
                        name,
                        alias,
                        ty: None,
                        kind: TypedefKind::Alias,
                        api: aset,
                        comment,
                        dep,
                        depr,
                        provided_by: vec![],
                    });
                } else if let Some(req) = attr(type_node, "requires")
                    && [
                        "windows.h",
                        "X11/",
                        "wayland-",
                        "xcb/",
                        "directfb.h",
                        "zircon/",
                        "ggp_c/",
                        "screen/",
                        "nvscisync.h",
                        "nvscibuf.h",
                        "ubm.h",
                        "android/",
                        "Metal/",
                        "objc/",
                    ]
                    .iter()
                    .any(|s| req.contains(s))
                {
                    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
                        name,
                        alias: None,
                        ty: None,
                        kind: TypedefKind::OpaqueExtern,
                        api: aset,
                        comment,
                        dep,
                        depr,
                        provided_by: vec![],
                    });
                }
            }
        }
    }
}

/// Parses a struct or union type definition.
#[allow(clippy::too_many_arguments)]
fn parse_struct_union(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    is_union: bool,
) {
    let members = node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "member")
        .map(parse_member)
        .collect();
    let struct_extends = attr(node, "structextends")
        .map(|s| s.split(',').map(str::to_owned).collect())
        .unwrap_or_default();
    reg.structs.entry(name.clone()).or_default().push(Struct {
        name,
        alias,
        members,
        is_union,
        returned_only: attr(node, "returnedonly").is_some_and(true_or_panic),
        required_limit_type: attr(node, "requiredlimittype").is_some_and(true_or_panic),
        struct_extends,
        api: aset,
        comment,
        dep,
        provided_by: vec![],
        depr: depr_info(node),
    });
}

/// Parses a handle type definition.
#[allow(clippy::too_many_arguments)]
fn parse_handle(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    let dispatchable = node
        .text()
        .is_none_or(|t| !t.contains("VK_DEFINE_NON_DISPATCHABLE_HANDLE"));

    let parent = node.attribute("parent").map(String::from);
    let objtypeenum = node.attribute("objtypeenum").map(String::from);

    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
        name,
        alias,
        ty: None,
        kind: TypedefKind::Handle {
            dispatchable,
            parent,
            objtypeenum,
        },
        api: aset,
        comment,
        dep,
        depr,
        provided_by: vec![],
    });
}

/// Parses an enum type definition.
fn parse_enum(
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    if reg.enums.get(&name).is_none() {
        reg.enums.entry(name.clone()).or_default().push(Enum {
            name: name.clone(),
            alias,
            variants: vec![],
            is_bitmask: false,
            bit_width: 32,
            api: aset,
            comment,
            dep,
            provided_by: vec![],
            depr,
        });
    }
}

/// Parses a bitmask type definition.
#[allow(clippy::too_many_arguments)]
fn parse_bitmask(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    let underlying = child_text(node, "type").unwrap_or_else(|| "VkFlags".into());
    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
        name,
        alias,
        ty: Some(underlying),
        kind: TypedefKind::Bitmask,
        api: aset,
        comment,
        dep,
        depr,
        provided_by: vec![],
    });
}

/// Parses a base type definition.
#[allow(clippy::too_many_arguments)]
fn parse_basetype(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    let has_type_child = child_text(node, "type").is_some();
    let (kind, ty) = if has_type_child {
        (TypedefKind::Basetype, child_text(node, "type"))
    } else {
        let stripped = text_of(node)
            .trim_start_matches("typedef")
            .replace(';', "")
            .trim()
            .to_owned();
        if stripped.starts_with("struct")
            || stripped.starts_with("#ifdef")
            || stripped.starts_with("@class")
            || stripped.is_empty()
        {
            (TypedefKind::OpaqueExtern, None)
        } else {
            (TypedefKind::Basetype, Some(stripped.trim().to_owned()))
        }
    };
    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
        name,
        alias,
        ty,
        kind,
        api: aset,
        comment,
        dep,
        depr,
        provided_by: vec![],
    });
}

/// Parses a function pointer type definition.
#[allow(clippy::too_many_arguments)]
fn parse_funcpointer(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    let ret_rust = node
        .children()
        .find(|n| n.is_element() && n.tag_name().name() == "proto")
        .map_or_else(
            || "core::ffi::c_void".into(),
            |proto| ctype_to_rust_str(&parse_c_type(proto)),
        );
    let params: Vec<String> = node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "param")
        .map(|pn| {
            format!(
                "{}:{}",
                child_name(pn).unwrap_or_default(),
                ctype_to_rust_str(&parse_c_type(pn))
            )
        })
        .collect();
    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
        name,
        alias,
        ty: Some(format!("{}|{}", ret_rust, params.join(","))),
        kind: TypedefKind::FuncPointer,
        api: aset,
        comment,
        dep,
        depr,
        provided_by: vec![],
    });
}

/// Parses a define type definition.
#[allow(clippy::too_many_arguments)]
fn parse_define(
    node: Node,
    reg: &mut Registry,
    name: String,
    alias: Option<String>,
    comment: Option<String>,
    dep: Option<DepExpr>,
    aset: ApiSet,
    depr: DeprecationInfo,
) {
    let raw = text_of(node);
    let type_child = child_text(node, "type");
    let ty_field: Option<String> = if let Some(ref tc) = type_child {
        let tail = node
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "type")
            .and_then(|n| n.tail())
            .unwrap_or("")
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_owned();
        if tail.is_empty() {
            None
        } else if tc == "VK_MAKE_VIDEO_STD_VERSION" {
            Some(format!("vkver:{tail}"))
        } else if tc == "VK_MAKE_API_VERSION" {
            Some(format!("apiconst:make_api_version({tail})"))
        } else {
            None
        }
    } else if let Some(name_str) = Some(name.as_str())
        && matches!(
            name_str,
            "VK_MAKE_VIDEO_STD_VERSION"
                | "VK_MAKE_VERSION"
                | "VK_MAKE_API_VERSION"
                | "VK_VERSION_MAJOR"
                | "VK_VERSION_MINOR"
                | "VK_VERSION_PATCH"
                | "VK_API_VERSION_VARIANT"
                | "VK_API_VERSION_MAJOR"
                | "VK_API_VERSION_MINOR"
                | "VK_API_VERSION_PATCH"
        )
    {
        match name.as_str() {
            "VK_MAKE_VIDEO_STD_VERSION" | "VK_MAKE_VERSION" => Some("fn:major,minor,patch|(major << 22) | (minor << 12) | patch".to_owned()),
            "VK_MAKE_API_VERSION" => Some("fn:variant,major,minor,patch|(variant << 29) | (major << 22) | (minor << 12) | patch".to_owned()),
            "VK_VERSION_MAJOR" => Some("fn:version|version >> 22".to_owned()),
            "VK_VERSION_MINOR" => Some("fn:version|(version >> 12) & 0x3FF".to_owned()),
            "VK_VERSION_PATCH" => Some("fn:version|version & 0xFFF".to_owned()),
            "VK_API_VERSION_VARIANT" => Some("fn:version|version >> 29".to_owned()),
            "VK_API_VERSION_MAJOR" => Some("fn:version|(version >> 22) & 0x7F".to_owned()),
            "VK_API_VERSION_MINOR" => Some("fn:version|(version >> 12) & 0x3FF".to_owned()),
            "VK_API_VERSION_PATCH" => Some("fn:version|version & 0xFFF".to_owned()),
            _ => None,
        }
    } else {
        let is_commented = raw.trim_start().starts_with("//") && !raw.contains("#define");
        if is_commented {
            None
        } else {
            let name_tail = node
                .children()
                .find(|n| {
                    n.is_element() && (n.tag_name().name() == "name" || n.tag_name().name() == "n")
                })
                .and_then(|n| n.tail())
                .unwrap_or("")
                .trim()
                .to_owned();
            if let Ok(v) = name_tail.parse::<u32>() {
                Some(format!("apiconst:{v}"))
            } else {
                None
            }
        }
    };
    reg.typedefs.entry(name.clone()).or_default().push(Typedef {
        name,
        alias,
        ty: ty_field,
        kind: TypedefKind::Define,
        api: aset,
        comment,
        dep,
        depr,
        provided_by: vec![],
    });
}
