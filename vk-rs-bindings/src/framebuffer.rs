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
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct FramebufferDispatchTable {
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkDestroyFramebuffer: Option<PFN_vkDestroyFramebuffer>,
  #[cfg(feature = "VK_QCOM_tile_properties")]
  pub vkGetFramebufferTilePropertiesQCOM: Option<PFN_vkGetFramebufferTilePropertiesQCOM>,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl FramebufferDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkDestroyFramebuffer: None,
    #[cfg(feature = "VK_QCOM_tile_properties")]
    vkGetFramebufferTilePropertiesQCOM: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkDestroyFramebuffer: loader(c"vkDestroyFramebuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_properties")]
      vkGetFramebufferTilePropertiesQCOM: loader(c"vkGetFramebufferTilePropertiesQCOM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub struct Framebuffer<'dev> {
  pub(crate) raw: VkFramebuffer,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev FramebufferDispatchTable,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'dev> Send for Framebuffer<'dev> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
unsafe impl<'dev> Sync for Framebuffer<'dev> {}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> Drop for Framebuffer<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    if let Some(destroy_fn) = self.table.vkDestroyFramebuffer {
      unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
    }
  }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> Framebuffer<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkFramebuffer {
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
  pub const fn table(&self) -> &FramebufferDispatchTable {
    self.table
  }
  /// [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `framebuffer`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline(always)]
  pub fn vkDestroyFramebuffer(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyFramebuffer.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkFramebuffer::NULL;
  }
  /// [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)
  ///
  /// Provided by:
  /// - `VK_QCOM_tile_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `framebuffer`
  /// - `pPropertiesCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertiesCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_QCOM_tile_properties")]
  #[inline(always)]
  pub fn vkGetFramebufferTilePropertiesQCOM(
    &self,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetFramebufferTilePropertiesQCOM
        .unwrap_unchecked()(self.device().raw(), self.raw, pPropertiesCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
}
