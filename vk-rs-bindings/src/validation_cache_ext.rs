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
#[cfg(feature = "VK_EXT_validation_cache")]
#[derive(Debug, Clone)]
pub struct ValidationCacheEXTDispatchTable {
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkDestroyValidationCacheEXT: Option<PFN_vkDestroyValidationCacheEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkGetValidationCacheDataEXT: Option<PFN_vkGetValidationCacheDataEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkMergeValidationCachesEXT: Option<PFN_vkMergeValidationCachesEXT>,
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl ValidationCacheEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_validation_cache")]
    vkDestroyValidationCacheEXT: None,
    #[cfg(feature = "VK_EXT_validation_cache")]
    vkGetValidationCacheDataEXT: None,
    #[cfg(feature = "VK_EXT_validation_cache")]
    vkMergeValidationCachesEXT: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_validation_cache")]
      vkDestroyValidationCacheEXT: loader(c"vkDestroyValidationCacheEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_validation_cache")]
      vkGetValidationCacheDataEXT: loader(c"vkGetValidationCacheDataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_validation_cache")]
      vkMergeValidationCachesEXT: loader(c"vkMergeValidationCachesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
pub struct ValidationCacheEXT<'dev> {
  pub(crate) raw: VkValidationCacheEXT,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev ValidationCacheEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'dev> Send for ValidationCacheEXT<'dev> {}
#[cfg(feature = "VK_EXT_validation_cache")]
unsafe impl<'dev> Sync for ValidationCacheEXT<'dev> {}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'dev> Drop for ValidationCacheEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    if let Some(destroy_fn) = self.table.vkDestroyValidationCacheEXT {
      unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
    }
  }
}
#[cfg(feature = "VK_EXT_validation_cache")]
impl<'dev> ValidationCacheEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkValidationCacheEXT {
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
  pub const fn table(&self) -> &ValidationCacheEXTDispatchTable {
    self.table
  }
  /// [`vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_validation_cache`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `validationCache`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_validation_cache")]
  #[inline(always)]
  pub fn vkDestroyValidationCacheEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyValidationCacheEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkValidationCacheEXT::NULL;
  }
  /// [`vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_validation_cache`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `validationCache`
  /// - `pDataSize`: optional: pointer required, values optional if pointer not null
  /// - `pData`: optional: true, len: pDataSize
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_validation_cache")]
  #[inline(always)]
  pub fn vkGetValidationCacheDataEXT(
    &self,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkGetValidationCacheDataEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pDataSize,
        pData,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_validation_cache`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `dstCache`
  /// - `srcCacheCount`
  /// - `pSrcCaches`: len: srcCacheCount
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
  #[cfg(feature = "VK_EXT_validation_cache")]
  #[inline(always)]
  pub fn vkMergeValidationCachesEXT(
    &self,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkMergeValidationCachesEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        srcCacheCount,
        pSrcCaches,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
