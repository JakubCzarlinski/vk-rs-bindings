#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::commands::PFN_vkDestroySamplerYcbcrConversionKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAllocationCallbacks;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkSamplerYcbcrConversionKHR;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
#[derive(Debug, Clone)]
pub struct SamplerYcbcrConversionKHRDispatchTable {
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  pub vkDestroySamplerYcbcrConversionKHR: Option<PFN_vkDestroySamplerYcbcrConversionKHR>,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl SamplerYcbcrConversionKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    vkDestroySamplerYcbcrConversionKHR: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
      vkDestroySamplerYcbcrConversionKHR: loader(c"vkDestroySamplerYcbcrConversionKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub struct SamplerYcbcrConversionKHR<'dev> {
  pub(crate) raw: VkSamplerYcbcrConversionKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev SamplerYcbcrConversionKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'dev> Send for SamplerYcbcrConversionKHR<'dev> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
unsafe impl<'dev> Sync for SamplerYcbcrConversionKHR<'dev> {}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'dev> Drop for SamplerYcbcrConversionKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroySamplerYcbcrConversionKHR).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
impl<'dev> SamplerYcbcrConversionKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkSamplerYcbcrConversionKHR {
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
  pub const fn table(&self) -> &SamplerYcbcrConversionKHRDispatchTable {
    self.table
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
  pub fn vkDestroySamplerYcbcrConversionKHR(
    &mut self,
    pAllocator: *const VkAllocationCallbacks<'_>,
  ) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroySamplerYcbcrConversionKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkSamplerYcbcrConversionKHR::NULL;
  }
}
