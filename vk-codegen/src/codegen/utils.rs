use crate::cfggen::cfg_availability;
use crate::codegen::handles_rs::HandleMeta;
use crate::codegen::{deprecate_attr, refpage_url};
use crate::ir::{Availability, CType, Command, Member, Optional, Registry, TypedefKind};
use crate::types::c_type_to_rust;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Tier {
    Entry,
    Instance,
    PhysicalDevice,
    Device,
    Handle(String),
}

#[derive(Clone, Debug, Default)]
pub struct ExplicitImports {
    commands: BTreeSet<String>,
    consts: BTreeSet<String>,
    enums: BTreeSet<String>,
    types: BTreeSet<String>,
}

impl ExplicitImports {
    pub fn add_command_pfn(&mut self, name: &str) {
        self.commands.insert(format!("PFN_{name}"));
    }

    pub fn add_command_signature(&mut self, reg: &Registry, cmd: &Command, name: &str) {
        self.add_command_pfn(name);
        self.add_ctype(reg, &cmd.return_type);
        for param in &cmd.params {
            self.add_ctype(reg, &param.ty);
        }
    }

    pub fn add_ctype(&mut self, reg: &Registry, ty: &CType) {
        self.add_type_name(reg, &ty.base);
        if let Some(array_size) = &ty.is_array
            && array_size.parse::<u64>().is_err()
        {
            self.add_const_name(reg, array_size);
        }
    }

    pub fn add_const_name(&mut self, reg: &Registry, name: &str) {
        if reg.constants.contains_key(name) {
            self.consts.insert(name.to_owned());
        }
    }

    pub fn add_enum_name(&mut self, reg: &Registry, name: &str) {
        if reg.enums.contains_key(name) {
            self.enums.insert(name.to_owned());
        }
    }

    pub fn add_all_enum_names(&mut self, reg: &Registry) {
        self.enums.extend(reg.enums.keys().cloned());
    }

    pub fn add_all_type_names(&mut self, reg: &Registry) {
        self.types.extend(reg.structs.keys().cloned());
        self.types.extend(reg.typedefs.keys().cloned());
    }

    pub fn add_ctype_external_to_types_rs(&mut self, reg: &Registry, ty: &CType) {
        self.add_enum_name(reg, &ty.base);
        if let Some(array_size) = &ty.is_array
            && array_size.parse::<u64>().is_err()
        {
            self.add_const_name(reg, array_size);
        }
    }

    pub fn add_type_name(&mut self, reg: &Registry, name: &str) {
        if reg.enums.contains_key(name) {
            self.enums.insert(name.to_owned());
        } else if reg.structs.contains_key(name) || reg.typedefs.contains_key(name) {
            self.types.insert(name.to_owned());
        }
    }

    pub fn add_vk_result(&mut self) {
        self.enums.insert("VkResult".to_owned());
    }

    pub fn extend_command_variants(&mut self, reg: &Registry, name: &str) {
        let Some(variants) = reg.commands.get(name) else {
            return;
        };
        self.add_command_pfn(name);
        if let Some(cmd) = variants.last() {
            self.add_ctype(reg, &cmd.return_type);
            for param in &cmd.params {
                self.add_ctype(reg, &param.ty);
            }
        }
    }

    pub fn to_tokens(&self, reg: &Registry) -> TokenStream {
        let mut ts = TokenStream::new();

        for pfn in &self.commands {
            let Some(command_name) = pfn.strip_prefix("PFN_") else {
                continue;
            };
            let cfg = command_import_cfg(reg, command_name);
            let ident = format_ident!("{}", pfn);
            ts.extend(quote! { #cfg use crate::commands::#ident; });
        }

        for name in &self.consts {
            let Some(cfg) = const_import_cfg(reg, name) else {
                continue;
            };
            let ident = format_ident!("{}", name);
            ts.extend(quote! { #cfg use crate::consts::#ident; });
        }

        for name in &self.enums {
            let Some(cfg) = enum_import_cfg(reg, name) else {
                continue;
            };
            let ident = format_ident!("{}", name);
            ts.extend(quote! { #cfg use crate::enums::#ident; });
        }

        for name in &self.types {
            let Some(cfg) = type_import_cfg(reg, name) else {
                continue;
            };
            let ident = format_ident!("{}", name);
            ts.extend(quote! { #cfg use crate::types::#ident; });
        }

        ts
    }
}

fn command_import_cfg(reg: &Registry, name: &str) -> TokenStream {
    let Some(variants) = reg.commands.get(name) else {
        return quote! {};
    };
    let mut providers: Vec<String> = variants
        .iter()
        .flat_map(|cmd| cmd.provided_by.clone())
        .collect();
    providers.sort();
    providers.dedup();
    let availability: Vec<_> = variants
        .iter()
        .flat_map(|cmd| cmd.availability.clone())
        .collect();
    if providers.is_empty() {
        quote! {}
    } else {
        cfg_availability(
            &availability,
            &providers,
            variants.iter().find_map(|cmd| cmd.dep.as_ref()),
        )
    }
}

fn enum_import_cfg(reg: &Registry, name: &str) -> Option<TokenStream> {
    let variants = reg.enums.get(name)?;
    let enum_def = variants.first()?;
    let mut providers = Vec::<String>::new();
    let mut availability = Vec::<Availability>::new();
    for enum_def in variants {
        providers.extend(enum_def.provided_by.clone());
        availability.extend(enum_def.availability.clone());
        for variant in &enum_def.variants {
            providers.extend(variant.provided_by.clone());
            availability.extend(variant.availability.clone());
        }
    }
    providers.sort();
    providers.dedup();
    Some(cfg_availability(
        &availability,
        &providers,
        enum_def.dep.as_ref(),
    ))
}

fn const_import_cfg(reg: &Registry, name: &str) -> Option<TokenStream> {
    let variants = reg.constants.get(name)?;
    let constant = variants.first()?;
    let mut providers = Vec::<String>::new();
    let mut availability = Vec::<Availability>::new();
    for constant in variants {
        providers.extend(constant.provided_by.clone());
        availability.extend(constant.availability.clone());
    }
    providers.sort();
    providers.dedup();
    Some(cfg_availability(
        &availability,
        &providers,
        constant.dep.as_ref(),
    ))
}

fn type_import_cfg(reg: &Registry, name: &str) -> Option<TokenStream> {
    if let Some(structs) = reg.structs.get(name) {
        let s = structs.first()?;
        if s.provided_by.is_empty() {
            return None;
        }
        return Some(cfg_availability(
            &s.availability,
            &s.provided_by,
            s.dep.as_ref(),
        ));
    }
    if let Some(typedefs) = reg.typedefs.get(name) {
        let td = typedefs.first()?;
        if td.provided_by.is_empty() || matches!(td.kind, TypedefKind::Define) {
            return None;
        }
        return Some(cfg_availability(
            &td.availability,
            &td.provided_by,
            td.dep.as_ref(),
        ));
    }
    None
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

#[must_use]
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

#[must_use]
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

#[must_use]
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

#[must_use]
pub fn vk_result_return_if_err() -> TokenStream {
    quote! {
        if r < VkResult::VK_SUCCESS {
            core::hint::cold_path();
            return Err(r);
        }
    }
}

#[must_use]
pub fn result_check_arms() -> TokenStream {
    quote! {
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            core::hint::cold_path();
            Err(r)
        }
    }
}

// Command grouping

/// Feature-keyed map of `(name, providers, resolved_command)`.
/// Commands within each bucket are sorted alphabetically; both the dispatch
/// table and the safe wrapper iterate this in the same order.
pub type Groups = BTreeMap<String, Vec<(String, Vec<String>, Command)>>;

#[must_use]
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

        if !variants.iter().any(|c| c.api.vulkan || c.api.vulkanbase) {
            continue;
        }
        let cmd_raw = variants.last().unwrap();
        let mut cmd = if name == "vkCreateDevice" || name == "vkGetDeviceProcAddr" {
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
        let dep = variants.iter().find_map(|c| c.dep.clone());
        let availability = variants
            .iter()
            .flat_map(|c| c.availability.clone())
            .collect();
        cmd.provided_by = providers.clone();
        cmd.dep = dep;
        cmd.availability = availability;
        if cmd_raw.alias.is_some() {
            rewrite_command_types_for_providers(&mut cmd, reg, &providers);
        }
        let t = cmd_tier(&cmd, name, reg);
        if t != tier {
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

pub fn rewrite_command_types_for_providers(
    cmd: &mut Command,
    reg: &Registry,
    providers: &[String],
) {
    rewrite_ctype_for_item(&mut cmd.return_type, reg, providers, &cmd.availability);
    for param in &mut cmd.params {
        rewrite_ctype_for_item(&mut param.ty, reg, providers, &cmd.availability);
    }
}

pub fn rewrite_member_types_for_providers(
    members: &mut [Member],
    reg: &Registry,
    providers: &[String],
    availability: &[Availability],
) {
    for member in members {
        rewrite_ctype_for_item(&mut member.ty, reg, providers, availability);
    }
}

pub fn alias_type_for_providers(
    reg: &Registry,
    target: &str,
    providers: &[String],
    availability: &[Availability],
) -> Option<String> {
    let aliases = reg
        .typedefs
        .values()
        .flatten()
        .filter_map(|item| {
            item.alias
                .as_ref()
                .map(|alias| (alias, &item.name, &item.provided_by, &item.availability))
        })
        .chain(reg.structs.values().flatten().filter_map(|item| {
            item.alias
                .as_ref()
                .map(|alias| (alias, &item.name, &item.provided_by, &item.availability))
        }))
        .chain(reg.enums.values().flatten().filter_map(|item| {
            item.alias
                .as_ref()
                .map(|alias| (alias, &item.name, &item.provided_by, &item.availability))
        }));

    aliases
        .filter(|(alias, name, _, _)| alias.as_str() == target && name.as_str() != target)
        .filter(|(_, _, provided_by, alias_availability)| {
            alias_is_available_for_item(provided_by, alias_availability, providers, availability)
        })
        .map(|(_, name, _, _)| name.clone())
        .next()
}

fn rewrite_ctype_for_item(
    ty: &mut CType,
    reg: &Registry,
    providers: &[String],
    availability: &[Availability],
) {
    if let Some(alias) = alias_type_for_providers(reg, &ty.base, providers, availability) {
        ty.base = alias;
    }
}

fn availability_clauses(providers: &[String], availability: &[Availability]) -> Vec<Vec<String>> {
    if availability.is_empty() {
        return providers
            .iter()
            .map(|provider| vec![provider.clone()])
            .collect();
    }

    let mut clauses = Vec::new();
    for item in availability {
        let dep_clauses = item
            .dep
            .as_ref()
            .map(|dep| dep.to_dnf_clauses())
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

fn alias_is_available_for_item(
    alias_providers: &[String],
    alias_availability: &[Availability],
    item_providers: &[String],
    item_availability: &[Availability],
) -> bool {
    let item_clauses = availability_clauses(item_providers, item_availability);
    let alias_clauses = availability_clauses(alias_providers, alias_availability);

    !item_clauses.is_empty()
        && !alias_clauses.is_empty()
        && item_clauses.iter().all(|item_clause| {
            alias_clauses.iter().any(|alias_clause| {
                alias_clause
                    .iter()
                    .all(|feature| item_clause.contains(feature))
            })
        })
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
#[must_use]
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
        assert!(i != 9, "alias chain too long resolving {}", cmd.name)
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

#[must_use]
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

#[must_use]
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

#[must_use]
pub fn is_cmd_buf_cmd(cmd: &Command) -> bool {
    cmd.params
        .first()
        .is_some_and(|m| m.ty.base == "VkCommandBuffer")
}

// Token helpers

#[must_use]
pub fn params_to_tokens(params: &[Member]) -> (Vec<TokenStream>, Vec<TokenStream>) {
    params
        .iter()
        .map(|m| {
            let n = format_ident!("{}", kw_escape(&m.name));
            let t = param_sig_type(m);
            (quote! { #n: #t }, quote! { #n })
        })
        .unzip()
}

#[must_use]
pub fn struct_name_has_lifetime(base: &str, reg: &Registry) -> bool {
    struct_name_has_lifetime_inner(base, reg, &mut HashSet::new())
}

fn struct_name_has_lifetime_inner(
    base: &str,
    reg: &Registry,
    visiting: &mut HashSet<String>,
) -> bool {
    let Some(mut s) = reg.structs.get(base).and_then(|items| {
        items
            .iter()
            .find(|s| !s.provided_by.is_empty())
            .or_else(|| items.first())
    }) else {
        return false;
    };
    if let Some(alias) = s.alias.as_deref()
        && let Some(target) = reg.structs.get(alias).and_then(|items| {
            items
                .iter()
                .find(|s| !s.provided_by.is_empty())
                .or_else(|| items.first())
        })
    {
        s = target;
    }
    if !visiting.insert(base.to_owned()) {
        return false;
    }

    s.members.iter().any(|m| {
        m.ty.pointer_depth > 0
            || ((m.ty.pointer_depth == 0 || m.ty.is_array.is_some())
                && struct_name_has_lifetime_inner(&m.ty.base, reg, visiting))
    })
}

#[must_use]
pub fn param_sig_type(m: &Member) -> TokenStream {
    // In safe wrappers, prefer references for required single-pointer
    // params that are not array-like (for example `pCreateInfo: &Vk*CreateInfo`).
    // Keep optional and array/len-linked pointers raw so null and pointer
    // semantics remain explicit in the signature.
    if m.ty.pointer_depth == 1
        && m.optional == Optional::False
        && m.len.is_none()
        && m.ty.base != "void"
    {
        let base = base_type_tokens(&m.ty.base);
        if m.ty.is_const {
            quote! { &#base }
        } else {
            quote! { &mut #base }
        }
    } else {
        ctype_to_tokens(&m.ty)
    }
}

#[must_use]
pub fn param_sig_type_for_registry(m: &Member, reg: &Registry) -> TokenStream {
    if m.ty.pointer_depth == 1
        && m.optional == Optional::False
        && m.len.is_none()
        && m.ty.base != "void"
    {
        let base = base_type_tokens_for_registry(&m.ty.base, reg, quote! { '_ });
        if m.ty.is_const {
            quote! { &#base }
        } else {
            quote! { &mut #base }
        }
    } else {
        ctype_to_tokens_for_registry(&m.ty, reg, quote! { '_ })
    }
}

#[derive(Clone, Copy, Debug)]
struct SliceParamPair {
    count_full_idx: usize,
    ptr_full_idx: usize,
}

#[must_use]
fn collect_required_slice_pairs(cmd: &Command, strip_count: usize) -> Vec<SliceParamPair> {
    let mut pairs = Vec::new();
    for (ptr_idx, ptr) in cmd.params.iter().enumerate().skip(strip_count) {
        if ptr.ty.pointer_depth != 1
            || !matches!(ptr.optional, Optional::False | Optional::FalseTrue)
        {
            continue;
        }
        let Some(len_name) = ptr.len.as_deref() else {
            continue;
        };
        let Some((count_idx, count)) = cmd
            .params
            .iter()
            .enumerate()
            .skip(strip_count)
            .find(|(_, m)| m.name == len_name)
        else {
            continue;
        };
        if count.ty.pointer_depth != 0 || count.ty.is_array.is_some() {
            continue;
        }
        pairs.push(SliceParamPair {
            count_full_idx: count_idx,
            ptr_full_idx: ptr_idx,
        });
    }
    pairs
}

#[must_use]
fn params_to_tokens_with_required_slices(
    cmd: &Command,
    sig_params: &[Member],
    strip_count: usize,
    reg: &Registry,
) -> Vec<TokenStream> {
    let pairs = collect_required_slice_pairs(cmd, strip_count);
    let mut defs = Vec::new();
    for (i, m) in sig_params.iter().enumerate() {
        let full_idx = i + strip_count;
        if pairs.iter().any(|p| p.count_full_idx == full_idx) {
            continue;
        }

        let n = format_ident!("{}", kw_escape(&m.name));
        if pairs.iter().any(|p| p.ptr_full_idx == full_idx) {
            let elem = if m.ty.base == "void" {
                quote! { u8 }
            } else {
                base_type_tokens_for_registry(&m.ty.base, reg, quote! { '_ })
            };
            let t = if m.ty.is_const {
                quote! { &[#elem] }
            } else {
                quote! { &mut [#elem] }
            };
            defs.push(quote! { #n: #t });
        } else {
            let t = param_sig_type_for_registry(m, reg);
            defs.push(quote! { #n: #t });
        }
    }
    defs
}

#[must_use]
pub fn strip_first_param(params: &[Member]) -> &[Member] {
    params.get(1..).unwrap_or(&[])
}

#[must_use]
pub fn strip_out_param(params: &[Member]) -> Vec<Member> {
    let mut out = params.to_vec();
    if out
        .last()
        .is_some_and(|m| m.ty.pointer_depth == 1 && !m.ty.is_const && m.ty.base.starts_with("Vk"))
    {
        out.pop();
    }
    out
}

#[must_use]
pub fn deref_ctype(ty: &CType) -> TokenStream {
    base_type_tokens(&ty.base)
}

#[must_use]
pub fn ctype_to_tokens(ty: &CType) -> TokenStream {
    if (ty.base.is_empty() || ty.base == "void") && ty.pointer_depth == 0 && ty.is_array.is_none() {
        return quote! { () };
    }
    let base = base_type_tokens(&ty.base);
    let mut ts = if let Some(ref size) = ty.is_array {
        let size_ts: TokenStream = if size.parse::<u64>().is_ok() {
            size.parse().unwrap()
        } else {
            format!("{size} as usize").parse().unwrap()
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

#[must_use]
pub fn ctype_to_tokens_for_registry(
    ty: &CType,
    reg: &Registry,
    lifetime: TokenStream,
) -> TokenStream {
    if (ty.base.is_empty() || ty.base == "void") && ty.pointer_depth == 0 && ty.is_array.is_none() {
        return quote! { () };
    }
    let base = base_type_tokens_for_registry(&ty.base, reg, lifetime);
    let mut ts = if let Some(ref size) = ty.is_array {
        let size_ts: TokenStream = if size.parse::<u64>().is_ok() {
            size.parse().unwrap()
        } else {
            format!("{size} as usize").parse().unwrap()
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

#[must_use]
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

#[must_use]
pub fn base_type_tokens_for_registry(
    base: &str,
    reg: &Registry,
    lifetime: TokenStream,
) -> TokenStream {
    let resolved = c_type_to_rust(base);
    if resolved.is_empty() && struct_name_has_lifetime(base, reg) {
        let ident = format_ident!("{}", base);
        quote! { #ident <#lifetime> }
    } else {
        base_type_tokens(base)
    }
}

#[must_use]
pub fn c_str_lit(name: &str) -> TokenStream {
    format!("c\"{name}\"").parse().unwrap()
}

#[must_use]
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
#[must_use]
pub fn safe_method(
    reg: &Registry,
    cmd: &Command,
    name: &str,
    providers: &[String],
    handle_base: &str,        // "" = no stripping (Entry)
    self_handle: TokenStream, // value to substitute for param[0]
    table_expr: TokenStream,  // how to reach the PFN table
    handle_types: &HashSet<String>,
    handle_meta: Option<&BTreeMap<String, HandleMeta>>,
    device_accessor: TokenStream,
    instance_accessor: TokenStream,
) -> TokenStream {
    let cfg = cfg_availability(&cmd.availability, providers, cmd.dep.as_ref());
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
    for pair in collect_required_slice_pairs(cmd, strip_count) {
        let ptr_param = &cmd.params[pair.ptr_full_idx];
        let ptr_name = format_ident!("{}", kw_escape(&ptr_param.name));
        let count_ty = ctype_to_tokens(&cmd.params[pair.count_full_idx].ty);
        fwd[pair.count_full_idx] = quote! { #ptr_name.len() as #count_ty };
        fwd[pair.ptr_full_idx] = if ptr_param.ty.is_const {
            if ptr_param.ty.base == "void" {
                quote! { #ptr_name.as_ptr().cast::<core::ffi::c_void>() }
            } else {
                quote! { #ptr_name.as_ptr() }
            }
        } else {
            if ptr_param.ty.base == "void" {
                quote! { #ptr_name.as_mut_ptr().cast::<core::ffi::c_void>() }
            } else {
                quote! { #ptr_name.as_mut_ptr() }
            }
        };
    }

    let p_defs = params_to_tokens_with_required_slices(cmd, sig_params, strip_count, reg);

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
                  #fp(#(#fwd),*)
                }
            }
        },

        WrapperReturn::Raw(ret_ty) => {
            let ret = ctype_to_tokens_for_registry(ret_ty, reg, quote! { '_ });
            quote! {
                #cfg #depr
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> #ret {
                    unsafe {
                      #[comment = #safety_comment]
                      #fp(#(#fwd),*)
                    }
                }
            }
        }

        WrapperReturn::ResultHandle { handle_ty } => {
            let h_ty = deref_ctype(handle_ty);
            let h_ty_str = h_ty.to_string();
            let inner = strip_out_param(sig_params);
            let p_defs: Vec<_> = inner
                .iter()
                .map(|m| {
                    let n = format_ident!("{}", kw_escape(&m.name));
                    let t = param_sig_type_for_registry(m, reg);
                    quote! { #n: #t }
                })
                .collect();
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
                } else if meta.parent_vk_name == "VkInstance" && handle_base != "VkInstance" {
                    quote! { #instance_accessor }
                } else {
                    quote! { self }
                };
                let table_owner = if meta.root_vk_name == "VkDevice" {
                    quote! { #device_accessor }
                } else {
                    quote! { #instance_accessor }
                };

                quote! {
                    #cfg #depr
                    #[inline]
                    pub fn #fname<'ret>(&'ret self, #(#p_defs),*) -> Result<crate::#md::#st<'ret>, VkResult> {
                        let mut handle = #h_ty::NULL;
                        let r = unsafe { #fp(#(#fwd),*) };
                        if r >= VkResult::VK_SUCCESS {
                            Ok(crate::#md::#st {
                                raw: handle,
                                parent: #parent_expr,
                                table: &#table_owner.#tf
                            })
                        } else {
                            core::hint::cold_path();
                            Err(r)
                        }
                    }
                }
            } else {
                quote! {
                    #cfg #depr
                    #[inline]
                    pub fn #fname(&self, #(#p_defs),*) -> Result<#h_ty, VkResult> {
                        let mut handle = #h_ty::NULL;
                        let r = unsafe { #fp(#(#fwd),*) };
                        if r >= VkResult::VK_SUCCESS {
                            Ok(handle)
                        } else {
                            core::hint::cold_path();
                            Err(r)
                        }
                    }
                }
            }
        }

        WrapperReturn::Enumerate {
            item_ty,
            count_idx,
            array_idx,
        } => {
            let elem_ty = ctype_to_tokens_for_registry(item_ty, reg, quote! { '_ });
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
            let p_defs: Vec<_> = keep
                .iter()
                .map(|m| {
                    let n = format_ident!("{}", kw_escape(&m.name));
                    let t = param_sig_type_for_registry(m, reg);
                    quote! { #n: #t }
                })
                .collect();

            let return_if_err = vk_result_return_if_err();

            let mut fwd_first = fwd.clone();
            fwd_first[ci] = quote! { &mut count };
            fwd_first[ai] = quote! { core::ptr::null_mut() };

            let mut fwd_second = fwd.clone();
            fwd_second[ci] = quote! { &mut count };
            fwd_second[ai] = quote! { out.as_mut_ptr().cast() };

            quote! {
                #cfg #depr
                #[inline]
                pub fn #fname(&self, #(#p_defs),*) -> Result<alloc::boxed::Box<[#elem_ty]>, VkResult> {
                    let mut count: u32 = 0;
                    let r = unsafe { #fp(#(#fwd_first),*) };
                    #return_if_err
                    if count == 0 { return Ok(alloc::boxed::Box::<[#elem_ty; 0]>::new([])); }
                    let mut out = alloc::boxed::Box::<[#elem_ty]>::new_uninit_slice(count as usize);
                    let r = unsafe { #fp(#(#fwd_second),*) };
                    if r >= VkResult::VK_SUCCESS {
                        Ok(unsafe { out.assume_init() })
                    } else {
                        core::hint::cold_path();
                        Err(r)
                    }
                }
            }
        }

        WrapperReturn::ResultRaw => {
            let check = result_check_arms();
            quote! {
                #cfg #depr
                #[inline(always)]
                pub fn #fname(&self, #(#p_defs),*) -> Result<VkResult, VkResult> {
                    let r = unsafe { #fp(#(#fwd),*) };
                    #check
                }
            }
        }
    };
    token_stream.extend(func);
    token_stream
}

// Safe method body for commands that must mutate wrapper state after calling
// into Vulkan (for example explicit destroy/free wrappers that null `self.raw`).
//
// For non-unit return commands this falls back to `safe_method`.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn safe_method_unit_with_overrides(
    reg: &Registry,
    cmd: &Command,
    name: &str,
    providers: &[String],
    handle_base: &str,        // "" = no stripping (Entry)
    self_handle: TokenStream, // value to substitute for param[0]
    table_expr: TokenStream,  // how to reach the PFN table
    handle_types: &HashSet<String>,
    handle_meta: Option<&BTreeMap<String, HandleMeta>>,
    device_accessor: TokenStream,
    instance_accessor: TokenStream,
    receiver: TokenStream,
    pre_call: TokenStream,
    post_call: TokenStream,
) -> TokenStream {
    if !matches!(classify_return(cmd, handle_types), WrapperReturn::Unit) {
        return safe_method(
            reg,
            cmd,
            name,
            providers,
            handle_base,
            self_handle,
            table_expr,
            handle_types,
            handle_meta,
            device_accessor,
            instance_accessor,
        );
    }

    let cfg = cfg_availability(&cmd.availability, providers, cmd.dep.as_ref());
    let fname = format_ident!("{}", name);
    let safety_comment = " SAFETY: table is fully loaded at creation.";
    let fp = quote! {
        (#table_expr).#fname.unwrap_unchecked()
    };

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
        let p0_ty = cmd.params[0].ty.base.as_str();
        if p0_ty == "VkDevice" {
            fwd[0] = quote! { self.device().raw() };
        } else {
            fwd[0] = quote! { self.parent().raw() };
        }
        fwd[1] = quote! { #self_handle };
    }
    for pair in collect_required_slice_pairs(cmd, strip_count) {
        let ptr_param = &cmd.params[pair.ptr_full_idx];
        let ptr_name = format_ident!("{}", kw_escape(&ptr_param.name));
        let count_ty = ctype_to_tokens(&cmd.params[pair.count_full_idx].ty);
        fwd[pair.count_full_idx] = quote! { #ptr_name.len() as #count_ty };
        fwd[pair.ptr_full_idx] = if ptr_param.ty.is_const {
            if ptr_param.ty.base == "void" {
                quote! { #ptr_name.as_ptr().cast::<core::ffi::c_void>() }
            } else {
                quote! { #ptr_name.as_ptr() }
            }
        } else {
            if ptr_param.ty.base == "void" {
                quote! { #ptr_name.as_mut_ptr().cast::<core::ffi::c_void>() }
            } else {
                quote! { #ptr_name.as_mut_ptr() }
            }
        };
    }

    let p_defs = params_to_tokens_with_required_slices(cmd, sig_params, strip_count, reg);
    let depr = deprecate_attr(&cmd.depr);
    let doc = create_doc(cmd, providers);
    let mut token_stream = TokenStream::new();
    for doc_lines in doc.lines() {
        token_stream.extend(quote! { #[doc = #doc_lines] });
    }
    token_stream.extend(quote! {
        #cfg #depr
        #[inline(always)]
        pub fn #fname(#receiver, #(#p_defs),*) {
            #pre_call
            unsafe {
              #[comment = #safety_comment]
              #fp(#(#fwd),*)
            }
            #post_call
        }
    });
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
        let q_names: Vec<_> = cmd.queues.iter().map(|q| format!("{q:?}")).collect();
        doc.push_str(&format!("\n - **Queues:** {}", q_names.join(", ")));
    }
    if let Some(ref rp) = cmd.render_pass {
        doc.push_str(&format!("\n - **Render Pass:** {rp:?}"));
    }
    if cmd.conditional_rendering {
        doc.push_str("\n - **Conditional Rendering:** Affected");
    }
    if cmd.allow_no_queues {
        doc.push_str("\n - **Allow No Queues:** True");
    }
    if !cmd.tasks.is_empty() {
        let t_names: Vec<_> = cmd.tasks.iter().map(|t| format!("{t:?}")).collect();
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
            .map(|l| format!("{l:?}"))
            .collect();
        doc.push_str(&format!(
            "\n - **Command Buffer Levels:** {}",
            l_names.join(", ")
        ));
    }
    if let Some(ref es) = cmd.extern_sync {
        doc.push_str(&format!("\n - **External Sync:** {es}"));
    }
    if !cmd.export.is_empty() {
        let e_names: Vec<_> = cmd.export.iter().map(|e| format!("{e:?}")).collect();
        doc.push_str(&format!("\n - **Export Scopes:** {}", e_names.join(", ")));
    }

    if !cmd.params.is_empty() {
        doc.push_str("\n\n # Parameters");

        for p in &cmd.params {
            let mut line = format!(" - `{}`", p.name);

            let mut p_meta = Vec::new();
            if p.optional != Optional::False {
                p_meta.push(format!("optional: {:?}", p.optional));
            }
            if let Some(ref len) = p.len {
                p_meta.push(format!("len: {len}"));
            }
            if let Some(ref vals) = p.values {
                p_meta.push(format!("values: {vals}"));
            }
            if let Some(ref ot) = p.object_type {
                p_meta.push(format!("object_type: {ot}"));
            }

            if !p_meta.is_empty() {
                line.push_str(&format!(": {}", p_meta.join(", ")));
            }

            doc.push_str(&format!("\n{line}"));
        }
    }

    if !cmd.success_codes.is_empty() || !cmd.error_codes.is_empty() {
        doc.push_str("\n\n # Returns");
    }

    if !cmd.success_codes.is_empty() {
        doc.push_str(&format!(
            "\n\n **Success Codes:**\n   - `{}`",
            cmd.success_codes.join("`\n   - `")
        ));
    }
    if !cmd.error_codes.is_empty() {
        doc.push_str(&format!(
            "\n\n **Error Codes:**\n   - `{}`",
            cmd.error_codes.join("`\n   - `")
        ));
    }

    doc.push('\n');

    doc
}
