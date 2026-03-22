use crate::cfggen::cfg_any;
use crate::codegen::entry_rs::entry_set;
use crate::codegen::pretty;
use crate::codegen::utils::{
    Tier, c_str_lit, collect_groups, enabled_set, is_cmd_buf_cmd, is_core, raw_dispatch_method,
    safe_method_body,
};
use crate::ir::Registry;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::{HashMap, HashSet};

/// Generate `device.rs` - the device dispatch table, the `Device` safe wrapper,
/// and the `CommandBuffer` wrapper.
///
/// # Generated types
///
/// - [`DeviceDispatchTable`] — raw `Option<PFN_*>` table for all device-tier
///   commands, heap-allocated inside both `Device` and `CommandBuffer`.
/// - [`Device`] — ergonomic safe wrapper created directly from an [`Instance`]
///   via [`Device::create`]; no need to manually drive `vkCreateDevice` and
///   then separately load the table.
/// - [`CommandBuffer`] — borrows its parent `Device`, exposes all `vkCmd*`
///   commands as methods.
pub fn gen_device_rs(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> String {
    let mut ts = TokenStream::new();
    ts.extend(device_preamble());
    ts.extend(gen_device_dispatch_table(reg));
    ts.extend(gen_device_wrapper(reg, result_cfgs, handle_types));
    ts.extend(gen_command_buffer_wrapper(reg, result_cfgs, handle_types));
    pretty(ts)
}

// Preamble
fn device_preamble() -> TokenStream {
    quote! {
        //! Device-tier dispatch table, safe `Device` wrapper, and `CommandBuffer`.
        //!
        //! # Usage
        //!
        //! ```rust
        //! use vk::{VulkanLib, Instance, Device};
        //!
        //! let lib      = VulkanLib::load()?;
        //! let instance = Instance::create(&lib, &inst_info, None)?;
        //! let gpus     = instance.vkEnumeratePhysicalDevices()?;
        //!
        //! // One call: create VkDevice + load all device function pointers.
        //! let device = Device::create(&instance, gpus[0], &dev_info, None)?;
        //!
        //! // Borrow a CommandBuffer wrapper for recording.
        //! let cmd = device.command_buffer(cmd_buf_handle);
        //! cmd.vkCmdDraw(3, 1, 0, 0);
        //! ```

        #![allow(
            non_snake_case,
            unused_imports,
            clippy::too_many_arguments,
            clippy::missing_safety_doc,
        )]

        use core::ffi::{c_char, c_void};
        use std::boxed::Box;
        use crate::commands::*;
        use crate::types::*;
        use crate::enums::*;
        use crate::instance::{Instance, InstanceDispatchTable};
    }
}

// DeviceDispatchTable
fn gen_device_dispatch_table(reg: &Registry) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut fields_ts = TokenStream::new();
    let mut empty_ts = TokenStream::new();
    let mut load_ts = TokenStream::new();
    let mut methods_ts = TokenStream::new();

    for (feature, cmds) in &groups {
        let sec = format!(" `{}`", feature);
        fields_ts.extend(quote! { #[doc = #sec] });

        for (name, providers, cmd) in cmds {
            let cfg = cfg_any(providers);
            let fname = format_ident!("{}", name);
            let pfn = format_ident!("PFN_{}", name);
            let clit = c_str_lit(name);
            let core_fn = is_core(providers);

            fields_ts.extend(quote! { #cfg pub #fname: Option<#pfn>, });
            empty_ts.extend(quote! { #cfg #fname: None, });
            load_ts.extend(quote! {
                #cfg {
                    table.#fname = loader(#clit.as_ptr())
                        .map(|f| unsafe { core::mem::transmute(f) });
                }
            });
            methods_ts.extend(raw_dispatch_method(cmd, name, providers, core_fn));
        }
    }

    quote! {
        /// Raw dispatch table for `VkDevice` commands.
        ///
        /// All fields are `Option<PFN_*>`.  `None` means the command was absent at
        /// load time.  For the ergonomic safe API see [`Device`].
        ///
        /// Heap-allocated inside [`Device`] so that moving a `Device` does not
        /// relocate the ~1500 function pointers.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        #[derive(Clone)]
        pub struct DeviceDispatchTable { #fields_ts }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl DeviceDispatchTable {
            pub const EMPTY: Self = Self { #empty_ts };

            /// Resolve all device commands via `vkGetDeviceProcAddr`.
            pub fn load<F>(mut loader: F) -> Self
            where
                F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
            {
                let mut table = Self::EMPTY;
                #load_ts
                table
            }

            /// Capture `device` in the `vkGetDeviceProcAddr` closure.
            pub fn load_with_handle<F>(device: VkDevice, mut get_proc: F) -> Self
            where
                F: FnMut(VkDevice, *const c_char) -> Option<unsafe extern "system" fn()>,
            {
                Self::load(|name| get_proc(device, name))
            }

            #methods_ts
        }
    }
}

// Device safe wrapper
fn gen_device_wrapper(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if is_cmd_buf_cmd(cmd) {
                continue;
            }
            methods_ts.extend(safe_method_body(
                cmd,
                name,
                providers,
                Tier::Device,
                quote! { self.raw },
                quote! { &self.table },
                result_cfgs,
                handle_types,
            ));
        }
    }

    // We need the vkCreateDevice params to build Device::create.
    // Pull them from the instance table generator's special-cased params.
    // Instead we generate a clean Device::create that delegates to Instance::vkCreateDevice.
    quote! {
        /// Safe `VkDevice` wrapper with owned dispatch table.
        ///
        /// # Creation
        ///
        /// Use [`Device::create`] for the one-call ergonomic path:
        ///
        /// ```rust
        /// let device = Device::create(&instance, physical_device, &dev_info, None)?;
        /// ```
        ///
        /// Or build manually from a raw handle if you drive `vkCreateDevice` yourself:
        ///
        /// ```rust
        /// let table = DeviceDispatchTable::load_with_handle(raw_dev, |dev, name| {
        ///     unsafe { instance.table().vkGetDeviceProcAddr.unwrap()(dev, name) }
        /// });
        /// let device = unsafe { Device::from_raw(raw_dev, table) };
        /// ```
        ///
        /// # Cleanup
        ///
        /// No implicit `Drop`.  Call [`Device::vkDestroyDevice`] explicitly after
        /// destroying all child resources (buffers, pipelines, command pools, ...).
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct Device {
            raw:   VkDevice,
            /// Heap-allocated so that moving a `Device` doesn't relocate ~1500 fn ptrs.
            table: Box<DeviceDispatchTable>,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl Device {
            // -------------------------------------------------------------------
            // Internal constructor (called from Instance::vkCreateDevice)
            // -------------------------------------------------------------------
            #[inline]
            pub(crate) fn new_owned(raw: VkDevice, table: DeviceDispatchTable) -> Self {
                Self { raw, table: Box::new(table) }
            }

            // -------------------------------------------------------------------
            // Construction
            // -------------------------------------------------------------------

            /// **One-call ergonomic constructor.**
            ///
            /// Calls `vkCreateDevice` on `instance` then immediately loads the full
            /// [`DeviceDispatchTable`] via `vkGetDeviceProcAddr`, bypassing the
            /// loader dispatch layer for direct driver function pointers.
            ///
            /// # Safety
            /// `physical_device` must have been enumerated from `instance`.
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            #[inline]
            pub unsafe fn create(
                instance:        &Instance,
                physical_device: VkPhysicalDevice,
                create_info:     *const VkDeviceCreateInfo,
                allocator:       Option<&VkAllocationCallbacks>,
            ) -> Result<Self, VkResult> {
                // Delegate to the special-cased method on Instance which handles
                // the full create + table-load sequence.
                unsafe { instance.vkCreateDevice(physical_device, create_info, allocator) }
            }

            /// Wrap a raw `VkDevice` with a pre-loaded dispatch table.
            ///
            /// # Safety
            /// `raw` must be a valid live `VkDevice`.
            #[inline]
            pub unsafe fn from_raw(raw: VkDevice, table: DeviceDispatchTable) -> Self {
                Self { raw, table: Box::new(table) }
            }

            // -------------------------------------------------------------------
            // Accessors
            // -------------------------------------------------------------------

            /// The raw `VkDevice` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkDevice { self.raw }

            /// The underlying dispatch table.
            #[inline(always)]
            pub fn table(&self) -> &DeviceDispatchTable { &self.table }

            /// Borrow a [`CommandBuffer`] wrapper for recording.
            ///
            /// The returned `CommandBuffer` borrows this `Device` so it cannot
            /// outlive it.
            #[inline(always)]
            pub fn command_buffer(&self, cmd_buf: VkCommandBuffer) -> CommandBuffer<'_> {
                CommandBuffer { raw: cmd_buf, device: self }
            }

            // -------------------------------------------------------------------
            // Safe device methods
            // -------------------------------------------------------------------
            #methods_ts
        }
    }
}

// CommandBuffer wrapper
fn gen_command_buffer_wrapper(
    reg: &Registry,
    result_cfgs: &HashMap<String, TokenStream>,
    handle_types: &HashSet<String>,
) -> TokenStream {
    let skip = entry_set();
    let enabled = enabled_set(reg);
    let groups = collect_groups(reg, Tier::Device, &skip, &enabled);

    let mut methods_ts = TokenStream::new();
    for cmds in groups.values() {
        for (name, providers, cmd) in cmds {
            if !is_cmd_buf_cmd(cmd) {
                continue;
            }
            methods_ts.extend(safe_method_body(
                cmd,
                name,
                providers,
                Tier::Device,
                quote! { self.raw },
                quote! { &self.device.table },
                result_cfgs,
                handle_types,
            ));
        }
    }

    quote! {
        /// Borrowed `VkCommandBuffer` wrapper.
        ///
        /// Borrows the [`Device`] it was obtained from, so it cannot outlive it.
        /// Obtain via [`Device::command_buffer`].
        ///
        /// All `vkCmd*` commands are methods on this type.  The first Vulkan
        /// parameter (`commandBuffer`) is supplied from `self.raw`.
        ///
        /// Recording, submission, and lifecycle management (allocation, reset, free)
        /// remain the caller's responsibility.
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        pub struct CommandBuffer<'dev> {
            raw:    VkCommandBuffer,
            device: &'dev Device,
        }

        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        impl<'dev> CommandBuffer<'dev> {
            /// The raw `VkCommandBuffer` handle.
            #[inline(always)]
            pub fn raw(&self) -> VkCommandBuffer { self.raw }

            /// The parent [`Device`].
            #[inline(always)]
            pub fn device(&self) -> &Device { self.device }

            #methods_ts
        }
    }
}
