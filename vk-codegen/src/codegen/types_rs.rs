use crate::cfggen::{cfg_availability, cfg_availability_expr, cfg_not_availability};
use crate::codegen::utils::{
    base_type_tokens_for_registry, ctype_to_tokens_for_registry,
    rewrite_member_types_for_providers, struct_name_has_lifetime,
};
use crate::codegen::{deprecate_attr, feature_key, pretty, refpage_url, sanitize_ident};
use crate::ir::{Member, Optional, Registry, Struct, Typedef, TypedefKind};
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

fn typed_bitmask_target(td: &Typedef, reg: &Registry) -> Option<String> {
    if td.bitmask_bits.is_none()
        && let Some(alias) = td.alias.as_deref()
        && let Some(target) = reg.typedefs.get(alias).and_then(|items| items.first())
    {
        let mut resolved = target.clone();
        resolved.name = td.name.clone();
        resolved.alias = None;
        resolved.provided_by = td.provided_by.clone();
        resolved.availability = td.availability.clone();
        resolved.dep = td.dep.clone();
        return typed_bitmask_target(&resolved, reg);
    }

    let bits = td.bitmask_bits.as_deref()?;
    let bit_enum = reg.enums.get(bits).and_then(|items| items.first())?;
    if !bit_enum.is_bitmask {
        return None;
    }

    let mut providers = bit_enum.provided_by.clone();
    for variant in &bit_enum.variants {
        for provider in &variant.provided_by {
            if !providers.contains(provider) {
                providers.push(provider.clone());
            }
        }
    }

    td.provided_by
        .iter()
        .all(|provider| providers.contains(provider))
        .then(|| bits.to_owned())
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
        let ts = gen_typedef_ts(td, reg);
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

    let mut out = TokenStream::new();
    out.extend(quote! {
        //! Struct, union, handle, typedef, and platform handle definitions.
        #[allow(unused_imports)] use core::ffi::{c_char, c_void};
        #[allow(unused_imports)] use crate::consts::*;
        #[allow(unused_imports)] use crate::enums::*;
        /// Marker trait for Vulkan structs that are valid in the `pNext` chain rooted at `Root`.
        ///
        /// # Safety
        /// Implementors must be Vulkan structs whose `structextends` metadata includes `Root`.
        pub unsafe trait VkPNextExtends<Root> {}
    });
    for (_key, items) in groups {
        out.extend(items);
    }
    pretty(&out)
}

/// Generate typedef tokens using quote!.
fn gen_typedef_ts(td: &Typedef, reg: &Registry) -> TokenStream {
    if td.provided_by.is_empty() {
        return quote! {};
    }

    let cfg = cfg_availability(&td.availability, &td.provided_by, td.dep.as_ref());
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

    if let Some(ref alias) = td.alias
        && let Some(target) = reg.typedefs.get(alias).and_then(|items| items.first())
    {
        let mut resolved = target.clone();
        resolved.name = td.name.clone();
        resolved.alias = None;
        resolved.comment = td.comment.clone().or(resolved.comment);
        resolved.dep = td.dep.clone();
        resolved.availability = td.availability.clone();
        resolved.depr = td.depr.clone();
        resolved.provided_by = td.provided_by.clone();
        resolved.bitmask_bits = td.bitmask_bits.clone().or(resolved.bitmask_bits);
        return gen_typedef_ts(&resolved, reg);
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
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
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
                    #cfg
                    unsafe impl Send for #name {}
                    #cfg
                    unsafe impl Sync for #name {}
                });
                doc
            }
        }
        TypedefKind::Bitmask => {
            if let Some(ref alias) = td.alias {
                let a = parse_ty(alias);
                quote! { #cfg pub type #name = #a; }
            } else if let Some(bits) = typed_bitmask_target(td, reg) {
                let bits = format_ident!("{}", bits);
                doc.extend(quote! {
                    #cfg
                    pub type #name = #bits;
                });
                doc
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
                #cfg
                unsafe impl Send for #name {}
                #cfg
                unsafe impl Sync for #name {}
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

fn struct_has_lifetime(s: &Struct, reg: &Registry) -> bool {
    !s.is_union
        && s.members.iter().any(|m| {
            m.ty.pointer_depth > 0
                || ((m.ty.pointer_depth == 0 || m.ty.is_array.is_some())
                    && struct_name_has_lifetime(&m.ty.base, reg))
        })
}

fn union_has_lifetime(s: &Struct, reg: &Registry) -> bool {
    s.is_union
        && s.members.iter().any(|m| {
            m.ty.pointer_depth > 0
                || ((m.ty.pointer_depth == 0 || m.ty.is_array.is_some())
                    && struct_name_has_lifetime(&m.ty.base, reg))
        })
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

struct HandleFallbackMember {
    typed_cfg: TokenStream,
    raw_cfg: TokenStream,
    raw_ty: TokenStream,
    raw_default: TokenStream,
    typed_ty: TokenStream,
}

fn handle_fallback_member(
    m: &Member,
    owner: &Struct,
    reg: &Registry,
) -> Option<HandleFallbackMember> {
    if m.ty.pointer_depth != 0 || m.ty.is_array.is_some() {
        return None;
    }

    let td = reg.typedefs.get(&m.ty.base)?.first()?;
    let TypedefKind::Handle { dispatchable, .. } = td.kind else {
        return None;
    };
    if td.provided_by.is_empty() {
        return None;
    }

    let deps = reg.transitive_deps();
    let handle_available_with_owner = owner.provided_by.iter().any(|owner_provider| {
        td.provided_by.contains(owner_provider)
            || deps
                .get(owner_provider)
                .is_some_and(|owner_deps| td.provided_by.iter().any(|p| owner_deps.contains(p)))
    });
    if handle_available_with_owner {
        return None;
    }

    let typed_cfg = cfg_availability(&td.availability, &td.provided_by, td.dep.as_ref());
    let raw_cfg = cfg_not_availability(&td.availability, &td.provided_by, td.dep.as_ref());
    let typed_ty = parse_ty(&ctype_to_rust_str(&m.ty));
    let (raw_ty, raw_default) = if dispatchable {
        (
            quote! { *mut core::ffi::c_void },
            quote! { core::ptr::null_mut() },
        )
    } else {
        (quote! { u64 }, quote! { 0 })
    };

    Some(HandleFallbackMember {
        typed_cfg,
        raw_cfg,
        raw_ty,
        raw_default,
        typed_ty,
    })
}

#[derive(Clone, Copy, Debug)]
struct RequiredSliceMemberPair {
    count_idx: usize,
    ptr_idx: usize,
    kind: SliceSetterKind,
}

#[derive(Clone, Debug)]
struct SliceMemberGroup {
    count_idx: usize,
    ptr_indices: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
enum SliceSetterKind {
    Default,
    ShaderModuleCode,
}

/// Pointer + count struct members that can be exposed as builder slice setters.
fn collect_required_slice_member_pairs(s: &Struct) -> Vec<RequiredSliceMemberPair> {
    let mut pairs = Vec::new();
    for (ptr_idx, ptr) in s.members.iter().enumerate() {
        if !member_can_use_slice_setter(ptr) {
            continue;
        }
        let Some((len_name, kind)) = slice_len_count_member_name(ptr, s) else {
            continue;
        };
        let Some((count_idx, count)) = s
            .members
            .iter()
            .enumerate()
            .find(|(_, m)| m.name == len_name.as_str())
        else {
            continue;
        };
        if count.ty.pointer_depth != 0 || count.ty.is_array.is_some() {
            continue;
        }
        pairs.push(RequiredSliceMemberPair {
            count_idx,
            ptr_idx,
            kind,
        });
    }
    pairs
}

fn slice_len_count_member_name(ptr: &Member, s: &Struct) -> Option<(String, SliceSetterKind)> {
    let len = ptr.len.as_deref()?;
    if s.name == "VkShaderModuleCreateInfo"
        && ptr.name == "pCode"
        && len == r"latexmath:[\textrm{codeSize} \over 4]"
    {
        return Some(("codeSize".to_owned(), SliceSetterKind::ShaderModuleCode));
    }

    let count_name = len.split(',').next()?.trim();
    if count_name.is_empty()
        || count_name == "null-terminated"
        || count_name.starts_with("latexmath:[")
    {
        return None;
    }

    Some((count_name.to_owned(), SliceSetterKind::Default))
}

fn member_can_use_slice_setter(m: &Member) -> bool {
    if m.ty.is_array.is_some() {
        return false;
    }

    match m.ty.pointer_depth {
        1 => m.ty.base != "char",
        2 => true,
        _ => false,
    }
}

fn member_base_rust_ty(m: &Member) -> TokenStream {
    let mapped = c_type_to_rust(&m.ty.base);
    parse_ty(if mapped.is_empty() {
        &m.ty.base
    } else {
        mapped
    })
}

fn member_base_rust_ty_for_registry(m: &Member, reg: &Registry) -> TokenStream {
    base_type_tokens_for_registry(&m.ty.base, reg, quote! { 'a })
}

fn slice_setter_value_type_and_pointer_expr(m: &Member) -> (TokenStream, TokenStream) {
    let arg = quote! { val };
    slice_arg_type_and_pointer_expr(m, &arg)
}

fn shader_module_code_value_type_and_pointer_expr() -> (TokenStream, TokenStream) {
    (quote! { &[u32] }, quote! { val.as_ptr().cast::<u32>() })
}

fn slice_arg_type_and_pointer_expr(m: &Member, arg: &TokenStream) -> (TokenStream, TokenStream) {
    if m.ty.pointer_depth == 1 && m.ty.base == "void" {
        if m.ty.is_const {
            return (
                quote! { &[u8] },
                quote! { #arg.as_ptr().cast::<core::ffi::c_void>() },
            );
        }
        return (
            quote! { &mut [u8] },
            quote! { #arg.as_mut_ptr().cast::<core::ffi::c_void>() },
        );
    }

    let elem_ty = member_base_rust_ty(m);
    let slice_elem_ty = if m.ty.pointer_depth == 2 {
        if m.ty.is_const {
            quote! { *const #elem_ty }
        } else {
            quote! { *mut #elem_ty }
        }
    } else {
        elem_ty
    };

    if m.ty.is_const {
        (quote! { &[#slice_elem_ty] }, quote! { #arg.as_ptr() })
    } else {
        (
            quote! { &mut [#slice_elem_ty] },
            quote! { #arg.as_mut_ptr() },
        )
    }
}

fn slice_setter_value_type_and_pointer_expr_for_registry(
    m: &Member,
    reg: &Registry,
) -> (TokenStream, TokenStream) {
    let arg = quote! { val };
    slice_arg_type_and_pointer_expr_for_registry(m, &arg, reg)
}

fn slice_arg_type_and_pointer_expr_for_registry(
    m: &Member,
    arg: &TokenStream,
    reg: &Registry,
) -> (TokenStream, TokenStream) {
    if m.ty.pointer_depth == 1 && m.ty.base == "void" {
        if m.ty.is_const {
            return (
                quote! { &'a [u8] },
                quote! { #arg.as_ptr().cast::<core::ffi::c_void>() },
            );
        }
        return (
            quote! { &'a mut [u8] },
            quote! { #arg.as_mut_ptr().cast::<core::ffi::c_void>() },
        );
    }

    let elem_ty = member_base_rust_ty_for_registry(m, reg);
    let slice_elem_ty = if m.ty.pointer_depth == 2 {
        if m.ty.is_const {
            quote! { *const #elem_ty }
        } else {
            quote! { *mut #elem_ty }
        }
    } else {
        elem_ty
    };

    if m.ty.is_const {
        (quote! { &'a [#slice_elem_ty] }, quote! { #arg.as_ptr() })
    } else {
        (
            quote! { &'a mut [#slice_elem_ty] },
            quote! { #arg.as_mut_ptr() },
        )
    }
}

fn collect_slice_member_groups(s: &Struct, reg: &Registry) -> Vec<SliceMemberGroup> {
    let mut by_count: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (ptr_idx, ptr) in s.members.iter().enumerate() {
        if !member_can_use_slice_setter(ptr) || member_uses_opaque_extern_pointee(ptr, reg) {
            continue;
        }

        let Some((len_name, _)) = slice_len_count_member_name(ptr, s) else {
            continue;
        };
        let Some((count_idx, count)) = s
            .members
            .iter()
            .enumerate()
            .find(|(_, m)| m.name == len_name.as_str())
        else {
            continue;
        };
        if count.ty.pointer_depth != 0 || count.ty.is_array.is_some() {
            continue;
        }

        by_count.entry(count_idx).or_default().push(ptr_idx);
    }

    by_count
        .into_iter()
        .filter_map(|(count_idx, ptr_indices)| {
            let has_required = ptr_indices
                .iter()
                .any(|idx| s.members[*idx].optional == Optional::False);
            let mut names = HashSet::new();
            let has_duplicate_names = ptr_indices
                .iter()
                .any(|idx| !names.insert(sanitize_ident(&s.members[*idx].name)));
            (ptr_indices.len() >= 2 && has_required && !has_duplicate_names).then_some(
                SliceMemberGroup {
                    count_idx,
                    ptr_indices,
                },
            )
        })
        .collect()
}

fn member_uses_opaque_extern_pointee(m: &Member, reg: &Registry) -> bool {
    reg.typedefs
        .get(&m.ty.base)
        .and_then(|variants| variants.last())
        .is_some_and(|td| matches!(td.kind, TypedefKind::OpaqueExtern))
}

fn pnext_member(s: &Struct) -> Option<&Member> {
    s.members.iter().find(|m| {
        m.name == "pNext"
            && m.ty.pointer_depth == 1
            && m.ty.base == "void"
            && m.ty.is_array.is_none()
    })
}

fn collect_pnext_extenders<'a>(s: &Struct, reg: &'a Registry) -> Vec<&'a Struct> {
    let mut seen = HashSet::new();
    let mut extenders = Vec::new();
    for child in reg.structs.values().filter_map(|items| items.first()) {
        if child.provided_by.is_empty()
            || child.is_union
            || !child.struct_extends.iter().any(|parent| parent == &s.name)
            || !seen.insert(child.name.clone())
        {
            continue;
        }
        extenders.push(child);
    }
    extenders.sort_by(|a, b| a.name.cmp(&b.name));
    extenders
}

fn pnext_chain_root_structs<'a>(s: &'a Struct, reg: &'a Registry) -> Vec<&'a Struct> {
    if s.struct_extends.is_empty() {
        return vec![s];
    }

    let mut roots = Vec::new();
    for root_name in &s.struct_extends {
        let Some(root) = reg.structs.get(root_name).and_then(|items| items.first()) else {
            continue;
        };
        if !roots
            .iter()
            .any(|existing: &&Struct| existing.name == root.name)
        {
            roots.push(root);
        }
    }
    roots
}

fn gen_pnext_marker_impls(s: &Struct, reg: &Registry) -> TokenStream {
    if s.is_union || s.struct_extends.is_empty() {
        return quote! {};
    }

    let mut ts = TokenStream::new();
    let child = format_ident!("{}", &s.name);
    let child_has_lifetime = struct_has_lifetime(s, reg) || union_has_lifetime(s, reg);
    let child_lifetime = child_has_lifetime.then_some(quote! { <'child> });
    let child_cfg_expr = cfg_availability_expr(&s.availability, &s.provided_by, s.dep.as_ref());
    for root in pnext_chain_root_structs(s, reg) {
        let root_cfg_expr =
            cfg_availability_expr(&root.availability, &root.provided_by, root.dep.as_ref());
        let root_has_lifetime = struct_has_lifetime(root, reg) || union_has_lifetime(root, reg);
        let root_lifetime = root_has_lifetime.then_some(quote! { <'root> });
        let root = format_ident!("{}", &root.name);
        let impl_generics = match (child_has_lifetime, root_has_lifetime) {
            (true, true) => quote! { <'child, 'root> },
            (true, false) => quote! { <'child> },
            (false, true) => quote! { <'root> },
            (false, false) => quote! {},
        };
        if child_has_lifetime || root_has_lifetime {
            ts.extend(quote! {
                #[cfg(all(#child_cfg_expr, #root_cfg_expr))]
                unsafe impl #impl_generics VkPNextExtends<#root #root_lifetime> for #child #child_lifetime {}
            });
        } else {
            ts.extend(quote! {
                #[cfg(all(#child_cfg_expr, #root_cfg_expr))]
                unsafe impl VkPNextExtends<#root> for #child {}
            });
        }
    }
    ts
}

/// Generate builder setter methods for all struct members except `sType`.
///
/// - Non-pointer fields  -> `pub const fn with_<name>(mut self, val: T) -> Self`
/// - Pointer fields      -> `pub fn with_<name>(mut self, val: T) -> Self`
///   with a `// SAFETY:` doc comment reminding callers of
///   their lifetime obligations.
///
/// Each setter carries the same `#[cfg(...)]` guard as its corresponding field.
fn gen_builder_setters(s: &Struct, reg: &Registry) -> TokenStream {
    let mut ts = TokenStream::new();
    let has_lifetime = struct_has_lifetime(s, reg) || union_has_lifetime(s, reg);
    let required_slice_pairs = collect_required_slice_member_pairs(s);
    let slice_groups = collect_slice_member_groups(s, reg);
    let safety_doc = quote! {
        /// # Safety
        /// The caller must ensure `val` remains valid and outlives any use of this struct
        /// instance. The pointer is stored as-is without any lifetime tracking.
    };

    for (member_idx, m) in s.members.iter().enumerate() {
        // sType is always fixed by the spec - no setter.
        if m.name == "sType" {
            continue;
        }

        let fname = format_ident!("{}", sanitize_ident(&m.name));
        // Builder method keeps original Vulkan casing: with_pNext, with_stageIndexOffset, etc.
        let method_name = format_ident!("with_{}", &m.name);
        let fcfg = member_cfg(m);
        let fdepr = deprecate_attr(&m.depr);

        if let Some(fallback) = handle_fallback_member(m, s, reg) {
            let ftype = fallback.typed_ty;
            let typed_cfg = fallback.typed_cfg;
            ts.extend(quote! {
                #typed_cfg
                #fdepr
                #[inline]
                pub const fn #method_name(mut self, val: #ftype) -> Self {
                    self.#fname = val;
                    self
                }
            });
            continue;
        }

        let ftype = if has_lifetime {
            ctype_to_tokens_for_registry(&m.ty, reg, quote! { 'a })
        } else {
            parse_ty(&ctype_to_rust_str(&m.ty))
        };

        if let Some(pair) = required_slice_pairs
            .iter()
            .find(|p| p.ptr_idx == member_idx)
        {
            let count_member = &s.members[pair.count_idx];
            let count_fname = format_ident!("{}", sanitize_ident(&count_member.name));
            let count_ty = parse_ty(&ctype_to_rust_str(&count_member.ty));
            let (val_ty, ptr_expr) = match pair.kind {
                SliceSetterKind::Default if has_lifetime => {
                    slice_setter_value_type_and_pointer_expr_for_registry(m, reg)
                }
                SliceSetterKind::Default => slice_setter_value_type_and_pointer_expr(m),
                SliceSetterKind::ShaderModuleCode => {
                    shader_module_code_value_type_and_pointer_expr()
                }
            };
            let count_expr = match pair.kind {
                SliceSetterKind::Default => quote! { val.len() as #count_ty },
                SliceSetterKind::ShaderModuleCode => quote! { val.len() as usize * 4 },
            };
            ts.extend(quote! {
                #fcfg
                #fdepr
                #safety_doc
                #[inline]
                pub const fn #method_name(mut self, val: #val_ty) -> Self {
                    self.#count_fname = #count_expr;
                    self.#fname = #ptr_expr;
                    self
                }
            });
        } else if m.ty.pointer_depth == 1
            && m.optional == Optional::False
            && m.len.is_none()
            && m.ty.base != "void"
            && c_type_to_rust(&m.ty.base) != "core::ffi::c_char"
            && !member_uses_opaque_extern_pointee(m, reg)
        {
            let mapped = c_type_to_rust(&m.ty.base);
            let elem_ty = if has_lifetime {
                base_type_tokens_for_registry(&m.ty.base, reg, quote! { 'a })
            } else {
                parse_ty(if mapped.is_empty() {
                    &m.ty.base
                } else {
                    mapped
                })
            };
            let val_ty = if m.ty.is_const {
                if has_lifetime {
                    quote! { &'a #elem_ty }
                } else {
                    quote! { &#elem_ty }
                }
            } else {
                if has_lifetime {
                    quote! { &'a mut #elem_ty }
                } else {
                    quote! { &mut #elem_ty }
                }
            };
            let ptr_ty = if m.ty.is_const {
                quote! { *const #elem_ty }
            } else {
                quote! { *mut #elem_ty }
            };
            ts.extend(quote! {
                #fcfg
                #fdepr
                #safety_doc
                #[inline]
                pub const fn #method_name(mut self, val: #val_ty) -> Self {
                    self.#fname = val as #ptr_ty;
                    self
                }
            });
        } else if member_is_pointer(m) {
            // Raw pointer setter fallback for optional/void/complex pointer fields.
            ts.extend(quote! {
                #fcfg
                #fdepr
                #safety_doc
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

    for group in slice_groups {
        let count_member = &s.members[group.count_idx];
        let count_fname = format_ident!("{}", sanitize_ident(&count_member.name));
        let count_ty = parse_ty(&ctype_to_rust_str(&count_member.ty));
        let method_name = format_ident!("with_{}_slices", &count_member.name);
        let fcfg = member_cfg(count_member);
        let fdepr = deprecate_attr(&count_member.depr);
        let group_safety_doc = format!(
            " The caller must ensure every provided array constrained by `{}` has the same length. \
             Optional pointer arguments may be null, but non-null pointers must be valid for that \
             same length and outlive any use of this struct instance.",
            count_member.name
        );
        let Some(first_ptr_idx) = group
            .ptr_indices
            .iter()
            .copied()
            .find(|idx| s.members[*idx].optional == Optional::False)
        else {
            continue;
        };
        let first_arg = format_ident!("{}", sanitize_ident(&s.members[first_ptr_idx].name));

        let mut params = TokenStream::new();
        let mut setters = TokenStream::new();
        for ptr_idx in group.ptr_indices {
            let ptr = &s.members[ptr_idx];
            let arg_name = format_ident!("{}", sanitize_ident(&ptr.name));
            let fname = format_ident!("{}", sanitize_ident(&ptr.name));
            let arg_ts = quote! { #arg_name };
            let (slice_ty, ptr_expr) = if has_lifetime {
                slice_arg_type_and_pointer_expr_for_registry(ptr, &arg_ts, reg)
            } else {
                slice_arg_type_and_pointer_expr(ptr, &arg_ts)
            };
            let raw_ty = if has_lifetime {
                ctype_to_tokens_for_registry(&ptr.ty, reg, quote! { 'a })
            } else {
                parse_ty(&ctype_to_rust_str(&ptr.ty))
            };
            let arg_ty = if ptr.optional == Optional::False {
                slice_ty
            } else {
                raw_ty
            };
            params.extend(quote! { #arg_name: #arg_ty, });

            if ptr.optional == Optional::False {
                setters.extend(quote! {
                    self.#fname = #ptr_expr;
                });
            } else {
                setters.extend(quote! {
                    self.#fname = #arg_name;
                });
            }
        }

        ts.extend(quote! {
            #fcfg
            #fdepr
            #[doc = " # Safety"]
            #[doc = #group_safety_doc]
            #[inline]
            pub const fn #method_name(mut self, #params) -> Self {
                let len = #first_arg.len();
                self.#count_fname = len as #count_ty;
                #setters
                self
            }
        });
    }

    if let Some(pnext) = pnext_member(s) {
        let pnext_fname = format_ident!("{}", sanitize_ident(&pnext.name));
        let pnext_safety_doc = quote! {
            /// # Safety
            /// The caller must ensure `val` remains valid and outlives any use of this struct
            /// instance. The pointer is stored as-is without any lifetime tracking.
        };

        for child in collect_pnext_extenders(s, reg) {
            let child_cfg =
                cfg_availability(&child.availability, &child.provided_by, child.dep.as_ref());
            let child_depr = deprecate_attr(&child.depr);
            let child_ty = format_ident!("{}", &child.name);
            let child_has_lifetime =
                struct_has_lifetime(child, reg) || union_has_lifetime(child, reg);
            let child_lifetime = child_has_lifetime.then_some(quote! { <'child> });
            let child_generics = child_has_lifetime.then_some(quote! { <'child> });
            let method_name = format_ident!("with_pNext_{}", &child.name);

            if pnext.ty.is_const {
                ts.extend(quote! {
                    #child_cfg
                    #child_depr
                    #pnext_safety_doc
                    #[inline]
                    pub const fn #method_name #child_generics(mut self, val: &'a #child_ty #child_lifetime) -> Self {
                        self.#pnext_fname = (val as *const #child_ty #child_lifetime).cast::<core::ffi::c_void>();
                        self
                    }
                });
            } else {
                ts.extend(quote! {
                    #child_cfg
                    #child_depr
                    #pnext_safety_doc
                    #[inline]
                    pub const fn #method_name #child_generics(mut self, val: &'a mut #child_ty #child_lifetime) -> Self {
                        self.#pnext_fname = (val as *mut #child_ty #child_lifetime).cast::<core::ffi::c_void>();
                        self
                    }
                });
            }
        }

        for root in pnext_chain_root_structs(s, reg) {
            let root_cfg =
                cfg_availability(&root.availability, &root.provided_by, root.dep.as_ref());
            let root_ty = format_ident!("{}", &root.name);
            let root_has_lifetime = struct_has_lifetime(root, reg) || union_has_lifetime(root, reg);
            let root_lifetime = root_has_lifetime.then_some(quote! { <'root> });
            let root_generics = if root_has_lifetime {
                quote! { <'root, T: VkPNextExtends<#root_ty #root_lifetime>> }
            } else {
                quote! { <T: VkPNextExtends<#root_ty>> }
            };
            let method_name = format_ident!("with_pNext_chain_{}", &root.name);

            if pnext.ty.is_const {
                ts.extend(quote! {
                    #root_cfg
                    #pnext_safety_doc
                    #[inline]
                    pub const fn #method_name #root_generics(mut self, val: &'a T) -> Self {
                        self.#pnext_fname = (val as *const T).cast::<core::ffi::c_void>();
                        self
                    }
                });
            } else {
                ts.extend(quote! {
                    #root_cfg
                    #pnext_safety_doc
                    #[inline]
                    pub const fn #method_name #root_generics(mut self, val: &'a mut T) -> Self {
                        self.#pnext_fname = (val as *mut T).cast::<core::ffi::c_void>();
                        self
                    }
                });
            }
        }
    }

    ts
}

/// Generate struct/union tokens using quote!.
fn gen_struct_ts(s: &Struct, reg: &Registry) -> TokenStream {
    if s.provided_by.is_empty() {
        return quote! {};
    }

    let cfg = cfg_availability(&s.availability, &s.provided_by, s.dep.as_ref());
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
    let has_lifetime = struct_has_lifetime(s, reg) || union_has_lifetime(s, reg);
    let decl_lifetime = has_lifetime.then_some(quote! { <'a> });
    let impl_lifetime = has_lifetime.then_some(quote! { <'a> });
    let impl_generics = has_lifetime.then_some(quote! { <'a> });

    if let Some(ref alias) = s.alias {
        if let Some(target) = reg.structs.get(alias).and_then(|items| items.first()) {
            let mut resolved = target.clone();
            resolved.name = s.name.clone();
            resolved.alias = None;
            resolved.comment = s.comment.clone().or(resolved.comment);
            resolved.dep = s.dep.clone();
            resolved.availability = s.availability.clone();
            resolved.depr = s.depr.clone();
            resolved.provided_by = s.provided_by.clone();
            rewrite_member_types_for_providers(
                &mut resolved.members,
                reg,
                &s.provided_by,
                &s.availability,
            );
            return gen_struct_ts(&resolved, reg);
        }
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
            let stype = structure_type_value_for_providers(reg, &v, &s.provided_by);
            let const_name = format_ident!("{}", stype);
            quote! { VkStructureType::#const_name }
        });

    // Build field token streams
    let field_toks: TokenStream = s
        .members
        .iter()
        .map(|m| {
            let fname = format_ident!("{}", sanitize_ident(&m.name));
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

            if let Some(fallback) = handle_fallback_member(m, s, reg) {
                let typed_cfg = fallback.typed_cfg;
                let raw_cfg = fallback.raw_cfg;
                let typed_ty = fallback.typed_ty;
                let raw_ty = fallback.raw_ty;
                if full_doc.is_empty() {
                    return quote! {
                        #typed_cfg #fdepr pub #fname: #typed_ty,
                        #raw_cfg #fdepr pub #fname: #raw_ty,
                    };
                }
                return quote! {
                    #typed_cfg #[doc = #full_doc] #fdepr pub #fname: #typed_ty,
                    #raw_cfg #[doc = #full_doc] #fdepr pub #fname: #raw_ty,
                };
            }

            let ftype = if has_lifetime {
                ctype_to_tokens_for_registry(&m.ty, reg, quote! { 'a })
            } else {
                parse_ty(&ctype_to_rust_str(&m.ty))
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
            if let Some(fallback) = handle_fallback_member(m, s, reg) {
                let typed_cfg = fallback.typed_cfg;
                let raw_cfg = fallback.raw_cfg;
                let typed_default = parse_expr(&format!("{}::DEFAULT", m.ty.base));
                let raw_default = fallback.raw_default;
                return quote! {
                    #typed_cfg #fname: #typed_default,
                    #raw_cfg #fname: #raw_default,
                };
            }

            let (def_str, safe) = member_default_const(m, reg);
            if !safe {
                needs_unsafe = true;
            }

            let fcfg = member_cfg(m);

            let def = parse_expr(&def_str);
            quote! { #fcfg #fname: #def, }
        })
        .collect();
    let marker_field = has_lifetime.then_some(quote! {
        #[doc(hidden)]
        pub _marker: core::marker::PhantomData<&'a ()>,
    });
    let marker_default = has_lifetime.then_some(quote! {
        _marker: core::marker::PhantomData,
    });
    let default_body: TokenStream = default_fields.into_iter().chain(marker_default).collect();

    if s.is_union {
        // Union: Copy+Clone derive, manual Debug, unsafe const DEFAULT.
        // Builder setters are intentionally omitted for unions - their fields
        // are semantically exclusive and a setter on one variant is misleading.
        let first_fname = s.members.first().map_or_else(
            || format_ident!("_"),
            |m| format_ident!("{}", sanitize_ident(&m.name)),
        );
        let first_ftype = s.members.first().map_or_else(
            || quote! { u8 },
            |m| {
                if has_lifetime {
                    ctype_to_tokens_for_registry(&m.ty, reg, quote! { 'a })
                } else {
                    parse_ty(&ctype_to_rust_str(&m.ty))
                }
            },
        );
        let name_str = s.name.as_str();
        let fname_str = s.members.first().map_or("_", |m| m.name.as_str());
        let marker_union_field = has_lifetime.then_some(quote! {
            pub _marker: core::marker::PhantomData<&'a ()>,
        });
        doc.extend(quote! {
            #cfg #depr
            #[repr(C)]
            #[derive(Copy, Clone)]
            pub union #name #decl_lifetime {
                #field_toks
                #marker_union_field
            }

            #cfg
            unsafe impl #impl_generics Send for #name #impl_lifetime {}
            #cfg
            unsafe impl #impl_generics Sync for #name #impl_lifetime {}

            #cfg
            impl #impl_generics #name #impl_lifetime {
                pub const DEFAULT: Self = unsafe {
                    Self { #first_fname: core::mem::zeroed::<#first_ftype>() }
                };
                #[inline]
                pub const fn new() -> Self { Self::DEFAULT }
            }

            #cfg
            impl #impl_generics core::fmt::Debug for #name #impl_lifetime {
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
        let builder_setters = gen_builder_setters(s, reg);
        let pnext_marker_impls = gen_pnext_marker_impls(s, reg);
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
            pub struct #name #decl_lifetime {
                #field_toks
                #marker_field
            }

            #cfg
            unsafe impl #impl_generics Send for #name #impl_lifetime {}
            #cfg
            unsafe impl #impl_generics Sync for #name #impl_lifetime {}

            #pnext_marker_impls

            #cfg
            impl #impl_generics #name #impl_lifetime { #impl_body }
        });
        doc
    }
}

fn structure_type_value_for_providers(
    reg: &Registry,
    target_name: &str,
    providers: &[String],
) -> String {
    let Some(enums) = reg.enums.get("VkStructureType") else {
        return target_name.to_owned();
    };
    for e in enums {
        for variant in &e.variants {
            if variant.name == target_name
                && let Some(alias) = &variant.alias
            {
                return alias.clone();
            }
        }
    }
    for e in enums {
        for variant in &e.variants {
            if variant.name == target_name
                && variant
                    .provided_by
                    .iter()
                    .any(|provider| providers.contains(provider))
            {
                return target_name.to_owned();
            }
        }
    }
    for e in enums {
        for variant in &e.variants {
            if variant.alias.as_deref() == Some(target_name)
                && variant
                    .provided_by
                    .iter()
                    .any(|provider| providers.contains(provider))
            {
                return variant.name.clone();
            }
        }
    }
    target_name.to_owned()
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

    if let Some(td) = reg.typedefs.get(base).and_then(|v| v.first()) {
        return match td.kind {
            TypedefKind::Handle { .. } => (TypeClass::StructWithDefault, base.to_owned()),
            TypedefKind::Basetype => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::Bitmask => {
                if let Some(bits) = typed_bitmask_target(td, reg) {
                    (TypeClass::EnumNewtype, bits)
                } else {
                    (TypeClass::PrimitiveAlias, base.to_owned())
                }
            }
            TypedefKind::Alias => (TypeClass::PrimitiveAlias, base.to_owned()),
            TypedefKind::FuncPointer => (TypeClass::OptionType, base.to_owned()),
            TypedefKind::OpaqueExtern => (TypeClass::NullMutAlias, base.to_owned()),
            TypedefKind::Define => (TypeClass::PrimitiveAlias, base.to_owned()),
        };
    }

    if reg.enums.contains_key(base) {
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
