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
pub struct DescriptorUpdateTemplateDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    pub vkDestroyDescriptorUpdateTemplate: Option<PFN_vkDestroyDescriptorUpdateTemplate>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub vkDestroyDescriptorUpdateTemplateKHR: Option<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DescriptorUpdateTemplateDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        vkDestroyDescriptorUpdateTemplate: None,
        #[cfg(feature = "VK_KHR_descriptor_update_template")]
        vkDestroyDescriptorUpdateTemplateKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        {
            table.vkDestroyDescriptorUpdateTemplate =
                loader(c"vkDestroyDescriptorUpdateTemplate".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_descriptor_update_template")]
        {
            table.vkDestroyDescriptorUpdateTemplateKHR =
                loader(c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub struct DescriptorUpdateTemplate<'dev> {
    pub(crate) raw: VkDescriptorUpdateTemplate,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev DescriptorUpdateTemplateDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'dev> Drop for DescriptorUpdateTemplate<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyDescriptorUpdateTemplate {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'dev> DescriptorUpdateTemplate<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDescriptorUpdateTemplate {
        self.raw
    }
    #[inline]
    pub fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub fn table(&self) -> &DescriptorUpdateTemplateDispatchTable {
        self.table
    }
    /// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_1`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorUpdateTemplate`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    #[inline(always)]
    pub fn vkDestroyDescriptorUpdateTemplate(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyDescriptorUpdateTemplate
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkDescriptorUpdateTemplate::NULL;
    }
    /// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)
    ///
    /// Provided by:
    /// - `VK_KHR_descriptor_update_template`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorUpdateTemplate`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[inline(always)]
    pub fn vkDestroyDescriptorUpdateTemplateKHR(&self, pAllocator: *const VkAllocationCallbacks) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyDescriptorUpdateTemplateKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
    }
}
