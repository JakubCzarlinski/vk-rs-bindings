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
#[cfg(feature = "VK_EXT_private_data")]
#[derive(Debug, Clone)]
pub struct PrivateDataSlotEXTDispatchTable {
  #[cfg(feature = "VK_EXT_private_data")]
  pub vkDestroyPrivateDataSlotEXT: Option<PFN_vkDestroyPrivateDataSlotEXT>,
}
#[cfg(feature = "VK_EXT_private_data")]
impl PrivateDataSlotEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_private_data")]
    vkDestroyPrivateDataSlotEXT: None,
  };
  #[inline]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_private_data")]
      vkDestroyPrivateDataSlotEXT: loader(c"vkDestroyPrivateDataSlotEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_private_data")]
pub struct PrivateDataSlotEXT<'dev> {
  pub(crate) raw: VkPrivateDataSlotEXT,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev PrivateDataSlotEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_private_data")]
unsafe impl<'dev> Send for PrivateDataSlotEXT<'dev> {}
#[cfg(feature = "VK_EXT_private_data")]
unsafe impl<'dev> Sync for PrivateDataSlotEXT<'dev> {}
#[cfg(feature = "VK_EXT_private_data")]
impl<'dev> Drop for PrivateDataSlotEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyPrivateDataSlotEXT).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_EXT_private_data")]
impl<'dev> PrivateDataSlotEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkPrivateDataSlotEXT {
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
  pub const fn table(&self) -> &PrivateDataSlotEXTDispatchTable {
    self.table
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
  pub fn vkDestroyPrivateDataSlotEXT(&mut self, pAllocator: *const VkAllocationCallbacks<'_>) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyPrivateDataSlotEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkPrivateDataSlotEXT::NULL;
  }
}
