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
#[cfg(feature = "VK_NV_ray_tracing")]
#[derive(Debug, Clone)]
pub struct AccelerationStructureNVDispatchTable {
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkDestroyAccelerationStructureNV: Option<PFN_vkDestroyAccelerationStructureNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkGetAccelerationStructureHandleNV: Option<PFN_vkGetAccelerationStructureHandleNV>,
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl AccelerationStructureNVDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkDestroyAccelerationStructureNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkGetAccelerationStructureHandleNV: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkDestroyAccelerationStructureNV: loader(c"vkDestroyAccelerationStructureNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkGetAccelerationStructureHandleNV: loader(c"vkGetAccelerationStructureHandleNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_NV_ray_tracing")]
pub struct AccelerationStructureNV<'dev> {
  pub(crate) raw: VkAccelerationStructureNV,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev AccelerationStructureNVDispatchTable,
}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'dev> Send for AccelerationStructureNV<'dev> {}
#[cfg(feature = "VK_NV_ray_tracing")]
unsafe impl<'dev> Sync for AccelerationStructureNV<'dev> {}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'dev> Drop for AccelerationStructureNV<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyAccelerationStructureNV).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'dev> AccelerationStructureNV<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkAccelerationStructureNV {
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
  pub const fn table(&self) -> &AccelerationStructureNVDispatchTable {
    self.table
  }
  /// [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_ray_tracing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `accelerationStructure`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_NV_ray_tracing")]
  #[inline(always)]
  pub fn vkDestroyAccelerationStructureNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyAccelerationStructureNV
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkAccelerationStructureNV::NULL;
  }
  /// [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_ray_tracing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `accelerationStructure`
  /// - `dataSize`
  /// - `pData`: len: dataSize
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
  #[cfg(feature = "VK_NV_ray_tracing")]
  #[inline(always)]
  pub fn vkGetAccelerationStructureHandleNV(
    &self,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetAccelerationStructureHandleNV
        .unwrap_unchecked()(self.device().raw(), self.raw, dataSize, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
