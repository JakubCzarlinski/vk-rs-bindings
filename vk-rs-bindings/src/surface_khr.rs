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
pub struct SurfaceKHRDispatchTable {
  #[cfg(feature = "VK_KHR_surface")]
  pub vkDestroySurfaceKHR: Option<PFN_vkDestroySurfaceKHR>,
}
#[cfg(feature = "VK_KHR_surface")]
impl SurfaceKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_KHR_surface")]
    vkDestroySurfaceKHR: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_KHR_surface")]
      vkDestroySurfaceKHR: loader(c"vkDestroySurfaceKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
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
      (self.table.vkDestroySurfaceKHR).unwrap_unchecked()(
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
  /// [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_surface`
  ///
  ///
  /// # Parameters
  /// - `instance`
  /// - `surface`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_KHR_surface")]
  #[inline(always)]
  pub fn vkDestroySurfaceKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroySurfaceKHR.unwrap_unchecked()(self.parent().raw(), self.raw, pAllocator)
    }
    self.raw = VkSurfaceKHR::NULL;
  }
}
