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
pub struct SwapchainKHRDispatchTable {
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    pub vkSetLocalDimmingAMD: Option<PFN_vkSetLocalDimmingAMD>,
    #[cfg(feature = "VK_EXT_display_control")]
    pub vkGetSwapchainCounterEXT: Option<PFN_vkGetSwapchainCounterEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    pub vkAcquireFullScreenExclusiveModeEXT: Option<PFN_vkAcquireFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    pub vkReleaseFullScreenExclusiveModeEXT: Option<PFN_vkReleaseFullScreenExclusiveModeEXT>,
    #[cfg(feature = "VK_EXT_present_timing")]
    pub vkGetSwapchainTimeDomainPropertiesEXT: Option<PFN_vkGetSwapchainTimeDomainPropertiesEXT>,
    #[cfg(feature = "VK_EXT_present_timing")]
    pub vkGetSwapchainTimingPropertiesEXT: Option<PFN_vkGetSwapchainTimingPropertiesEXT>,
    #[cfg(feature = "VK_EXT_present_timing")]
    pub vkSetSwapchainPresentTimingQueueSizeEXT:
        Option<PFN_vkSetSwapchainPresentTimingQueueSizeEXT>,
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    pub vkGetPastPresentationTimingGOOGLE: Option<PFN_vkGetPastPresentationTimingGOOGLE>,
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    pub vkGetRefreshCycleDurationGOOGLE: Option<PFN_vkGetRefreshCycleDurationGOOGLE>,
    #[cfg(feature = "VK_KHR_present_wait")]
    pub vkWaitForPresentKHR: Option<PFN_vkWaitForPresentKHR>,
    #[cfg(feature = "VK_KHR_present_wait2")]
    pub vkWaitForPresent2KHR: Option<PFN_vkWaitForPresent2KHR>,
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    pub vkGetSwapchainStatusKHR: Option<PFN_vkGetSwapchainStatusKHR>,
    #[cfg(feature = "VK_KHR_swapchain")]
    pub vkAcquireNextImageKHR: Option<PFN_vkAcquireNextImageKHR>,
    #[cfg(feature = "VK_KHR_swapchain")]
    pub vkDestroySwapchainKHR: Option<PFN_vkDestroySwapchainKHR>,
    #[cfg(feature = "VK_KHR_swapchain")]
    pub vkGetSwapchainImagesKHR: Option<PFN_vkGetSwapchainImagesKHR>,
    #[cfg(feature = "VK_NV_low_latency2")]
    pub vkGetLatencyTimingsNV: Option<PFN_vkGetLatencyTimingsNV>,
    #[cfg(feature = "VK_NV_low_latency2")]
    pub vkLatencySleepNV: Option<PFN_vkLatencySleepNV>,
    #[cfg(feature = "VK_NV_low_latency2")]
    pub vkSetLatencyMarkerNV: Option<PFN_vkSetLatencyMarkerNV>,
    #[cfg(feature = "VK_NV_low_latency2")]
    pub vkSetLatencySleepModeNV: Option<PFN_vkSetLatencySleepModeNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl SwapchainKHRDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_AMD_display_native_hdr")]
        vkSetLocalDimmingAMD: None,
        #[cfg(feature = "VK_EXT_display_control")]
        vkGetSwapchainCounterEXT: None,
        #[cfg(feature = "VK_EXT_full_screen_exclusive")]
        vkAcquireFullScreenExclusiveModeEXT: None,
        #[cfg(feature = "VK_EXT_full_screen_exclusive")]
        vkReleaseFullScreenExclusiveModeEXT: None,
        #[cfg(feature = "VK_EXT_present_timing")]
        vkGetSwapchainTimeDomainPropertiesEXT: None,
        #[cfg(feature = "VK_EXT_present_timing")]
        vkGetSwapchainTimingPropertiesEXT: None,
        #[cfg(feature = "VK_EXT_present_timing")]
        vkSetSwapchainPresentTimingQueueSizeEXT: None,
        #[cfg(feature = "VK_GOOGLE_display_timing")]
        vkGetPastPresentationTimingGOOGLE: None,
        #[cfg(feature = "VK_GOOGLE_display_timing")]
        vkGetRefreshCycleDurationGOOGLE: None,
        #[cfg(feature = "VK_KHR_present_wait")]
        vkWaitForPresentKHR: None,
        #[cfg(feature = "VK_KHR_present_wait2")]
        vkWaitForPresent2KHR: None,
        #[cfg(feature = "VK_KHR_shared_presentable_image")]
        vkGetSwapchainStatusKHR: None,
        #[cfg(feature = "VK_KHR_swapchain")]
        vkAcquireNextImageKHR: None,
        #[cfg(feature = "VK_KHR_swapchain")]
        vkDestroySwapchainKHR: None,
        #[cfg(feature = "VK_KHR_swapchain")]
        vkGetSwapchainImagesKHR: None,
        #[cfg(feature = "VK_NV_low_latency2")]
        vkGetLatencyTimingsNV: None,
        #[cfg(feature = "VK_NV_low_latency2")]
        vkLatencySleepNV: None,
        #[cfg(feature = "VK_NV_low_latency2")]
        vkSetLatencyMarkerNV: None,
        #[cfg(feature = "VK_NV_low_latency2")]
        vkSetLatencySleepModeNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_AMD_display_native_hdr")]
        {
            table.vkSetLocalDimmingAMD = loader(c"vkSetLocalDimmingAMD".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_display_control")]
        {
            table.vkGetSwapchainCounterEXT = loader(c"vkGetSwapchainCounterEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_full_screen_exclusive")]
        {
            table.vkAcquireFullScreenExclusiveModeEXT =
                loader(c"vkAcquireFullScreenExclusiveModeEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_full_screen_exclusive")]
        {
            table.vkReleaseFullScreenExclusiveModeEXT =
                loader(c"vkReleaseFullScreenExclusiveModeEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_present_timing")]
        {
            table.vkGetSwapchainTimeDomainPropertiesEXT =
                loader(c"vkGetSwapchainTimeDomainPropertiesEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_present_timing")]
        {
            table.vkGetSwapchainTimingPropertiesEXT =
                loader(c"vkGetSwapchainTimingPropertiesEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_present_timing")]
        {
            table.vkSetSwapchainPresentTimingQueueSizeEXT =
                loader(c"vkSetSwapchainPresentTimingQueueSizeEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_GOOGLE_display_timing")]
        {
            table.vkGetPastPresentationTimingGOOGLE =
                loader(c"vkGetPastPresentationTimingGOOGLE".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_GOOGLE_display_timing")]
        {
            table.vkGetRefreshCycleDurationGOOGLE =
                loader(c"vkGetRefreshCycleDurationGOOGLE".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_present_wait")]
        {
            table.vkWaitForPresentKHR =
                loader(c"vkWaitForPresentKHR".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_present_wait2")]
        {
            table.vkWaitForPresent2KHR = loader(c"vkWaitForPresent2KHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_shared_presentable_image")]
        {
            table.vkGetSwapchainStatusKHR = loader(c"vkGetSwapchainStatusKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            table.vkAcquireNextImageKHR = loader(c"vkAcquireNextImageKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            table.vkDestroySwapchainKHR = loader(c"vkDestroySwapchainKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            table.vkGetSwapchainImagesKHR = loader(c"vkGetSwapchainImagesKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_low_latency2")]
        {
            table.vkGetLatencyTimingsNV = loader(c"vkGetLatencyTimingsNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_low_latency2")]
        {
            table.vkLatencySleepNV =
                loader(c"vkLatencySleepNV".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_low_latency2")]
        {
            table.vkSetLatencyMarkerNV = loader(c"vkSetLatencyMarkerNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_low_latency2")]
        {
            table.vkSetLatencySleepModeNV = loader(c"vkSetLatencySleepModeNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
pub struct SwapchainKHR<'dev> {
    pub(crate) raw: VkSwapchainKHR,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev SwapchainKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'dev> Send for SwapchainKHR<'dev> {}
#[cfg(feature = "VK_KHR_swapchain")]
unsafe impl<'dev> Sync for SwapchainKHR<'dev> {}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'dev> Drop for SwapchainKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroySwapchainKHR {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_swapchain")]
impl<'dev> SwapchainKHR<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkSwapchainKHR {
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
    pub const fn table(&self) -> &SwapchainKHRDispatchTable {
        self.table
    }
    /// [`vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html)
    ///
    /// Provided by:
    /// - `VK_AMD_display_native_hdr`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapChain`
    /// - `localDimmingEnable`
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    #[inline(always)]
    pub fn vkSetLocalDimmingAMD(&self, localDimmingEnable: VkBool32) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkSetLocalDimmingAMD.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                localDimmingEnable,
            )
        }
    }
    /// [`vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_display_control`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `counter`
    /// - `pCounterValue`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_display_control")]
    #[inline(always)]
    pub fn vkGetSwapchainCounterEXT(
        &self,
        counter: VkSurfaceCounterFlagBitsEXT,
        pCounterValue: *mut u64,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetSwapchainCounterEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                counter,
                pCounterValue,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_full_screen_exclusive`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_INITIALIZATION_FAILED`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    pub fn vkAcquireFullScreenExclusiveModeEXT(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkAcquireFullScreenExclusiveModeEXT
                .unwrap_unchecked()(self.device().raw(), self.raw)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_full_screen_exclusive`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    #[inline(always)]
    pub fn vkReleaseFullScreenExclusiveModeEXT(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkReleaseFullScreenExclusiveModeEXT
                .unwrap_unchecked()(self.device().raw(), self.raw)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_present_timing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pSwapchainTimeDomainProperties`
    /// - `pTimeDomainsCounter`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_INCOMPLETE`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_present_timing")]
    #[inline(always)]
    pub fn vkGetSwapchainTimeDomainPropertiesEXT(
        &self,
        pSwapchainTimeDomainProperties: *mut VkSwapchainTimeDomainPropertiesEXT,
        pTimeDomainsCounter: *mut u64,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetSwapchainTimeDomainPropertiesEXT
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSwapchainTimeDomainProperties,
                pTimeDomainsCounter,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_present_timing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pSwapchainTimingProperties`
    /// - `pSwapchainTimingPropertiesCounter`: optional: true
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
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_present_timing")]
    #[inline(always)]
    pub fn vkGetSwapchainTimingPropertiesEXT(
        &self,
        pSwapchainTimingProperties: *mut VkSwapchainTimingPropertiesEXT,
        pSwapchainTimingPropertiesCounter: *mut u64,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetSwapchainTimingPropertiesEXT
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSwapchainTimingProperties,
                pSwapchainTimingPropertiesCounter,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_present_timing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `size`
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
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_EXT_present_timing")]
    #[inline(always)]
    pub fn vkSetSwapchainPresentTimingQueueSizeEXT(&self, size: u32) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkSetSwapchainPresentTimingQueueSizeEXT
                .unwrap_unchecked()(self.device().raw(), self.raw, size)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)
    ///
    /// Provided by:
    /// - `VK_GOOGLE_display_timing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pPresentationTimingCount`: optional: pointer required, values optional if pointer not null
    /// - `pPresentationTimings`: optional: true, len: pPresentationTimingCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_INCOMPLETE`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    #[inline(always)]
    pub fn vkGetPastPresentationTimingGOOGLE(
        &self,
        pPresentationTimingCount: *mut u32,
        pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetPastPresentationTimingGOOGLE
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pPresentationTimingCount,
                pPresentationTimings,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html)
    ///
    /// Provided by:
    /// - `VK_GOOGLE_display_timing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pDisplayTimingProperties`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    #[inline(always)]
    pub fn vkGetRefreshCycleDurationGOOGLE(
        &self,
        pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetRefreshCycleDurationGOOGLE
                .unwrap_unchecked()(
                self.device().raw(), self.raw, pDisplayTimingProperties
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_present_wait`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `presentId`
    /// - `timeout`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_TIMEOUT`
    ///   - `VK_SUBOPTIMAL_KHR`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_KHR_present_wait")]
    #[inline(always)]
    pub fn vkWaitForPresentKHR(&self, presentId: u64, timeout: u64) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkWaitForPresentKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                presentId,
                timeout,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_present_wait2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pPresentWait2Info`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_TIMEOUT`
    ///   - `VK_SUBOPTIMAL_KHR`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_KHR_present_wait2")]
    #[inline(always)]
    pub fn vkWaitForPresent2KHR(
        &self,
        pPresentWait2Info: *const VkPresentWait2InfoKHR,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkWaitForPresent2KHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pPresentWait2Info,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_shared_presentable_image`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_SUBOPTIMAL_KHR`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    #[inline(always)]
    pub fn vkGetSwapchainStatusKHR(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetSwapchainStatusKHR.unwrap_unchecked()(self.device().raw(), self.raw)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_swapchain`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `timeout`
    /// - `semaphore`: optional: true
    /// - `fence`: optional: true
    /// - `pImageIndex`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_TIMEOUT`
    ///   - `VK_NOT_READY`
    ///   - `VK_SUBOPTIMAL_KHR`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_OUT_OF_DATE_KHR`
    ///   - `VK_ERROR_SURFACE_LOST_KHR`
    ///   - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_KHR_swapchain")]
    #[inline(always)]
    pub fn vkAcquireNextImageKHR(
        &self,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        pImageIndex: *mut u32,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkAcquireNextImageKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                timeout,
                semaphore,
                fence,
                pImageIndex,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_swapchain`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_swapchain")]
    #[inline(always)]
    pub fn vkDestroySwapchainKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroySwapchainKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkSwapchainKHR::NULL;
    }
    /// [`vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_swapchain`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pSwapchainImageCount`: optional: pointer required, values optional if pointer not null
    /// - `pSwapchainImages`: optional: true, len: pSwapchainImageCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_INCOMPLETE`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_KHR_swapchain")]
    #[inline(always)]
    pub fn vkGetSwapchainImagesKHR(
        &self,
        pSwapchainImageCount: *mut u32,
        pSwapchainImages: *mut VkImage,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetSwapchainImagesKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSwapchainImageCount,
                pSwapchainImages,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_low_latency2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pLatencyMarkerInfo`
    #[cfg(feature = "VK_NV_low_latency2")]
    #[inline(always)]
    pub fn vkGetLatencyTimingsNV(&self, pLatencyMarkerInfo: *mut VkGetLatencyMarkerInfoNV) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkGetLatencyTimingsNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pLatencyMarkerInfo,
            )
        }
    }
    /// [`vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_low_latency2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pSleepInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_NV_low_latency2")]
    #[inline(always)]
    pub fn vkLatencySleepNV(
        &self,
        pSleepInfo: *const VkLatencySleepInfoNV,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkLatencySleepNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSleepInfo,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_low_latency2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pLatencyMarkerInfo`
    #[cfg(feature = "VK_NV_low_latency2")]
    #[inline(always)]
    pub fn vkSetLatencyMarkerNV(&self, pLatencyMarkerInfo: *const VkSetLatencyMarkerInfoNV) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkSetLatencyMarkerNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pLatencyMarkerInfo,
            )
        }
    }
    /// [`vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_low_latency2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `swapchain`
    /// - `pSleepModeInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_INITIALIZATION_FAILED`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_NV_low_latency2")]
    #[inline(always)]
    pub fn vkSetLatencySleepModeNV(
        &self,
        pSleepModeInfo: *const VkLatencySleepModeInfoNV,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkSetLatencySleepModeNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSleepModeInfo,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
}
