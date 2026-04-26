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
#[cfg(feature = "VK_KHR_video_queue")]
#[derive(Debug, Clone)]
pub struct VideoSessionParametersKHRDispatchTable {
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkDestroyVideoSessionParametersKHR: Option<PFN_vkDestroyVideoSessionParametersKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkUpdateVideoSessionParametersKHR: Option<PFN_vkUpdateVideoSessionParametersKHR>,
}
#[cfg(feature = "VK_KHR_video_queue")]
impl VideoSessionParametersKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_KHR_video_queue")]
    vkDestroyVideoSessionParametersKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkUpdateVideoSessionParametersKHR: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_KHR_video_queue")]
      vkDestroyVideoSessionParametersKHR: loader(c"vkDestroyVideoSessionParametersKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkUpdateVideoSessionParametersKHR: loader(c"vkUpdateVideoSessionParametersKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_KHR_video_queue")]
pub struct VideoSessionParametersKHR<'dev> {
  pub(crate) raw: VkVideoSessionParametersKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev VideoSessionParametersKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_video_queue")]
unsafe impl<'dev> Send for VideoSessionParametersKHR<'dev> {}
#[cfg(feature = "VK_KHR_video_queue")]
unsafe impl<'dev> Sync for VideoSessionParametersKHR<'dev> {}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> Drop for VideoSessionParametersKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyVideoSessionParametersKHR).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> VideoSessionParametersKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkVideoSessionParametersKHR {
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
  pub const fn table(&self) -> &VideoSessionParametersKHRDispatchTable {
    self.table
  }
  /// [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `videoSessionParameters`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline(always)]
  pub fn vkDestroyVideoSessionParametersKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyVideoSessionParametersKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkVideoSessionParametersKHR::NULL;
  }
  /// [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `videoSessionParameters`
  /// - `pUpdateInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline(always)]
  pub fn vkUpdateVideoSessionParametersKHR(
    &self,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkUpdateVideoSessionParametersKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pUpdateInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
