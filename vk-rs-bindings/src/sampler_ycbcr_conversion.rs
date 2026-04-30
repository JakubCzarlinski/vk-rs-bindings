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
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
#[derive(Debug, Clone)]
pub struct SamplerYcbcrConversionDispatchTable {
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkDestroySamplerYcbcrConversion: Option<PFN_vkDestroySamplerYcbcrConversion>,
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  pub vkDestroySamplerYcbcrConversionKHR: Option<PFN_vkDestroySamplerYcbcrConversionKHR>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl SamplerYcbcrConversionDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkDestroySamplerYcbcrConversion: None,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    vkDestroySamplerYcbcrConversionKHR: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkDestroySamplerYcbcrConversion: loader(c"vkDestroySamplerYcbcrConversion".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
      vkDestroySamplerYcbcrConversionKHR: loader(c"vkDestroySamplerYcbcrConversionKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub struct SamplerYcbcrConversion<'dev> {
  pub(crate) raw: VkSamplerYcbcrConversion,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev SamplerYcbcrConversionDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'dev> Send for SamplerYcbcrConversion<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
unsafe impl<'dev> Sync for SamplerYcbcrConversion<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'dev> Drop for SamplerYcbcrConversion<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroySamplerYcbcrConversion).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
impl<'dev> SamplerYcbcrConversion<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkSamplerYcbcrConversion {
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
  pub const fn table(&self) -> &SamplerYcbcrConversionDispatchTable {
    self.table
  }
  /// [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `ycbcrConversion`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkDestroySamplerYcbcrConversion(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroySamplerYcbcrConversion
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkSamplerYcbcrConversion::NULL;
  }
  /// [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)
  ///
  /// Provided by:
  /// - `VK_KHR_sampler_ycbcr_conversion`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `ycbcrConversion`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  #[inline(always)]
  pub fn vkDestroySamplerYcbcrConversionKHR(&self, pAllocator: *const VkAllocationCallbacks) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroySamplerYcbcrConversionKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
  }
}
