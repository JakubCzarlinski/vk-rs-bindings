use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::pretty;
use crate::codegen::utils::{
    c_str_lit, collect_groups, enabled_set, result_check_arms, safe_method,
    safe_method_unit_with_overrides,
};
use crate::ir::{Command, Registry, TypedefKind};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct HandleMeta {
    pub vk_name: String,
    pub struct_name: String,
    pub mod_name: String,
    pub table_name: String,
    pub table_field: String,
    pub parent_vk_name: String,
    pub root_vk_name: String,
    pub providers: Vec<String>,
}

pub fn get_handle_metadata(reg: &Registry) -> BTreeMap<String, HandleMeta> {
    let mut parents: HashMap<String, String> = HashMap::new();
    let mut typedef_providers: HashMap<String, Vec<String>> = HashMap::new();
    for (name, tds) in &reg.typedefs {
        for td in tds {
            if let TypedefKind::Handle {
                parent: Some(p), ..
            } = &td.kind
            {
                parents.insert(name.clone(), p.clone());
                typedef_providers.insert(name.clone(), td.provided_by.clone());
            }
        }
    }

    let mut desc = HashSet::new();
    let mut changed = true;
    while changed {
        changed = false;
        for (child, p) in &parents {
            if (p == "VkDevice" || desc.contains(p)) && desc.insert(child.clone()) {
                changed = true;
            }
        }
    }

    let mut instance_children = HashSet::new();
    for (child, parent) in &parents {
        if parent == "VkInstance" && child != "VkPhysicalDevice" {
            instance_children.insert(child.clone());
        }
    }

    let mut map = BTreeMap::new();
    for name in desc.into_iter().chain(instance_children) {
        let struct_name = name.replace("Vk", "");
        let mod_name = snake_case(&struct_name);
        let table_name = format!("{struct_name}DispatchTable");
        let table_field = format!("{mod_name}_table");
        let parent = parents
            .get(&name)
            .cloned()
            .unwrap_or_else(|| "VkDevice".into());
        let root = root_owner(&name, &parents);
        let providers = typedef_providers.get(&name).cloned().unwrap_or_default();

        map.insert(
            name.clone(),
            HandleMeta {
                vk_name: name,
                struct_name,
                mod_name,
                table_name,
                table_field,
                parent_vk_name: parent,
                root_vk_name: root,
                providers,
            },
        );
    }
    map
}

fn root_owner(name: &str, parents: &HashMap<String, String>) -> String {
    let mut current = name;
    while let Some(parent) = parents.get(current) {
        match parent.as_str() {
            "VkInstance" | "VkPhysicalDevice" | "VkDevice" => return parent.clone(),
            other => current = other,
        }
    }
    String::new()
}

pub fn gen_handles(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let meta_map = get_handle_metadata(reg);

    for meta in meta_map.values() {
        let content = gen_handle_module(
            reg,
            meta,
            &meta_map,
            result_cfgs,
            handle_types,
            &skip,
            &enabled,
        );
        map.insert(meta.mod_name.clone(), pretty(&content));
    }
    map
}

pub fn snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_is_upper = false;
    for c in s.chars() {
        if c.is_uppercase() {
            if !out.is_empty() && !prev_is_upper {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
            prev_is_upper = true;
        } else {
            out.push(c);
            prev_is_upper = false;
        }
    }
    out.replace("k_h_r", "_khr")
        .replace("e_x_t", "_ext")
        .replace("n_v", "_nv")
}

fn is_self_destructor_command(cmd_name: &str, cmd: &Command, meta: &HandleMeta) -> bool {
    let destroy_name = format!("vkDestroy{}", meta.struct_name);
    let free_group_name = format!("vkFree{}s", meta.struct_name);
    let free_name = format!("vkFree{}", meta.struct_name);
    let custom_free_name = if meta.vk_name == "VkDeviceMemory" {
        Some("vkFreeMemory")
    } else {
        None
    };

    let is_match = |name: &str| {
        name == destroy_name
            || name == free_group_name
            || name == free_name
            || custom_free_name.is_some_and(|custom| name == custom)
    };

    is_match(cmd_name) || cmd.alias.as_deref().is_some_and(is_match)
}

fn is_supported_handle_method(cmd: &Command, meta: &HandleMeta) -> bool {
    let Some(first) = cmd.params.first() else {
        return true;
    };
    if first.ty.base == meta.vk_name {
        return true;
    }

    let Some(second) = cmd.params.get(1) else {
        return true;
    };
    if second.ty.base != meta.vk_name {
        return true;
    }

    let dispatch_root = first.ty.base.as_str();
    dispatch_root == meta.parent_vk_name
        || (meta.root_vk_name == "VkDevice" && dispatch_root == "VkDevice")
}

fn gen_handle_module(
    reg: &Registry,
    meta: &HandleMeta,
    meta_map: &BTreeMap<String, HandleMeta>,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
    skip: &HashSet<&str>,
    enabled: &HashSet<String>,
) -> TokenStream {
    use crate::codegen::utils::Tier;
    use quote::{format_ident, quote};

    let skip_set = skip.clone();
    let groups = collect_groups(reg, Tier::Handle(meta.vk_name.clone()), &skip_set, enabled);
    let _mod_name = format_ident!("{}", meta.mod_name);
    let struct_name = format_ident!("{}", meta.struct_name);
    let table_name = format_ident!("{}", meta.table_name);

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    let mut methods_ts = TokenStream::new();

    let mut destroy_stmt = quote! {};
    let destroy_name = format!("vkDestroy{}", meta.struct_name);
    let free_group_name = format!("vkFree{}s", meta.struct_name);
    let free_name = format!("vkFree{}", meta.struct_name);
    let custom_free_name = if meta.vk_name == "VkDeviceMemory" {
        "vkFreeMemory".to_string()
    } else {
        String::new()
    };

    if reg.commands.contains_key(&destroy_name) {
        let fp = format_ident!("{}", destroy_name);
        destroy_stmt = if meta.parent_vk_name == "VkInstance" {
            quote! {
                if let Some(destroy_fn) = self.parent.table.#fp {
                    unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
                }
            }
        } else {
            quote! {
                if let Some(destroy_fn) = self.table.#fp {
                    unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
                }
            }
        };
    } else if reg.commands.contains_key(&free_group_name) {
        let fp = format_ident!("{}", free_group_name);
        destroy_stmt = quote! {
            if let Some(free_fn) = self.parent.table.#fp {
                unsafe { free_fn(self.device().raw, self.parent.raw, 1, &self.raw) };
            }
        };
    } else if reg.commands.contains_key(&free_name) {
        let fp = format_ident!("{}", free_name);
        destroy_stmt = quote! {
            if let Some(free_fn) = self.table.#fp {
                unsafe { free_fn(self.device().raw, self.raw, core::ptr::null()) };
            }
        };
    } else if !custom_free_name.is_empty() && reg.commands.contains_key(&custom_free_name) {
        let fp = format_ident!("{}", custom_free_name);
        destroy_stmt = quote! {
            if let Some(free_fn) = self.table.#fp {
                unsafe { free_fn(self.device().raw, self.raw, core::ptr::null()) };
            }
        };
    }

    for cmds in groups.values() {
        for (cmd_name, providers, cmd) in cmds {
            if !is_supported_handle_method(cmd, meta) {
                continue;
            }
            let cfg = cfg_any(providers);
            let fname = format_ident!("{}", cmd_name);
            let pfn = format_ident!("PFN_{}", cmd_name);
            let clit = c_str_lit(cmd_name);

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! { #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg { table.#fname = loader(#clit.as_ptr()).map(|f| unsafe { core::mem::transmute(f) }); }
            });

            if cmd_name == "vkAllocateCommandBuffers" {
                methods_ts.extend(gen_allocate_command_buffers(cmd, providers, result_cfgs));
            } else if cmd_name == "vkFreeCommandBuffers" {
                methods_ts.extend(gen_free_command_buffers(cmd, providers));
            } else if cmd_name == "vkAllocateDescriptorSets" {
                methods_ts.extend(gen_allocate_descriptor_sets(cmd, providers, result_cfgs));
            } else if cmd_name == "vkFreeDescriptorSets" {
                methods_ts.extend(gen_free_descriptor_sets(cmd, providers));
            } else if is_self_destructor_command(cmd_name, cmd, meta) {
                let handle_ty = format_ident!("{}", meta.vk_name);
                methods_ts.extend(safe_method_unit_with_overrides(
                    cmd,
                    cmd_name,
                    providers,
                    &meta.vk_name, // handle_base to strip
                    quote! { self.raw },
                    quote! { self.table },
                    result_cfgs,
                    handle_types,
                    Some(meta_map),
                    quote! { self.device() },
                    quote! { self.instance() },
                    quote! { &mut self },
                    quote! {
                        if self.raw.0.is_null() {
                            return;
                        }
                    },
                    quote! {
                        self.raw = #handle_ty::NULL;
                    },
                ));
            } else {
                let device_acc = quote! { self.device() };
                methods_ts.extend(safe_method(
                    cmd,
                    cmd_name,
                    providers,
                    &meta.vk_name, // handle_base to strip
                    quote! { self.raw },
                    quote! { self.table },
                    result_cfgs,
                    handle_types,
                    Some(meta_map),
                    device_acc,
                    quote! { self.instance() },
                ));
            }
        }
    }

    let parent_type = format_ident!("{}", meta.parent_vk_name.replace("Vk", ""));
    let parent_mod = format_ident!("{}", snake_case(&meta.parent_vk_name.replace("Vk", "")));
    let vk_ident = format_ident!("{}", meta.vk_name);
    let (parent_ty_decl, device_accessor, instance_method) = if meta.parent_vk_name == "VkDevice" {
        (
            quote! { crate::device::Device<'dev> },
            quote! { self.parent },
            quote! {
                #[inline]
                pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> { self.parent.instance() }
            },
        )
    } else if meta.parent_vk_name == "VkInstance" {
        (
            quote! { crate::instance::Instance<'dev> },
            quote! {},
            quote! {
                #[inline]
                pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> { self.parent }
            },
        )
    } else {
        (
            quote! { crate::#parent_mod::#parent_type<'dev> },
            quote! { self.parent.device() },
            quote! {
                #[inline]
                pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> { self.parent.instance() }
            },
        )
    };
    let device_method = if meta.root_vk_name == "VkDevice" {
        quote! {
            #[inline]
            pub fn device(&self) -> &'dev crate::device::Device<'dev> { #device_accessor }
        }
    } else {
        quote! {}
    };
    let wrapper_cfg = cfg_any(&meta.providers);

    quote! {
        #![allow(non_snake_case, unused_imports, clippy::too_many_arguments, clippy::missing_safety_doc)]
        use core::ffi::{c_char, c_void};
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Debug, Clone)]
        pub struct #table_name { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl #table_name {
            pub const EMPTY: Self = Self { #empty_ts };
            #[allow(unused_mut, unused_variables)]
            pub fn load<F>(mut loader: F) -> Self where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }
        }

        #wrapper_cfg
        pub struct #struct_name<'dev> {
            pub(crate) raw: #vk_ident,
            pub(crate) parent: &'dev #parent_ty_decl,
            pub(crate) table: &'dev #table_name,
        }

        #wrapper_cfg
        impl<'dev> Drop for #struct_name<'dev> {
            fn drop(&mut self) {
                if self.raw.0.is_null() {
                    return;
                }
                #destroy_stmt
            }
        }

        #wrapper_cfg
        impl<'dev> #struct_name<'dev> {
            #[inline] pub fn raw(&self) -> #vk_ident { self.raw }
            #[inline] pub fn parent(&self) -> &'dev #parent_ty_decl { self.parent }
            #device_method
            #instance_method
            #[inline] pub fn table(&self) -> &#table_name { self.table }
            #methods_ts
        }
    }
}

fn gen_allocate_command_buffers(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);

    quote! {
        #cfg
        #[inline]
        pub fn vkAllocateCommandBuffers<'pool>(
            &'pool self,
            pAllocateInfo: *const VkCommandBufferAllocateInfo,
        ) -> Result<alloc::vec::Vec<crate::command_buffer::CommandBuffer<'pool>>, VkResult> {
            let count = unsafe { (*pAllocateInfo).commandBufferCount };
            let mut raw_buffers = alloc::vec::Vec::with_capacity(count as usize);
            let fp = unsafe { self.table.vkAllocateCommandBuffers.unwrap_unchecked() };
            let r = unsafe { fp(self.device().raw, pAllocateInfo, raw_buffers.as_mut_ptr()) };
            if let Err(e) = { #result_check } { return Err(e); }
            unsafe { raw_buffers.set_len(count as usize); }

            Ok(raw_buffers.into_iter().map(|raw| crate::command_buffer::CommandBuffer {
                raw,
                parent: self,
                table: &self.device().command_buffer_table
            }).collect())
        }
    }
}

fn gen_free_command_buffers(_cmd: &Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    quote! {
        #cfg
        #[inline]
        pub fn vkFreeCommandBuffers(
            &self,
            commandBufferCount: u32,
            pCommandBuffers: *const VkCommandBuffer,
        ) {
            let fp = unsafe { self.table.vkFreeCommandBuffers.unwrap_unchecked() };
            unsafe { fp(self.device().raw, self.raw, commandBufferCount, pCommandBuffers) }
        }
    }
}

fn gen_allocate_descriptor_sets(
    cmd: &Command,
    providers: &[String],
    result_cfgs: &HashMap<String, TokenStream>,
) -> TokenStream {
    let cfg = cfg_any(providers);
    let result_check = result_check_arms(&cmd.success_codes, &cmd.error_codes, result_cfgs);

    quote! {
        #cfg
        #[inline]
        pub fn vkAllocateDescriptorSets<'pool>(
            &'pool self,
            pAllocateInfo: *const VkDescriptorSetAllocateInfo,
        ) -> Result<alloc::vec::Vec<crate::descriptor_set::DescriptorSet<'pool>>, VkResult> {
            let count = unsafe { (*pAllocateInfo).descriptorSetCount };
            let mut raw_sets = alloc::vec::Vec::with_capacity(count as usize);
            let fp = unsafe { self.table.vkAllocateDescriptorSets.unwrap_unchecked() };
            let r = unsafe { fp(self.device().raw, pAllocateInfo, raw_sets.as_mut_ptr()) };
            if let Err(e) = { #result_check } { return Err(e); }
            unsafe { raw_sets.set_len(count as usize); }

            Ok(raw_sets.into_iter().map(|raw| crate::descriptor_set::DescriptorSet {
                raw,
                parent: self,
                table: &self.device().descriptor_set_table
            }).collect())
        }
    }
}

fn gen_free_descriptor_sets(_cmd: &Command, providers: &[String]) -> TokenStream {
    let cfg = cfg_any(providers);
    quote! {
        #cfg
        #[inline]
        pub fn vkFreeDescriptorSets(
            &self,
            descriptorSetCount: u32,
            pDescriptorSets: *const VkDescriptorSet,
        ) -> Result<VkResult, VkResult> {
            let fp = unsafe { self.table.vkFreeDescriptorSets.unwrap_unchecked() };
            let r = unsafe { fp(self.device().raw, self.raw, descriptorSetCount, pDescriptorSets) };
            if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
