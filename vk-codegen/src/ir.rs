//! Intermediate Representation for the Vulkan API
//!
//! The grammar for `depends=` strings is:
//!   expr     := and_expr (',' and_expr)*
//!   and_expr := atom ('+' atom)*
//!   atom     := IDENT | '(' expr ')'
//!
//! This handles the full nesting used in practice, e.g.
//!   ((VK_KHR_get_physical_device_properties2,VK_VERSION_1_1)+VK_KHR_depth_stencil_resolve),VK_VERSION_1_2
#![allow(dead_code)]

use indexmap::IndexMap;

// -- Dependency expression -----------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum DepExpr {
    And(Box<DepExpr>, Box<DepExpr>),
    Or(Box<DepExpr>, Box<DepExpr>),
    Atom(String),
}

impl DepExpr {
    /// All leaf feature names referenced anywhere in this expression.
    pub fn atoms(&self) -> Vec<String> {
        match self {
            DepExpr::Atom(s) => vec![s.clone()],
            DepExpr::And(a, b) | DepExpr::Or(a, b) => {
                let mut v = a.atoms();
                for s in b.atoms() {
                    if !v.contains(&s) {
                        v.push(s);
                    }
                }
                v
            }
        }
    }

    /// Convert to DNF: a list of AND-clauses whose disjunction equals this expression.
    pub fn to_dnf_clauses(&self) -> Vec<Vec<String>> {
        match self {
            DepExpr::Atom(s) => vec![vec![s.clone()]],
            DepExpr::Or(a, b) => {
                let mut c = a.to_dnf_clauses();
                c.extend(b.to_dnf_clauses());
                c
            }
            DepExpr::And(a, b) => {
                let mut out = Vec::new();
                for ac in a.to_dnf_clauses() {
                    for bc in b.to_dnf_clauses() {
                        let mut merged = ac.clone();
                        for item in &bc {
                            if !merged.contains(item) {
                                merged.push(item.clone());
                            }
                        }
                        out.push(merged);
                    }
                }
                out
            }
        }
    }
}

/// Parse a `depends=` attribute string into a `DepExpr`.
/// Supports full nesting with parentheses.
pub fn parse_dep_expr(s: &str) -> DepExpr {
    let b = s.as_bytes();
    let (expr, _) = parse_or(b, 0);
    expr
}

fn skip_ws(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && (b[i] == b' ' || b[i] == b'\t') {
        i += 1;
    }
    i
}

fn parse_or(b: &[u8], i: usize) -> (DepExpr, usize) {
    let (mut lhs, mut i) = parse_and(b, skip_ws(b, i));
    loop {
        let j = skip_ws(b, i);
        if j < b.len() && b[j] == b',' {
            let (rhs, k) = parse_and(b, skip_ws(b, j + 1));
            lhs = DepExpr::Or(Box::new(lhs), Box::new(rhs));
            i = k;
        } else {
            return (lhs, j);
        }
    }
}

fn parse_and(b: &[u8], i: usize) -> (DepExpr, usize) {
    let (mut lhs, mut i) = parse_atom(b, skip_ws(b, i));
    loop {
        let j = skip_ws(b, i);
        if j < b.len() && b[j] == b'+' {
            let (rhs, k) = parse_atom(b, skip_ws(b, j + 1));
            lhs = DepExpr::And(Box::new(lhs), Box::new(rhs));
            i = k;
        } else {
            return (lhs, j);
        }
    }
}

fn parse_atom(b: &[u8], i: usize) -> (DepExpr, usize) {
    let i = skip_ws(b, i);
    if i < b.len() && b[i] == b'(' {
        let (inner, j) = parse_or(b, i + 1);
        let j = skip_ws(b, j);
        let j = if j < b.len() && b[j] == b')' {
            j + 1
        } else {
            j
        };
        return (inner, j);
    }
    let start = i;
    let mut j = i;
    while j < b.len() {
        match b[j] {
            b'+' | b',' | b'(' | b')' | b' ' | b'\t' => break,
            _ => j += 1,
        }
    }
    let ident = std::str::from_utf8(&b[start..j])
        .unwrap_or("")
        .trim()
        .to_owned();
    (DepExpr::Atom(ident), j)
}

// -- API set -------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ApiSet {
    pub vulkan: bool,
    pub vulkansc: bool,
    pub vulkanbase: bool,
}

impl ApiSet {
    pub fn all() -> Self {
        ApiSet {
            vulkan: true,
            vulkansc: true,
            vulkanbase: true,
        }
    }
    pub fn vulkan_only() -> Self {
        ApiSet {
            vulkan: true,
            ..Default::default()
        }
    }
    pub fn parse(s: &str) -> Self {
        let mut a = ApiSet::default();
        for part in s.split(',') {
            match part.trim() {
                "vulkan" => a.vulkan = true,
                "vulkansc" => a.vulkansc = true,
                "vulkanbase" => a.vulkanbase = true,
                _ => {}
            }
        }
        a
    }
}

// -- C type --------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CType {
    pub is_const: bool,
    pub base: String,
    pub pointer_depth: u8,
    pub is_array: Option<String>,
}

impl CType {
    pub fn simple(name: &str) -> Self {
        CType {
            is_const: false,
            base: name.to_owned(),
            pointer_depth: 0,
            is_array: None,
        }
    }
}

// -- Member --------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub ty: CType,
    pub optional: bool,
    pub len: Option<String>,
    pub values: Option<String>,
    pub api: Option<ApiSet>,
    pub comment: Option<String>,
}

// -- Deprecation info ----------------------------------------------------------

/// Aggregates all forms of Vulkan deprecation/supersession found in the XML.
#[derive(Debug, Clone, Default)]
pub struct DeprecationInfo {
    pub deprecated: Option<String>,
    pub superseded_by: Option<String>,
    pub obsoleted_by: Option<String>,
    pub promoted_to: Option<String>,
}

impl DeprecationInfo {
    pub fn is_any(&self) -> bool {
        self.deprecated.is_some()
            || self.superseded_by.is_some()
            || self.obsoleted_by.is_some()
            || self.promoted_to.is_some()
    }
    pub fn note(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(ref s) = self.superseded_by {
            parts.push(format!("superseded by `{s}`"));
        }
        if let Some(ref s) = self.obsoleted_by {
            parts.push(format!("obsoleted by `{s}`"));
        }
        if let Some(ref s) = self.promoted_to {
            parts.push(format!("promoted to `{s}`"));
        }
        if let Some(ref s) = self.deprecated {
            if s != "true" && s != "aliased" {
                parts.push(s.clone());
            } else {
                parts.push("deprecated".into());
            }
        }
        if parts.is_empty() {
            "deprecated".into()
        } else {
            parts.join("; ")
        }
    }
}

// -- IR nodes ------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Typedef {
    pub name: String,
    pub alias: Option<String>,
    pub ty: Option<String>,
    pub kind: TypedefKind,
    pub api: ApiSet,
    pub comment: Option<String>,
    pub dep: Option<DepExpr>,
    pub depr: DeprecationInfo,
    pub provided_by: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedefKind {
    Alias,
    Basetype,
    Handle,
    Bitmask,
    FuncPointer,
    Define,
    /// An opaque C struct declared as `struct Foo;` - emitted as `extern type` or `u8` placeholder.
    OpaqueExtern,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub alias: Option<String>,
    pub members: Vec<Member>,
    pub is_union: bool,
    pub returned_only: bool,
    pub struct_extends: Vec<String>,
    pub api: ApiSet,
    pub comment: Option<String>,
    pub dep: Option<DepExpr>,
    pub provided_by: Vec<String>,
    pub depr: DeprecationInfo,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub value: EnumValue,
    pub comment: Option<String>,
    pub api: Option<ApiSet>,
    pub depr: DeprecationInfo,
    pub alias: Option<String>,
    pub provided_by: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum EnumValue {
    Integer(i64),
    Hex(u64),
    BitPos(u32),
    /// Khronos extension-offset encoding:
    ///   value = 1_000_000_000 + (extnumber − 1) * 1000 + offset
    Offset {
        extnumber: u32,
        offset: u32,
        negative: bool,
    },
    Alias(String),
    Expr(String),
}

impl EnumValue {
    pub fn resolve(&self) -> Option<i64> {
        match self {
            EnumValue::Integer(n) => Some(*n),
            EnumValue::Hex(n) => Some(*n as i64),
            EnumValue::BitPos(p) => Some(1i64 << p),
            EnumValue::Offset {
                extnumber,
                offset,
                negative,
            } => {
                let v = 1_000_000_000i64 + (*extnumber as i64 - 1) * 1000 + *offset as i64;
                Some(if *negative { -v } else { v })
            }
            EnumValue::Alias(_) | EnumValue::Expr(_) => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub alias: Option<String>,
    pub variants: Vec<EnumVariant>,
    pub is_bitmask: bool,
    pub bit_width: u8,
    pub api: ApiSet,
    pub comment: Option<String>,
    pub dep: Option<DepExpr>,
    pub provided_by: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub alias: Option<String>,
    pub return_type: CType,
    pub params: Vec<Member>,
    pub api: ApiSet,
    pub comment: Option<String>,
    pub dep: Option<DepExpr>,
    pub provided_by: Vec<String>,
    pub depr: DeprecationInfo,
    pub success_codes: Vec<String>,
    pub error_codes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub value: String,
    pub ty: String,
    pub comment: Option<String>,
    pub provided_by: Vec<String>,
    pub depr: DeprecationInfo,
    pub alias: Option<String>,
}

// -- Feature / Extension -------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Feature {
    pub name: String,
    pub api: ApiSet,
    pub number: String,
    pub depends: Option<DepExpr>,
    pub requires: Vec<Require>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub name: String,
    pub number: u32,
    pub depends: Option<DepExpr>,
    pub api: ApiSet,
    pub supported: String,
    pub requires: Vec<Require>,
    pub comment: Option<String>,
    pub depr: DeprecationInfo,
    pub requires_core: Option<String>,
    pub ext_type: Option<String>,
}

impl Extension {
    /// True when this extension should be excluded from code generation output
    /// (Cargo.toml features, DOT graph enabled nodes, validation checks).
    pub fn is_disabled(&self) -> bool {
        if self.supported == "disabled" || self.supported.is_empty() {
            return true;
        }

        // Must look like a Vulkan extension name OR a video codec header.
        // Video headers are processed during parsing but excluded from code-gen
        // features after being remapped.
        if self.name.starts_with("VK_") || self.name.starts_with("VKSC_") {
            return false;
        }
        // vulkan_video_* names are allowed through parse but excluded from
        // feature emission (they're internal; items get remapped to VK_KHR_*)
        if self.name.starts_with("vulkan_video_") {
            return false; // not disabled for *parsing*, but see is_video_header()
        }

        true // Unknown namespace -> disabled
    }

    /// True if this is a video.xml codec header pseudo-extension (e.g. `vulkan_video_codec_h264std`).
    /// These are processed during parsing to populate types/constants but are NOT
    /// emitted as Cargo features - their items are remapped to real VK_KHR_* features.
    pub fn is_video_header(&self) -> bool {
        self.name.starts_with("vulkan_video_")
    }
}

#[derive(Debug, Clone, Default)]
pub struct Require {
    pub depends: Option<DepExpr>,
    pub api: Option<ApiSet>,
    pub types: Vec<String>,
    pub commands: Vec<String>,
    pub enums: Vec<RequireEnum>,
}

#[derive(Debug, Clone)]
pub struct RequireEnum {
    pub extends: Option<String>,
    pub name: String,
    pub value: Option<EnumValue>,
    pub alias: Option<String>,
    pub comment: Option<String>,
    pub depr: DeprecationInfo,
    pub api: Option<ApiSet>,
    pub extnumber: Option<u32>,
}

// -- Registry ------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Registry {
    pub typedefs: IndexMap<String, Typedef>,
    pub structs: IndexMap<String, Struct>,
    pub enums: IndexMap<String, Enum>,
    pub commands: IndexMap<String, Command>,
    pub constants: IndexMap<String, Constant>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
}

impl Registry {
    pub fn feature_deps(&self) -> IndexMap<String, Vec<String>> {
        let mut map: IndexMap<String, Vec<String>> = IndexMap::new();
        for feat in &self.features {
            let deps = feat.depends.as_ref().map(|d| d.atoms()).unwrap_or_default();
            map.entry(feat.name.clone()).or_default().extend(deps);
        }
        for ext in &self.extensions {
            // Skip disabled extensions AND video codec headers (remapped, not emitted as features)
            if ext.is_disabled() || ext.is_video_header() {
                continue;
            }
            let deps = ext.depends.as_ref().map(|d| d.atoms()).unwrap_or_default();
            let entry = map.entry(ext.name.clone()).or_default();
            for dep in deps {
                if !entry.contains(&dep) {
                    entry.push(dep);
                }
            }
        }
        map
    }

    pub fn all_feature_names(&self) -> Vec<String> {
        let mut v: Vec<String> = self.features.iter().map(|f| f.name.clone()).collect();
        v.extend(
            self.extensions
                .iter()
                .filter(|e| !e.is_disabled() && !e.is_video_header())
                .map(|e| e.name.clone()),
        );
        v
    }

    pub fn remap_video_header_names(&mut self) {
        let remap = video_header_remap();
        let fix = |v: &mut Vec<String>| {
            for s in v.iter_mut() {
                if let Some(&r) = remap.get(s.as_str()) {
                    *s = r.to_owned();
                }
            }
        };
        for td in self.typedefs.values_mut() {
            fix(&mut td.provided_by);
        }
        for s in self.structs.values_mut() {
            fix(&mut s.provided_by);
        }
        for e in self.enums.values_mut() {
            fix(&mut e.provided_by);
            for v in e.variants.iter_mut() {
                fix(&mut v.provided_by);
            }
        }
        for c in self.commands.values_mut() {
            fix(&mut c.provided_by);
        }
        // Constants include SPEC_VERSION / EXTENSION_NAME from video headers
        for c in self.constants.values_mut() {
            fix(&mut c.provided_by);
        }
    }
}

pub fn video_header_remap() -> IndexMap<&'static str, &'static str> {
    // Maps video.xml extension names -> the VK_KHR_* feature that gates them.
    // "vulkan_video_codecs_common" covers VkVideoCodecOperationFlagBitsKHR etc.
    [
        ("vulkan_video_codecs_common", "VK_KHR_video_queue"),
        ("vulkan_video_codec_h264std", "VK_KHR_video_queue"),
        (
            "vulkan_video_codec_h264std_decode",
            "VK_KHR_video_decode_h264",
        ),
        (
            "vulkan_video_codec_h264std_encode",
            "VK_KHR_video_encode_h264",
        ),
        ("vulkan_video_codec_h265std", "VK_KHR_video_queue"),
        (
            "vulkan_video_codec_h265std_decode",
            "VK_KHR_video_decode_h265",
        ),
        (
            "vulkan_video_codec_h265std_encode",
            "VK_KHR_video_encode_h265",
        ),
        ("vulkan_video_codec_av1std", "VK_KHR_video_queue"),
        (
            "vulkan_video_codec_av1std_decode",
            "VK_KHR_video_decode_av1",
        ),
        (
            "vulkan_video_codec_av1std_encode",
            "VK_KHR_video_encode_av1",
        ),
        ("vulkan_video_codec_vp9std", "VK_KHR_video_queue"),
        (
            "vulkan_video_codec_vp9std_decode",
            "VK_KHR_video_decode_vp9",
        ),
        (
            "vulkan_video_codec_vp9std_encode",
            "VK_KHR_video_encode_vp9",
        ),
    ]
    .iter()
    .cloned()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clauses(s: &str) -> Vec<Vec<String>> {
        parse_dep_expr(s).to_dnf_clauses()
    }

    #[test]
    fn simple_atom() {
        assert_eq!(
            parse_dep_expr("VK_VERSION_1_0"),
            DepExpr::Atom("VK_VERSION_1_0".into())
        );
    }
    #[test]
    fn simple_or() {
        let c = clauses("VK_VERSION_1_1,VK_VERSION_1_2");
        assert_eq!(c.len(), 2);
    }
    #[test]
    fn simple_and() {
        let c = clauses("VK_KHR_a+VK_KHR_b");
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 2);
    }
    #[test]
    fn nested_parens() {
        // ((A,B)+C),D  ->  3 clauses: [A,C], [B,C], [D]
        let c = clauses("((VK_KHR_a,VK_VERSION_1_1)+VK_KHR_c),VK_VERSION_1_2");
        assert_eq!(c.len(), 3, "got: {:?}", c);
    }
    #[test]
    fn real_world_complex() {
        let s = "((VK_KHR_get_physical_device_properties2,VK_VERSION_1_1)+VK_KHR_depth_stencil_resolve),VK_VERSION_1_2";
        let c = clauses(s);
        assert_eq!(c.len(), 3, "got: {:?}", c);
        assert!(c.iter().any(|cl| cl == &["VK_VERSION_1_2"]));
    }
    #[test]
    fn offset_enum_value() {
        assert_eq!(
            EnumValue::Offset {
                extnumber: 272,
                offset: 0,
                negative: false
            }
            .resolve(),
            Some(1_000_271_000)
        );
        assert_eq!(
            EnumValue::Offset {
                extnumber: 272,
                offset: 1,
                negative: false
            }
            .resolve(),
            Some(1_000_271_001)
        );
    }
}
