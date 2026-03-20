//! XML → IR parser for vk.xml / video.xml

use roxmltree::{Document, Node};
use crate::ir::*;
use crate::types::ctype_to_rust_str;

// ── Entry points ──────────────────────────────────────────────────────────────

pub fn parse_registry(xml: &str) -> Registry {
    let doc = Document::parse(xml).expect("valid XML");
    let root = doc.root_element();
    let mut reg = Registry::default();
    for child in root.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "types"      => parse_types(child, &mut reg),
            "enums"      => parse_enums_block(child, &mut reg),
            "commands"   => parse_commands_block(child, &mut reg),
            "feature"    => parse_feature(child, &mut reg),
            "extensions" => parse_extensions(child, &mut reg),
            _ => {}
        }
    }
    reg
}

/// Merge `video.xml` (or any secondary registry) into `base`.
/// For duplicate keys, the base value wins (vk.xml takes precedence).
pub fn merge_registry(base: &mut Registry, xml: &str) {
    let sec = parse_registry(xml);
    for (k, v) in sec.typedefs  { base.typedefs.entry(k).or_insert(v); }
    for (k, v) in sec.structs   { base.structs.entry(k).or_insert(v); }
    for (k, v) in sec.enums     { base.enums.entry(k).or_insert(v); }
    for (k, v) in sec.commands  { base.commands.entry(k).or_insert(v); }
    for (k, v) in sec.constants { base.constants.entry(k).or_insert(v); }
    base.features.extend(sec.features);
    base.extensions.extend(sec.extensions);
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn attr<'a>(node: Node<'a, '_>, name: &str) -> Option<&'a str> {
    node.attribute(name)
}

fn text_of(node: Node) -> String {
    node.children()
        .filter(|n| n.is_text())
        .map(|n| n.text().unwrap_or(""))
        .collect::<String>()
        .trim()
        .to_owned()
}

fn child_text(node: Node, tag: &str) -> Option<String> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == tag)
        .map(text_of)
        .filter(|s| !s.is_empty())
}

/// Like `child_text` but also checks the `<n>` alias used by vk.xml for name elements.
fn child_name(node: Node) -> Option<String> {
    child_text(node, "name").or_else(|| child_text(node, "n"))
}

fn api_set(node: Node) -> ApiSet {
    attr(node, "api").map(ApiSet::from_str).unwrap_or_else(ApiSet::all)
}

/// Read all deprecation-related attributes from a node.
///
/// Sources:
/// - `deprecated="..."` — general deprecation marker
/// - `supersededby="..."` — on <command name="..." supersededby="..."/> inside <require>
/// - `obsoletedby="..."` — on <extension>
/// - `promotedto="..."` — on <extension>
fn depr_info(node: Node) -> DeprecationInfo {
    DeprecationInfo {
        deprecated:    attr(node, "deprecated").map(str::to_owned),
        superseded_by: attr(node, "supersededby").map(str::to_owned),
        obsoleted_by:  attr(node, "obsoletedby").map(str::to_owned),
        promoted_to:   attr(node, "promotedto").map(str::to_owned),
    }
}

// ── Enum value parsing ────────────────────────────────────────────────────────

/// Parse an enum value node, using `ext_number` as the fallback extension
/// number when the node itself lacks `extnumber=`.
fn parse_enum_value_node(node: Node, ext_number: Option<u32>) -> EnumValue {
    if let Some(alias) = attr(node, "alias") {
        return EnumValue::Alias(alias.to_owned());
    }
    if let Some(bp) = attr(node, "bitpos") {
        return EnumValue::BitPos(bp.parse().unwrap_or(0));
    }
    // Offset-based value (Khronos extension enum allocation scheme)
    // value = 1_000_000_000 + (extnumber − 1) × 1000 + offset
    if let Some(offset_str) = attr(node, "offset") {
        let offset: u32 = offset_str.parse().unwrap_or(0);
        let extnumber: u32 = attr(node, "extnumber")
            .and_then(|s| s.parse().ok())
            .or(ext_number)
            .unwrap_or(1);
        let negative = attr(node, "dir") == Some("-");
        return EnumValue::Offset { extnumber, offset, negative };
    }
    if let Some(value) = attr(node, "value") {
        let v = value.trim();
        if v.starts_with("0x") || v.starts_with("0X") {
            let hex_part = v.trim_start_matches("0x").trim_start_matches("0X");
            if let Ok(n) = u64::from_str_radix(hex_part, 16) {
                return EnumValue::Hex(n);
            }
        }
        // (~0U) etc. → Expr for later normalisation
        if v.starts_with('(') || v.starts_with('~') {
            return EnumValue::Expr(v.to_owned());
        }
        if let Ok(n) = v.trim_end_matches(['U','u','L','l']).parse::<i64>() {
            return EnumValue::Integer(n);
        }
        return EnumValue::Expr(v.to_owned());
    }
    EnumValue::Integer(0)
}

// ── C type parsing ────────────────────────────────────────────────────────────

fn parse_c_type(node: Node) -> CType {
    let mut base = String::new();
    let mut is_const = false;
    let mut pointer_depth: u8 = 0;
    let mut is_array: Option<String> = None;
    let mut full_text = String::new();

    for child in node.children() {
        if child.is_text() {
            let t = child.text().unwrap_or("");
            full_text.push_str(t);
            if t.contains("const") { is_const = true; }
            pointer_depth += t.chars().filter(|&c| c == '*').count() as u8;
        } else if child.is_element() {
            match child.tag_name().name() {
                "type" => { base = text_of(child); full_text.push_str(&base); }
                "enum" => { is_array = Some(text_of(child)); }
                "name" | "n" => {} // member/parameter name — skip
                _ => {}
            }
        }
    }

    // Fallback: extract base type from full text
    if base.is_empty() {
        base = full_text
            .split('*').next().unwrap_or("")
            .replace("const", "").replace("struct", "")
            .trim().to_owned();
    }

    // Static array from [] in text (when <enum> child is absent)
    if is_array.is_none() && full_text.contains('[') {
        if let (Some(a), Some(b)) = (full_text.find('['), full_text.find(']')) {
            let sz = full_text[a+1..b].trim().to_owned();
            if !sz.is_empty() { is_array = Some(sz); }
        }
    }

    CType { is_const, base, pointer_depth, is_array }
}

fn parse_member(node: Node) -> Member {
    Member {
        name:     child_name(node).unwrap_or_default(),
        ty:       parse_c_type(node),
        optional: attr(node, "optional").map_or(false, |v| v.contains("true")),
        len:      attr(node, "len").map(str::to_owned),
        values:   attr(node, "values").map(str::to_owned),
        api:      attr(node, "api").map(ApiSet::from_str),
        comment:  attr(node, "comment").map(str::to_owned),
    }
}

// ── Types block ───────────────────────────────────────────────────────────────

fn parse_types(node: Node, reg: &mut Registry) {
    for tn in node.children().filter(|n| n.is_element() && n.tag_name().name() == "type") {
        let category = attr(tn, "category").unwrap_or("");
        let alias    = attr(tn, "alias").map(str::to_owned);
        let comment  = attr(tn, "comment").map(str::to_owned);
        let dep      = attr(tn, "depends").map(parse_dep_expr);
        let depr     = depr_info(tn);
        let aset     = api_set(tn);
        let name: String = attr(tn, "name")
            .map(str::to_owned)
            .or_else(|| child_name(tn))
            // funcpointer: name lives inside <proto><name>...</name></proto>
            .or_else(|| {
                tn.children()
                    .find(|n| n.is_element() && n.tag_name().name() == "proto")
                    .and_then(|proto| child_name(proto))
            })
            .unwrap_or_default();
        if name.is_empty() { continue; }

        match category {
            "struct" | "union" => {
                let members: Vec<Member> = tn.children()
                    .filter(|n| n.is_element() && n.tag_name().name() == "member")
                    .map(parse_member).collect();
                let struct_extends = attr(tn, "structextends")
                    .map(|s| s.split(',').map(str::to_owned).collect())
                    .unwrap_or_default();
                reg.structs.insert(name.clone(), Struct {
                    name, alias, members,
                    is_union: category == "union",
                    returned_only: attr(tn, "returnedonly") == Some("true"),
                    struct_extends, api: aset, comment, dep, provided_by: vec![], depr,
                });
            }
            "handle" => {
                reg.typedefs.insert(name.clone(), Typedef {
                    name, alias, ty: Some("u64".into()),
                    kind: TypedefKind::Handle, api: aset, comment, dep, depr,
                    provided_by: vec![],
                });
            }
            "enum" => {
                // Forward declaration — variants come from <enums> and <require><enum extends=...>
                reg.enums.entry(name.clone()).or_insert(Enum {
                    name: name.clone(), alias, variants: vec![],
                    is_bitmask: false, bit_width: 32,
                    api: aset, comment, dep, provided_by: vec![],
                });
            }
            "bitmask" => {
                // Underlying type from a <type> child, or fallback to VkFlags
                let underlying = child_text(tn, "type")
                    .unwrap_or_else(|| "VkFlags".into());
                reg.typedefs.insert(name.clone(), Typedef {
                    name, alias, ty: Some(underlying),
                    kind: TypedefKind::Bitmask, api: aset, comment, dep, depr,
                    provided_by: vec![],
                });
            }
            "basetype" => {
                // Handles multiple forms found in vk.xml:
                //
                // 1. Plain C typedef:  typedef unsigned int <name>VkFlags</name>;
                //    → Basetype with ty = "unsigned int"
                //
                // 2. Opaque struct fwd-decl:  struct <name>ANativeWindow</name>;
                //    (no "typedef" keyword, no <type> child)
                //    → OpaqueExtern
                //
                // 3. Platform #ifdef block:  #ifdef __OBJC__ @class CAMetalLayer; #else typedef void <name>CAMetalLayer</name>; #endif
                //    → OpaqueExtern (we take the void/opaque path)
                //
                // 4. Typedef to known type:  typedef struct ANativeWindow* <name>ANativeWindow</name>;
                //    → Basetype with the <type> child

                let raw_text = text_of(tn); // text nodes only, <name>/<n> content excluded
                let has_type_child = child_text(tn, "type").is_some();

                let (kind, ty) = if has_type_child {
                    // Form 4 or form 1 with a <type> child
                    let inner = child_text(tn, "type").unwrap();
                    (TypedefKind::Basetype, Some(inner))
                } else {
                    // No <type> child — inspect the raw text
                    let stripped = raw_text
                        .trim_start_matches("typedef")
                        .replace(';', "")
                        .trim()
                        .to_owned();

                    // Opaque struct: "struct " prefix with no real type body
                    // Platform ifdef: contains "#ifdef" or "@class"
                    if stripped.starts_with("struct")
                        || stripped.starts_with("#ifdef")
                        || stripped.starts_with("@class")
                        || stripped.is_empty()
                    {
                        (TypedefKind::OpaqueExtern, None)
                    } else {
                        // Plain text type like "unsigned int" after stripping typedef
                        let ty_str = stripped.trim().to_owned();
                        if ty_str.is_empty() {
                            (TypedefKind::OpaqueExtern, None)
                        } else {
                            (TypedefKind::Basetype, Some(ty_str))
                        }
                    }
                };

                reg.typedefs.insert(name.clone(), Typedef {
                    name, alias, ty, kind,
                    api: aset, comment, dep, depr, provided_by: vec![],
                });
            }
            "funcpointer" => {
                // Parse the full function pointer signature.
                // The `ty` field stores a compact encoding:
                //   "<rust_return_type>|<pname>:<rust_ptype>,..."
                //
                // Return type: parse the <proto> child as a CType so we capture
                // pointer decorations (e.g. "void*" → "*mut c_void").
                let ret_rust = tn.children()
                    .find(|n| n.is_element() && n.tag_name().name() == "proto")
                    .map(|proto| {
                        let cty = parse_c_type(proto);
                        ctype_to_rust_str(&cty)
                    })
                    .unwrap_or_else(|| "core::ffi::c_void".into());

                // Also handle the old-style typedef void VKAPI_PTR *PFN_Foo(); form
                // where there is no <proto> but the return type is in the text.
                // In that case ret_rust will be "core::ffi::c_void" which is correct.

                let params: Vec<String> = tn.children()
                    .filter(|n| n.is_element() && n.tag_name().name() == "param")
                    .map(|pn| {
                        let pname = child_name(pn).unwrap_or_default();
                        let pty   = parse_c_type(pn);
                        let pty_str = ctype_to_rust_str(&pty);
                        format!("{pname}:{pty_str}")
                    })
                    .collect();

                let encoded = format!("{}|{}", ret_rust, params.join(","));
                reg.typedefs.insert(name.clone(), Typedef {
                    name, alias, ty: Some(encoded), kind: TypedefKind::FuncPointer,
                    api: aset, comment, dep, depr, provided_by: vec![],
                });
            }
            "define" => {
                // Parse #define macros from vk.xml and video.xml.
                // We store a structured encoding in `ty` for codegen:
                //
                //   "fn:<params>|<body>"    → const fn (function-like macro)
                //   "vkver:<maj>,<min>,<patch>"  → VK_MAKE_VIDEO_STD_VERSION call
                //   "apiconst:<value>"      → pub const u32 (pre-computed integer)
                //   None                    → skip (comment-defines, deprecated, etc.)
                //
                // Function-like macros detected by parameter list in raw text.
                let raw = text_of(tn);
                let type_child = child_text(tn, "type");

                let ty_field: Option<String> = if let Some(ref tc) = type_child {
                    // #define NAME <type>MACRO</type>(args) — a version constant
                    let tail = tn.children()
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
                        // video version constant: evaluate (1,0,0) → 4194304
                        Some(format!("vkver:{tail}"))
                    } else if tc == "VK_MAKE_API_VERSION" {
                        // API version constant: evaluate (0,1,0,0) → packed u32
                        Some(format!("apiconst:make_api_version({tail})"))
                    } else {
                        None
                    }
                } else if raw.contains("major") || raw.contains("minor") || raw.contains("patch")
                         || raw.contains("version") || raw.contains("variant")
                {
                    // Function-like macro — emit as const fn.
                    // Classify by name pattern to pick the right body.
                    let body: Option<&str> = if name.contains("MAKE_VIDEO_STD_VERSION") {
                        Some("fn:major,minor,patch|(major << 22) | (minor << 12) | patch")
                    } else if name == "VK_MAKE_VERSION" {
                        Some("fn:major,minor,patch|(major << 22) | (minor << 12) | patch")
                    } else if name == "VK_MAKE_API_VERSION" {
                        Some("fn:variant,major,minor,patch|(variant << 29) | (major << 22) | (minor << 12) | patch")
                    } else if name == "VK_VERSION_MAJOR" {
                        Some("fn:version|version >> 22")
                    } else if name == "VK_VERSION_MINOR" {
                        Some("fn:version|(version >> 12) & 0x3FF")
                    } else if name == "VK_VERSION_PATCH" {
                        Some("fn:version|version & 0xFFF")
                    } else if name == "VK_API_VERSION_VARIANT" {
                        Some("fn:version|version >> 29")
                    } else if name == "VK_API_VERSION_MAJOR" {
                        Some("fn:version|(version >> 22) & 0x7F")
                    } else if name == "VK_API_VERSION_MINOR" {
                        Some("fn:version|(version >> 12) & 0x3FF")
                    } else if name == "VK_API_VERSION_PATCH" {
                        Some("fn:version|version & 0xFFF")
                    } else {
                        None
                    };
                    body.map(str::to_owned)
                } else {
                    // Simple integer constant: the value is the tail text of the <name>/<n> child.
                    // e.g. "#define VK_HEADER_VERSION 346" → name child tail is " 346"
                    // Skip commented-out defines (text before the name starts with //)
                    let is_commented = raw.trim_start().starts_with("//")
                        && !raw.contains(&format!("#define"));
                    if is_commented {
                        None
                    } else {
                        // Get the tail of the <name> or <n> child — that's the value
                        let name_tail = tn.children()
                            .find(|n| n.is_element()
                                && (n.tag_name().name() == "name" || n.tag_name().name() == "n"))
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

                if ty_field.is_some() || matches!(ty_field, None) {
                    // Always register so apply_require_extensions can set provided_by
                    reg.typedefs.insert(name.clone(), Typedef {
                        name, alias, ty: ty_field, kind: TypedefKind::Define,
                        api: aset, comment, dep, depr, provided_by: vec![],
                    });
                }
            }
            _ => {
                if alias.is_some() {
                    reg.typedefs.insert(name.clone(), Typedef {
                        name, alias, ty: None, kind: TypedefKind::Alias,
                        api: aset, comment, dep, depr, provided_by: vec![],
                    });
                } else if let Some(req) = attr(tn, "requires") {
                    // Only register platform OS/windowing system types as opaque pointer handles.
                    // Video codec headers (vk_video/*, vulkan_video_*) are handled by video.xml
                    // parsing and must NOT be registered here as opaque pointers.
                    let is_platform_header = req.contains("windows.h")
                        || req.contains("X11/")
                        || req.contains("wayland-")
                        || req.contains("xcb/")
                        || req.contains("directfb.h")
                        || req.contains("zircon/")
                        || req.contains("ggp_c/")
                        || req.contains("screen/")
                        || req.contains("nvscisync.h")
                        || req.contains("nvscibuf.h")
                        || req.contains("ubm.h")
                        || req.contains("android/")
                        || req.contains("Metal/")
                        || req.contains("objc/");
                    if is_platform_header {
                        reg.typedefs.insert(name.clone(), Typedef {
                            name, alias: None, ty: None, kind: TypedefKind::OpaqueExtern,
                            api: aset, comment, dep, depr, provided_by: vec![],
                        });
                    }
                    // Otherwise (vk_video/ headers etc.) — ignore; they're declared elsewhere.
                }
            }
        }
    }
}

// ── Enums block ───────────────────────────────────────────────────────────────

fn parse_enums_block(node: Node, reg: &mut Registry) {
    let name = match attr(node, "name") { Some(n) => n.to_owned(), None => return };
    let is_bitmask = attr(node, "type") == Some("bitmask");
    let bit_width: u8 = attr(node, "bitwidth").and_then(|s| s.parse().ok()).unwrap_or(32);
    let comment = attr(node, "comment").map(str::to_owned);

    // API Constants block → global constants
    if name == "API Constants" {
        for en in node.children().filter(|n| n.is_element() && n.tag_name().name() == "enum") {
            let ename = attr(en, "name").unwrap_or("").to_owned();
            if ename.is_empty() { continue; }
            // May be an alias
            let alias = attr(en, "alias").map(str::to_owned);
            let value = attr(en, "value").unwrap_or("0").to_owned();
            let ty    = attr(en, "type").unwrap_or("u32").to_owned();
            let depr  = depr_info(en);
            reg.constants.insert(ename.clone(), Constant {
                name: ename, value, ty, alias,
                comment: attr(en, "comment").map(str::to_owned),
                provided_by: vec![], depr,
            });
        }
        return;
    }

    let entry = reg.enums.entry(name.clone()).or_insert_with(|| Enum {
        name: name.clone(), alias: None, variants: vec![], is_bitmask, bit_width,
        api: ApiSet::all(), comment: comment.clone(), dep: None, provided_by: vec![],
    });
    entry.is_bitmask = is_bitmask;
    entry.bit_width  = bit_width;
    if entry.comment.is_none() { entry.comment = comment; }

    for en in node.children().filter(|n| n.is_element() && n.tag_name().name() == "enum") {
        let vname = attr(en, "name").unwrap_or("").to_owned();
        if vname.is_empty() { continue; }
        let value = parse_enum_value_node(en, None);
        entry.variants.push(EnumVariant {
            name: vname, value, alias: attr(en, "alias").map(str::to_owned),
            comment: attr(en, "comment").map(str::to_owned),
            api: attr(en, "api").map(ApiSet::from_str),
            depr: depr_info(en), provided_by: vec![],
        });
    }
}

// ── Commands block ────────────────────────────────────────────────────────────

fn parse_commands_block(node: Node, reg: &mut Registry) {
    for cn in node.children().filter(|n| n.is_element() && n.tag_name().name() == "command") {
        let alias    = attr(cn, "alias").map(str::to_owned);
        let name_attr = attr(cn, "name").map(str::to_owned);
        let aset     = api_set(cn);
        let dep      = attr(cn, "depends").map(parse_dep_expr);
        let depr     = depr_info(cn);
        let comment  = attr(cn, "comment").map(str::to_owned);

        // Alias shorthand: <command name="vkFoo" alias="vkBar"/>
        if alias.is_some() {
            if let Some(name) = name_attr {
                reg.commands.insert(name.clone(), Command {
                    name, alias, return_type: CType::simple("VkResult"),
                    params: vec![], api: aset, comment, dep,
                    provided_by: vec![], depr,
                    success_codes: vec![], error_codes: vec![],
                });
            }
            continue;
        }

        // Full command definition
        let proto = cn.children()
            .find(|n| n.is_element() && n.tag_name().name() == "proto");
        let (name, return_type) = if let Some(proto) = proto {
            (child_name(proto).unwrap_or_default(), parse_c_type(proto))
        } else {
            (name_attr.unwrap_or_default(), CType::simple("void"))
        };
        if name.is_empty() { continue; }

        let params: Vec<Member> = cn.children()
            .filter(|n| n.is_element() && n.tag_name().name() == "param")
            .map(parse_member).collect();

        reg.commands.insert(name.clone(), Command {
            name, alias, return_type, params, api: aset, comment, dep,
            provided_by: vec![], depr,
            success_codes: attr(cn, "successcodes")
                .map(|s| s.split(',').map(str::to_owned).collect()).unwrap_or_default(),
            error_codes: attr(cn, "errorcodes")
                .map(|s| s.split(',').map(str::to_owned).collect()).unwrap_or_default(),
        });
    }
}

// ── Feature block ─────────────────────────────────────────────────────────────

fn parse_feature(node: Node, reg: &mut Registry) {
    let name    = attr(node, "name").unwrap_or("").to_owned();
    let api     = ApiSet::from_str(attr(node, "api").unwrap_or("vulkan"));
    let number  = attr(node, "number").unwrap_or("0").to_owned();
    let depends = attr(node, "depends").map(parse_dep_expr);
    let comment = attr(node, "comment").map(str::to_owned);

    let mut requires = Vec::new();
    for rn in node.children().filter(|n| n.is_element() && n.tag_name().name() == "require") {
        let req = parse_require(rn, None);
        // Use the full depr-aware variant so supersededby/deprecated attrs on
        // individual <command> and <type> nodes inside <require> are captured.
        mark_provided_with_depr(&req, &name, rn, reg);
        requires.push(req);
    }
    reg.features.push(Feature { name, api, number, depends, requires, comment });
}

// ── Extensions block ──────────────────────────────────────────────────────────

fn parse_extensions(node: Node, reg: &mut Registry) {
    for en in node.children().filter(|n| n.is_element() && n.tag_name().name() == "extension") {
        let name = attr(en, "name").unwrap_or("").to_owned();
        if name.is_empty() { continue; }

        let number: u32 = attr(en, "number").and_then(|s| s.parse().ok()).unwrap_or(0);
        let depends     = attr(en, "depends").map(parse_dep_expr);
        let api         = ApiSet::from_str(attr(en, "api").unwrap_or("vulkan"));
        let supported   = attr(en, "supported").unwrap_or("vulkan").to_owned();
        let depr = DeprecationInfo {
            deprecated:    attr(en, "deprecated").map(str::to_owned),
            superseded_by: attr(en, "supersededby").map(str::to_owned),
            obsoleted_by:  attr(en, "obsoletedby").map(str::to_owned),
            promoted_to:   attr(en, "promotedto").map(str::to_owned),
        };

        let ext_shell = Extension {
            name: name.clone(), number, depends, api, supported,
            requires: vec![], comment: attr(en, "comment").map(str::to_owned),
            depr, requires_core: attr(en, "requiresCore").map(str::to_owned),
            ext_type: attr(en, "type").map(str::to_owned),
        };

        let disabled = ext_shell.is_disabled();
        let mut requires = Vec::new();
        for rn in en.children().filter(|n| n.is_element() && n.tag_name().name() == "require") {
            let req = parse_require(rn, Some(number));
            if !disabled {
                // Also pick up supersededby on <command> / <type> nodes inside <require>
                mark_provided_with_depr(&req, &name, rn, reg);
            }
            requires.push(req);
        }

        let mut ext = ext_shell;
        ext.requires = requires;
        reg.extensions.push(ext);
    }
}

// ── Require block ─────────────────────────────────────────────────────────────

fn parse_require(node: Node, ext_number: Option<u32>) -> Require {
    let mut req = Require {
        depends: attr(node, "depends").map(parse_dep_expr),
        api:     attr(node, "api").map(ApiSet::from_str),
        ..Default::default()
    };

    for child in node.children().filter(|n| n.is_element()) {
        match child.tag_name().name() {
            "type"    => { if let Some(n) = attr(child, "name") { req.types.push(n.to_owned()); } }
            "command" => { if let Some(n) = attr(child, "name") { req.commands.push(n.to_owned()); } }
            "enum"    => {
                let rname    = attr(child, "name").unwrap_or("").to_owned();
                let extends  = attr(child, "extends").map(str::to_owned);
                let alias    = attr(child, "alias").map(str::to_owned);
                let depr     = depr_info(child);
                let api      = attr(child, "api").map(ApiSet::from_str);
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
                    extends, name: rname, value, alias, depr, api, extnumber,
                    comment: attr(child, "comment").map(str::to_owned),
                });
            }
            _ => {}
        }
    }
    req
}

/// Record `feature_name` as a provider for all items in a Require block,
/// also capturing any `supersededby=` / `deprecated=` attributes on the
/// individual `<command>` and `<type>` child nodes of `raw_require`.
fn mark_provided_with_depr(req: &Require, feature_name: &str, raw_require: Node, reg: &mut Registry) {
    // Walk children to capture per-item supersededby / deprecated
    for child in raw_require.children().filter(|n| n.is_element()) {
        let item_name = attr(child, "name").unwrap_or("");
        let item_depr = depr_info(child);

        match child.tag_name().name() {
            "type" => {
                let fname = feature_name.to_owned();
                if let Some(s) = reg.structs.get_mut(item_name) {
                    s.provided_by.push(fname.clone());
                    // Merge deprecation info
                    if item_depr.superseded_by.is_some() && s.depr.superseded_by.is_none() {
                        s.depr.superseded_by = item_depr.superseded_by.clone();
                    }
                }
                if let Some(t) = reg.typedefs.get_mut(item_name) {
                    t.provided_by.push(fname.clone());
                    if item_depr.superseded_by.is_some() && t.depr.superseded_by.is_none() {
                        t.depr.superseded_by = item_depr.superseded_by.clone();
                    }
                }
                if let Some(e) = reg.enums.get_mut(item_name) {
                    e.provided_by.push(fname);
                }
            }
            "command" => {
                let fname = feature_name.to_owned();
                if let Some(c) = reg.commands.get_mut(item_name) {
                    c.provided_by.push(fname);
                    if item_depr.superseded_by.is_some() && c.depr.superseded_by.is_none() {
                        c.depr.superseded_by = item_depr.superseded_by.clone();
                    }
                    if item_depr.deprecated.is_some() && c.depr.deprecated.is_none() {
                        c.depr.deprecated = item_depr.deprecated.clone();
                    }
                }
            }
            _ => {}
        }
    }
    // Fallback for items that may not have been caught above (from parsed req.types/commands)
    for tname in &req.types {
        if let Some(s) = reg.structs.get_mut(tname) {
            if !s.provided_by.contains(&feature_name.to_owned()) {
                s.provided_by.push(feature_name.to_owned());
            }
        }
        if let Some(t) = reg.typedefs.get_mut(tname) {
            if !t.provided_by.contains(&feature_name.to_owned()) {
                t.provided_by.push(feature_name.to_owned());
            }
        }
        if let Some(e) = reg.enums.get_mut(tname) {
            if !e.provided_by.contains(&feature_name.to_owned()) {
                e.provided_by.push(feature_name.to_owned());
            }
        }
    }
    for cname in &req.commands {
        if let Some(c) = reg.commands.get_mut(cname) {
            if !c.provided_by.contains(&feature_name.to_owned()) {
                c.provided_by.push(feature_name.to_owned());
            }
        }
    }
}

// ── Post-processing ───────────────────────────────────────────────────────────

/// Propagate `<require><enum extends=...>` variants into the target enum,
/// collect `<require><enum>` without `extends` as extension constants,
/// and strip disabled-extension names from provided_by.
pub fn apply_require_extensions(reg: &mut Registry) {
    // Build set of disabled extension names for quick lookup
    let disabled: std::collections::HashSet<String> = reg.extensions.iter()
        .filter(|e| e.is_disabled())
        .map(|e| e.name.clone())
        .collect();

    let mut to_add:   Vec<(String, EnumVariant)> = Vec::new();
    let mut ext_consts: Vec<(String, Constant)>  = Vec::new(); // (source_feature, constant)

    for feat in &reg.features {
        collect_extend_enums(&feat.requires, &feat.name, None, &mut to_add);
        collect_extension_consts(&feat.requires, &feat.name, &mut ext_consts);
    }
    for ext in &reg.extensions {
        if ext.is_disabled() { continue; }
        collect_extend_enums(&ext.requires, &ext.name, Some(ext.number), &mut to_add);
        collect_extension_consts(&ext.requires, &ext.name, &mut ext_consts);
    }

    // Push enum variant extensions
    for (enum_name, variant) in to_add {
        if let Some(e) = reg.enums.get_mut(&enum_name) {
            if !e.variants.iter().any(|v| v.name == variant.name) {
                e.variants.push(variant);
            }
        }
    }

    // Register extension constants (SPEC_VERSION, EXTENSION_NAME, etc.)
    for (source, constant) in ext_consts {
        let entry = reg.constants.entry(constant.name.clone()).or_insert(constant);
        if !entry.provided_by.contains(&source) {
            entry.provided_by.push(source);
        }
    }

    // Strip disabled-extension names from provided_by on all items.
    let scrub = |v: &mut Vec<String>| { v.retain(|f| !disabled.contains(f)); };
    for td in reg.typedefs.values_mut()  { scrub(&mut td.provided_by); }
    for s  in reg.structs.values_mut()   { scrub(&mut s.provided_by); }
    for e  in reg.enums.values_mut() {
        scrub(&mut e.provided_by);
        for v in e.variants.iter_mut() { scrub(&mut v.provided_by); }
    }
    for c  in reg.commands.values_mut()  { scrub(&mut c.provided_by); }
    for c  in reg.constants.values_mut() { scrub(&mut c.provided_by); }

    // For enums that only gained variants via extends (no direct <require><type>),
    // infer provided_by from the union of variant sources.
    for e in reg.enums.values_mut() {
        if e.provided_by.is_empty() {
            let mut inferred: Vec<String> = Vec::new();
            for v in &e.variants {
                for f in &v.provided_by {
                    if !inferred.contains(f) { inferred.push(f.clone()); }
                }
            }
            e.provided_by = inferred;
        }
    }

    // Back-propagate provided_by to platform/opaque types that are referenced as
    // pointer parameters in structs/commands but never listed in <require><type>.
    // For example, wl_display appears in VkWaylandSurfaceCreateInfoKHR.display as
    // a pointer field, so it should be gated by VK_KHR_wayland_surface.
    //
    // Strategy: for any typedef with empty provided_by (only OpaqueExtern needs this
    // since real Vk types are always explicitly required), collect all feature names
    // from structs/commands that reference it as a base type in any member/param.
    let opaque_names: Vec<String> = reg.typedefs.iter()
        .filter(|(_, td)| td.provided_by.is_empty()
            && td.kind == TypedefKind::OpaqueExtern)
        .map(|(n, _)| n.clone())
        .collect();

    for tname in &opaque_names {
        let mut providers: Vec<String> = Vec::new();

        // Check all structs: if any member's base type is tname, inherit struct's providers
        for s in reg.structs.values() {
            if s.provided_by.is_empty() { continue; }
            let references = s.members.iter().any(|m| &m.ty.base == tname);
            if references {
                for pb in &s.provided_by {
                    if !providers.contains(pb) { providers.push(pb.clone()); }
                }
            }
        }

        // Check all commands: if any param's base type is tname, inherit command's providers
        for c in reg.commands.values() {
            if c.provided_by.is_empty() { continue; }
            let references = c.params.iter().any(|p| &p.ty.base == tname);
            if references {
                for pb in &c.provided_by {
                    if !providers.contains(pb) { providers.push(pb.clone()); }
                }
            }
        }

        if !providers.is_empty() {
            if let Some(td) = reg.typedefs.get_mut(tname) {
                td.provided_by = providers;
            }
        }
    }

    // Populate provided_by for #define constants that are never in <require><type>.
    // Two groups:
    //   • vk.xml API version helpers (VK_MAKE_API_VERSION etc.) → ungated (empty provided_by
    //     means they appear unconditionally in consts.rs)
    //   • vk.xml API version constants (VK_API_VERSION_1_0 etc.) → ungated
    //   • video codec version defines → gated by VK_KHR_video_* feature
    for td in reg.typedefs.values_mut() {
        if td.kind != TypedefKind::Define || !td.provided_by.is_empty() { continue; }
        if td.ty.is_none() { continue; } // no emittable content

        let gate: Option<&str> =
            // API version helpers — ungated (provided_by empty → no cfg gate in consts.rs)
            if td.name.starts_with("VK_MAKE_")
            || td.name.starts_with("VK_VERSION_")
            || td.name.starts_with("VK_API_VERSION_")
            || td.name.starts_with("VK_API_VERSION")
            || td.name == "VK_HEADER_VERSION"
            || td.name == "VK_HEADER_VERSION_COMPLETE"
            || td.name == "VKSC_API_VARIANT"
            || td.name.starts_with("VKSC_API_")
            { None /* ungated */ }
            // video defines
            else if td.name == "VK_MAKE_VIDEO_STD_VERSION"       { Some("VK_KHR_video_queue") }
            else if td.name.contains("_H264_DECODE_")             { Some("VK_KHR_video_decode_h264") }
            else if td.name.contains("_H264_ENCODE_")             { Some("VK_KHR_video_encode_h264") }
            else if td.name.contains("_H265_DECODE_")             { Some("VK_KHR_video_decode_h265") }
            else if td.name.contains("_H265_ENCODE_")             { Some("VK_KHR_video_encode_h265") }
            else if td.name.contains("_VP9_DECODE_")              { Some("VK_KHR_video_decode_vp9") }
            else if td.name.contains("_VP9_ENCODE_")              { Some("VK_KHR_video_encode_vp9") }
            else if td.name.contains("_AV1_DECODE_")              { Some("VK_KHR_video_decode_av1") }
            else if td.name.contains("_AV1_ENCODE_")              { Some("VK_KHR_video_encode_av1") }
            else { None };

        if let Some(g) = gate {
            td.provided_by = vec![g.to_owned()];
        }
        // ungated defines keep provided_by empty; gen_consts_rs emits them without cfg
    }
}

fn collect_extend_enums(
    requires: &[Require],
    source: &str,
    ext_number: Option<u32>,
    out: &mut Vec<(String, EnumVariant)>,
) {
    for req in requires {
        for re in &req.enums {
            let Some(ref extends) = re.extends else { continue };

            // Re-resolve offset values with the correct ext_number if needed
            let value = match re.value.clone() {
                Some(EnumValue::Offset { extnumber, offset, negative }) => {
                    // The extnumber on the value may be 1 (default) if neither the
                    // node nor the require block carried it — use ext_number in that case.
                    let en = if extnumber == 1 { ext_number.unwrap_or(extnumber) } else { extnumber };
                    EnumValue::Offset { extnumber: en, offset, negative }
                }
                Some(v) => v,
                None => re.alias.as_ref()
                    .map(|a| EnumValue::Alias(a.clone()))
                    .unwrap_or(EnumValue::Integer(0)),
            };

            out.push((extends.clone(), EnumVariant {
                name: re.name.clone(), value,
                comment: re.comment.clone(), api: re.api.clone(),
                depr: re.depr.clone(), alias: re.alias.clone(),
                provided_by: vec![source.to_owned()],
            }));
        }
    }
}
/// Collect `<require><enum>` entries that have no `extends=` attribute —
/// these are per-extension constants like `VK_KHR_SWAPCHAIN_SPEC_VERSION`
/// and `VK_KHR_SWAPCHAIN_EXTENSION_NAME`.
fn collect_extension_consts(
    requires: &[Require],
    source: &str,
    out: &mut Vec<(String, Constant)>,
) {
    for req in requires {
        for re in &req.enums {
            // Only handle entries with no `extends` (not enum variant extensions)
            if re.extends.is_some() { continue; }
            if re.name.is_empty()   { continue; }
            // Must have an explicit value (not just an alias to an enum variant)
            let Some(ref val) = re.value else { continue };

            let (rust_ty, rust_val) = match val {
                EnumValue::Integer(n) => ("u32".into(), format!("{n}")),
                EnumValue::Hex(n)     => ("u32".into(), format!("0x{n:X}")),
                EnumValue::Expr(s) => {
                    // String constants arrive as `"\"VK_KHR_swapchain\""` (escaped quotes)
                    // Strip outer C-style quotes → emit as &'static str
                    let trimmed = s.trim();
                    if trimmed.starts_with('"') && trimmed.ends_with('"') {
                        let inner = &trimmed[1..trimmed.len()-1];
                        ("&'static str".into(), format!("\"{}\"", inner))
                    } else {
                        ("u32".into(), trimmed.to_owned())
                    }
                }
                _ => continue, // skip bitpos / offset / alias entries here
            };

            out.push((source.to_owned(), Constant {
                name: re.name.clone(),
                value: rust_val,
                ty: rust_ty,
                alias: None,
                comment: re.comment.clone(),
                provided_by: vec![source.to_owned()],
                depr: re.depr.clone(),
            }));
        }
    }
}
