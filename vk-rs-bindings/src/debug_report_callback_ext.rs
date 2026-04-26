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
pub struct DebugReportCallbackEXTDispatchTable {}
#[cfg(feature = "VK_EXT_debug_report")]
impl DebugReportCallbackEXTDispatchTable {
  pub const EMPTY: Self = Self {};
  pub fn load<F>(_loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {}
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
      (self.parent.table.vkDestroyDebugReportCallbackEXT).unwrap_unchecked()(
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
}
