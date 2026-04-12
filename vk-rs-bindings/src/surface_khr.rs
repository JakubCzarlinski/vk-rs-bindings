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
pub struct SurfaceKHRDispatchTable {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl SurfaceKHRDispatchTable {
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
#[cfg(feature = "VK_KHR_surface")]
pub struct SurfaceKHR<'dev> {
    pub(crate) raw: VkSurfaceKHR,
    pub(crate) parent: &'dev crate::instance::Instance<'dev>,
    pub(crate) table: &'dev SurfaceKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_surface")]
impl<'dev> Drop for SurfaceKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.parent.table.vkDestroySurfaceKHR {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_surface")]
impl<'dev> SurfaceKHR<'dev> {
    #[inline]
    pub fn raw(&self) -> VkSurfaceKHR {
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
    pub fn table(&self) -> &SurfaceKHRDispatchTable {
        self.table
    }
}
