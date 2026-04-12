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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct DebugReportCallbackEXTDispatchTable {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DebugReportCallbackEXTDispatchTable {
    pub const EMPTY: Self = Self {};
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        table
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
pub struct DebugReportCallbackEXT<'dev> {
    pub(crate) raw: VkDebugReportCallbackEXT,
    pub(crate) parent: &'dev crate::instance::Instance<'dev>,
    pub(crate) table: &'dev DebugReportCallbackEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'dev> Drop for DebugReportCallbackEXT<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.parent.table.vkDestroyDebugReportCallbackEXT {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_EXT_debug_report")]
impl<'dev> DebugReportCallbackEXT<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDebugReportCallbackEXT {
        self.raw
    }
    #[inline]
    pub fn parent(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent
    }
    #[inline]
    pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent
    }
    #[inline]
    pub fn table(&self) -> &DebugReportCallbackEXTDispatchTable {
        self.table
    }
}
