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
#[cfg(feature = "VK_ARM_tensors")]
#[derive(Debug, Clone)]
pub struct TensorViewARMDispatchTable {
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkDestroyTensorViewARM: Option<PFN_vkDestroyTensorViewARM>,
}
#[cfg(feature = "VK_ARM_tensors")]
impl TensorViewARMDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_ARM_tensors")]
    vkDestroyTensorViewARM: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_ARM_tensors")]
      vkDestroyTensorViewARM: loader(c"vkDestroyTensorViewARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_ARM_tensors")]
pub struct TensorViewARM<'dev> {
  pub(crate) raw: VkTensorViewARM,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev TensorViewARMDispatchTable,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'dev> Send for TensorViewARM<'dev> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'dev> Sync for TensorViewARM<'dev> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'dev> Drop for TensorViewARM<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyTensorViewARM).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'dev> TensorViewARM<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkTensorViewARM {
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
  pub const fn table(&self) -> &TensorViewARMDispatchTable {
    self.table
  }
  /// [`vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `tensorView`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkDestroyTensorViewARM(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyTensorViewARM.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkTensorViewARM::NULL;
  }
}
