#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use crate::commands::*;
use crate::enums::*;
use crate::types::*;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct IndirectExecutionSetEXTDispatchTable {
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    pub vkDestroyIndirectExecutionSetEXT: Option<PFN_vkDestroyIndirectExecutionSetEXT>,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    pub vkUpdateIndirectExecutionSetPipelineEXT:
        Option<PFN_vkUpdateIndirectExecutionSetPipelineEXT>,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    pub vkUpdateIndirectExecutionSetShaderEXT: Option<PFN_vkUpdateIndirectExecutionSetShaderEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl IndirectExecutionSetEXTDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_EXT_device_generated_commands")]
        vkDestroyIndirectExecutionSetEXT: None,
        #[cfg(feature = "VK_EXT_device_generated_commands")]
        vkUpdateIndirectExecutionSetPipelineEXT: None,
        #[cfg(feature = "VK_EXT_device_generated_commands")]
        vkUpdateIndirectExecutionSetShaderEXT: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_EXT_device_generated_commands")]
            vkDestroyIndirectExecutionSetEXT: loader(c"vkDestroyIndirectExecutionSetEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_EXT_device_generated_commands")]
            vkUpdateIndirectExecutionSetPipelineEXT: loader(
                c"vkUpdateIndirectExecutionSetPipelineEXT".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_EXT_device_generated_commands")]
            vkUpdateIndirectExecutionSetShaderEXT: loader(
                c"vkUpdateIndirectExecutionSetShaderEXT".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub struct IndirectExecutionSetEXT<'dev> {
    pub(crate) raw: VkIndirectExecutionSetEXT,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev IndirectExecutionSetEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
unsafe impl<'dev> Send for IndirectExecutionSetEXT<'dev> {}
#[cfg(feature = "VK_EXT_device_generated_commands")]
unsafe impl<'dev> Sync for IndirectExecutionSetEXT<'dev> {}
#[cfg(feature = "VK_EXT_device_generated_commands")]
impl<'dev> Drop for IndirectExecutionSetEXT<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyIndirectExecutionSetEXT {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
impl<'dev> IndirectExecutionSetEXT<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkIndirectExecutionSetEXT {
        self.raw
    }
    #[inline]
    pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub const fn table(&self) -> &IndirectExecutionSetEXTDispatchTable {
        self.table
    }
    /// [`vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_device_generated_commands`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `indirectExecutionSet`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    #[inline(always)]
    pub fn vkDestroyIndirectExecutionSetEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyIndirectExecutionSetEXT
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkIndirectExecutionSetEXT::NULL;
    }
    /// [`vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_device_generated_commands`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `indirectExecutionSet`
    /// - `executionSetWriteCount`
    /// - `pExecutionSetWrites`: len: executionSetWriteCount
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    #[inline(always)]
    pub fn vkUpdateIndirectExecutionSetPipelineEXT(
        &self,
        executionSetWriteCount: u32,
        pExecutionSetWrites: *const VkWriteIndirectExecutionSetPipelineEXT,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkUpdateIndirectExecutionSetPipelineEXT
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                executionSetWriteCount,
                pExecutionSetWrites,
            )
        }
    }
    /// [`vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_device_generated_commands`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `indirectExecutionSet`
    /// - `executionSetWriteCount`
    /// - `pExecutionSetWrites`: len: executionSetWriteCount
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    #[inline(always)]
    pub fn vkUpdateIndirectExecutionSetShaderEXT(
        &self,
        executionSetWriteCount: u32,
        pExecutionSetWrites: *const VkWriteIndirectExecutionSetShaderEXT,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkUpdateIndirectExecutionSetShaderEXT
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                executionSetWriteCount,
                pExecutionSetWrites,
            )
        }
    }
}
