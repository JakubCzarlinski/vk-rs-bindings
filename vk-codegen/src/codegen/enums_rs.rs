use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

use crate::cfggen::cfg_any;
use crate::codegen::{depr_attr, feat_key, pretty, refpage_url};
use crate::ir::{Enum, EnumValue, Registry};

pub fn gen_enums_rs(reg: &Registry) -> String {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<Vec<String>, TokenStream> = BTreeMap::new();
    for e in reg.enums.values() {
        let ts = gen_enum(e);
        if !ts.is_empty() {
            let mut all_feats: Vec<String> = e.provided_by.clone();
            for v in &e.variants {
                for f in &v.provided_by {
                    if !all_feats.contains(f) {
                        all_feats.push(f.clone());
                    }
                }
            }
            groups.entry(feat_key(&all_feats)).or_default().extend(ts);
        }
    }

    let mut ts = TokenStream::new();
    ts.extend(quote! {
        //! Vulkan enum and bitmask types.
        //!
        //! Enums are `repr(transparent)` newtypes over `i32`/`i64`.
        //! Bitmasks are `repr(transparent)` newtypes over `u32`/`u64`
        //! with `|`, `&`, `^`, `!` and compound-assignment operators.
    });
    for (_, items) in groups {
        ts.extend(items);
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
        #cfg impl core::ops::BitOr        for #name { type Output=Self; #[inline] fn bitor   (self,r:Self)->Self{Self(self.0|r.0)} }
        #cfg impl core::ops::BitOrAssign  for #name { #[inline] fn bitor_assign   (&mut self,r:Self){self.0|=r.0} }
        #cfg impl core::ops::BitAnd       for #name { type Output=Self; #[inline] fn bitand  (self,r:Self)->Self{Self(self.0&r.0)} }
        #cfg impl core::ops::BitAndAssign for #name { #[inline] fn bitand_assign  (&mut self,r:Self){self.0&=r.0} }
        #cfg impl core::ops::BitXor       for #name { type Output=Self; #[inline] fn bitxor  (self,r:Self)->Self{Self(self.0^r.0)} }
        #cfg impl core::ops::BitXorAssign for #name { #[inline] fn bitxor_assign  (&mut self,r:Self){self.0^=r.0} }
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
