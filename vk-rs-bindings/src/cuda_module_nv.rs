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
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
#[derive(Debug, Clone)]
pub struct CudaModuleNVDispatchTable {
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub vkDestroyCudaModuleNV: Option<PFN_vkDestroyCudaModuleNV>,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub vkGetCudaModuleCacheNV: Option<PFN_vkGetCudaModuleCacheNV>,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl CudaModuleNVDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    vkDestroyCudaModuleNV: None,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    vkGetCudaModuleCacheNV: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      vkDestroyCudaModuleNV: loader(c"vkDestroyCudaModuleNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      vkGetCudaModuleCacheNV: loader(c"vkGetCudaModuleCacheNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub struct CudaModuleNV<'dev> {
  pub(crate) raw: VkCudaModuleNV,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev CudaModuleNVDispatchTable,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'dev> Send for CudaModuleNV<'dev> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'dev> Sync for CudaModuleNV<'dev> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'dev> Drop for CudaModuleNV<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyCudaModuleNV).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'dev> CudaModuleNV<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkCudaModuleNV {
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
  pub const fn table(&self) -> &CudaModuleNVDispatchTable {
    self.table
  }
  /// [`vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cuda_kernel_launch`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `module`
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  #[inline(always)]
  pub fn vkDestroyCudaModuleNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyCudaModuleNV.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkCudaModuleNV::NULL;
  }
  /// [`vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cuda_kernel_launch`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `module`
  /// - `pCacheSize`: optional: pointer required, values optional if pointer not null
  /// - `pCacheData`: optional: true, len: pCacheSize
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  #[inline(always)]
  pub fn vkGetCudaModuleCacheNV(
    &self,
    pCacheSize: *mut usize,
    pCacheData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkGetCudaModuleCacheNV.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pCacheSize,
        pCacheData,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
