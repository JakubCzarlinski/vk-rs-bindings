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
pub struct FenceDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyFence: Option<PFN_vkDestroyFence>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetFenceStatus: Option<PFN_vkGetFenceStatus>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl FenceDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyFence: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetFenceStatus: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroyFence =
                loader(c"vkDestroyFence".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetFenceStatus =
                loader(c"vkGetFenceStatus".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Fence<'dev> {
    pub(crate) raw: VkFence,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev FenceDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for Fence<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for Fence<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for Fence<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyFence {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Fence<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkFence {
        self.raw
    }
    #[inline]
    pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub const fn table(&self) -> &FenceDispatchTable {
        self.table
    }
    /// [`vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `fence`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyFence(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyFence.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkFence::NULL;
    }
    /// [`vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `fence`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_NOT_READY`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetFenceStatus(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetFenceStatus.unwrap_unchecked()(self.device().raw(), self.raw)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
}
