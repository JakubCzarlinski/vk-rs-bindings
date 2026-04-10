use crate::cfggen::cfg_any;
use crate::codegen::{deprecate_attr, feature_key, pretty, refpage_url};
use crate::ir::{Enum, EnumValue, Registry};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, HashSet};

pub fn gen_enums_rs(reg: &Registry) -> String {
    let disabled: HashSet<String> = reg
        .extensions
        .iter()
        .filter(|e| e.is_disabled())
        .map(|e| e.name.clone())
        .collect();

    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();

    let mut seen_features = HashSet::new();
    for e in reg.enums.values().flatten() {
        let token_stream = gen_enum(e, &disabled);
        if token_stream.is_empty() {
            continue;
        }

        // Collect unique features.
        let all_feats: Vec<String> = e
            .provided_by
            .iter()
            .chain(e.variants.iter().flat_map(|v| v.provided_by.iter()))
            .filter(|&feature| !disabled.contains(feature))
            .filter(|&feature| seen_features.insert(feature.clone()))
            .cloned()
            .collect();

        groups
            .entry(feature_key(&all_feats))
            .or_default()
            .extend(token_stream);
        seen_features.clear();
    }

    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan enum and bitmask types.
        //!
        //! Enums are `repr(transparent)` newtypes over `i32`/`i64`.
        //! Bitmasks are `repr(transparent)` newtypes over `u32`/`u64`
        //! with `|`, `&`, `^`, `!` and compound-assignment operators.
    });

    for items in groups.into_values() {
        ts.extend(items);
    }

    pretty(&ts)
}

fn gen_enum(e: &Enum, disabled: &HashSet<String>) -> TokenStream {
    // Filter variants to those that are enabled or core.
    let variants: Vec<_> = e
        .variants
        .iter()
        .filter(|v| v.provided_by.is_empty() || v.provided_by.iter().any(|f| !disabled.contains(f)))
        .cloned()
        .collect();

    // Features for the enum: its own enabled providers + enabled providers of its kept variants.
    let mut all_feats: Vec<String> = e
        .provided_by
        .iter()
        .filter(|f| !disabled.contains(*f))
        .cloned()
        .collect();
    for variant in &variants {
        for feature in &variant.provided_by {
            if !disabled.contains(feature) && !all_feats.contains(feature) {
                all_feats.push(feature.clone());
            }
        }
    }

    // If the enum was introduced by an extension (non-empty provided_by) but all its providers
    // and variants are disabled, skip it entirely.
    if !e.provided_by.is_empty() && all_feats.is_empty() && variants.is_empty() {
        return quote! {};
    }

    let cfg = cfg_any(&all_feats);
    let name = format_ident!("{}", &e.name);
    let url_str = format!(" [{}]({})", &e.name, refpage_url(&e.name));
    let mut doc = quote! { #[doc = #url_str] };
    if let Some(ref comment) = e.comment {
        let comment = comment.trim();
        if !comment.is_empty() {
            doc.extend(quote! { #[doc = " "] });
            let comment = " ".to_string() + comment;
            doc.extend(quote! { #[doc = #comment] });
        }
    }
    if let Some(ref dep) = e.dep {
        doc.extend(quote! { #[doc = " "] });
        let depends_on = dep.atoms().join("`, `");
        let comment = format!(" **Availability:** depends on `{depends_on}`.");
        doc.extend(quote! { #[doc = #comment] });
    }

    let depr = deprecate_attr(&e.depr);

    if let Some(ref alias) = e.alias {
        let a = format_ident!("{}", alias);
        return quote! { #cfg pub type #name = #a; };
    }

    if e.is_bitmask {
        return gen_bitmask_type(e, cfg, name, doc.clone(), &all_feats, depr, disabled);
    }

    let inner = if e.bit_width == 64 {
        quote! {i64}
    } else {
        quote! {i32}
    };
    let mut variant_token_stream = TokenStream::new();
    let mut seen_features: HashSet<String> = HashSet::new();

    for variant in variants {
        if !seen_features.insert(variant.name.clone()) {
            continue;
        }
        let variant_name = format_ident!("{}", &variant.name);
        let variant_doc = variant.comment.as_deref().unwrap_or("");
        let variant_depr = deprecate_attr(&variant.depr);
        let v_feats: Vec<_> = variant
            .provided_by
            .iter()
            .filter(|f| !disabled.contains(*f))
            .cloned()
            .collect();
        let mut variant_cfg = if v_feats.is_empty() || v_feats == all_feats {
            quote! {}
        } else {
            cfg_any(&v_feats)
        };

        if let Some(ref aset) = variant.api {
            if aset.vulkansc && !aset.vulkan {
                variant_cfg = quote! { #variant_cfg #[cfg(feature = "VKSC_VERSION_1_0")] };
            } else if aset.vulkan && !aset.vulkansc {
                variant_cfg = quote! { #variant_cfg #[cfg(not(feature = "VKSC_VERSION_1_0"))] };
            }
        }

        let val = enum_val_tokens(&variant.value, false);
        if variant_doc.is_empty() {
            variant_token_stream.extend(quote! {
                #variant_cfg #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        } else {
            variant_token_stream.extend(quote! {
                #variant_cfg #[doc = #variant_doc] #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        }
    }

    doc.extend(quote! {
        #cfg #depr
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
        pub struct #name(pub #inner);

        #cfg
        impl #name { #variant_token_stream }
    });
    doc
}

fn gen_bitmask_type(
    e: &Enum,
    cfg: TokenStream,
    name: proc_macro2::Ident,
    doc: TokenStream,
    all_feats: &[String],
    depr: TokenStream,
    disabled: &HashSet<String>,
) -> TokenStream {
    let mut doc = doc;
    let inner = if e.bit_width == 64 {
        quote! {u64}
    } else {
        quote! {u32}
    };
    let mut bit_token_stream = TokenStream::new();
    let mut seen_features: HashSet<String> = HashSet::new();

    for variant in &e.variants {
        if !seen_features.insert(variant.name.clone()) {
            continue;
        }

        let variant_name = format_ident!("{}", &variant.name);
        let variant_doc = variant.comment.as_deref().unwrap_or("");
        let variant_depr = deprecate_attr(&variant.depr);
        let v_feats: Vec<_> = variant
            .provided_by
            .iter()
            .filter(|f| !disabled.contains(*f))
            .cloned()
            .collect();
        let mut variant_cfg = if v_feats.is_empty() || v_feats == all_feats {
            quote! {}
        } else {
            cfg_any(&v_feats)
        };

        if let Some(ref aset) = variant.api {
            if aset.vulkansc && !aset.vulkan {
                variant_cfg = quote! { #variant_cfg #[cfg(feature = "VKSC_VERSION_1_0")] };
            } else if aset.vulkan && !aset.vulkansc {
                variant_cfg = quote! { #variant_cfg #[cfg(not(feature = "VKSC_VERSION_1_0"))] };
            }
        }

        let val = enum_val_tokens(&variant.value, true);
        if variant_doc.is_empty() {
            bit_token_stream.extend(quote! {
                #variant_cfg #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        } else {
            bit_token_stream.extend(quote! {
                #variant_cfg #[doc = #variant_doc] #variant_depr
                pub const #variant_name: Self = Self(#val);
            });
        }
    }

    doc.extend(
        quote! {
            #cfg #depr
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            pub struct #name(pub #inner);

            #cfg
            impl #name {
                pub const EMPTY: Self = Self(0);
                #bit_token_stream
                #[inline] pub const fn contains(self, o: Self) -> bool { (self.0 & o.0) == o.0 }
                #[inline] pub const fn intersects(self, o: Self) -> bool { (self.0 & o.0) != 0 }
                #[inline] pub const fn is_empty(self) -> bool { self.0 == 0 }
            }
            #cfg impl core::ops::BitOr        for #name { type Output=Self; #[inline] fn bitor   (self,r:Self)->Self{Self(self.0|r.0)} }
            #cfg impl core::ops::BitOrAssign  for #name { #[inline] fn bitor_assign   (&mut self,r:Self){self.0|=r.0} }
            #cfg impl core::ops::BitAnd       for #name { type Output=Self; #[inline] fn bitand  (self,r:Self)->Self{Self(self.0&r.0)} }
            #cfg impl core::ops::BitAndAssign for #name { #[inline] fn bitand_assign  (&mut self,r:Self){self.0&=r.0} }
            #cfg impl core::ops::BitXor       for #name { type Output=Self; #[inline] fn bitxor  (self,r:Self)->Self{Self(self.0^r.0)} }
            #cfg impl core::ops::BitXorAssign for #name { #[inline] fn bitxor_assign  (&mut self,r:Self){self.0^=r.0} }
            #cfg impl core::ops::Not          for #name { type Output=Self; #[inline] fn not(self)->Self{Self(!self.0)} }

            #cfg impl core::ops::BitOr        <#inner> for #name { type Output=Self; #[inline] fn bitor   (self,r:#inner)-> Self{Self(self.0|r)} }
            #cfg impl core::ops::BitOrAssign  <#inner> for #name { #[inline] fn bitor_assign   (&mut self,r:#inner){self.0|=r} }
            #cfg impl core::ops::BitAnd       <#inner> for #name { type Output=Self; #[inline] fn bitand  (self,r:#inner)-> Self{Self(self.0&r)} }
            #cfg impl core::ops::BitAndAssign  <#inner> for #name { #[inline] fn bitand_assign  (&mut self,r:#inner){self.0&=r} }
            #cfg impl core::ops::BitXor       <#inner> for #name { type Output=Self; #[inline] fn bitxor  (self,r:#inner)-> Self{Self(self.0^r)} }
            #cfg impl core::ops::BitXorAssign  <#inner> for #name { #[inline] fn bitxor_assign  (&mut self,r:#inner){self.0^=r} }
        }
    );
    doc
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
            let p = u64::from(*p);
            quote! { 1 << #p }
        }
        EnumValue::Offset {
            extnumber,
            offset,
            negative,
        } => {
            let v = 1_000_000_000i64 + (i64::from(*extnumber) - 1) * 1000 + i64::from(*offset);
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
