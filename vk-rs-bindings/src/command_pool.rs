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
pub struct CommandPoolDispatchTable {
  #[cfg(feature = "VKSC_VERSION_1_0")]
  pub vkGetCommandPoolMemoryConsumption: Option<PFN_vkGetCommandPoolMemoryConsumption>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkAllocateCommandBuffers: Option<PFN_vkAllocateCommandBuffers>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkDestroyCommandPool: Option<PFN_vkDestroyCommandPool>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkFreeCommandBuffers: Option<PFN_vkFreeCommandBuffers>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkResetCommandPool: Option<PFN_vkResetCommandPool>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkTrimCommandPool: Option<PFN_vkTrimCommandPool>,
  #[cfg(feature = "VK_KHR_maintenance1")]
  pub vkTrimCommandPoolKHR: Option<PFN_vkTrimCommandPoolKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl CommandPoolDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VKSC_VERSION_1_0")]
    vkGetCommandPoolMemoryConsumption: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkAllocateCommandBuffers: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkDestroyCommandPool: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkFreeCommandBuffers: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkResetCommandPool: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkTrimCommandPool: None,
    #[cfg(feature = "VK_KHR_maintenance1")]
    vkTrimCommandPoolKHR: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VKSC_VERSION_1_0")]
      vkGetCommandPoolMemoryConsumption: loader(c"vkGetCommandPoolMemoryConsumption".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkAllocateCommandBuffers: loader(c"vkAllocateCommandBuffers".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkDestroyCommandPool: loader(c"vkDestroyCommandPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkFreeCommandBuffers: loader(c"vkFreeCommandBuffers".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkResetCommandPool: loader(c"vkResetCommandPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkTrimCommandPool: loader(c"vkTrimCommandPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance1")]
      vkTrimCommandPoolKHR: loader(c"vkTrimCommandPoolKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct CommandPool<'dev> {
  pub(crate) raw: VkCommandPool,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev CommandPoolDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for CommandPool<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for CommandPool<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for CommandPool<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.table.vkDestroyCommandPool).unwrap_unchecked()(
        self.parent.raw(),
        self.raw,
        core::ptr::null(),
      )
    };
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> CommandPool<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkCommandPool {
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
  pub const fn table(&self) -> &CommandPoolDispatchTable {
    self.table
  }
  /// [`vkGetCommandPoolMemoryConsumption`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCommandPoolMemoryConsumption.html)
  ///
  /// Provided by:
  /// - `VKSC_VERSION_1_0`
  ///
  /// - **Export Scopes:** VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `commandPool`
  /// - `commandBuffer`: optional: true
  /// - `pConsumption`
  #[cfg(feature = "VKSC_VERSION_1_0")]
  #[inline(always)]
  pub fn vkGetCommandPoolMemoryConsumption(
    &self,
    commandBuffer: VkCommandBuffer,
    pConsumption: *mut VkCommandPoolMemoryConsumption,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetCommandPoolMemoryConsumption
        .unwrap_unchecked()(self.device().raw(), self.raw, commandBuffer, pConsumption)
    }
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkAllocateCommandBuffers<'pool>(
    &'pool self,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
  ) -> Result<alloc::boxed::Box<[crate::command_buffer::CommandBuffer<'pool>]>, VkResult> {
    let count = unsafe { (*pAllocateInfo).commandBufferCount };
    let mut raw_buffers = alloc::boxed::Box::<[VkCommandBuffer]>::new_uninit_slice(count as usize);
    let fp = unsafe { self.table.vkAllocateCommandBuffers.unwrap_unchecked() };
    let r = unsafe {
      fp(
        self.device().raw,
        pAllocateInfo,
        raw_buffers.as_mut_ptr().cast(),
      )
    };
    if r < VkResult::VK_SUCCESS {
      return Err(r);
    }
    let raw_buffers = unsafe { raw_buffers.assume_init() };
    Ok(
      raw_buffers
        .into_iter()
        .map(|raw| crate::command_buffer::CommandBuffer {
          raw,
          parent: self,
          table: &self.device().command_buffer_table,
        })
        .collect(),
    )
  }
  /// [`vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `commandPool`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkDestroyCommandPool(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkDestroyCommandPool.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pAllocator,
      )
    }
    self.raw = VkCommandPool::NULL;
  }
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkFreeCommandBuffers(
    &self,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
  ) {
    let fp = unsafe { self.table.vkFreeCommandBuffers.unwrap_unchecked() };
    unsafe {
      fp(
        self.device().raw,
        self.raw,
        commandBufferCount,
        pCommandBuffers,
      )
    }
  }
  /// [`vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `commandPool`
  /// - `flags`: optional: true
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkResetCommandPool(&self, flags: VkCommandPoolResetFlags) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkResetCommandPool.unwrap_unchecked()(self.device().raw(), self.raw, flags)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `commandPool`
  /// - `flags`: optional: true
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkTrimCommandPool(&self, flags: VkCommandPoolTrimFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkTrimCommandPool.unwrap_unchecked()(self.device().raw(), self.raw, flags)
    }
  }
  /// [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance1`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `commandPool`
  /// - `flags`: optional: true
  #[cfg(feature = "VK_KHR_maintenance1")]
  #[inline(always)]
  pub fn vkTrimCommandPoolKHR(&self, flags: VkCommandPoolTrimFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkTrimCommandPoolKHR.unwrap_unchecked()(self.device().raw(), self.raw, flags)
    }
  }
}
