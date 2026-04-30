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
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[derive(Debug, Clone)]
pub struct PrivateDataSlotDispatchTable {
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkDestroyPrivateDataSlot: Option<PFN_vkDestroyPrivateDataSlot>,
  #[cfg(feature = "VK_EXT_private_data")]
  pub vkDestroyPrivateDataSlotEXT: Option<PFN_vkDestroyPrivateDataSlotEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl PrivateDataSlotDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkDestroyPrivateDataSlot: None,
    #[cfg(feature = "VK_EXT_private_data")]
    vkDestroyPrivateDataSlotEXT: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkDestroyPrivateDataSlot: loader(c"vkDestroyPrivateDataSlot".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_private_data")]
      vkDestroyPrivateDataSlotEXT: loader(c"vkDestroyPrivateDataSlotEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub struct PrivateDataSlot<'dev> {
  pub(crate) raw: VkPrivateDataSlot,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev PrivateDataSlotDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'dev> Send for PrivateDataSlot<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
unsafe impl<'dev> Sync for PrivateDataSlot<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'dev> Drop for PrivateDataSlot<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyPrivateDataSlot).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_3")]
impl<'dev> PrivateDataSlot<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkPrivateDataSlot {
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
  pub const fn table(&self) -> &PrivateDataSlotDispatchTable {
    self.table
  }
  /// [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `privateDataSlot`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkDestroyPrivateDataSlot(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyPrivateDataSlot.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkPrivateDataSlot::NULL;
  }
  /// [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)
  ///
  /// Provided by:
  /// - `VK_EXT_private_data`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `privateDataSlot`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_private_data")]
  #[inline(always)]
  pub fn vkDestroyPrivateDataSlotEXT(&self, pAllocator: *const VkAllocationCallbacks) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyPrivateDataSlotEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
  }
}
