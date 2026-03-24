use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_cmd_set;
use crate::codegen::pretty;
use crate::codegen::utils::*;
use crate::ir::{Command, Registry};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

// TODO(czarlinski): add drop
pub fn gen_command_pool_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(preamble());
    ts.extend(gen_command_pool_dispatch_table(reg));
    ts.extend(gen_command_pool(reg, result_cfgs, handle_types));
    pretty(ts)
}

fn preamble() -> TokenStream {
    quote! {
        //! CommandPool-tier dispatch table and wrapper.
        #![allow(non_snake_case, unused_imports, clippy::too_many_arguments, clippy::missing_safety_doc)]
        use core::ffi::{c_char, c_void};
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use crate::device::Device;
    }
}

fn gen_command_pool_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::CommandPool, &skip, &enabled);
    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, _cmd) in cmds {
            let cfg = cfg_any(providers);
            let fname = format_ident!("{}", name);
            let pfn = format_ident!("PFN_{}", name);
            let clit = c_str_lit(name);
            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! { #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg { table.#fname = loader(#clit.as_ptr()).map(|f| unsafe { core::mem::transmute(f) }); }
            });
        }
    }
    quote! {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Debug, Clone)]
        pub struct CommandPoolDispatchTable { #fields_ts }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl CommandPoolDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };
            pub fn load<F>(mut loader: F) -> Self where F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()> {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }
        }
    }
}

fn gen_command_pool(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_cmd_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::CommandPool, &skip, &enabled);
    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if name == "vkAllocateCommandBuffers" {
                methods_ts.extend(gen_allocate_command_buffers(cmd, providers, result_cfgs));
            } else if name == "vkFreeCommandBuffers" {
                methods_ts.extend(gen_free_command_buffers(cmd, providers));
            } else {
                // Strip VkDevice
                methods_ts.extend(safe_method(
                    cmd,
                    name,
                    providers,
                    "VkDevice",
                    quote! { self.device.raw() },
                    quote! { &self.device.command_pool_table },
                    result_cfgs,
                    handle_types,
                ));
            }
        }
    }
    quote! {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct CommandPool<'dev> {
            pub(crate) raw: VkCommandPool,
            pub(crate) device: &'dev Device<'dev>,
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'dev> CommandPool<'dev> {
            #[inline] pub fn raw(&self) -> VkCommandPool { self.raw }
            #[inline] pub fn device(&self) -> &Device<'dev> { self.device }
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

    // PFN requires (device, allocateInfo, pCommandBuffers).
    // We take &VkCommandBufferAllocateInfo. But we also want to return Vec<crate::command_buffer::CommandBuffer<'pool>>
    // Wait, the allocate info provides commandPool. The pool wrapper already holds `raw`.

    quote! {
        #cfg
        #[inline]
        pub fn vkAllocateCommandBuffers(
            &self,
            pAllocateInfo: *const VkCommandBufferAllocateInfo,
        ) -> Result<alloc::vec::Vec<crate::command_buffer::CommandBuffer<'_>>, VkResult> {
            let mut count = unsafe { (*pAllocateInfo).commandBufferCount };
            let mut raw_buffers = alloc::vec::Vec::with_capacity(count as usize);
            let fp = unsafe { self.device.command_pool_table.vkAllocateCommandBuffers.unwrap_unchecked() };
            let r = unsafe { fp(self.device.raw(), pAllocateInfo, raw_buffers.as_mut_ptr()) };
            if let Err(e) = { #result_check } { return Err(e); }
            unsafe { raw_buffers.set_len(count as usize); }

            Ok(raw_buffers.into_iter().map(|raw| crate::command_buffer::CommandBuffer { raw, pool: self }).collect())
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
            let fp = unsafe { self.device.command_pool_table.vkFreeCommandBuffers.unwrap_unchecked() };
            unsafe { fp(self.device.raw(), self.raw, commandBufferCount, pCommandBuffers) }
        }
    }
}
