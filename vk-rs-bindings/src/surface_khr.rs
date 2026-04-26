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
#[cfg(feature = "VK_KHR_surface")]
#[derive(Debug, Clone)]
pub struct SurfaceKHRDispatchTable {}
#[cfg(feature = "VK_KHR_surface")]
impl SurfaceKHRDispatchTable {
  pub const EMPTY: Self = Self {};
  pub fn load<F>(_loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {}
  }
}
#[cfg(feature = "VK_KHR_surface")]
pub struct SurfaceKHR<'dev> {
  pub(crate) raw: VkSurfaceKHR,
  pub(crate) parent: &'dev crate::instance::Instance<'dev>,
  pub(crate) table: &'dev SurfaceKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<'dev> Send for SurfaceKHR<'dev> {}
#[cfg(feature = "VK_KHR_surface")]
unsafe impl<'dev> Sync for SurfaceKHR<'dev> {}
#[cfg(feature = "VK_KHR_surface")]
impl<'dev> Drop for SurfaceKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.parent.table.vkDestroySurfaceKHR).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_KHR_surface")]
impl<'dev> SurfaceKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkSurfaceKHR {
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
  pub const fn table(&self) -> &SurfaceKHRDispatchTable {
    self.table
  }
}
