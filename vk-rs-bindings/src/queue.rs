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
pub struct QueueDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkQueueBindSparse: Option<PFN_vkQueueBindSparse>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkQueueSubmit: Option<PFN_vkQueueSubmit>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkQueueWaitIdle: Option<PFN_vkQueueWaitIdle>,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    pub vkQueueSubmit2: Option<PFN_vkQueueSubmit2>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkQueueBeginDebugUtilsLabelEXT: Option<PFN_vkQueueBeginDebugUtilsLabelEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkQueueEndDebugUtilsLabelEXT: Option<PFN_vkQueueEndDebugUtilsLabelEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkQueueInsertDebugUtilsLabelEXT: Option<PFN_vkQueueInsertDebugUtilsLabelEXT>,
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub vkQueueSetPerformanceConfigurationINTEL:
        Option<PFN_vkQueueSetPerformanceConfigurationINTEL>,
    #[cfg(feature = "VK_KHR_swapchain")]
    pub vkQueuePresentKHR: Option<PFN_vkQueuePresentKHR>,
    #[cfg(feature = "VK_KHR_synchronization2")]
    pub vkQueueSubmit2KHR: Option<PFN_vkQueueSubmit2KHR>,
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    pub vkGetQueueCheckpointData2NV: Option<PFN_vkGetQueueCheckpointData2NV>,
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    pub vkGetQueueCheckpointDataNV: Option<PFN_vkGetQueueCheckpointDataNV>,
    #[cfg(feature = "VK_NV_low_latency2")]
    pub vkQueueNotifyOutOfBandNV: Option<PFN_vkQueueNotifyOutOfBandNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl QueueDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkQueueBindSparse: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkQueueSubmit: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkQueueWaitIdle: None,
        #[cfg(feature = "VK_BASE_VERSION_1_3")]
        vkQueueSubmit2: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkQueueBeginDebugUtilsLabelEXT: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkQueueEndDebugUtilsLabelEXT: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkQueueInsertDebugUtilsLabelEXT: None,
        #[cfg(feature = "VK_INTEL_performance_query")]
        vkQueueSetPerformanceConfigurationINTEL: None,
        #[cfg(feature = "VK_KHR_swapchain")]
        vkQueuePresentKHR: None,
        #[cfg(feature = "VK_KHR_synchronization2")]
        vkQueueSubmit2KHR: None,
        #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
        vkGetQueueCheckpointData2NV: None,
        #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
        vkGetQueueCheckpointDataNV: None,
        #[cfg(feature = "VK_NV_low_latency2")]
        vkQueueNotifyOutOfBandNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkQueueBindSparse =
                loader(c"vkQueueBindSparse".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkQueueSubmit =
                loader(c"vkQueueSubmit".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkQueueWaitIdle =
                loader(c"vkQueueWaitIdle".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_3")]
        {
            table.vkQueueSubmit2 =
                loader(c"vkQueueSubmit2".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkQueueBeginDebugUtilsLabelEXT =
                loader(c"vkQueueBeginDebugUtilsLabelEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkQueueEndDebugUtilsLabelEXT = loader(c"vkQueueEndDebugUtilsLabelEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkQueueInsertDebugUtilsLabelEXT =
                loader(c"vkQueueInsertDebugUtilsLabelEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_INTEL_performance_query")]
        {
            table.vkQueueSetPerformanceConfigurationINTEL =
                loader(c"vkQueueSetPerformanceConfigurationINTEL".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            table.vkQueuePresentKHR =
                loader(c"vkQueuePresentKHR".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_synchronization2")]
        {
            table.vkQueueSubmit2KHR =
                loader(c"vkQueueSubmit2KHR".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
        {
            table.vkGetQueueCheckpointData2NV = loader(c"vkGetQueueCheckpointData2NV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
        {
            table.vkGetQueueCheckpointDataNV = loader(c"vkGetQueueCheckpointDataNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_low_latency2")]
        {
            table.vkQueueNotifyOutOfBandNV = loader(c"vkQueueNotifyOutOfBandNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Queue<'dev> {
    pub(crate) raw: VkQueue,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev QueueDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for Queue<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Queue<'dev> {
    #[inline]
    pub fn raw(&self) -> VkQueue {
        self.raw
    }
    #[inline]
    pub fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub fn table(&self) -> &QueueDispatchTable {
        self.table
    }
    /// [`vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Queues:** SparseBinding
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `queue`
    /// - `bindInfoCount`: optional: true
    /// - `pBindInfo`: len: bindInfoCount
    /// - `fence`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkQueueBindSparse(
        &self,
        bindInfoCount: u32,
        pBindInfo: *const VkBindSparseInfo,
        fence: VkFence,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkQueueBindSparse.unwrap_unchecked()(
                self.raw,
                bindInfoCount,
                pBindInfo,
                fence,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `queue`
    /// - `submitCount`: optional: true
    /// - `pSubmits`: len: submitCount
    /// - `fence`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[deprecated(note = "superseded by `vkQueueSubmit2`")]
    #[inline(always)]
    pub fn vkQueueSubmit(
        &self,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkQueueSubmit.unwrap_unchecked()(self.raw, submitCount, pSubmits, fence)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `queue`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkQueueWaitIdle(&self) -> Result<VkResult, VkResult> {
        let r = unsafe { (self.table).vkQueueWaitIdle.unwrap_unchecked()(self.raw) };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_3`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `queue`
    /// - `submitCount`: optional: true
    /// - `pSubmits`: len: submitCount
    /// - `fence`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    #[inline(always)]
    pub fn vkQueueSubmit2(
        &self,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo2,
        fence: VkFence,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkQueueSubmit2.unwrap_unchecked()(self.raw, submitCount, pSubmits, fence)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pLabelInfo`
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline(always)]
    pub fn vkQueueBeginDebugUtilsLabelEXT(&self, pLabelInfo: *const VkDebugUtilsLabelEXT) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkQueueBeginDebugUtilsLabelEXT
                .unwrap_unchecked()(self.raw, pLabelInfo)
        }
    }
    /// [`vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `queue`
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline(always)]
    pub fn vkQueueEndDebugUtilsLabelEXT(&self) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkQueueEndDebugUtilsLabelEXT.unwrap_unchecked()(self.raw)
        }
    }
    /// [`vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pLabelInfo`
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline(always)]
    pub fn vkQueueInsertDebugUtilsLabelEXT(&self, pLabelInfo: *const VkDebugUtilsLabelEXT) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkQueueInsertDebugUtilsLabelEXT
                .unwrap_unchecked()(self.raw, pLabelInfo)
        }
    }
    /// [`vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html)
    ///
    /// Provided by:
    /// - `VK_INTEL_performance_query`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `configuration`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_TOO_MANY_OBJECTS
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_INTEL_performance_query")]
    #[inline(always)]
    pub fn vkQueueSetPerformanceConfigurationINTEL(
        &self,
        configuration: VkPerformanceConfigurationINTEL,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkQueueSetPerformanceConfigurationINTEL
                .unwrap_unchecked()(self.raw, configuration)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_TOO_MANY_OBJECTS
            | VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_swapchain`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pPresentInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///   - VK_SUBOPTIMAL_KHR
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_OUT_OF_DATE_KHR
    ///   - VK_ERROR_SURFACE_LOST_KHR
    ///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    ///   - VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT
    #[cfg(feature = "VK_KHR_swapchain")]
    #[inline(always)]
    pub fn vkQueuePresentKHR(
        &self,
        pPresentInfo: *const VkPresentInfoKHR,
    ) -> Result<VkResult, VkResult> {
        let r =
            unsafe { (self.table).vkQueuePresentKHR.unwrap_unchecked()(self.raw, pPresentInfo) };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            #[cfg(feature = "VK_KHR_swapchain")]
            VkResult::VK_SUBOPTIMAL_KHR => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            VkResult::VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => Err(r),
            #[cfg(feature = "VK_EXT_present_timing")]
            VkResult::VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_SURFACE_LOST_KHR => Err(r),
            #[cfg(feature = "VK_KHR_swapchain")]
            VkResult::VK_ERROR_OUT_OF_DATE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)
    ///
    /// Provided by:
    /// - `VK_KHR_synchronization2`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `queue`
    /// - `submitCount`: optional: true
    /// - `pSubmits`: len: submitCount
    /// - `fence`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_DEVICE_LOST
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_synchronization2")]
    #[inline(always)]
    pub fn vkQueueSubmit2KHR(
        &self,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo2,
        fence: VkFence,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkQueueSubmit2KHR.unwrap_unchecked()(
                self.raw,
                submitCount,
                pSubmits,
                fence,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_DEVICE_LOST
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html)
    ///
    /// Provided by:
    /// - `VK_NV_device_diagnostic_checkpoints`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pCheckpointDataCount`: optional: pointer required, values optional if pointer not null
    /// - `pCheckpointData`: optional: true, len: pCheckpointDataCount
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    #[inline(always)]
    pub fn vkGetQueueCheckpointData2NV(
        &self,
        pCheckpointDataCount: *mut u32,
        pCheckpointData: *mut VkCheckpointData2NV,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkGetQueueCheckpointData2NV.unwrap_unchecked()(
                self.raw,
                pCheckpointDataCount,
                pCheckpointData,
            )
        }
    }
    /// [`vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_device_diagnostic_checkpoints`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pCheckpointDataCount`: optional: pointer required, values optional if pointer not null
    /// - `pCheckpointData`: optional: true, len: pCheckpointDataCount
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    #[inline(always)]
    pub fn vkGetQueueCheckpointDataNV(
        &self,
        pCheckpointDataCount: *mut u32,
        pCheckpointData: *mut VkCheckpointDataNV,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkGetQueueCheckpointDataNV.unwrap_unchecked()(
                self.raw,
                pCheckpointDataCount,
                pCheckpointData,
            )
        }
    }
    /// [`vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_low_latency2`
    ///
    ///
    /// # Parameters
    /// - `queue`
    /// - `pQueueTypeInfo`
    #[cfg(feature = "VK_NV_low_latency2")]
    #[inline(always)]
    pub fn vkQueueNotifyOutOfBandNV(&self, pQueueTypeInfo: *const VkOutOfBandQueueTypeInfoNV) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkQueueNotifyOutOfBandNV.unwrap_unchecked()(self.raw, pQueueTypeInfo)
        }
    }
}
