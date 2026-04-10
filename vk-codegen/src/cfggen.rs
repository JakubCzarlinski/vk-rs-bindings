//! Generates `#[cfg(...)]` token streams from `DepExpr` and feature-name lists.

use proc_macro2::TokenStream;
use quote::quote;

/// `#[cfg(feature = "A")]`  or  `#[cfg(any(feature="A", feature="B", ...))]`
#[must_use]
pub fn cfg_any(features: &[String]) -> TokenStream {
    match features.len() {
        0 => quote! {},
        1 => {
            let f = &features[0];
            quote! { #[cfg(feature = #f)] }
        }
        _ => {
            let items: Vec<TokenStream> =
                features.iter().map(|f| quote! { feature = #f }).collect();
            quote! { #[cfg(any(#(#items),*))] }
        }
    }
}

/// Convert DNF clauses to a `cfg` expression.
#[must_use]
pub fn cfg_expr_from_dnf(clauses: &[Vec<String>]) -> TokenStream {
    clauses_to_ts(clauses)
}

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
        1 => {
            let f = &clause[0];
            quote! { feature = #f }
        }
        _ => {
            let items: Vec<TokenStream> = clause.iter().map(|f| quote! { feature = #f }).collect();
            quote! { all(#(#items),*) }
        }
    }
}
