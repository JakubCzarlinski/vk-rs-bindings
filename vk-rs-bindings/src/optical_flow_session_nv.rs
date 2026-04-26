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
#[cfg(feature = "VK_NV_optical_flow")]
#[derive(Debug, Clone)]
pub struct OpticalFlowSessionNVDispatchTable {
  #[cfg(feature = "VK_NV_optical_flow")]
  pub vkBindOpticalFlowSessionImageNV: Option<PFN_vkBindOpticalFlowSessionImageNV>,
  #[cfg(feature = "VK_NV_optical_flow")]
  pub vkDestroyOpticalFlowSessionNV: Option<PFN_vkDestroyOpticalFlowSessionNV>,
}
#[cfg(feature = "VK_NV_optical_flow")]
impl OpticalFlowSessionNVDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_NV_optical_flow")]
    vkBindOpticalFlowSessionImageNV: None,
    #[cfg(feature = "VK_NV_optical_flow")]
    vkDestroyOpticalFlowSessionNV: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_NV_optical_flow")]
      vkBindOpticalFlowSessionImageNV: loader(c"vkBindOpticalFlowSessionImageNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_optical_flow")]
      vkDestroyOpticalFlowSessionNV: loader(c"vkDestroyOpticalFlowSessionNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_NV_optical_flow")]
pub struct OpticalFlowSessionNV<'dev> {
  pub(crate) raw: VkOpticalFlowSessionNV,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev OpticalFlowSessionNVDispatchTable,
}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'dev> Send for OpticalFlowSessionNV<'dev> {}
#[cfg(feature = "VK_NV_optical_flow")]
unsafe impl<'dev> Sync for OpticalFlowSessionNV<'dev> {}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'dev> Drop for OpticalFlowSessionNV<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyOpticalFlowSessionNV).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_NV_optical_flow")]
impl<'dev> OpticalFlowSessionNV<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkOpticalFlowSessionNV {
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
  pub const fn table(&self) -> &OpticalFlowSessionNVDispatchTable {
    self.table
  }
  /// [`vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `session`
  /// - `bindingPoint`
  /// - `view`: optional: true
  /// - `layout`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_optical_flow")]
  #[inline(always)]
  pub fn vkBindOpticalFlowSessionImageNV(
    &self,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkBindOpticalFlowSessionImageNV
        .unwrap_unchecked()(self.device().raw(), self.raw, bindingPoint, view, layout)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `session`
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_NV_optical_flow")]
  #[inline(always)]
  pub fn vkDestroyOpticalFlowSessionNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyOpticalFlowSessionNV
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkOpticalFlowSessionNV::NULL;
  }
}
