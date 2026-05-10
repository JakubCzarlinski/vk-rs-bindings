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
#[cfg(feature = "VK_KHR_descriptor_update_template")]
#[derive(Debug, Clone)]
pub struct DescriptorUpdateTemplateKHRDispatchTable {
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkDestroyDescriptorUpdateTemplateKHR: Option<PFN_vkDestroyDescriptorUpdateTemplateKHR>,
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl DescriptorUpdateTemplateKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    vkDestroyDescriptorUpdateTemplateKHR: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_KHR_descriptor_update_template")]
      vkDestroyDescriptorUpdateTemplateKHR: loader(
        c"vkDestroyDescriptorUpdateTemplateKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub struct DescriptorUpdateTemplateKHR<'dev> {
  pub(crate) raw: VkDescriptorUpdateTemplateKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev DescriptorUpdateTemplateKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
unsafe impl<'dev> Send for DescriptorUpdateTemplateKHR<'dev> {}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
unsafe impl<'dev> Sync for DescriptorUpdateTemplateKHR<'dev> {}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'dev> Drop for DescriptorUpdateTemplateKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyDescriptorUpdateTemplateKHR).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_KHR_descriptor_update_template")]
impl<'dev> DescriptorUpdateTemplateKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDescriptorUpdateTemplateKHR {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent.instance()
  }
  #[inline(always)]
  pub const fn table(&self) -> &DescriptorUpdateTemplateKHRDispatchTable {
    self.table
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
  pub fn vkDestroyDescriptorUpdateTemplateKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyDescriptorUpdateTemplateKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkDescriptorUpdateTemplateKHR::NULL;
  }
}
