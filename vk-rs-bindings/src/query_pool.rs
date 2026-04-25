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
pub struct QueryPoolDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyQueryPool: Option<PFN_vkDestroyQueryPool>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetQueryPoolResults: Option<PFN_vkGetQueryPoolResults>,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    pub vkResetQueryPool: Option<PFN_vkResetQueryPool>,
    #[cfg(feature = "VK_EXT_host_query_reset")]
    pub vkResetQueryPoolEXT: Option<PFN_vkResetQueryPoolEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl QueryPoolDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyQueryPool: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetQueryPoolResults: None,
        #[cfg(feature = "VK_BASE_VERSION_1_2")]
        vkResetQueryPool: None,
        #[cfg(feature = "VK_EXT_host_query_reset")]
        vkResetQueryPoolEXT: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            vkDestroyQueryPool: loader(c"vkDestroyQueryPool".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            vkGetQueryPoolResults: loader(c"vkGetQueryPoolResults".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_BASE_VERSION_1_2")]
            vkResetQueryPool: loader(c"vkResetQueryPool".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_EXT_host_query_reset")]
            vkResetQueryPoolEXT: loader(c"vkResetQueryPoolEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct QueryPool<'dev> {
    pub(crate) raw: VkQueryPool,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev QueryPoolDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for QueryPool<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for QueryPool<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for QueryPool<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyQueryPool {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> QueryPool<'dev> {
    #[inline(always)]
    pub const fn raw(&self) -> VkQueryPool {
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
    pub const fn table(&self) -> &QueryPoolDispatchTable {
        self.table
    }
    /// [`vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `queryPool`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyQueryPool(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyQueryPool.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkQueryPool::NULL;
    }
    /// [`vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `queryPool`
    /// - `firstQuery`
    /// - `queryCount`
    /// - `dataSize`
    /// - `pData`: len: dataSize
    /// - `stride`
    /// - `flags`: optional: true
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
    pub fn vkGetQueryPoolResults(
        &self,
        firstQuery: u32,
        queryCount: u32,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetQueryPoolResults.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstQuery,
                queryCount,
                dataSize,
                pData,
                stride,
                flags,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_2`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `queryPool`
    /// - `firstQuery`
    /// - `queryCount`
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    #[inline(always)]
    pub fn vkResetQueryPool(&self, firstQuery: u32, queryCount: u32) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkResetQueryPool.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstQuery,
                queryCount,
            )
        }
    }
    /// [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)
    ///
    /// Provided by:
    /// - `VK_EXT_host_query_reset`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `queryPool`
    /// - `firstQuery`
    /// - `queryCount`
    #[cfg(feature = "VK_EXT_host_query_reset")]
    #[inline(always)]
    pub fn vkResetQueryPoolEXT(&self, firstQuery: u32, queryCount: u32) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkResetQueryPoolEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstQuery,
                queryCount,
            )
        }
    }
}
