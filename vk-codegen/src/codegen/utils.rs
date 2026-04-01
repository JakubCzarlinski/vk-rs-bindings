use crate::cfggen::cfg_any;
use crate::codegen::{deprecate_attr, refpage_url};
use crate::ir::{CType, Command, Member, Optional, Registry, TypedefKind};
use crate::types::c_type_to_rust;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Tier {
    Entry,
    Instance,
    PhysicalDevice,
    Device,
    Handle(String),
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
const INSTANCE_PINNED: &[&str] = &["vkGetDeviceProcAddr"];

pub fn cmd_tier(cmd: &Command, name: &str, reg: &Registry) -> Tier {
    // ENTRY_CMDS should be classified as Entry
    const ENTRY_CMDS: &[&str] = &[
        "vkCreateInstance",
        "vkEnumerateInstanceExtensionProperties",
        "vkEnumerateInstanceLayerProperties",
        "vkEnumerateInstanceVersion",
    ];
    if ENTRY_CMDS.contains(&name) {
        return Tier::Entry;
    }
    if INSTANCE_PINNED.contains(&name) {
        return Tier::Instance;
    }

    // CommandPool special casing
    if name == "vkAllocateCommandBuffers"
        || name.starts_with("vkResetCommandPool")
        || name.starts_with("vkTrimCommandPool")
        || name.starts_with("vkFreeCommandBuffers")
    {
        return Tier::Handle("VkCommandPool".to_string());
    }

    if name == "vkAllocateDescriptorSets" || name == "vkFreeDescriptorSets" {
        return Tier::Handle("VkDescriptorPool".to_string());
    }

    if name.starts_with("vkCreate") && name.ends_with("Pipelines") {
        return Tier::Device;
    }

    if let Some(m0) = cmd.params.first() {
        let m0_ty = m0.ty.base.as_str();
        if m0_ty == "VkInstance" {
            Tier::Instance
        } else if m0_ty == "VkPhysicalDevice" {
            Tier::PhysicalDevice
        } else if m0_ty == "VkDevice" {
            if let Some(m1) = cmd.params.get(1) {
                let m1_ty = m1.ty.base.as_str();
                if let Some(tds) = reg.typedefs.get(m1_ty) {
                    for td in tds {
                        if let TypedefKind::Handle { .. } = &td.kind {
                            return Tier::Handle(m1_ty.to_string());
                        }
                    }
                }
            }
            Tier::Device
        } else {
            if let Some(tds) = reg.typedefs.get(m0_ty) {
                for td in tds {
                    if let TypedefKind::Handle { .. } = &td.kind {
                        return Tier::Handle(m0_ty.to_string());
                    }
                }
            }
            Tier::Entry
        }
    } else {
        Tier::Entry
    }
}

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
                && m.len.is_none()
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
    let err_arms = cfg_grouped_arms(&cmd.error_codes, cfg_map, quote! { true });
    quote! {
        match r {
            #(#err_arms)*
            _ => r < VkResult::VK_SUCCESS,
        }
    }
}

pub fn result_check_arms(
    success_codes: &[String],
    error_codes: &[String],
    cfg_map: &HashMap<String, TokenStream>,
) -> TokenStream {
    let ok_arms = cfg_grouped_arms(success_codes, cfg_map, quote! { Ok(r) });
    let err_arms = cfg_grouped_arms(error_codes, cfg_map, quote! { Err(r) });
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
    result_expr: TokenStream,
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
            quote! { #cfg #(#pats)|* => #result_expr, }
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
    let mut groups: Groups = BTreeMap::new();

    for (name, variants) in &reg.commands {
        if skip.contains(name.as_str()) {
            continue;
        }

        let cmd_raw = variants.last().unwrap();
        let owned_resolved = if name != "vkCreateDevice" && name != "vkGetDeviceProcAddr" {
            Some(resolve_alias(cmd_raw, reg))
        } else {
            None
        };
        let resolved = owned_resolved.as_ref().unwrap_or(cmd_raw);

        let t = cmd_tier(resolved, name, reg);
        if t != tier {
            continue;
        }

        if !variants.iter().any(|c| c.api.vulkan || c.api.vulkanbase) {
            continue;
        }
        let cmd = if name == "vkCreateDevice" || name == "vkGetDeviceProcAddr" {
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
    handle_meta: Option<&BTreeMap<String, crate::codegen::handles_rs::HandleMeta>>,
    device_accessor: TokenStream,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let fname = format_ident!("{}", name);
    let safety_comment = " SAFETY: table is fully loaded at creation.";
    let fp = quote! {
        (#table_expr).#fname.unwrap_unchecked()
    };

    // TODO(czarlinski): decide what is better - maybe we generate a debug version and release version.
    // let miss = format!("command not loaded: {}", name);
    // let core_fn = is_core(providers);
    // let fp = if core_fn {
    //     quote! { (#table_expr).#fname.unwrap_unchecked() }
    // } else {
    //     quote! { (#table_expr).#fname.expect(#miss) }
    // };

    // Strip param[0] when it matches the wrapper's handle type.
    // Or strip param[0] and param[1] when param[1] matches.
    let mut strip_count = 0;
    if !handle_base.is_empty()
        && let Some(p0) = cmd.params.first()
    {
        if p0.ty.base == handle_base {
            strip_count = 1;
        } else if let Some(p1) = cmd.params.get(1)
            && p1.ty.base == handle_base
        {
            strip_count = 2;
        }
    }

    let sig_params: &[Member] = &cmd.params[strip_count..];

    // Build the forwarding argument list.
    let mut fwd: Vec<TokenStream> = cmd
        .params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            quote! { #n }
        })
        .collect();

    if strip_count == 1 {
        fwd[0] = quote! { #self_handle };
    } else if strip_count == 2 {
        // In the tier resolution, this usually implies param[0] is the device/parent.
        // If we are in a handle, device_accessor represents the parent/device tree.
        // Actually, if param[0] is exactly the parent type, we should use self.parent().raw().
        // As a strong convention: if strip_count == 2, param[0] is the creator/parent.
        let p0_ty = cmd.params[0].ty.base.as_str();
        if p0_ty == "VkDevice" {
            fwd[0] = quote! { self.device().raw() };
        } else {
            fwd[0] = quote! { self.parent().raw() };
        }
        fwd[1] = quote! { #self_handle };
    }

    let (p_defs, _) = params_to_tokens(sig_params);

    let depr = deprecate_attr(&cmd.depr);
    let doc = create_doc(cmd, providers);
    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    let func = match classify_return(cmd, handle_types) {
        WrapperReturn::Unit => quote! {
            #cfg #depr
            #[inline(always)]
            pub fn #fname(&self, #(#p_defs),*) {
                unsafe {
                  #[comment = #safety_comment]
                  (#fp)(#(#fwd),*)
                }
            }
        },

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens(ret_ty);
            quote! {
                #cfg #depr
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> #ret {
                    unsafe {
                      #[comment = #safety_comment]
                      (#fp)(#(#fwd),*)
                    }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let h_ty_str = h_ty.to_string();
            let inner = strip_out_param(sig_params);
            let (p_defs, _) = params_to_tokens(&inner);
            let check = vk_result_check(cmd, result_cfgs);
            let last = fwd.len().saturating_sub(1);
            if !fwd.is_empty() {
                fwd[last] = quote! { &mut handle };
            }

            let is_handle = handle_meta.is_some_and(|m| m.contains_key(&h_ty_str));
            if is_handle {
                let meta = handle_meta.unwrap().get(&h_ty_str).unwrap();
                let md = format_ident!("{}", meta.mod_name);
                let st = format_ident!("{}", meta.struct_name);
                let tf = format_ident!("{}", meta.table_field);

                let parent_expr = if meta.parent_vk_name == handle_base {
                    quote! { self }
                } else if meta.parent_vk_name == "VkDevice" && handle_base != "VkDevice" {
                    quote! { #device_accessor }
                } else {
                    quote! { self }
                };

                quote! {
                    #cfg #depr
                    #[inline]
                    pub fn #fname<'ret>(&'ret self, #(#p_defs),*) -> Result<crate::#md::#st<'ret>, VkResult> {
                        let mut handle = #h_ty::NULL;
                        let r = unsafe { (#fp)(#(#fwd),*) };
                        #check .map(|_| crate::#md::#st {
                            raw: handle,
                            parent: #parent_expr,
                            table: &#device_accessor.#tf
                        })
                    }
                }
            } else {
                quote! {
                    #cfg #depr
                    #[inline]
                    pub fn #fname(&self, #(#p_defs),*) -> Result<#h_ty, VkResult> {
                        let mut handle = #h_ty::NULL;
                        let r = unsafe { (#fp)(#(#fwd),*) };
                        #check .map(|_| handle)
                    }
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
                    let full = i + strip_count;
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
                #cfg #depr
                #[inline]
                pub fn #fname(&self, #(#p_defs),*) -> Result<alloc::vec::Vec<#elem_ty>, VkResult> {
                    let mut count: u32 = 0;
                    let r = unsafe { (#fp)(#(#fwd_first),*) };
                    if #is_err { return Err(r); }
                    if count == 0 { return Ok(alloc::vec::Vec::new()); }
                    let mut out = alloc::vec::Vec::with_capacity(count as usize);
                    let r = unsafe { (#fp)(#(#fwd_second),*) };
                    #check2 .map(|_| {
                        unsafe { out.set_len(count as usize); }
                        out
                    })
                }
            }
        }

        WrapperReturn::ResultRaw => {
            let check = vk_result_check(cmd, result_cfgs);
            quote! {
                #cfg #depr
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> Result<VkResult, VkResult> {
                    let r = unsafe { (#fp)(#(#fwd),*) };
                    #check
                }
            }
        }
    };
    token_stream.extend(func);
    token_stream
}

pub(crate) fn create_doc(cmd: &Command, all_features: &[String]) -> String {
    let url = refpage_url(&cmd.name);
    let provided: String = all_features
        .iter()
        .map(|f| format!(" - `{f}`"))
        .collect::<Vec<_>>()
        .join("\n");
    let comment = cmd.comment.as_deref().unwrap_or("");
    let mut doc = format!(
        " [`{n}`]({url})\n\n Provided by:\n{provided}\n{comment}",
        n = cmd.name
    );

    if !cmd.queues.is_empty() {
        let q_names: Vec<_> = cmd.queues.iter().map(|q| format!("{:?}", q)).collect();
        doc.push_str(&format!("\n - **Queues:** {}", q_names.join(", ")));
    }
    if let Some(ref rp) = cmd.render_pass {
        doc.push_str(&format!("\n - **Render Pass:** {:?}", rp));
    }
    if cmd.conditional_rendering {
        doc.push_str("\n - **Conditional Rendering:** Affected");
    }
    if cmd.allow_no_queues {
        doc.push_str("\n - **Allow No Queues:** True");
    }
    if !cmd.tasks.is_empty() {
        let t_names: Vec<_> = cmd.tasks.iter().map(|t| format!("{:?}", t)).collect();
        doc.push_str(&format!("\n - **Tasks:** {}", t_names.join(", ")));
    }
    if let Some(ref dep) = cmd.dep {
        doc.push_str(&format!(
            "\n - **Availability:** depends on `{}`",
            dep.atoms().join(" + ")
        ));
    }
    if !cmd.cmd_buffer_levels.is_empty() {
        let l_names: Vec<_> = cmd
            .cmd_buffer_levels
            .iter()
            .map(|l| format!("{:?}", l))
            .collect();
        doc.push_str(&format!(
            "\n - **Command Buffer Levels:** {}",
            l_names.join(", ")
        ));
    }
    if let Some(ref es) = cmd.extern_sync {
        doc.push_str(&format!("\n - **External Sync:** {}", es));
    }
    if !cmd.export.is_empty() {
        let e_names: Vec<_> = cmd.export.iter().map(|e| format!("{:?}", e)).collect();
        doc.push_str(&format!("\n - **Export Scopes:** {}", e_names.join(", ")));
    }

    if !cmd.params.is_empty() {
        doc.push_str("\n\n # Parameters");

        for p in &cmd.params {
            let mut line = format!(" - `{}`", p.name);

            let mut p_meta = Vec::new();
            if p.optional != crate::ir::Optional::False {
                p_meta.push(format!("optional: {:?}", p.optional));
            }
            if let Some(ref len) = p.len {
                p_meta.push(format!("len: {}", len));
            }
            if let Some(ref vals) = p.values {
                p_meta.push(format!("values: {}", vals));
            }
            if let Some(ref ot) = p.object_type {
                p_meta.push(format!("object_type: {}", ot));
            }

            if !p_meta.is_empty() {
                line.push_str(&format!(": {}", p_meta.join(", ")));
            }

            doc.push_str(&format!("\n{}", line));
        }
    }

    if !cmd.success_codes.is_empty() || !cmd.error_codes.is_empty() {
        doc.push_str("\n\n # Returns");
    }

    if !cmd.success_codes.is_empty() {
        doc.push_str(&format!(
            "\n\n **Success Codes:**\n   - {}",
            cmd.success_codes.join("\n   - ")
        ));
    }
    if !cmd.error_codes.is_empty() {
        doc.push_str(&format!(
            "\n\n **Error Codes:**\n   - {}",
            cmd.error_codes.join("\n   - ")
        ));
    }

    doc.push('\n');

    doc
}
