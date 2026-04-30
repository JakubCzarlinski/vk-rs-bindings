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
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct DescriptorSetDispatchTable {
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkUpdateDescriptorSetWithTemplate: Option<PFN_vkUpdateDescriptorSetWithTemplate>,
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkUpdateDescriptorSetWithTemplateKHR: Option<PFN_vkUpdateDescriptorSetWithTemplateKHR>,
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  pub vkGetDescriptorSetHostMappingVALVE: Option<PFN_vkGetDescriptorSetHostMappingVALVE>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl DescriptorSetDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkUpdateDescriptorSetWithTemplate: None,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    vkUpdateDescriptorSetWithTemplateKHR: None,
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    vkGetDescriptorSetHostMappingVALVE: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkUpdateDescriptorSetWithTemplate: loader(c"vkUpdateDescriptorSetWithTemplate".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_descriptor_update_template")]
      vkUpdateDescriptorSetWithTemplateKHR: loader(
        c"vkUpdateDescriptorSetWithTemplateKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
      vkGetDescriptorSetHostMappingVALVE: loader(c"vkGetDescriptorSetHostMappingVALVE".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct DescriptorSet<'dev> {
  pub(crate) raw: VkDescriptorSet,
  pub(crate) parent: &'dev crate::descriptor_pool::DescriptorPool<'dev>,
  pub(crate) table: &'dev DescriptorSetDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for DescriptorSet<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for DescriptorSet<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for DescriptorSet<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.parent.table.vkFreeDescriptorSets).unwrap_unchecked()(
        self.device().raw,
        self.parent.raw,
        1,
        &self.raw,
      )
    };
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> DescriptorSet<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDescriptorSet {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::descriptor_pool::DescriptorPool<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
    self.parent.device()
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent.instance()
  }
  #[inline(always)]
  pub const fn table(&self) -> &DescriptorSetDispatchTable {
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
  pub fn vkGetDescriptorSetHostMappingVALVE(&self, ppData: *mut *mut core::ffi::c_void) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetDescriptorSetHostMappingVALVE
        .unwrap_unchecked()(self.device().raw(), self.raw, ppData)
    }
  }
}
