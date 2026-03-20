//! Generates `#[cfg(...)]` token streams from `DepExpr` and feature-name lists.

#![allow(dead_code)]

use proc_macro2::TokenStream;
use quote::quote;
use crate::ir::DepExpr;

// ── Public API ────────────────────────────────────────────────────────────────

/// `#[cfg(feature = "A")]`  or  `#[cfg(any(feature="A", feature="B", ...))]`
pub fn cfg_any(features: &[String]) -> TokenStream {
    match features.len() {
        0 => quote! {},
        1 => { let f = &features[0]; quote! { #[cfg(feature = #f)] } }
        _ => {
            let items: Vec<TokenStream> = features.iter().map(|f| quote! { feature = #f }).collect();
            quote! { #[cfg(any(#(#items),*))] }
        }
    }
}

/// `#[cfg(...)]` derived from a `DepExpr` (converts to DNF first).
pub fn cfg_from_dep(dep: &DepExpr) -> TokenStream {
    let clauses = dep.to_dnf_clauses();
    let inner = clauses_to_ts(&clauses);
    quote! { #[cfg(#inner)] }
}

/// Just the inner `cfg(...)` expression (no `#[...]` wrapper) for a list of features.
pub fn cfg_expr_any(features: &[String]) -> TokenStream {
    match features.len() {
        0 => quote! { all() },
        1 => { let f = &features[0]; quote! { feature = #f } }
        _ => {
            let items: Vec<TokenStream> = features.iter().map(|f| quote! { feature = #f }).collect();
            quote! { any(#(#items),*) }
        }
    }
}

// ── Internals ─────────────────────────────────────────────────────────────────

fn clauses_to_ts(clauses: &[Vec<String>]) -> TokenStream {
    match clauses.len() {
        0 => quote! { all() },
        1 => clause_to_ts(&clauses[0]),
        _ => {
            let cs: Vec<TokenStream> = clauses.iter().map(|c| clause_to_ts(c)).collect();
            quote! { any(#(#cs),*) }
        }
    }
}

fn clause_to_ts(clause: &[String]) -> TokenStream {
    match clause.len() {
        0 => quote! { all() },
        1 => { let f = &clause[0]; quote! { feature = #f } }
        _ => {
            let items: Vec<TokenStream> = clause.iter().map(|f| quote! { feature = #f }).collect();
            quote! { all(#(#items),*) }
        }
    }
}
