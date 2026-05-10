//! Generates `#[cfg(...)]` token streams from `DepExpr` and feature-name lists.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ir::{Availability, DepExpr};

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

#[must_use]
pub fn cfg_providers_with_dep(features: &[String], dep: Option<&DepExpr>) -> TokenStream {
    let Some(dep) = dep else {
        return cfg_any(features);
    };

    let mut clauses = Vec::<Vec<String>>::new();
    let dep_clauses = dep.to_dnf_clauses();

    if features.is_empty() {
        clauses = dep_clauses;
    } else {
        for provider in features {
            for dep_clause in &dep_clauses {
                let mut clause = dep_clause.clone();
                if !clause.contains(provider) {
                    clause.insert(0, provider.clone());
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
    }

    let expr = cfg_expr_from_dnf(&clauses);
    quote! { #[cfg(#expr)] }
}

#[must_use]
pub fn cfg_availability(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> TokenStream {
    if availability.is_empty() {
        return cfg_providers_with_dep(fallback_features, fallback_dep);
    }

    let clauses = availability_clauses(availability, fallback_features, fallback_dep);
    let expr = cfg_expr_from_dnf(&clauses);
    quote! { #[cfg(#expr)] }
}

#[must_use]
pub fn cfg_not_availability(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> TokenStream {
    let clauses = availability_clauses(availability, fallback_features, fallback_dep);
    let expr = cfg_expr_from_dnf(&clauses);
    quote! { #[cfg(not(#expr))] }
}

pub fn push_availability(
    availability: &mut Vec<Availability>,
    provider: &str,
    dep: &Option<DepExpr>,
) {
    if provider.is_empty() {
        return;
    }
    let item = Availability::new(provider.to_owned(), dep.clone());
    if !availability.contains(&item) {
        availability.push(item);
    }
}

pub fn set_dep_if_unset(dst: &mut Option<DepExpr>, dep: &Option<DepExpr>) {
    if dep.is_none() {
        *dst = None;
    } else if dst.is_none() {
        *dst = dep.clone();
    }
}

fn availability_clauses(
    availability: &[Availability],
    fallback_features: &[String],
    fallback_dep: Option<&DepExpr>,
) -> Vec<Vec<String>> {
    if availability.is_empty() {
        let Some(dep) = fallback_dep else {
            return fallback_features
                .iter()
                .map(|provider| vec![provider.clone()])
                .collect();
        };

        let dep_clauses = dep.to_dnf_clauses();
        if fallback_features.is_empty() {
            return dep_clauses;
        }

        let mut clauses = Vec::<Vec<String>>::new();
        for provider in fallback_features {
            for dep_clause in &dep_clauses {
                let mut clause = dep_clause.clone();
                if !clause.contains(provider) {
                    clause.insert(0, provider.clone());
                }
                clause.sort();
                if !clauses.contains(&clause) {
                    clauses.push(clause);
                }
            }
        }
        return clauses;
    }

    let mut clauses = Vec::<Vec<String>>::new();
    for item in availability {
        let dep_clauses = item
            .dep
            .as_ref()
            .map(DepExpr::to_dnf_clauses)
            .unwrap_or_else(|| vec![vec![]]);
        for mut clause in dep_clauses {
            if !clause.contains(&item.provider) {
                clause.insert(0, item.provider.clone());
            }
            clause.sort();
            if !clauses.contains(&clause) {
                clauses.push(clause);
            }
        }
    }
    clauses
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
