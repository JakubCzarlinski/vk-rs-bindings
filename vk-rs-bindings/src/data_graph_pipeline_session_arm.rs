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
#[cfg(feature = "VK_ARM_data_graph")]
#[derive(Debug, Clone)]
pub struct DataGraphPipelineSessionARMDispatchTable {
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkDestroyDataGraphPipelineSessionARM: Option<PFN_vkDestroyDataGraphPipelineSessionARM>,
}
#[cfg(feature = "VK_ARM_data_graph")]
impl DataGraphPipelineSessionARMDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_ARM_data_graph")]
    vkDestroyDataGraphPipelineSessionARM: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_ARM_data_graph")]
      vkDestroyDataGraphPipelineSessionARM: loader(
        c"vkDestroyDataGraphPipelineSessionARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_ARM_data_graph")]
pub struct DataGraphPipelineSessionARM<'dev> {
  pub(crate) raw: VkDataGraphPipelineSessionARM,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev DataGraphPipelineSessionARMDispatchTable,
}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'dev> Send for DataGraphPipelineSessionARM<'dev> {}
#[cfg(feature = "VK_ARM_data_graph")]
unsafe impl<'dev> Sync for DataGraphPipelineSessionARM<'dev> {}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'dev> Drop for DataGraphPipelineSessionARM<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyDataGraphPipelineSessionARM).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'dev> DataGraphPipelineSessionARM<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDataGraphPipelineSessionARM {
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
  pub const fn table(&self) -> &DataGraphPipelineSessionARMDispatchTable {
    self.table
  }
  /// [`vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `session`
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkDestroyDataGraphPipelineSessionARM(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyDataGraphPipelineSessionARM
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkDataGraphPipelineSessionARM::NULL;
  }
}
