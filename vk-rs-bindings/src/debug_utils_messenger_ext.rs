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
#[cfg(feature = "VK_EXT_debug_utils")]
#[derive(Debug, Clone)]
pub struct DebugUtilsMessengerEXTDispatchTable {
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkDestroyDebugUtilsMessengerEXT: Option<PFN_vkDestroyDebugUtilsMessengerEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkSubmitDebugUtilsMessageEXT: Option<PFN_vkSubmitDebugUtilsMessageEXT>,
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl DebugUtilsMessengerEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkDestroyDebugUtilsMessengerEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkSubmitDebugUtilsMessageEXT: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkDestroyDebugUtilsMessengerEXT: loader(c"vkDestroyDebugUtilsMessengerEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkSubmitDebugUtilsMessageEXT: loader(c"vkSubmitDebugUtilsMessageEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_debug_utils")]
pub struct DebugUtilsMessengerEXT<'dev> {
  pub(crate) raw: VkDebugUtilsMessengerEXT,
  pub(crate) parent: &'dev crate::instance::Instance<'dev>,
  pub(crate) table: &'dev DebugUtilsMessengerEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<'dev> Send for DebugUtilsMessengerEXT<'dev> {}
#[cfg(feature = "VK_EXT_debug_utils")]
unsafe impl<'dev> Sync for DebugUtilsMessengerEXT<'dev> {}
#[cfg(feature = "VK_EXT_debug_utils")]
impl<'dev> Drop for DebugUtilsMessengerEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyDebugUtilsMessengerEXT).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_EXT_debug_utils")]
impl<'dev> DebugUtilsMessengerEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDebugUtilsMessengerEXT {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn table(&self) -> &DebugUtilsMessengerEXTDispatchTable {
    self.table
  }
  /// [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_utils`
  ///
  ///
  /// # Parameters
  /// - `instance`
  /// - `messenger`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_debug_utils")]
  #[inline(always)]
  pub fn vkDestroyDebugUtilsMessengerEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyDebugUtilsMessengerEXT
        .unwrap_unchecked()(self.parent().raw(), self.raw, pAllocator)
    }
    self.raw = VkDebugUtilsMessengerEXT::NULL;
  }
  /// [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_utils`
  ///
  ///
  /// # Parameters
  /// - `instance`
  /// - `messageSeverity`
  /// - `messageTypes`
  /// - `pCallbackData`
  #[cfg(feature = "VK_EXT_debug_utils")]
  #[inline(always)]
  pub fn vkSubmitDebugUtilsMessageEXT(
    &self,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: &VkDebugUtilsMessengerCallbackDataEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkSubmitDebugUtilsMessageEXT.unwrap_unchecked()(
        self.instance().raw(),
        messageSeverity,
        messageTypes,
        pCallbackData,
      )
    }
  }
}
