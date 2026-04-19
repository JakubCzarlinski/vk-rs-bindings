use crate::cfggen::cfg_any;
use crate::codegen::{deprecate_attr, feature_key, pretty, refpage_url};
use crate::ir::{Constant, Registry, TypedefKind};
use crate::types::const_rust_type;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashMap};

pub fn gen_consts_rs(reg: &Registry) -> String {
    // Collect items grouped by feature gate for sorted, compact output.
    // Key = sorted provided_by vec (empty = ungated).
    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();
    let mut emitted = std::collections::HashSet::new();
    let mut extension_literal_by_prefix: HashMap<String, String> = HashMap::new();
    let mut extension_name_const_by_literal: HashMap<String, String> = HashMap::new();

    for c in reg.constants.values().flatten() {
        if !c.name.ends_with("_EXTENSION_NAME") || c.ty != "&'static str" {
            continue;
        }
        let Some(prefix) = c.name.strip_suffix("_EXTENSION_NAME") else {
            continue;
        };
        let Some(extension_literal) = parse_cstr_literal(&c.value) else {
            continue;
        };
        extension_literal_by_prefix.insert(prefix.to_owned(), extension_literal.clone());
        extension_name_const_by_literal.insert(extension_literal, c.name.clone());
    }

    let extension_type_by_name: HashMap<String, String> = reg
        .extensions
        .iter()
        .filter(|ext| !ext.is_disabled() && !ext.is_video_header())
        .filter_map(|ext| {
            ext.ext_type
                .as_ref()
                .map(|kind| (ext.name.clone(), kind.clone()))
        })
        .collect();

    let mut extension_type_by_prefix: HashMap<String, String> = HashMap::new();
    for (prefix, extension_literal) in &extension_literal_by_prefix {
        if let Some(kind) = extension_type_by_name.get(extension_literal) {
            extension_type_by_prefix.insert(prefix.clone(), kind.clone());
        }
    }

    let mut enabled_instance_extensions: Vec<(String, String)> = Vec::new();
    let mut enabled_device_extensions: Vec<(String, String)> = Vec::new();
    for ext in reg
        .extensions
        .iter()
        .filter(|ext| !ext.is_disabled() && !ext.is_video_header())
    {
        let Some(kind) = ext.ext_type.as_deref() else {
            continue;
        };
        let Some(const_name) = extension_name_const_by_literal.get(&ext.name) else {
            continue;
        };
        match kind {
            "instance" => {
                enabled_instance_extensions.push((ext.name.clone(), const_name.clone()));
            }
            "device" => {
                enabled_device_extensions.push((ext.name.clone(), const_name.clone()));
            }
            _ => {}
        }
    }
    enabled_instance_extensions.sort_by(|a, b| a.0.cmp(&b.0));
    enabled_device_extensions.sort_by(|a, b| a.0.cmp(&b.0));

    // #define typedefs -> const fn or pub const
    for typedef in reg.typedefs.values().flatten() {
        if typedef.kind != TypedefKind::Define {
            continue;
        }

        // Cross-loop and intra-loop deduplication: name + set of providing features
        let key = (typedef.name.clone(), feature_key(&typedef.provided_by));
        if !emitted.insert(key) {
            continue;
        }

        let Some(ref ty) = typedef.ty else { continue };
        let name_str = &typedef.name;
        let url_str = format!(" [{}]({})", name_str, refpage_url(name_str));
        let mut doc = quote! { #[doc = #url_str] };
        if let Some(ref comment) = typedef.comment {
            let comment = comment.trim();
            if !comment.is_empty() {
                doc.extend(quote! { #[doc = " "] });
                let comment = " ".to_string() + comment;
                doc.extend(quote! { #[doc = #comment] });
            }
        }

        let name = format_ident!("{}", name_str);
        let depr = deprecate_attr(&typedef.depr);

        let cfg = if name_str == "VK_HEADER_VERSION" || name_str == "VK_HEADER_VERSION_COMPLETE" {
            if typedef
                .provided_by
                .contains(&"VKSC_VERSION_1_0".to_string())
            {
                quote! { #[cfg(feature = "VKSC_VERSION_1_0")] }
            } else {
                quote! { #[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))] }
            }
        } else {
            cfg_any(&typedef.provided_by)
        };

        let item_ts: Option<TokenStream> = if let Some(rest) = ty.strip_prefix("fn:") {
            // "fn:param1,param2|body_expr" - emit as #[inline] pub const fn
            if let Some((params_str, body_str)) = rest.split_once('|') {
                // Build the full item as a string, then parse it via syn
                let param_list: String = params_str
                    .split(',')
                    .map(|p| format!("{}: u32", p.trim()))
                    .collect::<Vec<_>>()
                    .join(", ");
                let item_src = format!(
                    "#[inline] pub const fn {name_str}({param_list}) -> u32 {{ {body_str} }}"
                );
                syn::parse_str::<syn::ItemFn>(&item_src).ok().map(|f| {
                    quote! {
                        #cfg #depr
                        #f
                    }
                })
            } else {
                None
            }
        } else if let Some(rest) = ty.strip_prefix("vkver:") {
            // Video codec version: "1, 0, 0" -> pre-evaluated u32
            let parts: Vec<&str> = rest.split(',').map(str::trim).collect();
            if parts.len() == 3 {
                let (major, minor, patch) = (parts[0], parts[1], parts[2]);
                let major_val = major.parse::<u32>().unwrap_or(0);
                let minor_val = minor.parse::<u32>().unwrap_or(0);
                let patch_val = patch.parse::<u32>().unwrap_or(0);
                Some(quote! {
                    #cfg #depr
                    pub const #name: u32 = VK_MAKE_VIDEO_STD_VERSION(#major_val, #minor_val, #patch_val);
                })
            } else {
                None
            }
        } else if let Some(rest) = ty.strip_prefix("apiconst:") {
            // API version constant or simple integer
            if let Some(args) = rest.strip_prefix("make_api_version(") {
                let a = args.trim_end_matches(')');
                let parts: Vec<&str> = a.split(',').map(str::trim).collect();
                if parts.len() == 4 {
                    let (variant, major, minor, patch) = (parts[0], parts[1], parts[2], parts[3]);
                    let variant_val = variant.parse::<u32>().unwrap_or(0);
                    let major_val = major.parse::<u32>().unwrap_or(0);
                    let minor_val = minor.parse::<u32>().unwrap_or(0);
                    let patch_val = patch.parse::<u32>().unwrap_or(0);
                    Some(quote! {
                        #cfg #depr
                        pub const #name: u32 = VK_MAKE_API_VERSION(#variant_val, #major_val, #minor_val, #patch_val);
                    })
                } else {
                    None
                }
            } else if let Ok(val) = rest.parse::<u32>() {
                let lit = Literal::u32_suffixed(val);
                Some(quote! {
                    #cfg #depr
                    pub const #name: u32 = #lit;
                })
            } else {
                None
            }
        } else {
            None
        };
        if let Some(ts) = item_ts {
            doc.extend(ts);
            groups
                .entry(feature_key(&typedef.provided_by))
                .or_default()
                .extend(doc);
        }
    }

    // reg.constants -> API constants and extension name/version strings
    for c in reg.constants.values().flatten() {
        let key = (c.name.clone(), feature_key(&c.provided_by));
        if !emitted.insert(key) {
            continue;
        }

        let name = format_ident!("{}", &c.name);
        let url = constant_refpage_url(c, &extension_literal_by_prefix);
        let url_str = format!(" [{}]({})", c.name, url);
        let mut doc = quote! { #[doc = #url_str] };
        if let Some(kind) = extension_type_for_constant(&c.name, &extension_type_by_prefix) {
            doc.extend(quote! { #[doc = " "] });
            let ext_line = format!(" Extension type: {kind} extension.");
            doc.extend(quote! { #[doc = #ext_line] });
        }
        if let Some(ref comment) = c.comment {
            let comment = comment.trim();
            if !comment.is_empty() {
                doc.extend(quote! { #[doc = " "] });
                let comment = " ".to_string() + comment;
                doc.extend(quote! { #[doc = #comment] });
            }
        }
        let depr = deprecate_attr(&c.depr);

        let cfg = if c.name == "VK_HEADER_VERSION" || c.name == "VK_HEADER_VERSION_COMPLETE" {
            if c.provided_by.contains(&"VKSC_VERSION_1_0".to_string()) {
                quote! { #[cfg(feature = "VKSC_VERSION_1_0")] }
            } else {
                quote! { #[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))] }
            }
        } else {
            cfg_any(&c.provided_by)
        };

        let token_stream: TokenStream = if let Some(ref alias) = c.alias {
            let a = format_ident!("{}", alias);
            quote! {
                #cfg #depr
                pub const #name: u32 = #a;
            }
        } else if c.ty == "&'static str" {
            let mut val = c.value.clone();
            if val.starts_with('"') && val.ends_with('"') {
                val.insert(0, 'c');
            }
            let val_ts: TokenStream = val.parse().unwrap_or_else(|_| quote! { c"" });
            quote! {
                #cfg #depr
                pub const #name: &'static core::ffi::CStr = #val_ts;
            }
        } else {
            let type_str = const_rust_type(&c.ty, &c.value);
            let type_ts: TokenStream = type_str.parse().unwrap_or_else(|_| quote! { u32 });
            let val_str = normalize_const_value(&c.value, type_str);
            let val_ts: TokenStream = val_str.parse().unwrap_or_else(|_| quote! { 0 });
            quote! {
                #cfg #depr
                pub const #name: #type_ts = #val_ts;
            }
        };

        doc.extend(token_stream);

        groups
            .entry(feature_key(&c.provided_by))
            .or_default()
            .extend(doc);
    }

    let mut out = TokenStream::new();
    out.extend(quote! {
        //! Vulkan API constants, version helpers, and extension version/name constants.
    });
    for (_, items) in groups {
        out.extend(items);
    }
    pretty(&out)
}

fn constant_refpage_url(
    c: &Constant,
    extension_literal_by_prefix: &HashMap<String, String>,
) -> String {
    if c.name.ends_with("_EXTENSION_NAME")
        && c.ty == "&'static str"
        && let Some(extension_literal) = parse_cstr_literal(&c.value)
    {
        return refpage_url(&extension_literal);
    }
    if let Some(prefix) = c.name.strip_suffix("_SPEC_VERSION")
        && let Some(extension_literal) = extension_literal_by_prefix.get(prefix)
    {
        return refpage_url(extension_literal);
    }
    refpage_url(&c.name)
}

fn extension_type_for_constant<'a>(
    name: &str,
    extension_type_by_prefix: &'a HashMap<String, String>,
) -> Option<&'a str> {
    let prefix = name
        .strip_suffix("_SPEC_VERSION")
        .or_else(|| name.strip_suffix("_EXTENSION_NAME"))?;
    extension_type_by_prefix.get(prefix).map(String::as_str)
}

fn parse_cstr_literal(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let inner = trimmed
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))?;
    Some(inner.to_owned())
}

fn normalize_const_value(value: &str, ty: &str) -> String {
    let v = value.trim();
    match ty {
        "f32" => format!("{}f32", v.trim_end_matches(['f', 'F'])),
        "u32" => {
            if v.starts_with("0x") || v.starts_with("0X") {
                return v.to_owned();
            }
            if v.contains('~') {
                return "u32::MAX".into();
            }
            v.trim_end_matches(['U', 'u']).to_owned()
        }
        "u64" => {
            if v.starts_with("0x") || v.starts_with("0X") {
                return v.to_owned();
            }
            if v.contains('~') {
                return "u64::MAX".into();
            }
            v.trim_end_matches(['U', 'u', 'L', 'l']).to_owned()
        }
        _ => v.to_owned(),
    }
}
