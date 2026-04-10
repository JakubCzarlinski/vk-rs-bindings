//! Parser for Vulkan features and extensions.

use crate::ir::{
    ApiSet, Constant, EnumValue, EnumVariant, Extension, Feature, Registry, Require, RequireEnum,
    TypedefKind, parse_dep_expr,
};
use crate::parser::nodes::{attr, depr_info, parse_enum_value_node};
use roxmltree::Node;

/// Parses a feature block from an XML node into the Registry.
///
/// Arguments:
///
/// - `node`: The XML node containing the feature definition.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_feature(node: Node, reg: &mut Registry) {
    let name = attr(node, "name").unwrap_or("").to_owned();
    let api = ApiSet::parse(attr(node, "api").unwrap_or("vulkan"));
    let number = attr(node, "number").unwrap_or("0").to_owned();
    let depends = attr(node, "depends").map(parse_dep_expr);
    let comment = attr(node, "comment").map(str::to_owned);

    let mut requires = Vec::new();
    for rn in node.children().filter(|n| {
        n.is_element() && (n.tag_name().name() == "require" || n.tag_name().name() == "deprecate")
    }) {
        let req = parse_require(rn, None);
        if rn.tag_name().name() == "require" {
            mark_provided_with_depr(&req, &name, &api, rn, reg);
            requires.push(req);
        } else {
            // Deprecate blocks just apply supersession info, they don't "provide" the item.
            mark_provided_with_depr(&req, "", &api, rn, reg);
        }
    }
    reg.features.push(Feature {
        name,
        api,
        number,
        depends,
        requires,
        comment,
    });
}

/// Parses extension blocks from an XML node into the Registry.
///
/// Arguments:
///
/// - `node`: The XML node containing the extensions definition.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_extensions(node: Node, reg: &mut Registry) {
    for en in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "extension")
    {
        let name = attr(en, "name").unwrap_or("").to_owned();
        if name.is_empty() {
            continue;
        }
        let number: u32 = attr(en, "number").and_then(|s| s.parse().ok()).unwrap_or(0);
        let depends = attr(en, "depends").map(parse_dep_expr);
        let api = ApiSet::parse(attr(en, "api").unwrap_or("vulkan"));
        let supported = attr(en, "supported").unwrap_or("vulkan").to_owned();

        let ext_shell = Extension {
            name: name.clone(),
            number,
            depends,
            api: api.clone(),
            supported,
            requires: vec![],
            comment: attr(en, "comment").map(str::to_owned),
            depr: depr_info(en),
            requires_core: attr(en, "requiresCore").map(str::to_owned),
            ext_type: attr(en, "type").map(str::to_owned),
        };

        let disabled = ext_shell.is_disabled();
        let mut requires = Vec::new();
        for rn in en.children().filter(|n| {
            n.is_element()
                && (n.tag_name().name() == "require" || n.tag_name().name() == "deprecate")
        }) {
            let req = parse_require(rn, Some(number));
            if !disabled {
                if rn.tag_name().name() == "require" {
                    mark_provided_with_depr(&req, &name, &api, rn, reg);
                } else {
                    mark_provided_with_depr(&req, "", &api, rn, reg);
                }
            }
            if rn.tag_name().name() == "require" {
                requires.push(req);
            }
        }
        let mut ext = ext_shell;
        ext.requires = requires;
        reg.extensions.push(ext);
    }
}

/// Parses a require block, extracting types, commands, and enums.
///
/// Arguments:
///
/// - `node`: The XML node to parse.
/// - `ext_number`: Optional extension number for offset calculations.
///
/// Returns:
///
/// - `Require`: The parsed requirement descriptor.
pub fn parse_require(node: Node, ext_number: Option<u32>) -> Require {
    let mut req = Require {
        depends: attr(node, "depends").map(parse_dep_expr),
        api: attr(node, "api").map(ApiSet::parse),
        ..Default::default()
    };
    for child in node.children().filter(roxmltree::Node::is_element) {
        match child.tag_name().name() {
            "type" => {
                if let Some(n) = attr(child, "name") {
                    req.types.push(n.to_owned());
                }
            }
            "command" => {
                if let Some(n) = attr(child, "name") {
                    req.commands.push(n.to_owned());
                }
            }
            "enum" => {
                let rname = attr(child, "name").unwrap_or("").to_owned();
                let extends = attr(child, "extends").map(str::to_owned);
                let alias = attr(child, "alias").map(str::to_owned);
                let depr = depr_info(child);
                let api = attr(child, "api").map(ApiSet::parse);
                let extnumber: Option<u32> = attr(child, "extnumber")
                    .and_then(|s| s.parse().ok())
                    .or(ext_number);
                let value = if alias.is_some() {
                    Some(EnumValue::Alias(alias.clone().unwrap()))
                } else if attr(child, "offset").is_some()
                    || attr(child, "bitpos").is_some()
                    || attr(child, "value").is_some()
                {
                    Some(parse_enum_value_node(child, extnumber))
                } else {
                    None
                };
                req.enums.push(RequireEnum {
                    extends,
                    name: rname,
                    value,
                    alias,
                    depr,
                    api,
                    extnumber,
                    comment: attr(child, "comment").map(str::to_owned),
                });
            }
            _ => {}
        }
    }
    req
}

/// Marks items in a requirement block as provided by a specific feature.
///
/// Arguments:
///
/// - `req`: The requirement descriptor.
/// - `feature_name`: The name of the feature providing these items.
/// - `feature_api`: The API set of the feature mapping to items.
/// - `raw_require`: The raw XML node for the require block.
/// - `reg`: The mutable Registry instance to update.
pub fn mark_provided_with_depr(
    req: &Require,
    feature_name: &str,
    feature_api: &ApiSet,
    raw_require: Node,
    reg: &mut Registry,
) {
    for child in raw_require.children().filter(roxmltree::Node::is_element) {
        let item_name = attr(child, "name").unwrap_or("");
        let item_depr = depr_info(child);
        match child.tag_name().name() {
            "type" => {
                let fname = feature_name.to_owned();
                if let Some(ss) = reg.structs.get_mut(item_name) {
                    for s in ss {
                        if !s.api.intersects(feature_api) {
                            continue;
                        }
                        if !fname.is_empty() {
                            s.provided_by.push(fname.clone());
                        }
                        if item_depr.superseded_by.is_some() && s.depr.superseded_by.is_none() {
                            s.depr.superseded_by = item_depr.superseded_by.clone();
                        }
                        if item_depr.deprecated.is_some() && s.depr.deprecated.is_none() {
                            s.depr.deprecated = item_depr.deprecated.clone();
                        }
                    }
                }
                if let Some(tt) = reg.typedefs.get_mut(item_name) {
                    for t in tt {
                        if !t.api.intersects(feature_api) {
                            continue;
                        }
                        if !fname.is_empty() {
                            t.provided_by.push(fname.clone());
                        }
                        if item_depr.superseded_by.is_some() && t.depr.superseded_by.is_none() {
                            t.depr.superseded_by = item_depr.superseded_by.clone();
                        }
                        if item_depr.deprecated.is_some() && t.depr.deprecated.is_none() {
                            t.depr.deprecated = item_depr.deprecated.clone();
                        }
                    }
                }
                if let Some(ee) = reg.enums.get_mut(item_name) {
                    for e in ee {
                        if !e.api.intersects(feature_api) {
                            continue;
                        }
                        if !fname.is_empty() {
                            e.provided_by.push(fname.clone());
                        }
                        if item_depr.superseded_by.is_some() && e.depr.superseded_by.is_none() {
                            e.depr.superseded_by = item_depr.superseded_by.clone();
                        }
                        if item_depr.deprecated.is_some() && e.depr.deprecated.is_none() {
                            e.depr.deprecated = item_depr.deprecated.clone();
                        }
                    }
                }
            }
            "command" => {
                let fname = feature_name.to_owned();
                if let Some(cc) = reg.commands.get_mut(item_name) {
                    for c in cc {
                        if !c.api.intersects(feature_api) {
                            continue;
                        }
                        if !fname.is_empty() {
                            c.provided_by.push(fname.clone());
                        }
                        if item_depr.superseded_by.is_some() && c.depr.superseded_by.is_none() {
                            c.depr.superseded_by = item_depr.superseded_by.clone();
                        }
                        if item_depr.deprecated.is_some() && c.depr.deprecated.is_none() {
                            c.depr.deprecated = item_depr.deprecated.clone();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if feature_name.is_empty() {
        return;
    }
    for tname in &req.types {
        if let Some(ss) = reg.structs.get_mut(tname) {
            for s in ss {
                if !s.api.intersects(feature_api) {
                    continue;
                }
                if !s.provided_by.contains(&feature_name.to_owned()) {
                    s.provided_by.push(feature_name.to_owned());
                }
            }
        }
        if let Some(tt) = reg.typedefs.get_mut(tname) {
            for t in tt {
                if !t.api.intersects(feature_api) {
                    continue;
                }
                if !t.provided_by.contains(&feature_name.to_owned()) {
                    t.provided_by.push(feature_name.to_owned());
                }
            }
        }
        if let Some(ee) = reg.enums.get_mut(tname) {
            for e in ee {
                if !e.api.intersects(feature_api) {
                    continue;
                }
                if !e.provided_by.contains(&feature_name.to_owned()) {
                    e.provided_by.push(feature_name.to_owned());
                }
            }
        }
    }
    for cname in &req.commands {
        if let Some(cc) = reg.commands.get_mut(cname) {
            for c in cc {
                if !c.api.intersects(feature_api) {
                    continue;
                }
                if !c.provided_by.contains(&feature_name.to_owned()) {
                    c.provided_by.push(feature_name.to_owned());
                }
            }
        }
    }
}

/// Applies extension requirements to the registry.
///
/// Arguments:
///
/// - `reg`: The mutable Registry instance to update.
pub fn apply_require_extensions(reg: &mut Registry) {
    use std::collections::HashSet;
    let disabled: HashSet<String> = reg
        .extensions
        .iter()
        .filter(|e| e.is_disabled())
        .map(|e| e.name.clone())
        .collect();
    let mut to_add: Vec<(String, EnumVariant)> = Vec::new();
    let mut ext_consts: Vec<(String, Constant)> = Vec::new();

    for feat in &reg.features {
        collect_extend_enums(&feat.requires, &feat.name, None, &mut to_add);
        collect_extension_consts(&feat.requires, &feat.name, &mut ext_consts);
    }
    for ext in &reg.extensions {
        if ext.is_disabled() {
            continue;
        }
        collect_extend_enums(&ext.requires, &ext.name, Some(ext.number), &mut to_add);
        collect_extension_consts(&ext.requires, &ext.name, &mut ext_consts);
    }
    for (enum_name, variant) in to_add {
        if let Some(ee) = reg.enums.get_mut(&enum_name) {
            for e in ee {
                if !e.variants.iter().any(|v| v.name == variant.name) {
                    e.variants.push(variant.clone());
                }
            }
        }
    }
    for (source, constant) in ext_consts {
        let entries = reg.constants.entry(constant.name.clone()).or_default();
        if entries.is_empty() {
            entries.push(constant);
        }
        for entry in entries {
            if !entry.provided_by.contains(&source) {
                entry.provided_by.push(source.clone());
            }
        }
    }
    // Remove disabled extensions from provided_by lists.
    let scrub = |v: &mut Vec<String>| {
        v.retain(|f| !disabled.contains(f));
    };
    for td in reg.typedefs.values_mut().flatten() {
        scrub(&mut td.provided_by);
    }
    for s in reg.structs.values_mut().flatten() {
        scrub(&mut s.provided_by);
    }
    for e in reg.enums.values_mut().flatten() {
        scrub(&mut e.provided_by);
        for v in &mut e.variants {
            scrub(&mut v.provided_by);
        }
    }
    for c in reg.commands.values_mut().flatten() {
        scrub(&mut c.provided_by);
    }
    for c in reg.constants.values_mut().flatten() {
        scrub(&mut c.provided_by);
    }
    for e in reg.enums.values_mut().flatten() {
        if e.provided_by.is_empty() {
            let mut inferred: Vec<String> = Vec::new();
            for v in &e.variants {
                for f in &v.provided_by {
                    if !inferred.contains(f) {
                        inferred.push(f.clone());
                    }
                }
            }
            e.provided_by = inferred;
        }
    }
    let opaque_names: Vec<String> = reg
        .typedefs
        .iter()
        .flat_map(|(n, tds)| tds.iter().map(move |td| (n, td)))
        .filter(|(_, td)| td.provided_by.is_empty() && td.kind == TypedefKind::OpaqueExtern)
        .map(|(n, _)| n.clone())
        .collect();

    for tname in &opaque_names {
        let providers: Vec<String> = reg
            .structs
            .values()
            .flatten()
            .filter(|s| !s.provided_by.is_empty())
            .filter(|s| s.members.iter().any(|m| &m.ty.base == tname))
            .flat_map(|s| &s.provided_by)
            .chain(
                reg.commands
                    .values()
                    .flatten()
                    .filter(|c| !c.provided_by.is_empty())
                    .filter(|c| c.params.iter().any(|p| &p.ty.base == tname))
                    .flat_map(|c| &c.provided_by),
            )
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        if let Some(tt) = reg.typedefs.get_mut(tname) {
            for td in tt {
                td.provided_by = providers.clone();
            }
        }
    }
}

/// Collects enums extended by features or extensions.
///
/// Arguments:
///
/// - `requires`: The list of requirements.
/// - `source`: The source name (feature or extension).
/// - `ext_number`: Optional extension number.
/// - `out`: The output list to store collected enum variants.
fn collect_extend_enums(
    requires: &[Require],
    source: &str,
    ext_number: Option<u32>,
    out: &mut Vec<(String, EnumVariant)>,
) {
    for req in requires {
        for re in &req.enums {
            let Some(ref extends) = re.extends else {
                continue;
            };
            let value = match re.value.clone() {
                Some(EnumValue::Offset {
                    extnumber,
                    offset,
                    negative,
                }) => {
                    let en = if extnumber == 1 {
                        ext_number.unwrap_or(extnumber)
                    } else {
                        extnumber
                    };
                    EnumValue::Offset {
                        extnumber: en,
                        offset,
                        negative,
                    }
                }
                Some(v) => v,
                None => re
                    .alias
                    .as_ref()
                    .map_or(EnumValue::Integer(0), |a| EnumValue::Alias(a.clone())),
            };
            out.push((
                extends.clone(),
                EnumVariant {
                    name: re.name.clone(),
                    value,
                    comment: re.comment.clone(),
                    api: re.api.clone(),
                    depr: re.depr.clone(),
                    alias: re.alias.clone(),
                    provided_by: vec![source.to_owned()],
                },
            ));
        }
    }
}

/// Collects constants defined by extensions.
///
/// Arguments:
///
/// - `requires`: The list of requirements.
/// - `source`: The source name (feature or extension).
/// - `out`: The output list to store collected constants.
fn collect_extension_consts(requires: &[Require], source: &str, out: &mut Vec<(String, Constant)>) {
    for req in requires {
        for re in &req.enums {
            if re.extends.is_some() || re.name.is_empty() {
                continue;
            }
            let Some(ref val) = re.value else { continue };
            let (rust_ty, rust_val) = match val {
                EnumValue::Integer(n) => ("u32".into(), format!("{n}")),
                EnumValue::Hex(n) => ("u32".into(), format!("0x{n:X}")),
                EnumValue::Expr(s) => {
                    let trimmed = s.trim();
                    if trimmed.starts_with('"') && trimmed.ends_with('"') {
                        ("&'static str".into(), trimmed.to_owned())
                    } else {
                        ("u32".into(), trimmed.to_owned())
                    }
                }
                _ => continue,
            };
            out.push((
                source.to_owned(),
                Constant {
                    name: re.name.clone(),
                    value: rust_val,
                    ty: rust_ty,
                    alias: None,
                    comment: re.comment.clone(),
                    provided_by: vec![source.to_owned()],
                    depr: re.depr.clone(),
                },
            ));
        }
    }
}
