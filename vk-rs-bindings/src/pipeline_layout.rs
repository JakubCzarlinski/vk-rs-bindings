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
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct PipelineLayoutDispatchTable {
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkDestroyPipelineLayout: Option<PFN_vkDestroyPipelineLayout>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl PipelineLayoutDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkDestroyPipelineLayout: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkDestroyPipelineLayout: loader(c"vkDestroyPipelineLayout".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct PipelineLayout<'dev> {
  pub(crate) raw: VkPipelineLayout,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev PipelineLayoutDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for PipelineLayout<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for PipelineLayout<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for PipelineLayout<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyPipelineLayout).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> PipelineLayout<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkPipelineLayout {
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
  pub const fn table(&self) -> &PipelineLayoutDispatchTable {
    self.table
  }
  /// [`vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pipelineLayout`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkDestroyPipelineLayout(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyPipelineLayout.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkPipelineLayout::NULL;
  }
}
