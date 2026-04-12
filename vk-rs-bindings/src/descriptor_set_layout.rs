#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use core::ffi::{c_char, c_void};
use crate::commands::*;
use crate::types::*;
use crate::enums::*;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct DescriptorSetLayoutDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyDescriptorSetLayout: Option<PFN_vkDestroyDescriptorSetLayout>,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    pub vkGetDescriptorSetLayoutBindingOffsetEXT: Option<
        PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
    >,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    pub vkGetDescriptorSetLayoutSizeEXT: Option<PFN_vkGetDescriptorSetLayoutSizeEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DescriptorSetLayoutDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyDescriptorSetLayout: None,
        #[cfg(feature = "VK_EXT_descriptor_buffer")]
        vkGetDescriptorSetLayoutBindingOffsetEXT: None,
        #[cfg(feature = "VK_EXT_descriptor_buffer")]
        vkGetDescriptorSetLayoutSizeEXT: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkDestroyDescriptorSetLayout = loader(
                    c"vkDestroyDescriptorSetLayout".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_descriptor_buffer")]
        {
            table.vkGetDescriptorSetLayoutBindingOffsetEXT = loader(
                    c"vkGetDescriptorSetLayoutBindingOffsetEXT".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_descriptor_buffer")]
        {
            table.vkGetDescriptorSetLayoutSizeEXT = loader(
                    c"vkGetDescriptorSetLayoutSizeEXT".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct DescriptorSetLayout<'dev> {
    pub(crate) raw: VkDescriptorSetLayout,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev DescriptorSetLayoutDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for DescriptorSetLayout<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyDescriptorSetLayout {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> DescriptorSetLayout<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDescriptorSetLayout {
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
    pub fn table(&self) -> &DescriptorSetLayoutDispatchTable {
        self.table
    }
    /// [`vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorSetLayout`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyDescriptorSetLayout(
        &mut self,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyDescriptorSetLayout
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkDescriptorSetLayout::NULL;
    }
    /// [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_descriptor_buffer`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `layout`
    /// - `binding`
    /// - `pOffset`
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    #[inline(always)]
    pub fn vkGetDescriptorSetLayoutBindingOffsetEXT(
        &self,
        binding: u32,
        pOffset: *mut VkDeviceSize,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetDescriptorSetLayoutBindingOffsetEXT
                .unwrap_unchecked()(self.device().raw(), self.raw, binding, pOffset)
        }
    }
    /// [`vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_descriptor_buffer`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `layout`
    /// - `pLayoutSizeInBytes`
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    #[inline(always)]
    pub fn vkGetDescriptorSetLayoutSizeEXT(
        &self,
        pLayoutSizeInBytes: *mut VkDeviceSize,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetDescriptorSetLayoutSizeEXT
                .unwrap_unchecked()(self.device().raw(), self.raw, pLayoutSizeInBytes)
        }
    }
}
