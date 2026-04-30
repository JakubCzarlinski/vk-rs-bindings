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
#[cfg(feature = "VK_EXT_opacity_micromap")]
#[derive(Debug, Clone)]
pub struct MicromapEXTDispatchTable {
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkDestroyMicromapEXT: Option<PFN_vkDestroyMicromapEXT>,
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
impl MicromapEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkDestroyMicromapEXT: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkDestroyMicromapEXT: loader(c"vkDestroyMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub struct MicromapEXT<'dev> {
  pub(crate) raw: VkMicromapEXT,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev MicromapEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
unsafe impl<'dev> Send for MicromapEXT<'dev> {}
#[cfg(feature = "VK_EXT_opacity_micromap")]
unsafe impl<'dev> Sync for MicromapEXT<'dev> {}
#[cfg(feature = "VK_EXT_opacity_micromap")]
impl<'dev> Drop for MicromapEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyMicromapEXT).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
impl<'dev> MicromapEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkMicromapEXT {
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
  pub const fn table(&self) -> &MicromapEXTDispatchTable {
    self.table
  }
  /// [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `micromap`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkDestroyMicromapEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyMicromapEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkMicromapEXT::NULL;
  }
}
