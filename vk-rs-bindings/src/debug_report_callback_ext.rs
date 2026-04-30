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
#[cfg(feature = "VK_EXT_debug_report")]
#[derive(Debug, Clone)]
pub struct DebugReportCallbackEXTDispatchTable {
  #[cfg(feature = "VK_EXT_debug_report")]
  pub vkDebugReportMessageEXT: Option<PFN_vkDebugReportMessageEXT>,
  #[cfg(feature = "VK_EXT_debug_report")]
  pub vkDestroyDebugReportCallbackEXT: Option<PFN_vkDestroyDebugReportCallbackEXT>,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl DebugReportCallbackEXTDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_EXT_debug_report")]
    vkDebugReportMessageEXT: None,
    #[cfg(feature = "VK_EXT_debug_report")]
    vkDestroyDebugReportCallbackEXT: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_EXT_debug_report")]
      vkDebugReportMessageEXT: loader(c"vkDebugReportMessageEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_report")]
      vkDestroyDebugReportCallbackEXT: loader(c"vkDestroyDebugReportCallbackEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallbackEXT<'dev> {
  pub(crate) raw: VkDebugReportCallbackEXT,
  pub(crate) parent: &'dev crate::instance::Instance<'dev>,
  pub(crate) table: &'dev DebugReportCallbackEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'dev> Send for DebugReportCallbackEXT<'dev> {}
#[cfg(feature = "VK_EXT_debug_report")]
unsafe impl<'dev> Sync for DebugReportCallbackEXT<'dev> {}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'dev> Drop for DebugReportCallbackEXT<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyDebugReportCallbackEXT).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'dev> DebugReportCallbackEXT<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDebugReportCallbackEXT {
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
  pub const fn table(&self) -> &DebugReportCallbackEXTDispatchTable {
    self.table
  }
  /// [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_report`
  ///
  ///
  /// # Parameters
  /// - `instance`
  /// - `flags`
  /// - `objectType`
  /// - `object`: object_type: objectType
  /// - `location`
  /// - `messageCode`
  /// - `pLayerPrefix`: len: null-terminated
  /// - `pMessage`: len: null-terminated
  #[cfg(feature = "VK_EXT_debug_report")]
  #[inline(always)]
  pub fn vkDebugReportMessageEXT(
    &self,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    messageCode: i32,
    pLayerPrefix: *const core::ffi::c_char,
    pMessage: *const core::ffi::c_char,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDebugReportMessageEXT.unwrap_unchecked()(
        self.instance().raw(),
        flags,
        objectType,
        object,
        location,
        messageCode,
        pLayerPrefix,
        pMessage,
      )
    }
  }
  /// [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_report`
  ///
  ///
  /// # Parameters
  /// - `instance`
  /// - `callback`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_EXT_debug_report")]
  #[inline(always)]
  pub fn vkDestroyDebugReportCallbackEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyDebugReportCallbackEXT
        .unwrap_unchecked()(self.parent().raw(), self.raw, pAllocator)
    }
    self.raw = VkDebugReportCallbackEXT::NULL;
  }
}
