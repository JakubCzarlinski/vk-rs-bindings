use crate::cfggen::cfg_any;
use crate::ir::{CType, Command, Member, Optional, Registry, TypedefKind};
use crate::types::c_type_to_rust;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Copy, PartialEq)]
pub enum Tier {
    Instance,
    Device,
}

// Commands pinned to the instance tier.
//
// `vkCreateDevice` has `VkPhysicalDevice` as param[0], which is instance-tier
// by convention.  Some registry alias chains resolve to a variant with a
// different first param and would misroute it into the device tier, leaving
// `InstanceDispatchTable` without the field.  Pinning it here prevents that.
//
// `vkGetDeviceProcAddr` has `VkDevice` as param[0], which would normally route
// it to the device tier.  It is however a core instance-tier command: it is
// resolved via `vkGetInstanceProcAddr` and lives on `InstanceDispatchTable` so
// that `Instance::vkCreateDevice` can use it to load the device table.
const INSTANCE_PINNED: &[&str] = &["vkCreateDevice", "vkGetDeviceProcAddr"];

pub enum WrapperReturn<'a> {
    /// `void` -> `()`
    Unit,
    /// Non-`VkResult` -> forward raw return type
    Raw(&'a CType),
    /// `VkResult` + single handle out-param -> `Result<Handle, VkResult>`
    ResultHandle { handle_ty: &'a CType },
    /// Two-call enumerate -> `Result<Box<[T]>, VkResult>`
    Enumerate {
        item_ty: &'a CType,
        count_idx: usize,
        array_idx: usize,
    },
    /// All other `VkResult` commands -> `Result<VkResult, VkResult>`
    ResultRaw,
}

pub fn classify_return<'a>(cmd: &'a Command, handle_types: &HashSet<String>) -> WrapperReturn<'a> {
    let ret = &cmd.return_type;

    if ret.base.is_empty() || ret.base == "void" {
        return WrapperReturn::Unit;
    }
    if ret.base != "VkResult" {
        return WrapperReturn::Raw(ret);
    }

    // Enumerate: *mut u32 count param followed by Optional::TrueTrue array param.
    let count_idx = cmd
        .params
        .iter()
        .position(|m| m.ty.base == "uint32_t" && m.ty.pointer_depth == 1 && !m.ty.is_const);
    if let Some(ci) = count_idx {
        let array_idx = cmd.params.iter().enumerate().position(|(i, m)| {
            i > ci
                && m.ty.pointer_depth == 1
                && !m.ty.is_const
                && !m.ty.base.is_empty()
                && matches!(m.optional, Optional::TrueTrue)
        });
        if let Some(ai) = array_idx {
            return WrapperReturn::Enumerate {
                item_ty: &cmd.params[ai].ty,
                count_idx: ci,
                array_idx: ai,
            };
        }
    }

    // Single out-handle: writable pointer, no array, Optional::False, genuine
    // handle type, last parameter.
    let out_params: Vec<(usize, &Member)> = cmd
        .params
        .iter()
        .enumerate()
        .filter(|(_, m)| {
            m.ty.pointer_depth == 1
                && !m.ty.is_const
                && m.ty.is_array.is_none()
                && matches!(m.optional, Optional::False)
                && handle_types.contains(&m.ty.base)
        })
        .collect();

    if out_params.len() == 1 {
        let (idx, m) = out_params[0];
        if idx == cmd.params.len() - 1 {
            return WrapperReturn::ResultHandle { handle_ty: &m.ty };
        }
    }

    WrapperReturn::ResultRaw
}

pub fn build_result_cfg_map(reg: &Registry) -> HashMap<String, TokenStream> {
    let mut map = HashMap::new();
    let Some(variants) = reg.enums.get("VkResult") else {
        return map;
    };
    for enum_def in variants {
        for variant in &enum_def.variants {
            let cfg = if variant.provided_by.is_empty() {
                quote! {}
            } else {
                cfg_any(&variant.provided_by)
            };
            map.entry(variant.name.clone()).or_insert(cfg);
        }
    }
    map
}

pub fn build_handle_type_set(reg: &Registry) -> HashSet<String> {
    let mut set = HashSet::new();
    for (name, variants) in &reg.typedefs {
        if variants
            .iter()
            .any(|t| matches!(t.kind, TypedefKind::Handle { .. }))
        {
            set.insert(name.clone());
        }
    }
    set
}

pub fn vk_result_check(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    result_check_arms(&cmd.success_codes, &cmd.error_codes, cfg_map)
}

pub fn vk_result_is_err(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    if cmd.error_codes.is_empty() {
        return quote! { r < VkResult::VK_SUCCESS };
    }
    let arms: Vec<TokenStream> = cmd
        .error_codes
        .iter()
        .map(|s| {
            let id = format_ident!("{}", s);
            let cfg = cfg_map.get(s).cloned().unwrap_or_default();
            quote! { #cfg VkResult::#id }
        })
        .collect();
    quote! { matches!(r, #(#arms)|*) || r < VkResult::VK_SUCCESS }
}

pub fn result_check_arms(
    success_codes: &[String],
    error_codes: &[String],
    cfg_map: &HashMap<String, TokenStream>,
) -> TokenStream {
    let ok_arms = cfg_grouped_arms(success_codes, cfg_map, true);
    let err_arms = cfg_grouped_arms(error_codes, cfg_map, false);
    quote! {
        match r {
            #(#ok_arms)*
            #(#err_arms)*
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) },
        }
    }
}

fn cfg_grouped_arms(
    codes: &[String],
    cfg_map: &HashMap<String, TokenStream>,
    is_ok: bool,
) -> Vec<TokenStream> {
    if codes.is_empty() {
        return vec![];
    }
    let mut by_cfg: BTreeMap<String, (TokenStream, Vec<TokenStream>)> = BTreeMap::new();
    for s in codes {
        let id = format_ident!("{}", s);
        let cfg = cfg_map.get(s).cloned().unwrap_or_default();
        by_cfg
            .entry(cfg.to_string())
            .or_insert_with(|| (cfg, Vec::new()))
            .1
            .push(quote! { VkResult::#id });
    }
    by_cfg
        .into_values()
        .map(|(cfg, pats)| {
            let result = if is_ok {
                quote! { Ok(r) }
            } else {
                quote! { Err(r) }
            };
            quote! { #cfg #(#pats)|* => #result, }
        })
        .collect()
}

// Command grouping

/// Feature-keyed map of `(name, providers, resolved_command)`.
/// Commands within each bucket are sorted alphabetically; both the dispatch
/// table and the safe wrapper iterate this in the same order.
pub type Groups = BTreeMap<String, Vec<(String, Vec<String>, Command)>>;

pub fn collect_groups(
    reg: &Registry,
    tier: Tier,
    skip: &HashSet<&str>,
    enabled: &HashSet<String>,
) -> Groups {
    let pinned: HashSet<&str> = INSTANCE_PINNED.iter().copied().collect();
    let mut groups: Groups = BTreeMap::new();

    for (name, variants) in &reg.commands {
        if skip.contains(name.as_str()) {
            continue;
        }

        let is_instance = variants.iter().any(is_instance_cmd) || pinned.contains(name.as_str());

        let matches = match tier {
            Tier::Instance => is_instance,
            Tier::Device => !is_instance,
        };
        if !matches {
            continue;
        }

        let cmd_raw = variants.last().unwrap();
        if !variants.iter().any(|c| c.api.vulkan || c.api.vulkanbase) {
            continue;
        }
        let cmd = if pinned.contains(name.as_str()) {
            resolve_pinned(name, reg)
        } else {
            resolve_alias(cmd_raw, reg)
        };
        let mut providers: Vec<String> = variants
            .iter()
            .flat_map(|c| c.provided_by.clone())
            .collect();
        providers.sort();
        providers.dedup();
        if providers.is_empty() {
            continue;
        }
        let primary = pick_primary(&providers, enabled);
        groups
            .entry(primary)
            .or_default()
            .push((name.clone(), providers, cmd));
    }

    for cmds in groups.values_mut() {
        cmds.sort_by(|a, b| a.0.cmp(&b.0));
    }
    groups
}

/// For pinned commands: find the first variant with non-empty params rather
/// than following the alias chain, which may end at a device-tier variant.
fn resolve_pinned(name: &str, reg: &Registry) -> Command {
    let variants = reg
        .commands
        .get(name)
        .unwrap_or_else(|| panic!("pinned command {name} not found in registry"));
    variants
        .iter()
        .find(|c| !c.params.is_empty())
        .or_else(|| variants.last())
        .unwrap()
        .clone()
}
pub fn resolve_alias(cmd: &Command, reg: &Registry) -> Command {
    let mut current = cmd;
    for i in 0..10 {
        if !current.params.is_empty() {
            break;
        }
        let Some(alias_name) = &current.alias else {
            break;
        };
        let Some(alias_variants) = reg.commands.get(alias_name) else {
            break;
        };
        let Some(alias_cmd) = alias_variants.last() else {
            break;
        };
        current = alias_cmd;
        if i == 9 {
            panic!("alias chain too long resolving {}", cmd.name);
        }
    }
    let mut resolved = current.clone();
    if resolved.success_codes.is_empty() && !cmd.success_codes.is_empty() {
        resolved.success_codes = cmd.success_codes.clone();
    }
    if resolved.error_codes.is_empty() && !cmd.error_codes.is_empty() {
        resolved.error_codes = cmd.error_codes.clone();
    }
    resolved
}

pub fn enabled_set(reg: &Registry) -> HashSet<String> {
    reg.features
        .iter()
        .map(|f| f.name.clone())
        .chain(
            reg.extensions
                .iter()
                .filter(|e| !e.is_disabled())
                .map(|e| e.name.clone()),
        )
        .collect()
}

pub fn pick_primary(providers: &[String], enabled: &HashSet<String>) -> String {
    providers
        .iter()
        .find(|f| f.starts_with("VK_BASE_VERSION_") || f.starts_with("VK_VERSION_"))
        .or_else(|| providers.iter().find(|f| f.starts_with("VK_KHR_")))
        .or_else(|| providers.iter().find(|f| f.starts_with("VK_EXT_")))
        .or_else(|| providers.iter().find(|f| enabled.contains(*f)))
        .cloned()
        .unwrap_or_else(|| providers[0].clone())
}

pub fn is_core(providers: &[String]) -> bool {
    providers
        .iter()
        .any(|f| f.starts_with("VK_BASE_VERSION_") || f.starts_with("VK_VERSION_"))
}

pub fn is_instance_cmd(cmd: &Command) -> bool {
    match cmd.params.first() {
        Some(m) => m.ty.base == "VkInstance" || m.ty.base == "VkPhysicalDevice",
        None => true,
    }
}

pub fn is_cmd_buf_cmd(cmd: &Command) -> bool {
    cmd.params
        .first()
        .map(|m| m.ty.base == "VkCommandBuffer")
        .unwrap_or(false)
}

// Token helpers

pub fn params_to_tokens(params: &[Member]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        })
        .unzip()
}

pub fn strip_first_param(params: &[Member]) -> &[Member] {
    params.get(1..).unwrap_or(&[])
}

pub fn strip_out_param(params: &[Member]) -> Vec<Member> {
    let mut out = params.to_vec();
    if out
        .last()
        .map(|m| m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base.starts_with("Vk"))
        .unwrap_or(false)
    {
        out.pop();
    }
    out
}

pub fn deref_ctype(ty: &CType) -> TokenStream {
    base_type_tokens(&ty.base)
}

pub fn ctype_to_tokens(ty: &CType) -> TokenStream {
    if (ty.base.is_empty() || ty.base == "void") && ty.pointer_depth == 0 && ty.is_array.is_none() {
        return quote! { () };
    }
    let base = base_type_tokens(&ty.base);
    let mut ts = if let Some(ref size) = ty.is_array {
        let size_ts: TokenStream = if size.parse::<u64>().is_ok() {
            size.parse().unwrap()
        } else {
            format!("{} as usize", size).parse().unwrap()
        };
        quote! { [#base; #size_ts] }
    } else {
        base
    };
    for _ in 0..ty.pointer_depth {
        ts = if ty.is_const {
            quote! { *const #ts }
        } else {
            quote! { *mut #ts }
        };
    }
    ts
}

pub fn base_type_tokens(base: &str) -> TokenStream {
    let resolved = c_type_to_rust(base);
    let name = if resolved.is_empty() {
        if base.is_empty() {
            "core::ffi::c_void"
        } else {
            base
        }
    } else {
        resolved
    };
    name.parse::<TokenStream>()
        .unwrap_or_else(|_| format_ident!("{}", name).into_token_stream())
}

pub fn c_str_lit(name: &str) -> TokenStream {
    format!("c\"{}\"", name).parse().unwrap()
}

pub fn kw_escape(name: &str) -> &str {
    match name {
        "type" => "ty",
        "ref" => "reference",
        "mod" => "module",
        "in" => "input",
        "out" => "output",
        "use" => "usage",
        "loop" => "loop_",
        "match" => "match_",
        "where" => "where_",
        "return" => "return_",
        other => other,
    }
}

// Safe method body
//
// Shared by Entry, Instance, Device, CommandBuffer.  The caller supplies:
//   - `handle_base`  - the Vulkan type string of param[0] to strip from the
//                      signature and replace with `self_handle` in the call
//                      (e.g. "VkInstance").  Pass "" for Entry, where there is
//                      no dispatchable param[0] to strip.
//   - `self_handle`  - tokens for the handle value (`self.raw`, etc.)
//   - `table_expr`   - tokens to reach the dispatch table (`&self.table`, etc.)
#[allow(clippy::too_many_arguments)]
pub fn safe_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    handle_base: &str,        // "" = no stripping (Entry)
    self_handle: TokenStream, // value to substitute for param[0]
    table_expr: TokenStream,  // how to reach the PFN table
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", name);
    let miss = format!("command not loaded: {}", name);

    let core_fn = is_core(providers);
    let fp = if core_fn {
        quote! { (#table_expr).#fname.unwrap_unchecked() }
    } else {
        quote! { (#table_expr).#fname.expect(#miss) }
    };

    // Strip param[0] when it matches the wrapper's handle type.
    let strips_first = !handle_base.is_empty()
        && cmd
            .params
            .first()
            .map(|p| p.ty.base == handle_base)
            .unwrap_or(false);

    let sig_params: &[Member] = if strips_first {
        strip_first_param(&cmd.params)
    } else {
        &cmd.params
    };

    // Build the forwarding argument list.  param[0] becomes self_handle when
    // we strip it; the rest forward their names directly.
    let mut fwd: Vec<TokenStream> = cmd
        .params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            quote! { #n }
        })
        .collect();
    if strips_first {
        fwd[0] = quote! { #self_handle };
    }

    let (p_defs, _) = params_to_tokens(sig_params);

    match classify_return(cmd, handle_types) {
        WrapperReturn::Unit => quote! {
            #cfg
            #[inline(always)]
            pub fn #fname(&self, #(#p_defs),*) {
                unsafe { (#fp)(#(#fwd),*) }
            }
        },

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens(ret_ty);
            quote! {
                #cfg
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> #ret {
                    unsafe { (#fp)(#(#fwd),*) }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let inner = strip_out_param(sig_params);
            let (p_defs, _) = params_to_tokens(&inner);
            let check = vk_result_check(cmd, result_cfgs);
            let last = fwd.len().saturating_sub(1);
            if !fwd.is_empty() {
                fwd[last] = quote! { &mut handle };
            }
            quote! {
                #cfg
                #[inline]
                pub fn #fname(&self, #(#p_defs),*) -> Result<#h_ty, VkResult> {
                    let mut handle = #h_ty::NULL;
                    let r = unsafe { (#fp)(#(#fwd),*) };
                    #check .map(|_| handle)
                }
            }
        }

        WrapperReturn::Enumerate {
            item_ty,
            count_idx,
            array_idx,
        } => {
            let elem_ty = deref_ctype(item_ty);
            let ci = count_idx;
            let ai = array_idx;

            // Signature: drop the count and array params, keep everything else.
            let keep: Vec<Member> = sig_params
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    // Adjust index back to the full param list to compare with ci/ai.
                    let full = if strips_first { *i + 1 } else { *i };
                    full != ci && full != ai
                })
                .map(|(_, m)| m.clone())
                .collect();
            let (p_defs, _) = params_to_tokens(&keep);

            let is_err = vk_result_is_err(cmd, result_cfgs);
            let check2 = vk_result_check(cmd, result_cfgs);

            let mut fwd_first = fwd.clone();
            fwd_first[ci] = quote! { &mut count };
            fwd_first[ai] = quote! { core::ptr::null_mut() };

            let mut fwd_second = fwd.clone();
            fwd_second[ci] = quote! { &mut count };
            fwd_second[ai] = quote! { out.as_mut_ptr() };

            quote! {
                #cfg
                #[inline]
                pub fn #fname(&self, #(#p_defs),*) -> Result<Box<[#elem_ty]>, VkResult> {
                    let mut count: u32 = 0;
                    let r = unsafe { (#fp)(#(#fwd_first),*) };
                    if #is_err { return Err(r); }
                    if count == 0 { return Ok(Box::default()); }
                    let mut out = Box::<[#elem_ty]>::new_uninit_slice(count as usize);
                    let r = unsafe { (#fp)(#(#fwd_second),*) };
                    #check2 .map(|_| unsafe { out.assume_init() })
                }
            }
        }

        WrapperReturn::ResultRaw => {
            let check = vk_result_check(cmd, result_cfgs);
            quote! {
                #cfg
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> Result<VkResult, VkResult> {
                    let r = unsafe { (#fp)(#(#fwd),*) };
                    #check
                }
            }
        }
    }
}
