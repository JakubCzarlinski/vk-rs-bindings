use crate::cfggen::cfg_any;
use crate::codegen::{deprecate_attr, feature_key, pretty, refpage_url, sanitize_ident};
use crate::ir::{Member, Registry, Struct, Typedef, TypedefKind};
use crate::types::{c_type_to_rust, ctype_to_rust_str};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashSet};

/// Parse a Rust type string into a `TokenStream`, falling back to a raw identifier.
fn parse_ty(s: &str) -> TokenStream {
    syn::parse_str::<syn::Type>(s).map_or_else(|_| s.parse().unwrap_or_default(), |t| quote! { #t })
}

/// Parse a Rust expression string into a `TokenStream`.
fn parse_expr(s: &str) -> TokenStream {
    syn::parse_str::<syn::Expr>(s).map_or_else(|_| s.parse().unwrap_or_default(), |e| quote! { #e })
}

pub fn gen_types_rs(reg: &Registry) -> String {
    // Collect all items keyed by feature group, then emit groups sorted
    // so identical #[cfg(...)] attributes are adjacent - the compiler
    // evaluates each unique cfg expression once rather than per-item.

    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();
    let mut seen = HashSet::new();

    for td in reg.typedefs.values().flatten() {
        if td.provided_by.is_empty() || seen.contains(&td.name) {
            continue;
        }
        seen.insert(td.name.clone());
        let ts = gen_typedef_ts(td);
        if !ts.is_empty() {
            groups
                .entry(feature_key(&td.provided_by))
                .or_default()
                .extend(ts);
        }
    }
    for s in reg.structs.values().flatten() {
        if s.provided_by.is_empty() || seen.contains(&s.name) {
            continue;
        }
        seen.insert(s.name.clone());
        let ts = gen_struct_ts(s, reg);
        if !ts.is_empty() {
            groups
                .entry(feature_key(&s.provided_by))
                .or_default()
                .extend(ts);
        }
    }

    // Emit: file header + one #[cfg(...)] block per unique feature group
    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Struct, union, handle, typedef, and platform handle definitions.
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::consts::*;
        #[allow(unused_imports)] use crate::enums::*;
    });

    for (key, items) in &groups {
        let cfg = cfg_any(key);
        ts.extend(quote! {
            #cfg
            const _: () = ();  // cfg anchor - items follow
        });
        // Emit each item with its own cfg (they may have the same key but
        // we still need individual attributes for correct visibility)
        ts.extend(items.clone());
    }

    // Actually we want items emitted directly, not nested. Rebuild without the anchor:
    let mut out = TokenStream::new();
    out.extend(quote! {
        //! Struct, union, handle, typedef, and platform handle definitions.
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::consts::*;
        #[allow(unused_imports)] use crate::enums::*;
    });
    for (_key, items) in groups {
        out.extend(items);
    }
    pretty(&out)
}

/// Generate typedef tokens using quote!.
fn gen_typedef_ts(td: &Typedef) -> TokenStream {
    if td.provided_by.is_empty() {
        return quote! {};
    }

    let cfg = cfg_any(&td.provided_by);
    let depr = deprecate_attr(&td.depr);
    let name = format_ident!("{}", &td.name);

    let url_str = format!(" [{}]({})", &td.name, refpage_url(&td.name));
    let mut doc = quote! { #[doc = #url_str] };
    if let Some(ref comment) = td.comment {
        let comment = comment.trim();
        if !comment.is_empty() {
            doc.extend(quote! { #[doc = " "] });
            let comment = " ".to_string() + comment;
            doc.extend(quote! { #[doc = #comment] });
        }
    }
    if let Some(ref dep) = td.dep {
        doc.extend(quote! { #[doc = " "] });
        let depends_on = dep.atoms().join("`, `");
        let comment = format!(" **Availability:** depends on `{depends_on}`.");
        doc.extend(quote! { #[doc = #comment] });
    }

    match td.kind {
        TypedefKind::Handle { dispatchable, .. } => {
            if let Some(ref alias) = td.alias {
                let a = parse_ty(alias);
                quote! { #cfg #depr pub type #name = #a; }
            } else if dispatchable {
                doc.extend(quote! {
                    #cfg #depr
                    #[repr(transparent)]
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    pub struct #name(pub *mut core::ffi::c_void);
                    #cfg #depr
                    impl #name {
                        pub const NULL: Self = Self(core::ptr::null_mut());
                        pub const DEFAULT: Self = Self::NULL;
                    }
                    #cfg #depr
                    impl Default for #name {
                        fn default() -> Self { Self::NULL }
                    }
                });
                doc
            } else {
                doc.extend(quote! {
                    #cfg #depr
                    #[repr(transparent)]
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    pub struct #name(pub u64);
                    #cfg #depr
                    impl #name {
                        pub const NULL: Self = Self(0);
                        pub const DEFAULT: Self = Self::NULL;
                    }
                    #cfg #depr
                    impl Default for #name {
                        fn default() -> Self { Self::NULL }
                    }
                });
                doc
            }
        }
        TypedefKind::Bitmask => {
            if let Some(ref alias) = td.alias {
                let a = parse_ty(alias);
                quote! { #cfg pub type #name = #a; }
            } else if let Some(ref ty) = td.ty {
                let mapped = c_type_to_rust(ty);
                let rty = parse_ty(if mapped.is_empty() { ty } else { mapped });
                doc.extend(quote! {
                    #cfg
                    pub type #name = #rty;
                });
                doc
            } else {
                quote! {
                    #cfg
                    pub type #name = u32;
                }
            }
        }
        TypedefKind::Basetype => {
            if let Some(ref ty) = td.ty {
                let mapped = c_type_to_rust(ty);
                let rty = parse_ty(if mapped.is_empty() { ty } else { mapped });
                doc.extend(quote! {
                    #cfg
                    pub type #name = #rty;
                });
                doc
            } else {
                quote! {}
            }
        }
        TypedefKind::Alias => {
            if let Some(ref alias) = td.alias {
                let a = parse_ty(alias);
                quote! { #cfg #depr pub type #name = #a; }
            } else {
                quote! {}
            }
        }
        TypedefKind::FuncPointer => {
            if let Some(ref encoded) = td.ty {
                let (ret_raw, params_enc) = encoded.split_once('|').unwrap_or((encoded, ""));
                let mapped_ret = c_type_to_rust(ret_raw.trim());
                let ret_raw2 = if mapped_ret.is_empty() {
                    ret_raw.trim()
                } else {
                    mapped_ret
                };
                let ret_ts: TokenStream = if ret_raw2 == "core::ffi::c_void" || ret_raw2 == "void" {
                    quote! {}
                } else {
                    let r = parse_ty(ret_raw2);
                    quote! { -> #r }
                };
                let params_ts: TokenStream = params_enc
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|p| {
                        let (pn, pt) = p.split_once(':').unwrap_or(("_", p));
                        let pname = format_ident!("{}", if pn.is_empty() { "_" } else { pn });
                        let ptype = parse_ty(pt);
                        quote! { #pname: #ptype, }
                    })
                    .collect();
                doc.extend(quote! {
                    #cfg #depr
                    pub type #name = Option<unsafe extern "system" fn(#params_ts) #ret_ts>;
                });
                doc
            } else {
                quote! {}
            }
        }
        TypedefKind::OpaqueExtern => {
            // repr(transparent) newtype over *mut c_void - nominally distinct per platform type
            let doc2 = " Opaque platform handle - always used as a raw pointer.";
            doc.extend(quote! {
                #[doc = #doc2]
                #cfg
                #[repr(transparent)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub struct #name(pub *mut core::ffi::c_void);
                #cfg
                impl #name {
                    pub const NULL: Self = Self(core::ptr::null_mut());
                }
            });
            doc
        }
        TypedefKind::Define => quote! {}, // emitted in consts.rs
    }
}

/// Returns true if a member's Rust type involves a raw pointer at the top level
/// (i.e. `pointer_depth` > 0), meaning its setter cannot be `const fn`.
fn member_is_pointer(m: &Member) -> bool {
    m.ty.pointer_depth > 0
}

/// Emit a per-field cfg guard matching the one used on the field declaration itself.
fn member_cfg(m: &Member) -> TokenStream {
    if let Some(ref aset) = m.api {
        if aset.vulkansc && !aset.vulkan {
            return quote! { #[cfg(feature = "VKSC_VERSION_1_0")] };
        } else if aset.vulkan && !aset.vulkansc {
            return quote! { #[cfg(not(feature = "VKSC_VERSION_1_0"))] };
        }
    }
    quote! {}
}

/// Generate builder setter methods for all struct members except `sType`.
///
/// - Non-pointer fields  -> `pub const fn with_<name>(mut self, val: T) -> Self`
/// - Pointer fields      -> `pub fn with_<name>(mut self, val: T) -> Self`
///   with a `// SAFETY:` doc comment reminding callers of
///   their lifetime obligations.
///
/// Each setter carries the same `#[cfg(...)]` guard as its corresponding field.
fn gen_builder_setters(s: &Struct) -> TokenStream {
    let mut ts = TokenStream::new();

    for m in &s.members {
        // sType is always fixed by the spec - no setter.
        if m.name == "sType" {
            continue;
        }

        let fname = format_ident!("{}", sanitize_ident(&m.name));
        // Builder method keeps original Vulkan casing: with_pNext, with_stageIndexOffset, etc.
        let method_name = format_ident!("with_{}", &m.name);
        let ftype = parse_ty(&ctype_to_rust_str(&m.ty));
        let fcfg = member_cfg(m);
        let fdepr = deprecate_attr(&m.depr);

        if member_is_pointer(m) {
            // Raw pointer setter: safe fn (caller asserts lifetime), not const fn
            // (ptr-to-int casts are not stable in const context).
            let safety_doc = " # Safety\n \
                 The caller must ensure `val` remains valid and outlives \
                 any use of this struct instance. The pointer is stored as-is \
                 without any lifetime tracking."
                .to_string();
            ts.extend(quote! {
                #fcfg
                #fdepr
                #[doc = #safety_doc]
                #[inline]
                pub const fn #method_name(mut self, val: #ftype) -> Self {
                    self.#fname = val;
                    self
                }
            });
        } else {
            // Non-pointer setter: const fn, always safe.
            ts.extend(quote! {
                #fcfg
                #fdepr
                #[inline]
                pub const fn #method_name(mut self, val: #ftype) -> Self {
                    self.#fname = val;
                    self
                }
            });
        }
    }

    ts
}

/// Generate struct/union tokens using quote!.
fn gen_struct_ts(s: &Struct, reg: &Registry) -> TokenStream {
    if s.provided_by.is_empty() {
        return quote! {};
    }

    let cfg = cfg_any(&s.provided_by);
    let name_str = &s.name;
    let url_str = format!(" [{}]({})", name_str, refpage_url(name_str));
    let mut doc = quote! { #[doc = #url_str] };
    if let Some(ref comment) = s.comment {
        let comment = comment.trim();
        if !comment.is_empty() {
            doc.extend(quote! { #[doc = " "] });
            let comment = " ".to_string() + comment;
            doc.extend(quote! { #[doc = #comment] });
        }
    }
    if s.returned_only {
        doc.extend(quote! {
            #[doc = " "]
            #[doc = " *Note: This is a **returned only** struct.*"]
        });
    }
    if s.required_limit_type {
        doc.extend(quote! {
            #[doc = " "]
            #[doc = " *Note: This struct has **required limit types**.*"]
        });
    }
    if !s.struct_extends.is_empty() {
        let extends = s.struct_extends.join(", ");
        let comment = format!(" **Extends:** {extends}.");
        doc.extend(quote! {
            #[doc = " "]
            #[doc = #comment]
        });
    }
    if let Some(ref dep) = s.dep {
        let depends_on = dep.atoms().join(" + ");
        let comment = format!(" **Availability:** depends on `{depends_on}`.");
        doc.extend(quote! {
            #[doc = " "]
            #[doc = #comment]
        });
    }

    let depr = deprecate_attr(&s.depr);
    let name = format_ident!("{}", &s.name);

    if let Some(ref alias) = s.alias {
        let a = parse_ty(alias);
        return quote! { #cfg #depr pub type #name = #a; };
    }

    // sType default - full VK_STRUCTURE_TYPE_* constant name
    let stype_default: Option<TokenStream> = s
        .members
        .iter()
        .find(|m| m.name == "sType")
        .and_then(|m| m.values.as_deref())
        .map(|vals| vals.split(',').next().unwrap_or("").trim().to_owned())
        .filter(|v| !v.is_empty())
        .map(|v| {
            let const_name = format_ident!("{}", v);
            quote! { VkStructureType::#const_name }
        });

    // Build field token streams
    let field_toks: TokenStream = s
        .members
        .iter()
        .map(|m| {
            let fname = format_ident!("{}", sanitize_ident(&m.name));
            let ftype = parse_ty(&ctype_to_rust_str(&m.ty));
            let fdoc = m.comment.as_deref().unwrap_or("");
            let fdepr = deprecate_attr(&m.depr);

            let fcfg = member_cfg(m);

            let mut extra = Vec::new();
            if m.optional != crate::ir::Optional::False {
                extra.push(format!(" Optional: {:?}", m.optional));
            }
            if let Some(ref len) = m.len {
                extra.push(format!(" Length: {len}"));
            }
            if let Some(ref vals) = m.values {
                extra.push(format!(" Values: {vals}"));
            }
            if let Some(ref lt) = m.limit_type {
                extra.push(format!(" Limit Type: {lt:?}"));
            }
            if m.no_auto_validity {
                extra.push(" No Auto-Validity".to_string());
            }
            if let Some(ref ot) = m.object_type {
                extra.push(format!(" Object Type: {ot}"));
            }

            let full_doc = if extra.is_empty() {
                fdoc.to_owned()
            } else if fdoc.is_empty() {
                extra.join(", ")
            } else {
                format!("{} ({})", fdoc, extra.join(", "))
            };

            if full_doc.is_empty() {
                quote! { #fcfg #fdepr pub #fname: #ftype, }
            } else {
                quote! { #fcfg #[doc = #full_doc] #fdepr pub #fname: #ftype, }
            }
        })
        .collect();

    // Build default expressions for impl block
    let mut needs_unsafe = false;
    let default_fields: Vec<TokenStream> = s
        .members
        .iter()
        .filter(|m| !(m.name == "sType" && stype_default.is_some()))
        .map(|m| {
            let fname = format_ident!("{}", sanitize_ident(&m.name));
            let (def_str, safe) = member_default_const(m, reg);
            if !safe {
                needs_unsafe = true;
            }

            let fcfg = member_cfg(m);

            let def = parse_expr(&def_str);
            quote! { #fcfg #fname: #def, }
        })
        .collect();
    let default_body: TokenStream = default_fields.into_iter().collect();

    if s.is_union {
        // Union: Copy+Clone derive, manual Debug, unsafe const DEFAULT.
        // Builder setters are intentionally omitted for unions - their fields
        // are semantically exclusive and a setter on one variant is misleading.
        let first_fname = s.members.first().map_or_else(
            || format_ident!("_"),
            |m| format_ident!("{}", sanitize_ident(&m.name)),
        );
        let first_ftype = s
            .members
            .first()
            .map_or_else(|| quote! { u8 }, |m| parse_ty(&ctype_to_rust_str(&m.ty)));
        let name_str = s.name.as_str();
        let fname_str = s.members.first().map_or("_", |m| m.name.as_str());
        doc.extend(quote! {
            #cfg #depr
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub union #name { #field_toks }

            #cfg
            impl #name {
                pub const DEFAULT: Self = unsafe {
                    Self { #first_fname: core::mem::zeroed::<#first_ftype>() }
                };
                #[inline]
                pub const fn new() -> Self { Self::DEFAULT }
            }

            #cfg
            impl core::fmt::Debug for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    // SAFETY: all union variants are valid bit patterns for their types.
                    f.debug_struct(#name_str)
                     .field(#fname_str, unsafe { &self.#first_fname })
                     .finish()
                }
            }
        });
        doc
    } else {
        // Struct: Debug+Clone+Copy derive
        let builder_setters = gen_builder_setters(s);
        let impl_body: TokenStream = if let Some(ref sv) = stype_default {
            if needs_unsafe {
                quote! {
                    pub fn new() -> Self {
                        // SAFETY: zeroed fields are valid C-layout types.
                        unsafe { Self { sType: #sv, #default_body } }
                    }
                    #builder_setters
                }
            } else {
                quote! {
                    pub const DEFAULT: Self = Self { sType: #sv, #default_body };
                    #[inline]
                    pub const fn new() -> Self { Self::DEFAULT }
                    #builder_setters
                }
            }
        } else if needs_unsafe {
            quote! {
                pub fn new() -> Self {
                    // SAFETY: zeroed fields are valid C-layout types.
                    unsafe { Self { #default_body } }
                }
                #builder_setters
            }
        } else {
            quote! {
                pub const DEFAULT: Self = Self { #default_body };
                #[inline]
                pub const fn new() -> Self { Self::DEFAULT }
                #builder_setters
            }
        };

        doc.extend(quote! {
            #cfg #depr
            #[repr(C)]
            #[derive(Debug, Clone, Copy)]
            pub struct #name { #field_toks }

            #cfg
            impl #name { #impl_body }
        });
        doc
    }
}

/// Classify a base type name against the registry to determine how to zero-initialize it.
/// Returns `(TypeClass, canonical_name)` - the canonical name is the resolved type
/// name to use in default expressions (follows aliases to the defining type).
#[derive(Debug, Clone, Copy, PartialEq)]
enum TypeClass {
    /// Primitive Rust integer/float/bool or typedef alias to one.
    PrimitiveAlias,
    /// Platform opaque handle newtype (repr(transparent) over *mut `c_void`).
    /// Default is `TypeName::NULL`.
    NullMutAlias,
    /// Function pointer type alias - emitted as `Option<fn(...)>`, defaults to `None`.
    OptionType,
    /// repr(transparent) newtype over an integer (enum or bitmask).
    /// The returned canonical name is the actual newtype, not an alias name.
    EnumNewtype,
    /// A non-union struct that has its own `DEFAULT` constant.
    StructWithDefault,
    /// A union or unknown type that needs unsafe zeroed.
    StructUnsafe,
}

/// Resolve a type name through aliases to find the canonical `TypeClass` and name.
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

    if let Some(td) = reg.typedefs.get(base).and_then(|v| v.last()) {
        return match td.kind {
            TypedefKind::Handle { .. } => (TypeClass::StructWithDefault, base.to_owned()),
            TypedefKind::Basetype => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Bitmask => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Alias => {
                // Follow alias chain - return the canonical name of the target
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

    if let Some(e) = reg.enums.get(base).and_then(|v| v.last()) {
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
        // Unions use `unsafe { zeroed() }` for their first field, which is const-safe on Rust >= 1.75.
        return (TypeClass::StructWithDefault, base.to_owned());
    }

    (TypeClass::StructUnsafe, base.to_owned())
}

/// Returns `(default_expr, is_const_safe)` for a struct member's zero default.
///
/// Rules:
///   - `expr` is NEVER wrapped in `unsafe { }` - the caller wraps the whole struct literal.
///   - `is_const_safe = true`  -> expr is valid in `const` context
///   - `is_const_safe = false` -> expr requires being inside an `unsafe` block
///
/// Type dispatch:
///   - Pointers         -> `core::ptr::null()`/`null_mut()`  (const-safe)
///   - Option<T>        -> `None`                             (const-safe)
///   - Primitive arrays -> `[0u8; N]` etc.                   (const-safe)
///   - Struct arrays    -> `[T::DEFAULT; N]`                  (const-safe if T has DEFAULT)
///   - Enum arrays      -> `[T(0); N]`                        (const-safe)
///   - Unknown arrays   -> `core::mem::zeroed()`              (needs unsafe wrapper)
///   - Primitive scalars-> `0`, `0.0f32`, `false`             (const-safe)
///   - Enum newtypes    -> `T(0)`                             (const-safe)
///   - Structs w/ DEFAULT -> `T::DEFAULT`                     (const-safe)
///   - Option fn-ptrs   -> `None`                             (const-safe)
///   - Unknown          -> `core::mem::zeroed()`              (needs unsafe wrapper)
fn member_default_const(m: &Member, reg: &Registry) -> (String, bool) {
    // Pointer fields
    if m.ty.pointer_depth > 0 {
        let expr = if m.ty.is_const {
            "core::ptr::null()"
        } else {
            "core::ptr::null_mut()"
        };
        return (expr.into(), true);
    }

    // Static array fields
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
            // Use the canonical name so alias types (e.g. VkComponentTypeNV -> VkComponentTypeKHR)
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

    // Scalar fields
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

    // Registry-classified types - use canonical name for correct constructor
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
