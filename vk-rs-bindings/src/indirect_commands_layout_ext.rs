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
#[cfg(feature = "VK_EXT_device_generated_commands")]
#[derive(Debug, Clone)]
pub struct IndirectCommandsLayoutEXTDispatchTable {
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkDestroyIndirectCommandsLayoutEXT: Option<PFN_vkDestroyIndirectCommandsLayoutEXT>,
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
impl IndirectCommandsLayoutEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkDestroyIndirectCommandsLayoutEXT: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkDestroyIndirectCommandsLayoutEXT: loader(c"vkDestroyIndirectCommandsLayoutEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub struct IndirectCommandsLayoutEXT<'dev> {
  pub(crate) raw: VkIndirectCommandsLayoutEXT,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev IndirectCommandsLayoutEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
unsafe impl<'dev> Send for IndirectCommandsLayoutEXT<'dev> {}
#[cfg(feature = "VK_EXT_device_generated_commands")]
unsafe impl<'dev> Sync for IndirectCommandsLayoutEXT<'dev> {}
#[cfg(feature = "VK_EXT_device_generated_commands")]
impl<'dev> Drop for IndirectCommandsLayoutEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    if let Some(destroy_fn) = self.table.vkDestroyIndirectCommandsLayoutEXT {
      unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
    }
  }
}
#[cfg(feature = "VK_EXT_device_generated_commands")]
impl<'dev> IndirectCommandsLayoutEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkIndirectCommandsLayoutEXT {
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
  pub const fn table(&self) -> &IndirectCommandsLayoutEXTDispatchTable {
    self.table
  }
  /// [`vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `indirectCommandsLayout`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  #[inline(always)]
  pub fn vkDestroyIndirectCommandsLayoutEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyIndirectCommandsLayoutEXT
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkIndirectCommandsLayoutEXT::NULL;
  }
}
