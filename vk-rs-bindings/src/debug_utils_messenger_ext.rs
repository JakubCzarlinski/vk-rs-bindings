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
pub struct DebugUtilsMessengerEXTDispatchTable {}
#[cfg(feature = "VK_EXT_debug_utils")]
impl DebugUtilsMessengerEXTDispatchTable {
    pub const EMPTY: Self = Self {};
    pub fn load<F>(_loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {}
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
        if let Some(destroy_fn) = self.parent.table.vkDestroyDebugUtilsMessengerEXT {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
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
}
