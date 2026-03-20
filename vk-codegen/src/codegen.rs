//! Code generator: IR → Rust source files for `vk-sys`.

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

use crate::cfggen::*;
use crate::ir::*;
use crate::types::{c_type_to_rust, const_rust_type, ctype_to_rust_str};

// ── Public API ────────────────────────────────────────────────────────────────

pub struct GeneratedFiles {
    pub cargo_toml: String,
    pub lib_rs: String,
    pub types_rs: String,
    pub enums_rs: String,
    pub consts_rs: String,
    pub commands_rs: String,
    pub loader_rs: String,
    pub validation_rs: String,
    pub dot_graph: String,
}

pub fn generate(reg: &Registry) -> GeneratedFiles {
    GeneratedFiles {
        cargo_toml: gen_cargo_toml(reg),
        lib_rs: gen_lib_rs(),
        types_rs: gen_types_rs(reg),
        enums_rs: gen_enums_rs(reg),
        consts_rs: gen_consts_rs(reg),
        commands_rs: gen_commands_rs(reg),
        loader_rs: gen_loader_rs(reg),
        validation_rs: gen_validation_rs(reg),
        dot_graph: gen_dot_graph(reg),
    }
}

// ── Cargo.toml ────────────────────────────────────────────────────────────────

fn gen_cargo_toml(reg: &Registry) -> String {
    let feature_deps = reg.feature_deps();
    let mut lines: Vec<String> = vec![
        "[package]".into(),
        "name = \"vk-sys\"".into(),
        "version = \"0.1.0\"".into(),
        "edition = \"2021\"".into(),
        "description = \"Auto-generated Vulkan FFI (vk-codegen)\"".into(),
        "".into(),
        "[dependencies]".into(),
        "".into(),
        "[features]".into(),
    ];

    lines.push("# Core Vulkan versions".into());
    for feat in &reg.features {
        let deps = feature_deps.get(&feat.name).cloned().unwrap_or_default();
        lines.push(format!("{} = [{}]", feat.name, toml_feat_list(&deps)));
    }
    lines.push("".into());

    lines.push("# Extensions".into());
    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }
        let deps = feature_deps.get(&ext.name).cloned().unwrap_or_default();
        if let Some(ref s) = ext.depr.superseded_by {
            lines.push(format!("# superseded by: {s}"));
        }
        if let Some(ref s) = ext.depr.obsoleted_by {
            lines.push(format!("# obsoleted by: {s}"));
        }
        if let Some(ref s) = ext.depr.promoted_to {
            lines.push(format!("# promoted to: {s}"));
        }
        if let Some(ref s) = ext.depr.deprecated {
            lines.push(format!("# deprecated: {s}"));
        }
        lines.push(format!("{} = [{}]", ext.name, toml_feat_list(&deps)));
    }

    lines.join("\n")
}

fn toml_feat_list(deps: &[String]) -> String {
    deps.iter()
        .map(|d| format!("\"{d}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

// ── lib.rs ────────────────────────────────────────────────────────────────────

fn gen_lib_rs() -> String {
    let ts = quote! {
        //! Auto-generated Vulkan FFI (`vk-sys`).
        //!
        //! Generated from `vk.xml` + `video.xml` by `vk-codegen`.
        //!
        //! Every item is gated by `#[cfg(feature = "...")]` mirroring the Vulkan
        //! API version or extension name exactly.  Enabling a feature via Cargo
        //! automatically pulls in all transitive dependencies.
        //!
        //! ```toml
        //! [dependencies.vk-sys]
        //! features = ["VK_VERSION_1_3", "VK_KHR_swapchain"]
        //! ```
        #![no_std]
        #![allow(
            non_snake_case, non_camel_case_types, non_upper_case_globals,
            dead_code, unused_imports, clippy::all
        )]

        pub mod consts;
        pub mod enums;
        pub mod types;
        pub mod commands;
        pub mod loader;
        pub mod validation;

        pub use consts::*;
        pub use enums::*;
        pub use types::*;
        pub use commands::*;
        pub use loader::*;
    };
    pretty(ts)
}

// ── types.rs ─────────────────────────────────────────────────────────────────

fn gen_types_rs(reg: &Registry) -> String {
    let mut parts: Vec<String> = Vec::new();

    // File header (not run through quote! to avoid any parse issues)
    parts.push(
        "//! Struct, union, handle, and typedef definitions.\n\
         #[allow(unused_imports)] use core::ffi::{c_char, c_void};\n\
         #[allow(unused_imports)] use crate::consts::*;\n\
         #[allow(unused_imports)] use crate::enums::*;\n"
            .to_owned(),
    );

    for td in reg.typedefs.values() {
        let s = gen_typedef_str(td);
        if !s.is_empty() {
            parts.push(s);
        }
    }
    for s in reg.structs.values() {
        let text = gen_struct_str(s, reg);
        if !text.is_empty() {
            parts.push(text);
        }
    }

    parts.join("\n")
}

/// Generate a typedef as a raw string (avoids proc-macro parse issues with
/// complex multi-word type names).
fn gen_typedef_str(td: &Typedef) -> String {
    if td.provided_by.is_empty() {
        return String::new();
    }

    let cfg = cfg_any_str(&td.provided_by);
    let url = refpage_url(&td.name);
    let doc = format!("/// [`{n}`]({url})", n = td.name);
    let depr = depr_attr_str(&td.depr);
    let name = &td.name;

    match td.kind {
        TypedefKind::Handle => {
            let inner = td.alias.as_deref().unwrap_or("u64");
            format!("{doc}\n{cfg}\n{depr}pub type {name} = {inner};\n")
        }
        TypedefKind::Bitmask => {
            if let Some(ref alias) = td.alias {
                format!("{cfg}\npub type {name} = {alias};\n")
            } else if let Some(ref ty) = td.ty {
                let mapped = c_type_to_rust(ty);
                let rty = if mapped.is_empty() {
                    ty.as_str()
                } else {
                    mapped
                };
                format!("{doc}\n{cfg}\npub type {name} = {rty};\n")
            } else {
                format!("{cfg}\npub type {name} = u32;\n")
            }
        }
        TypedefKind::Basetype => {
            if let Some(ref ty) = td.ty {
                let mapped = c_type_to_rust(ty);
                let rty = if mapped.is_empty() {
                    ty.as_str()
                } else {
                    mapped
                };
                format!("{doc}\n{cfg}\npub type {name} = {rty};\n")
            } else {
                String::new()
            }
        }
        TypedefKind::Alias => {
            if let Some(ref alias) = td.alias {
                format!("{cfg}\n{depr}pub type {name} = {alias};\n")
            } else {
                String::new()
            }
        }
        TypedefKind::FuncPointer => {
            // ty encodes: "<return_c_type>|<pname>:<rust_type>,..."
            // We decode it and emit: pub type PFN_foo = Option<unsafe extern "system" fn(...) -> Ret>;
            if let Some(ref encoded) = td.ty {
                let (ret_c, params_enc) = encoded.split_once('|').unwrap_or((encoded, ""));
                let ret_rust = {
                    let mapped = c_type_to_rust(ret_c.trim());
                    if mapped.is_empty() {
                        ret_c.trim().to_owned()
                    } else {
                        mapped.to_owned()
                    }
                };
                let ret_str = if ret_rust == "core::ffi::c_void" || ret_rust == "void" {
                    String::new()
                } else {
                    format!(" -> {ret_rust}")
                };
                let params_str: String = if params_enc.is_empty() {
                    String::new()
                } else {
                    params_enc
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|p| {
                            // "pname:ptype" or just ":ptype" if name was empty
                            let (pn, pt) = p.split_once(':').unwrap_or(("_", p));
                            let pname = if pn.is_empty() { "_" } else { pn };
                            format!("{pname}: {pt}")
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                format!(
                    "{doc}\n{cfg}\n{depr}\
                     pub type {name} = Option<unsafe extern \"system\" fn({params_str}){ret_str}>;\n"
                )
            } else {
                String::new()
            }
        }
        TypedefKind::OpaqueExtern => {
            // Platform-specific opaque handle types (HWND, wl_display, Display, etc.).
            // Emitted as repr(transparent) newtype wrappers over *mut c_void so that:
            //   • Each type is nominally distinct (HWND ≠ wl_display at the type level)
            //   • Fields keep their named type rather than becoming raw *mut c_void
            //   • Implements Copy, Clone, Debug trivially via the newtype
            //   • Default is TypeName(core::ptr::null_mut()) — clear and const-safe
            format!(
                "{doc}\n/// Opaque platform handle — always used as a raw pointer.\n\
                 {cfg}\n#[repr(transparent)]\n\
                 #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
                 pub struct {name}(pub *mut core::ffi::c_void);\n\
                 {cfg}\nimpl {name} {{\n\
                     pub const NULL: Self = Self(core::ptr::null_mut());\n\
                 }}\n"
            )
        }
        TypedefKind::Define => {
            // Defines are emitted in consts.rs, not types.rs
            String::new()
        }
    }
}
fn gen_struct_str(s: &Struct, reg: &Registry) -> String {
    if s.provided_by.is_empty() {
        return String::new();
    }

    let cfg = cfg_any_str(&s.provided_by);
    let url = refpage_url(&s.name);
    let doc = format!("/// [`{n}`]({url})", n = s.name);
    let depr = depr_attr_str(&s.depr);
    let name = &s.name;

    if let Some(ref alias) = s.alias {
        return format!("{cfg}\n{depr}pub type {name} = {alias};\n");
    }

    // Find sType default value — keep the full VK_STRUCTURE_TYPE_* constant name.
    let stype_default: Option<String> = s
        .members
        .iter()
        .find(|m| m.name == "sType")
        .and_then(|m| m.values.as_deref())
        .map(|vals| vals.split(',').next().unwrap_or("").trim().to_owned())
        .filter(|v| !v.is_empty())
        .map(|v| format!("VkStructureType::{v}"));

    // De-duplicate members that appear with different `api=` attributes.
    // Keep the first occurrence (vulkan variant wins over vulkansc).
    let deduped_members: Vec<&Member> = {
        let mut seen: Vec<&str> = Vec::new();
        let mut out: Vec<&Member> = Vec::new();
        for m in &s.members {
            if m.name.is_empty() {
                continue;
            }
            if seen.contains(&m.name.as_str()) {
                continue;
            }
            seen.push(&m.name);
            out.push(m);
        }
        out
    };

    // Build field declarations and typed zero defaults.
    let mut fields = String::new();
    let mut defaults = String::new();
    let mut needs_unsafe = false;

    for m in &deduped_members {
        let fname = sanitize_ident(&m.name);
        let ftype = ctype_to_rust_str(&m.ty);
        let fdoc = m.comment.as_deref().unwrap_or("");

        if !fdoc.is_empty() {
            fields.push_str(&format!("    /// {fdoc}\n"));
        }
        fields.push_str(&format!("    pub {fname}: {ftype},\n"));

        // sType is initialised by the explicit stype_default expression
        if m.name == "sType" && stype_default.is_some() {
            continue;
        }

        let (def, is_const_safe) = member_default_const(m, reg);
        if !is_const_safe {
            needs_unsafe = true;
        }
        defaults.push_str(&format!("        {fname}: {def},\n"));
    }

    let kw = if s.is_union { "union" } else { "struct" };

    // All Vulkan structs are C FFI types with no heap allocation.
    // Deriving Copy on all structs means:
    //   • Union fields (which must be Copy) work without ManuallyDrop
    //   • Users can cheaply copy them on the stack (expected FFI usage)
    //   • Arrays of structs like [VkMemoryType; N] work in const contexts
    // Unions also derive Copy+Clone but NOT Debug (cannot auto-derive for union).
    // A manual Debug impl is provided for unions instead.
    let derive = if s.is_union {
        "#[derive(Copy, Clone)]"
    } else {
        "#[derive(Debug, Clone, Copy)]"
    };

    // Rebuild field string for unions — plain types, no ManuallyDrop wrapper
    let fields = if s.is_union {
        let mut union_fields = String::new();
        for m in &deduped_members {
            let fname = sanitize_ident(&m.name);
            let ftype = ctype_to_rust_str(&m.ty);
            let fdoc = m.comment.as_deref().unwrap_or("");
            if !fdoc.is_empty() {
                union_fields.push_str(&format!("    /// {fdoc}\n"));
            }
            union_fields.push_str(&format!("    pub {fname}: {ftype},\n"));
        }
        union_fields
    } else {
        fields
    };

    // Manual Debug impl for unions (auto-derive not supported)
    let union_debug = if s.is_union {
        let first_field = deduped_members
            .first()
            .map(|m| sanitize_ident(&m.name))
            .unwrap_or("_");
        format!(
            "{cfg}\nimpl core::fmt::Debug for {name} {{\n\
             fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{\n\
             // SAFETY: all union variants are valid bit patterns for their types.\n\
             f.debug_struct(\"{name}\")\n\
              .field(\"{first_field}\", unsafe {{ &self.{first_field} }})\n\
              .finish()\n\
             }}\n\
             }}\n"
        )
    } else {
        String::new()
    };

    // Unions get a DEFAULT impl using zeroed first field.
    // All Vulkan union fields are Copy C POD types, so zeroing is always valid.
    // Using `unsafe { core::mem::zeroed() }` in a const context is valid on Rust ≥ 1.75
    // (stabilised via const_mut_refs + const_precise_live_drops).
    let union_impl = if s.is_union {
        if let Some(first) = deduped_members.first() {
            let fname = sanitize_ident(&first.name);
            let ftype = ctype_to_rust_str(&first.ty);
            format!(
                "{cfg}\nimpl {name} {{\n\
                 pub const DEFAULT: Self = unsafe {{\n\
                 Self {{ {fname}: core::mem::zeroed::<{ftype}>() }}\n\
                 }};\n\
                 #[inline]\n\
                 pub const fn new() -> Self {{ Self::DEFAULT }}\n\
                 }}\n"
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let impl_block = if !s.is_union {
        let body = if let Some(ref sv) = stype_default {
            if needs_unsafe {
                format!(
                    "    // SAFETY: zeroed fields are valid C-layout types.\n\
                     pub fn new() -> Self {{\n\
                         unsafe {{ Self {{ sType: {sv},\n{defaults}        }} }}\n\
                     }}"
                )
            } else {
                format!(
                    "    pub const DEFAULT: Self = Self {{ sType: {sv},\n{defaults}    }};\n\
                     \n    #[inline]\n    pub const fn new() -> Self {{ Self::DEFAULT }}"
                )
            }
        } else if needs_unsafe {
            format!(
                "    // SAFETY: zeroed fields are valid C-layout types.\n\
                 pub fn new() -> Self {{\n\
                     unsafe {{ Self {{\n{defaults}        }} }}\n\
                 }}"
            )
        } else {
            format!(
                "    pub const DEFAULT: Self = Self {{\n{defaults}    }};\n\
                 \n    #[inline]\n    pub const fn new() -> Self {{ Self::DEFAULT }}"
            )
        };
        format!("{cfg}\nimpl {name} {{\n{body}\n}}\n")
    } else {
        String::new()
    };

    format!(
        "{doc}\n{cfg}\n{depr}#[repr(C)]\n{derive}\npub {kw} {name} {{\n{fields}}}\n\n{impl_block}{union_impl}{union_debug}"
    )
}

/// Classify a base type name against the registry to determine how to zero-initialize it.
/// Returns `(TypeClass, canonical_name)` — the canonical name is the resolved type
/// name to use in default expressions (follows aliases to the defining type).
#[derive(Debug, Clone, Copy, PartialEq)]
enum TypeClass {
    /// Primitive Rust integer/float/bool or typedef alias to one.
    PrimitiveAlias,
    /// Platform opaque handle newtype (repr(transparent) over *mut c_void).
    /// Default is `TypeName::NULL`.
    NullMutAlias,
    /// Function pointer type alias — emitted as `Option<fn(...)>`, defaults to `None`.
    OptionType,
    /// repr(transparent) newtype over an integer (enum or bitmask).
    /// The returned canonical name is the actual newtype, not an alias name.
    EnumNewtype,
    /// A non-union struct that has its own `DEFAULT` constant.
    StructWithDefault,
    /// A union or unknown type that needs unsafe zeroed.
    StructUnsafe,
}

/// Resolve a type name through aliases to find the canonical TypeClass and name.
fn classify_type(base: &str, reg: &Registry) -> (TypeClass, String) {
    classify_type_inner(base, reg, 0)
}

fn classify_type_inner(base: &str, reg: &Registry, depth: u8) -> (TypeClass, String) {
    // Guard against infinite alias loops
    if depth > 16 {
        return (TypeClass::StructUnsafe, base.to_owned());
    }

    // Primitive Rust types
    match base {
        "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "usize"
        | "isize" | "f32" | "f64" | "bool" | "core::ffi::c_void" | "core::ffi::c_char"
        | "core::ffi::c_int" => {
            return (TypeClass::PrimitiveAlias, base.to_owned());
        }
        _ => {}
    }

    if let Some(td) = reg.typedefs.get(base) {
        return match td.kind {
            TypedefKind::Handle => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Basetype => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Bitmask => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Alias => {
                // Follow alias chain — return the canonical name of the target
                if let Some(ref target) = td.alias {
                    classify_type_inner(target, reg, depth + 1)
                } else {
                    (TypeClass::PrimitiveAlias, base.to_owned())
                }
            }
            TypedefKind::FuncPointer => (TypeClass::OptionType, base.to_owned()),
            TypedefKind::OpaqueExtern => (TypeClass::NullMutAlias, base.to_owned()),
            TypedefKind::Define => (TypeClass::PrimitiveAlias, base.to_owned()),
        };
    }

    if let Some(e) = reg.enums.get(base) {
        // If this enum is itself an alias (e.g. VkScopeNV = VkScopeKHR),
        // follow the alias to the canonical defining enum so we emit Foo(0)
        // with the real newtype constructor, not a type-alias name.
        if let Some(ref target) = e.alias {
            // Only recurse if the target is different to avoid infinite loops
            if target != base {
                let (cls, canonical) = classify_type_inner(target, reg, depth + 1);
                // Return the canonical name regardless of what we found
                return (cls, canonical);
            }
        }
        return (TypeClass::EnumNewtype, base.to_owned());
    }

    if reg.structs.contains_key(base) {
        // All structs and unions now have a const DEFAULT.
        // Unions use `unsafe { zeroed() }` for their first field, which is const-safe on Rust ≥ 1.75.
        return (TypeClass::StructWithDefault, base.to_owned());
    }

    (TypeClass::StructUnsafe, base.to_owned())
}

/// Returns `(default_expr, is_const_safe)` for a struct member's zero default.
///
/// Rules:
///   - `expr` is NEVER wrapped in `unsafe { }` — the caller wraps the whole struct literal.
///   - `is_const_safe = true`  → expr is valid in `const` context
///   - `is_const_safe = false` → expr requires being inside an `unsafe` block
///
/// Type dispatch:
///   - Pointers         → `core::ptr::null()`/`null_mut()`  (const-safe)
///   - Option<T>        → `None`                             (const-safe)
///   - Primitive arrays → `[0u8; N]` etc.                   (const-safe)
///   - Struct arrays    → `[T::DEFAULT; N]`                  (const-safe if T has DEFAULT)
///   - Enum arrays      → `[T(0); N]`                        (const-safe)
///   - Unknown arrays   → `core::mem::zeroed()`              (needs unsafe wrapper)
///   - Primitive scalars→ `0`, `0.0f32`, `false`             (const-safe)
///   - Enum newtypes    → `T(0)`                             (const-safe)
///   - Structs w/ DEFAULT → `T::DEFAULT`                     (const-safe)
///   - Option fn-ptrs   → `None`                             (const-safe)
///   - Unknown          → `core::mem::zeroed()`              (needs unsafe wrapper)
fn member_default_const(m: &Member, reg: &Registry) -> (String, bool) {
    // ── Pointer fields ──────────────────────────────────────────────────────────
    if m.ty.pointer_depth > 0 {
        let expr = if m.ty.is_const {
            "core::ptr::null()"
        } else {
            "core::ptr::null_mut()"
        };
        return (expr.into(), true);
    }

    // ── Static array fields ─────────────────────────────────────────────────────
    if let Some(ref size) = m.ty.is_array {
        let size_expr = if size.parse::<u64>().is_ok() {
            size.clone()
        } else {
            format!("{size} as usize")
        };

        let elem_base = m.ty.base.as_str();
        let elem_rust = {
            let mapped = crate::types::c_type_to_rust(elem_base);
            if mapped.is_empty() { elem_base } else { mapped }
        };

        // Primitive element types
        let prim_zero = match elem_rust {
            "u8" => Some("0u8"),
            "u16" => Some("0u16"),
            "u32" => Some("0u32"),
            "u64" => Some("0u64"),
            "i8" | "core::ffi::c_char" | "core::ffi::c_int" => Some("0i8"),
            "i16" => Some("0i16"),
            "i32" => Some("0i32"),
            "i64" => Some("0i64"),
            "usize" => Some("0usize"),
            "isize" => Some("0isize"),
            "f32" => Some("0.0f32"),
            "f64" => Some("0.0f64"),
            _ => None,
        };
        if let Some(z) = prim_zero {
            return (format!("[{z}; {size_expr}]"), true);
        }

        // Non-primitive: look up canonical name and class
        let (cls, canonical) = classify_type(elem_base, reg);
        let zero_elem: Option<(String, bool)> = match cls {
            // Use the canonical name so alias types (e.g. VkComponentTypeNV → VkComponentTypeKHR)
            // get the right constructor.
            TypeClass::EnumNewtype => Some((format!("{canonical}(0)"), true)),
            TypeClass::StructWithDefault => Some((format!("{canonical}::DEFAULT"), true)),
            TypeClass::PrimitiveAlias => Some(("0".into(), true)),
            TypeClass::NullMutAlias => Some((format!("{canonical}::NULL"), true)),
            TypeClass::OptionType => Some(("None".into(), true)),
            TypeClass::StructUnsafe => None,
        };
        return if let Some((z, safe)) = zero_elem {
            (format!("[{z}; {size_expr}]"), safe)
        } else {
            (
                format!("core::mem::zeroed::<[{elem_base}; {size_expr}]>()"),
                false,
            )
        };
    }

    // ── Scalar fields ───────────────────────────────────────────────────────────
    let t = ctype_to_rust_str(&m.ty);

    // Option<T> (function pointers)
    if t.starts_with("Option<") {
        return ("None".into(), true);
    }

    // Primitive Rust scalars
    match t.as_str() {
        "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" | "usize"
        | "isize" => return ("0".into(), true),
        "f32" => return ("0.0f32".into(), true),
        "f64" => return ("0.0f64".into(), true),
        "bool" => return ("false".into(), true),
        "core::ffi::c_char" | "core::ffi::c_int" => return ("0".into(), true),
        _ => {}
    }

    // Registry-classified types — use canonical name for correct constructor
    let (cls, canonical) = classify_type(&m.ty.base, reg);
    match cls {
        TypeClass::PrimitiveAlias => ("0".into(), true),
        TypeClass::NullMutAlias => (format!("{canonical}::NULL"), true),
        TypeClass::OptionType => ("None".into(), true),
        TypeClass::EnumNewtype => (format!("{canonical}(0)"), true),
        TypeClass::StructWithDefault => (format!("{canonical}::DEFAULT"), true),
        TypeClass::StructUnsafe => (format!("core::mem::zeroed::<{t}>()"), false),
    }
}

fn sanitize_ident(s: &str) -> &str {
    match s {
        "type" => "type_",
        "match" => "match_",
        other => other,
    }
}

// ── enums.rs ─────────────────────────────────────────────────────────────────

fn gen_enums_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan enum and bitmask types.
        //!
        //! Enums are `repr(transparent)` newtypes over `i32`/`i64`.
        //! Bitmasks are `repr(transparent)` newtypes over `u32`/`u64`
        //! with `|`, `&`, `^`, `!` and compound-assignment operators.
    });
    for e in reg.enums.values() {
        ts.extend(gen_enum(e));
    }
    pretty(ts)
}

fn gen_enum(e: &Enum) -> TokenStream {
    let mut all_feats: Vec<String> = e.provided_by.clone();
    for v in &e.variants {
        for f in &v.provided_by {
            if !all_feats.contains(f) {
                all_feats.push(f.clone());
            }
        }
    }
    if all_feats.is_empty() && e.variants.is_empty() {
        return quote! {};
    }

    let cfg = cfg_any(&all_feats);
    let name = format_ident!("{}", &e.name);
    let url = refpage_url(&e.name);
    let doc = format!(" [`{n}`]({url})", n = e.name);

    if let Some(ref alias) = e.alias {
        let a = format_ident!("{}", alias);
        return quote! { #cfg pub type #name = #a; };
    }

    if e.is_bitmask {
        return gen_bitmask_type(e, cfg, name, doc, &all_feats);
    }

    let inner = if e.bit_width == 64 {
        quote! {i64}
    } else {
        quote! {i32}
    };
    let mut var_ts = TokenStream::new();
    let mut seen: HashSet<String> = HashSet::new();

    for v in &e.variants {
        if !seen.insert(v.name.clone()) {
            continue;
        }
        let vcfg = if v.provided_by.is_empty() || v.provided_by == all_feats {
            quote! {}
        } else {
            cfg_any(&v.provided_by)
        };
        let vname = format_ident!("{}", &v.name);
        let vdoc = v.comment.as_deref().unwrap_or("");
        let vdepr = depr_attr(&v.depr);
        let val = enum_val_tokens(&v.value, false);
        var_ts.extend(quote! {
            #vcfg #[doc = #vdoc] #vdepr
            pub const #vname: Self = Self(#val);
        });
    }

    quote! {
        #[doc = #doc]
        #cfg
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct #name(pub #inner);

        #cfg
        impl #name { #var_ts }
    }
}

fn gen_bitmask_type(
    e: &Enum,
    cfg: TokenStream,
    name: proc_macro2::Ident,
    doc: String,
    all_feats: &[String],
) -> TokenStream {
    let inner = if e.bit_width == 64 {
        quote! {u64}
    } else {
        quote! {u32}
    };
    let mut bit_ts = TokenStream::new();
    let mut seen: HashSet<String> = HashSet::new();

    for v in &e.variants {
        if !seen.insert(v.name.clone()) {
            continue;
        }
        let vcfg = if v.provided_by.is_empty() || v.provided_by == all_feats {
            quote! {}
        } else {
            cfg_any(&v.provided_by)
        };
        let vname = format_ident!("{}", &v.name);
        let vdoc = v.comment.as_deref().unwrap_or("");
        let vdepr = depr_attr(&v.depr);
        let val = enum_val_tokens(&v.value, true);
        bit_ts.extend(quote! {
            #vcfg #[doc = #vdoc] #vdepr
            pub const #vname: Self = Self(#val);
        });
    }

    quote! {
        #[doc = #doc]
        #cfg
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct #name(pub #inner);

        #cfg
        impl #name {
            pub const EMPTY: Self = Self(0);
            #bit_ts
            #[inline] pub const fn contains(self, o: Self) -> bool { (self.0 & o.0) == o.0 }
            #[inline] pub const fn intersects(self, o: Self) -> bool { (self.0 & o.0) != 0 }
            #[inline] pub const fn is_empty(self) -> bool { self.0 == 0 }
        }
        #cfg impl core::ops::BitOr        for #name { type Output=Self; #[inline] fn bitor   (self, r:Self)->Self{Self(self.0|r.0)} }
        #cfg impl core::ops::BitOrAssign  for #name { #[inline] fn bitor_assign   (&mut self, r:Self){self.0|=r.0} }
        #cfg impl core::ops::BitAnd       for #name { type Output=Self; #[inline] fn bitand  (self, r:Self)->Self{Self(self.0&r.0)} }
        #cfg impl core::ops::BitAndAssign for #name { #[inline] fn bitand_assign  (&mut self, r:Self){self.0&=r.0} }
        #cfg impl core::ops::BitXor       for #name { type Output=Self; #[inline] fn bitxor  (self, r:Self)->Self{Self(self.0^r.0)} }
        #cfg impl core::ops::BitXorAssign for #name { #[inline] fn bitxor_assign  (&mut self, r:Self){self.0^=r.0} }
        #cfg impl core::ops::Not          for #name { type Output=Self; #[inline] fn not(self)->Self{Self(!self.0)} }
    }
}

fn enum_val_tokens(val: &EnumValue, unsigned: bool) -> TokenStream {
    match val {
        EnumValue::Integer(n) => {
            if unsigned {
                let l = Literal::u64_unsuffixed(*n as u64);
                quote! {#l}
            } else {
                let l = Literal::i64_unsuffixed(*n);
                quote! {#l}
            }
        }
        EnumValue::Hex(n) => {
            let l = Literal::u64_unsuffixed(*n);
            quote! {#l}
        }
        EnumValue::BitPos(p) => {
            let p = *p as u64;
            quote! { 1 << #p }
        }
        EnumValue::Offset {
            extnumber,
            offset,
            negative,
        } => {
            let v = 1_000_000_000i64 + (*extnumber as i64 - 1) * 1000 + *offset as i64;
            let v = if *negative { -v } else { v };
            let l = Literal::i64_unsuffixed(v);
            quote! {#l}
        }
        EnumValue::Alias(a) => {
            let a = format_ident!("{}", a);
            quote! { Self::#a.0 }
        }
        EnumValue::Expr(s) => normalize_expr(s, unsigned).parse().unwrap_or_else(|_| {
            let l = Literal::i64_unsuffixed(0);
            quote! {#l}
        }),
    }
}

fn normalize_expr(s: &str, unsigned: bool) -> String {
    let s = s.trim();
    if s == "(~0U)" || s == "~0U" || s == "(~0u)" || s == "~0u" {
        return if unsigned {
            "u32::MAX".into()
        } else {
            "-1i32 as i32".into()
        };
    }
    if s == "(~0ULL)" || s == "~0ULL" || s == "(~0ull)" || s == "~0ull" {
        return if unsigned {
            "u64::MAX".into()
        } else {
            "-1i64 as i64".into()
        };
    }
    if s.starts_with("(~") || s.starts_with('~') {
        return if unsigned {
            "u32::MAX".into()
        } else {
            "-1i32 as i32".into()
        };
    }
    s.to_owned()
}

// ── consts.rs ─────────────────────────────────────────────────────────────────

fn gen_consts_rs(reg: &Registry) -> String {
    // consts.rs is built as a mix of raw strings (for defines/const fns) and
    // quote!-generated tokens (for API constants). We collect raw parts and join.
    let mut raw_parts: Vec<String> = Vec::new();
    raw_parts.push(
        "//! Vulkan API constants, version helpers, and extension version constants.\n".to_owned(),
    );

    // ── #define typedefs → const fn or pub const ──────────────────────────────
    for td in reg.typedefs.values() {
        if td.kind != TypedefKind::Define {
            continue;
        }
        let Some(ref ty) = td.ty else { continue };
        let name = &td.name;
        let url = refpage_url(name);
        let doc = format!("/// [`{name}`]({url})");
        let cfg = if td.provided_by.is_empty() {
            String::new()
        } else {
            cfg_any_str(&td.provided_by)
        };

        let emitted = if let Some(rest) = ty.strip_prefix("fn:") {
            // "fn:param1,param2|body_expr"
            if let Some((params, body)) = rest.split_once('|') {
                let param_list = params
                    .split(',')
                    .map(|p| format!("{}: u32", p.trim()))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{doc}\n{cfg}\n\
                     #[inline] pub const fn {name}({param_list}) -> u32 {{\n    {body}\n}}\n"
                )
            } else {
                String::new()
            }
        } else if let Some(rest) = ty.strip_prefix("vkver:") {
            // video codec version constant: "1, 0, 0"
            let parts: Vec<&str> = rest.split(',').map(str::trim).collect();
            if parts.len() == 3 {
                let (maj, min, pat) = (parts[0], parts[1], parts[2]);
                let val = maj.parse::<u32>().unwrap_or(0) << 22
                    | min.parse::<u32>().unwrap_or(0) << 12
                    | pat.parse::<u32>().unwrap_or(0);
                format!(
                    "{doc}\n{cfg}\n\
                     /// Encoded as `VK_MAKE_VIDEO_STD_VERSION({maj}, {min}, {pat})`\n\
                     pub const {name}: u32 = {val};\n"
                )
            } else {
                String::new()
            }
        } else if let Some(rest) = ty.strip_prefix("apiconst:") {
            // API version constant or simple integer
            let val_str = if let Some(args) = rest.strip_prefix("make_api_version(") {
                // "make_api_version(0, 1, 0, 0)" → evaluate
                let a = args.trim_end_matches(')');
                let parts: Vec<&str> = a.split(',').map(str::trim).collect();
                if parts.len() == 4 {
                    // Parse variant — may be a named constant (VKSC_API_VARIANT = 1)
                    let variant: u32 = parts[0].parse().unwrap_or(1);
                    let major: u32 = parts[1].parse().unwrap_or(0);
                    let minor: u32 = parts[2].parse().unwrap_or(0);
                    let patch: u32 = if let Ok(n) = parts[3].parse() { n } else { 0 }; // VK_HEADER_VERSION → 0 for now
                    let val = (variant << 29) | (major << 22) | (minor << 12) | patch;
                    format!("{val}")
                } else {
                    return String::new();
                }
            } else {
                rest.to_owned() // plain integer string
            };
            format!("{doc}\n{cfg}\npub const {name}: u32 = {val_str};\n")
        } else {
            String::new()
        };

        if !emitted.is_empty() {
            raw_parts.push(emitted);
        }
    }

    // ── reg.constants → regular API constants ─────────────────────────────────
    let mut ts = TokenStream::new();
    for c in reg.constants.values() {
        let name = format_ident!("{}", &c.name);
        let url = refpage_url(&c.name);
        let doc = format!(" [`{n}`]({url})", n = c.name);
        let depr = depr_attr(&c.depr);

        let cfg = if c.provided_by.is_empty() {
            quote! {}
        } else {
            cfg_any(&c.provided_by)
        };

        if let Some(ref alias) = c.alias {
            let a = format_ident!("{}", alias);
            ts.extend(quote! { #cfg #[doc = #doc] #depr pub const #name: u32 = #a; });
            continue;
        }

        // String constants (EXTENSION_NAME)
        if c.ty == "&'static str" {
            let val_ts: TokenStream = c.value.parse().unwrap_or_else(|_| quote! { "" });
            ts.extend(quote! {
                #cfg #[doc = #doc] #depr
                pub const #name: &'static str = #val_ts;
            });
            continue;
        }

        let ty_str = const_rust_type(&c.ty, &c.value);
        let ty_ts: TokenStream = ty_str.parse().unwrap_or_else(|_| quote! {u32});
        let val_str = normalize_const_value(&c.value, ty_str);
        let val_ts: TokenStream = val_str.parse().unwrap_or_else(|_| quote! {0});
        ts.extend(quote! {
            #cfg #[doc = #doc] #depr
            pub const #name: #ty_ts = #val_ts;
        });
    }

    raw_parts.push(pretty(ts));
    raw_parts.join("\n")
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

// ── commands.rs ───────────────────────────────────────────────────────────────

fn gen_commands_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan command function pointer types (`PFN_vk*`).
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::types::*;
        #[allow(unused_imports)] use crate::enums::*;
    });
    for cmd in reg.commands.values() {
        ts.extend(gen_command(cmd));
    }
    pretty(ts)
}

fn gen_command(cmd: &Command) -> TokenStream {
    if cmd.provided_by.is_empty() {
        return quote! {};
    }
    let cfg = cfg_any(&cmd.provided_by);
    let pfn = format_ident!("PFN_{}", &cmd.name);
    let url = refpage_url(&cmd.name);
    let provided: String = cmd
        .provided_by
        .iter()
        .map(|f| format!(" - `{f}`"))
        .collect::<Vec<_>>()
        .join("\n");
    let doc = format!(" [`{n}`]({url})\n\n Provided by:\n{provided}", n = cmd.name);
    let depr = depr_attr(&cmd.depr);

    if let Some(ref alias) = cmd.alias {
        let a = format_ident!("PFN_{}", alias);
        return quote! { #[doc = #doc] #cfg #depr pub type #pfn = #a; };
    }

    let ret_s = ctype_to_rust_str(&cmd.return_type);
    let ret_ts: TokenStream = if ret_s == "core::ffi::c_void" || ret_s == "void" {
        quote! {}
    } else {
        let r: TokenStream = ret_s.parse().unwrap_or_else(|_| quote! {()});
        quote! { -> #r }
    };

    let mut params = TokenStream::new();
    for p in &cmd.params {
        let pname = format_ident!("{}", sanitize_ident(&p.name));
        let pty: TokenStream = ctype_to_rust_str(&p.ty)
            .parse()
            .unwrap_or_else(|_| quote! { *mut core::ffi::c_void });
        params.extend(quote! { #pname: #pty, });
    }

    quote! {
        #[doc = #doc]
        #cfg #depr
        pub type #pfn = unsafe extern "system" fn(#params) #ret_ts;
    }
}

// ── loader.rs ─────────────────────────────────────────────────────────────────

fn gen_loader_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan dispatch tables.
        //!
        //! `InstanceDispatchTable` and `DeviceDispatchTable` hold `Option<PFN_*>` for
        //! every Vulkan command.  Populate them via a `vkGetXxxProcAddr` closure.
        //!
        //! For runtime extension detection, use the `VK_*_EXTENSION_NAME` string
        //! constants in `crate::consts` and compare against the names returned by
        //! `vkEnumerateXxxExtensionProperties`.
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::commands::*;
        #[allow(unused_imports)] use crate::types::*;
    });

    ts.extend(gen_dispatch_table(reg, "Instance", is_instance_cmd));
    ts.extend(gen_dispatch_table(reg, "Device", |n| !is_instance_cmd(n)));

    pretty(ts)
}

fn gen_dispatch_table<F: Fn(&str) -> bool>(reg: &Registry, kind: &str, filter: F) -> TokenStream {
    let tname = format_ident!("{}DispatchTable", kind);
    let doc = format!(
        " Dispatch table for Vulkan {k} commands.",
        k = kind.to_lowercase()
    );

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();

    for cmd in reg.commands.values() {
        if !filter(&cmd.name) || cmd.provided_by.is_empty() {
            continue;
        }
        let cfg = cfg_any(&cmd.provided_by);
        let fname = format_ident!("{}", cmd_field_name(&cmd.name));
        let pfn = format_ident!("PFN_{}", &cmd.name);
        let clit = Literal::byte_string(format!("{}\0", cmd.name).as_bytes());

        fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
        empty_ts.extend(quote! {  #cfg #fname: None, });
        load_ts.extend(quote! {
            #cfg {
                let raw = loader(#clit.as_ptr() as *const c_char);
                if !raw.is_null() {
                    table.#fname = Some(unsafe { core::mem::transmute(raw) });
                }
            }
        });
    }

    quote! {
        #[doc = #doc]
        #[cfg(feature = "VK_VERSION_1_0")]
        #[derive(Clone)]
        pub struct #tname { #fields_ts }

        #[cfg(feature = "VK_VERSION_1_0")]
        impl #tname {
            /// All entries set to `None`.
            pub const EMPTY: Self = Self { #empty_ts };

            /// Load commands via a `vkGetXxxProcAddr`-style loader function.
            ///
            /// `loader` receives a null-terminated command name and must return
            /// a raw function pointer cast to `*const c_void`, or null if unavailable.
            pub fn load<F>(mut loader: F) -> Self
            where F: FnMut(*const c_char) -> *const c_void
            {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }
        }
    }
}

fn is_instance_cmd(name: &str) -> bool {
    matches!(
        name,
        "vkCreateInstance"
            | "vkDestroyInstance"
            | "vkEnumeratePhysicalDevices"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceVersion"
            | "vkGetInstanceProcAddr"
    ) || name.starts_with("vkGetPhysicalDevice")
        || name.starts_with("vkEnumeratePhysicalDevice")
        || name.contains("Surface")
        || name.contains("Display")
}

fn cmd_field_name(cmd: &str) -> String {
    camel_to_snake(cmd.strip_prefix("vk").unwrap_or(cmd))
}

fn camel_to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut out = String::with_capacity(n + 8);
    let mut i = 0usize;
    while i < n {
        let c = chars[i];
        if c.is_uppercase() && i > 0 {
            let prev_lower = chars[i - 1].is_lowercase() || chars[i - 1].is_ascii_digit();
            let next_lower = i + 1 < n && chars[i + 1].is_lowercase();
            if prev_lower || next_lower {
                out.push('_');
            }
        }
        if c.is_ascii_digit() && i > 0 && chars[i - 1].is_alphabetic() {
            out.push('_');
        } else if c.is_alphabetic() && i > 0 && chars[i - 1].is_ascii_digit() {
            out.push('_');
        }
        out.push(c.to_ascii_lowercase());
        i += 1;
    }
    // Collapse double underscores
    let mut res = String::new();
    let mut prev = false;
    for c in out.chars() {
        if c == '_' {
            if !prev && !res.is_empty() {
                res.push('_');
            }
            prev = true;
        } else {
            res.push(c);
            prev = false;
        }
    }
    res.trim_matches('_').to_owned()
}

// ── validation.rs ─────────────────────────────────────────────────────────────

fn gen_validation_rs(reg: &Registry) -> String {
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Compile-time feature dependency validation.
        //!
        //! If you enable an extension feature without its required dependencies,
        //! a `compile_error!` will fire.  When using Cargo feature implication
        //! these checks are redundant but help with manual `cfg` usage.
    });

    for ext in &reg.extensions {
        if ext.is_disabled() || ext.is_video_header() {
            continue;
        }
        let dep_atoms: Vec<String> = ext.depends.as_ref().map(|d| d.atoms()).unwrap_or_default();
        if dep_atoms.is_empty() {
            continue;
        }

        let feat = ext.name.as_str();
        let url = refpage_url(feat);
        for dep in &dep_atoms {
            let dep = dep.as_str();
            let msg = format!(
                "Feature `{feat}` requires `{dep}`.\nAdd `\"{dep}\"` to [features] in Cargo.toml.\nSpec: {url}"
            );
            ts.extend(quote! {
                #[cfg(all(feature = #feat, not(feature = #dep)))]
                compile_error!(#msg);
            });
        }
    }
    pretty(ts)
}

// ── DOT graph ─────────────────────────────────────────────────────────────────

fn gen_dot_graph(reg: &Registry) -> String {
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
    name.replace('-', "_").replace('.', "_")
}

// ── URL helpers ───────────────────────────────────────────────────────────────

/// Vulkan refpage URL for a named symbol.
/// Format: https://docs.vulkan.org/refpages/latest/refpages/source/<name>.html
pub fn refpage_url(name: &str) -> String {
    format!("https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html")
}

// ── Shared token helpers ──────────────────────────────────────────────────────

fn depr_attr(d: &DeprecationInfo) -> TokenStream {
    if !d.is_any() {
        return quote! {};
    }
    let note = d.note();
    quote! { #[deprecated(note = #note)] }
}

/// Plain-string cfg attribute for use in raw-string generation paths.
fn cfg_any_str(features: &[String]) -> String {
    match features.len() {
        0 => String::new(),
        1 => format!("#[cfg(feature = \"{}\")]", features[0]),
        _ => {
            let items: Vec<String> = features
                .iter()
                .map(|f| format!("feature = \"{}\"", f))
                .collect();
            format!("#[cfg(any({}))]", items.join(", "))
        }
    }
}

/// Plain-string `#[deprecated(note = "...")]` for raw-string generation paths.
fn depr_attr_str(d: &DeprecationInfo) -> String {
    if !d.is_any() {
        return String::new();
    }
    let note = d.note().replace('"', "\\\"");
    format!("#[deprecated(note = \"{note}\")]\n")
}

fn pretty(ts: TokenStream) -> String {
    match syn::parse2::<syn::File>(ts.clone()) {
        Ok(f) => prettyplease::unparse(&f),
        Err(e) => {
            eprintln!("vk-codegen: prettyplease error: {e}");
            format!("// PARSE ERROR: {e}\n{ts}")
        }
    }
}
