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
#[cfg(feature = "VK_KHR_acceleration_structure")]
#[derive(Debug, Clone)]
pub struct AccelerationStructureKHRDispatchTable {
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkDestroyAccelerationStructureKHR: Option<PFN_vkDestroyAccelerationStructureKHR>,
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl AccelerationStructureKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkDestroyAccelerationStructureKHR: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkDestroyAccelerationStructureKHR: loader(c"vkDestroyAccelerationStructureKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub struct AccelerationStructureKHR<'dev> {
  pub(crate) raw: VkAccelerationStructureKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev AccelerationStructureKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
unsafe impl<'dev> Send for AccelerationStructureKHR<'dev> {}
#[cfg(feature = "VK_KHR_acceleration_structure")]
unsafe impl<'dev> Sync for AccelerationStructureKHR<'dev> {}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl<'dev> Drop for AccelerationStructureKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyAccelerationStructureKHR).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl<'dev> AccelerationStructureKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkAccelerationStructureKHR {
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
  pub const fn table(&self) -> &AccelerationStructureKHRDispatchTable {
    self.table
  }
  /// [`vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `accelerationStructure`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkDestroyAccelerationStructureKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyAccelerationStructureKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkAccelerationStructureKHR::NULL;
  }
}
