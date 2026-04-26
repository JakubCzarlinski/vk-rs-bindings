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
#[cfg(feature = "VK_NV_device_generated_commands")]
#[derive(Debug, Clone)]
pub struct IndirectCommandsLayoutNVDispatchTable {
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkDestroyIndirectCommandsLayoutNV: Option<PFN_vkDestroyIndirectCommandsLayoutNV>,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl IndirectCommandsLayoutNVDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkDestroyIndirectCommandsLayoutNV: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkDestroyIndirectCommandsLayoutNV: loader(c"vkDestroyIndirectCommandsLayoutNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
pub struct IndirectCommandsLayoutNV<'dev> {
  pub(crate) raw: VkIndirectCommandsLayoutNV,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev IndirectCommandsLayoutNVDispatchTable,
}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'dev> Send for IndirectCommandsLayoutNV<'dev> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
unsafe impl<'dev> Sync for IndirectCommandsLayoutNV<'dev> {}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'dev> Drop for IndirectCommandsLayoutNV<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyIndirectCommandsLayoutNV).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_NV_device_generated_commands")]
impl<'dev> IndirectCommandsLayoutNV<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkIndirectCommandsLayoutNV {
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
  pub const fn table(&self) -> &IndirectCommandsLayoutNVDispatchTable {
    self.table
  }
  /// [`vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `indirectCommandsLayout`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_NV_device_generated_commands")]
  #[inline(always)]
  pub fn vkDestroyIndirectCommandsLayoutNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyIndirectCommandsLayoutNV
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkIndirectCommandsLayoutNV::NULL;
  }
}
