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
pub struct DescriptorSetDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    pub vkUpdateDescriptorSetWithTemplate: Option<PFN_vkUpdateDescriptorSetWithTemplate>,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    pub vkUpdateDescriptorSetWithTemplateKHR: Option<
        PFN_vkUpdateDescriptorSetWithTemplateKHR,
    >,
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    pub vkGetDescriptorSetHostMappingVALVE: Option<
        PFN_vkGetDescriptorSetHostMappingVALVE,
    >,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DescriptorSetDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        vkUpdateDescriptorSetWithTemplate: None,
        #[cfg(feature = "VK_KHR_descriptor_update_template")]
        vkUpdateDescriptorSetWithTemplateKHR: None,
        #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
        vkGetDescriptorSetHostMappingVALVE: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        {
            table.vkUpdateDescriptorSetWithTemplate = loader(
                    c"vkUpdateDescriptorSetWithTemplate".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_descriptor_update_template")]
        {
            table.vkUpdateDescriptorSetWithTemplateKHR = loader(
                    c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
        {
            table.vkGetDescriptorSetHostMappingVALVE = loader(
                    c"vkGetDescriptorSetHostMappingVALVE".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct DescriptorSet<'dev> {
    pub(crate) raw: VkDescriptorSet,
    pub(crate) parent: &'dev crate::descriptor_pool::DescriptorPool<'dev>,
    pub(crate) table: &'dev DescriptorSetDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for DescriptorSet<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(free_fn) = self.parent.table.vkFreeDescriptorSets {
            unsafe { free_fn(self.device().raw, self.parent.raw, 1, &self.raw) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> DescriptorSet<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDescriptorSet {
        self.raw
    }
    #[inline]
    pub fn parent(&self) -> &'dev crate::descriptor_pool::DescriptorPool<'dev> {
        self.parent
    }
    #[inline]
    pub fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent.device()
    }
    #[inline]
    pub fn table(&self) -> &DescriptorSetDispatchTable {
        self.table
    }
    /// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_1`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorSet`
    /// - `descriptorUpdateTemplate`
    /// - `pData`
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    #[inline(always)]
    pub fn vkUpdateDescriptorSetWithTemplate(
        &self,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
        pData: *const core::ffi::c_void,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkUpdateDescriptorSetWithTemplate
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                descriptorUpdateTemplate,
                pData,
            )
        }
    }
    /// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
    ///
    /// Provided by:
    /// - `VK_KHR_descriptor_update_template`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorSet`
    /// - `descriptorUpdateTemplate`
    /// - `pData`
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    #[inline(always)]
    pub fn vkUpdateDescriptorSetWithTemplateKHR(
        &self,
        descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
        pData: *const core::ffi::c_void,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkUpdateDescriptorSetWithTemplateKHR
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                descriptorUpdateTemplate,
                pData,
            )
        }
    }
    /// [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)
    ///
    /// Provided by:
    /// - `VK_VALVE_descriptor_set_host_mapping`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorSet`
    /// - `ppData`
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    #[inline(always)]
    pub fn vkGetDescriptorSetHostMappingVALVE(
        &self,
        ppData: *mut *mut core::ffi::c_void,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetDescriptorSetHostMappingVALVE
                .unwrap_unchecked()(self.device().raw(), self.raw, ppData)
        }
    }
}
