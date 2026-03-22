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

/// Describes what a generated safe wrapper method should return.
pub enum WrapperReturn<'a> {
    /// `void` return -> `()`
    Unit,
    /// Non-`VkResult` return -> forward the raw type
    Raw(&'a CType),
    /// `VkResult` + single `*mut VkFoo` out-param at end -> `Result<VkFoo, VkResult>`
    ResultHandle { handle_ty: &'a CType },
    /// Two-call enumerate pattern -> `Result<Box<[T]>, VkResult>`
    Enumerate {
        item_ty: &'a CType,
        count_idx: usize,
        array_idx: usize,
    },
    /// Everything else fallible -> `Result<VkResult, VkResult>`
    ResultRaw,
}

/// Classify a command's return shape for safe wrapper generation.
pub fn classify_return<'a>(cmd: &'a Command, handle_types: &HashSet<String>) -> WrapperReturn<'a> {
    let ret = &cmd.return_type;

    if ret.base.is_empty() || ret.base == "void" {
        return WrapperReturn::Unit;
    }
    if ret.base != "VkResult" {
        return WrapperReturn::Raw(ret);
    }

    // Enumerate pattern
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

    // Single out-handle: pointer_depth==1, not const, no array, Optional::False,
    // is a genuine handle type, and is the last parameter.
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

// VkResult cfg map + handle type set

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

// VkResult check helpers
pub fn vk_result_check(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    result_check_arms(&cmd.success_codes, &cmd.error_codes, cfg_map)
}

pub fn vk_result_is_err(cmd: &Command, cfg_map: &HashMap<String, TokenStream>) -> TokenStream {
    if cmd.error_codes.is_empty() {
        quote! { r < VkResult::VK_SUCCESS }
    } else {
        let arms: Vec<TokenStream> = cmd
            .error_codes
            .iter()
            .map(|s| {
                let id = format_ident!("{}", s);
                let cfg = cfg_map.get(s).cloned().unwrap_or_default();
                quote! { #cfg VkResult::#id }
            })
            .collect();
        quote! {
            matches!(r, #(#arms)|*) || r < VkResult::VK_SUCCESS
        }
    }
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
        let key = cfg.to_string();
        by_cfg
            .entry(key)
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
pub type Groups = BTreeMap<String, Vec<(String, Vec<String>, Command)>>;

pub fn collect_groups(
    reg: &Registry,
    tier: Tier,
    skip: &HashSet<&str>,
    enabled: &HashSet<String>,
) -> Groups {
    let mut groups: Groups = BTreeMap::new();

    for (name, variants) in &reg.commands {
        if skip.contains(name.as_str()) {
            continue;
        }
        let matches = match tier {
            Tier::Instance => variants.iter().any(is_instance_cmd),
            Tier::Device => !variants.iter().any(is_instance_cmd),
        };
        if !matches {
            continue;
        }
        let cmd_raw = variants.last().unwrap();
        if !cmd_raw.api.vulkan && !cmd_raw.api.vulkanbase {
            continue;
        }
        let cmd = resolve_alias(cmd_raw, reg);
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
pub fn first_param_tokens(cmd: &Command) -> (TokenStream, TokenStream) {
    match cmd.params.first() {
        Some(m) => {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = ctype_to_tokens(&m.ty);
            (quote! { #n: #t }, quote! { #n })
        }
        None => (quote! { _handle: *mut c_void }, quote! { _handle }),
    }
}

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

pub fn strip_allocator(params: &[Member]) -> (Vec<Member>, Option<usize>) {
    match params
        .iter()
        .position(|m| m.ty.base == "VkAllocationCallbacks")
    {
        Some(i) => {
            let mut out = params.to_vec();
            out.remove(i);
            (out, Some(i))
        }
        None => (params.to_vec(), None),
    }
}

pub fn build_fwd_args(
    clean_params: &[Member],
    alloc_idx: Option<usize>,
    alloc_expr: &TokenStream,
) -> Vec<TokenStream> {
    let fwd: Vec<TokenStream> = clean_params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            quote! { #n }
        })
        .collect();
    match alloc_idx {
        None => fwd,
        Some(idx) => {
            let mut result = Vec::with_capacity(fwd.len() + 1);
            result.extend_from_slice(&fwd[..idx]);
            result.push(alloc_expr.clone());
            result.extend_from_slice(&fwd[idx..]);
            result
        }
    }
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

// Raw dispatch method (shared by Instance and Device table generators)

pub fn raw_dispatch_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    core_fn: bool,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", name);
    let miss = format!("command not loaded: {}", name);
    let (handle_def, handle_fwd) = first_param_tokens(cmd);
    let rest = cmd.params.get(1..).unwrap_or(&[]);
    let (p_defs, p_fwd) = params_to_tokens(rest);
    let ret = ctype_to_tokens(&cmd.return_type);
    let fp = if core_fn {
        quote! { self.#fname.unwrap_unchecked() }
    } else {
        quote! { self.#fname.expect(#miss) }
    };
    quote! {
        #cfg
        #[inline(always)]
        pub unsafe fn #fname(&self, #handle_def, #(#p_defs),*) -> #ret {
            unsafe { (#fp)(#handle_fwd, #(#p_fwd),*) }
        }
    }
}

// Safe method body (shared by Instance, Device, CommandBuffer)

pub fn safe_method_body(
    cmd: &Command,
    name: &str,
    providers: &[String],
    tier: Tier,
    self_handle: TokenStream,
    table_expr: TokenStream,
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

    let first_param = cmd.params.first();
    let matches_tier = first_param.is_some_and(|p| match tier {
        Tier::Instance => p.ty.base == "VkInstance",
        Tier::Device => p.ty.base == "VkDevice",
    });

    let (params_for_sig, _alloc_idx_for_sig) = if matches_tier {
        strip_allocator(cmd.params.get(1..).unwrap_or(&[]))
    } else {
        strip_allocator(cmd.params.as_slice())
    };

    let (clean_params_full, alloc_idx_full) = strip_allocator(cmd.params.as_slice());

    let has_alloc = _alloc_idx_for_sig.is_some();
    let alloc_param = if has_alloc {
        quote! { allocator: Option<&VkAllocationCallbacks>, }
    } else {
        quote! {}
    };
    let alloc_expr = quote! { allocator.map_or(core::ptr::null(), |a| a as *const _) };

    let mut fwd_args = build_fwd_args(&clean_params_full, alloc_idx_full, &alloc_expr);
    if matches_tier {
        fwd_args[0] = quote! { #self_handle };
    }

    match classify_return(cmd, handle_types) {
        WrapperReturn::Unit => {
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(&self, #alloc_param #(#p_defs),*) {
                    unsafe { (#fp)(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens(ret_ty);
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(&self, #alloc_param #(#p_defs),*) -> #ret {
                    unsafe { (#fp)(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let inner = strip_out_param(&params_for_sig);
            let (p_defs, _) = params_to_tokens(&inner);
            let check = vk_result_check(cmd, result_cfgs);

            let last_idx = fwd_args.len().saturating_sub(1);
            if !fwd_args.is_empty() {
                fwd_args[last_idx] = quote! { &mut handle };
            }

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<#h_ty, VkResult> {
                    let mut handle = #h_ty::NULL;
                    let r = unsafe { (#fp)(#(#fwd_args),*) };
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

            let keep_params: Vec<Member> = params_for_sig
                .iter()
                .enumerate()
                .filter(|(i, _)| {
                    let actual_idx = if matches_tier { *i + 1 } else { *i };
                    actual_idx != ci && actual_idx != ai
                })
                .map(|(_, m)| m.clone())
                .collect();

            let (p_defs, _) = params_to_tokens(&keep_params);
            let is_err = vk_result_is_err(cmd, result_cfgs);
            let check2 = vk_result_check(cmd, result_cfgs);

            let mut fwd_first = fwd_args.clone();
            fwd_first[ci] = quote! { &mut count };
            fwd_first[ai] = quote! { core::ptr::null_mut() };

            let mut fwd_second = fwd_args.clone();
            fwd_second[ci] = quote! { &mut count };
            fwd_second[ai] = quote! { out.as_mut_ptr() };

            quote! {
                #cfg
                #[inline]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<Box<[#elem_ty]>, VkResult> {
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
            let (p_defs, _) = params_to_tokens(&params_for_sig);
            let check = vk_result_check(cmd, result_cfgs);
            quote! {
                #cfg
                #[inline(always)]
                pub unsafe fn #fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<VkResult, VkResult> {
                    let r = unsafe { (#fp)(#(#fwd_args),*) };
                    #check
                }
            }
        }
    }
}

// Entry-specific safe method helper

/// Generate a safe wrapper for an entry-table command.
///
/// Unlike instance/device wrappers there is no implicit dispatchable handle
/// param[0] to strip - all params are user-visible except the allocator.
/// The method is `pub fn` (not `pub unsafe fn`) because all unsafe pointer
/// work is contained inside the body.
pub fn entry_safe_method(
    cmd: &Command,
    name: &str,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let cfg = if providers.is_empty() {
        quote! {}
    } else {
        cfg_any(providers)
    };
    let raw_fname = format_ident!("{}", name);
    let safe_fname = format_ident!("{}_safe", name);
    let miss = format!("entry point not loaded: {}", name);

    let (clean_params, alloc_idx) = strip_allocator(cmd.params.as_slice());
    let has_alloc = alloc_idx.is_some();
    let alloc_param = if has_alloc {
        quote! { allocator: Option<&VkAllocationCallbacks>, }
    } else {
        quote! {}
    };
    let alloc_expr = quote! { allocator.map_or(core::ptr::null(), |a| a as *const _) };
    let fwd_args = build_fwd_args(&clean_params, alloc_idx, &alloc_expr);

    match classify_return(cmd, handle_types) {
        WrapperReturn::Unit => {
            let (p_defs, _) = params_to_tokens(&clean_params);
            quote! {
                #cfg
                #[inline(always)]
                pub fn #safe_fname(&self, #alloc_param #(#p_defs),*) {
                    unsafe { (self.#raw_fname.expect(#miss))(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens(ret_ty);
            let (p_defs, _) = params_to_tokens(&clean_params);
            quote! {
                #cfg
                #[inline(always)]
                pub fn #safe_fname(&self, #alloc_param #(#p_defs),*) -> #ret {
                    unsafe { (self.#raw_fname.expect(#miss))(#(#fwd_args),*) }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let inner = strip_out_param(&clean_params);
            let (p_defs, _) = params_to_tokens(&inner);
            let check = vk_result_check(cmd, result_cfgs);

            let mut fa = fwd_args.clone();
            let last = fa.len().saturating_sub(1);
            if !fa.is_empty() {
                fa[last] = quote! { &mut handle };
            }

            quote! {
                #cfg
                #[inline]
                pub fn #safe_fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<#h_ty, VkResult> {
                    let mut handle = #h_ty::NULL;
                    let r = unsafe { (self.#raw_fname.expect(#miss))(#(#fa),*) };
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

            let keep_params: Vec<Member> = clean_params
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != ci && *i != ai)
                .map(|(_, m)| m.clone())
                .collect();
            let (p_defs, _) = params_to_tokens(&keep_params);
            let is_err = vk_result_is_err(cmd, result_cfgs);
            let check2 = vk_result_check(cmd, result_cfgs);

            let mut fwd_first = fwd_args.clone();
            fwd_first[ci] = quote! { &mut count };
            fwd_first[ai] = quote! { core::ptr::null_mut() };

            let mut fwd_second = fwd_args.clone();
            fwd_second[ci] = quote! { &mut count };
            fwd_second[ai] = quote! { out.as_mut_ptr() };

            quote! {
                #cfg
                #[inline]
                pub fn #safe_fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<Box<[#elem_ty]>, VkResult> {
                    let mut count: u32 = 0;
                    let r = unsafe { (self.#raw_fname.expect(#miss))(#(#fwd_first),*) };
                    if #is_err { return Err(r); }
                    if count == 0 { return Ok(Box::default()); }

                    let mut out = Box::<[#elem_ty]>::new_uninit_slice(count as usize);
                    let r = unsafe { (self.#raw_fname.expect(#miss))(#(#fwd_second),*) };
                    #check2 .map(|_| unsafe { out.assume_init() })
                }
            }
        }

        WrapperReturn::ResultRaw => {
            let (p_defs, _) = params_to_tokens(&clean_params);
            let check = vk_result_check(cmd, result_cfgs);
            quote! {
                #cfg
                #[inline(always)]
                pub fn #safe_fname(
                    &self,
                    #alloc_param
                    #(#p_defs),*
                ) -> Result<VkResult, VkResult> {
                    let r = unsafe { (self.#raw_fname.expect(#miss))(#(#fwd_args),*) };
                    #check
                }
            }
        }
    }
}
