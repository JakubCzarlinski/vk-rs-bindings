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
#[cfg(feature = "VK_AMD_gpa_interface")]
#[derive(Debug, Clone)]
pub struct GpaSessionAMDDispatchTable {
  #[cfg(feature = "VK_AMD_gpa_interface")]
  pub vkDestroyGpaSessionAMD: Option<PFN_vkDestroyGpaSessionAMD>,
  #[cfg(feature = "VK_AMD_gpa_interface")]
  pub vkGetGpaSessionResultsAMD: Option<PFN_vkGetGpaSessionResultsAMD>,
  #[cfg(feature = "VK_AMD_gpa_interface")]
  pub vkGetGpaSessionStatusAMD: Option<PFN_vkGetGpaSessionStatusAMD>,
  #[cfg(feature = "VK_AMD_gpa_interface")]
  pub vkResetGpaSessionAMD: Option<PFN_vkResetGpaSessionAMD>,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl GpaSessionAMDDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_AMD_gpa_interface")]
    vkDestroyGpaSessionAMD: None,
    #[cfg(feature = "VK_AMD_gpa_interface")]
    vkGetGpaSessionResultsAMD: None,
    #[cfg(feature = "VK_AMD_gpa_interface")]
    vkGetGpaSessionStatusAMD: None,
    #[cfg(feature = "VK_AMD_gpa_interface")]
    vkResetGpaSessionAMD: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_AMD_gpa_interface")]
      vkDestroyGpaSessionAMD: loader(c"vkDestroyGpaSessionAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_gpa_interface")]
      vkGetGpaSessionResultsAMD: loader(c"vkGetGpaSessionResultsAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_gpa_interface")]
      vkGetGpaSessionStatusAMD: loader(c"vkGetGpaSessionStatusAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_gpa_interface")]
      vkResetGpaSessionAMD: loader(c"vkResetGpaSessionAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_AMD_gpa_interface")]
pub struct GpaSessionAMD<'dev> {
  pub(crate) raw: VkGpaSessionAMD,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev GpaSessionAMDDispatchTable,
}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'dev> Send for GpaSessionAMD<'dev> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
unsafe impl<'dev> Sync for GpaSessionAMD<'dev> {}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'dev> Drop for GpaSessionAMD<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyGpaSessionAMD).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_AMD_gpa_interface")]
impl<'dev> GpaSessionAMD<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkGpaSessionAMD {
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
  pub const fn table(&self) -> &GpaSessionAMDDispatchTable {
    self.table
  }
  /// [`vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html)
  ///
  /// Provided by:
  /// - `VK_AMD_gpa_interface`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `gpaSession`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_AMD_gpa_interface")]
  #[inline(always)]
  pub fn vkDestroyGpaSessionAMD(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyGpaSessionAMD.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkGpaSessionAMD::NULL;
  }
  /// [`vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html)
  ///
  /// Provided by:
  /// - `VK_AMD_gpa_interface`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `gpaSession`
  /// - `sampleID`
  /// - `pSizeInBytes`
  /// - `pData`: optional: true, len: pSizeInBytes
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_AMD_gpa_interface")]
  #[inline(always)]
  pub fn vkGetGpaSessionResultsAMD(
    &self,
    sampleID: u32,
    pSizeInBytes: &mut usize,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkGetGpaSessionResultsAMD.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        sampleID,
        pSizeInBytes,
        pData,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html)
  ///
  /// Provided by:
  /// - `VK_AMD_gpa_interface`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `gpaSession`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_AMD_gpa_interface")]
  #[inline(always)]
  pub fn vkGetGpaSessionStatusAMD(&self) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkGetGpaSessionStatusAMD.unwrap_unchecked()(self.device().raw(), self.raw)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html)
  ///
  /// Provided by:
  /// - `VK_AMD_gpa_interface`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `gpaSession`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_AMD_gpa_interface")]
  #[inline(always)]
  pub fn vkResetGpaSessionAMD(&self) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkResetGpaSessionAMD.unwrap_unchecked()(self.device().raw(), self.raw)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
