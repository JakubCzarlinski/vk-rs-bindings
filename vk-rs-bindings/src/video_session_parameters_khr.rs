#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use core::ffi::{c_char, c_void};
use crate::commands::*;
use crate::types::*;
use crate::enums::*;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct VideoSessionParametersKHRDispatchTable {
    #[cfg(feature = "VK_KHR_video_queue")]
    pub vkDestroyVideoSessionParametersKHR: Option<
        PFN_vkDestroyVideoSessionParametersKHR,
    >,
    #[cfg(feature = "VK_KHR_video_queue")]
    pub vkUpdateVideoSessionParametersKHR: Option<PFN_vkUpdateVideoSessionParametersKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VideoSessionParametersKHRDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_KHR_video_queue")]
        vkDestroyVideoSessionParametersKHR: None,
        #[cfg(feature = "VK_KHR_video_queue")]
        vkUpdateVideoSessionParametersKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            table.vkDestroyVideoSessionParametersKHR = loader(
                    c"vkDestroyVideoSessionParametersKHR".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            table.vkUpdateVideoSessionParametersKHR = loader(
                    c"vkUpdateVideoSessionParametersKHR".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
pub struct VideoSessionParametersKHR<'dev> {
    pub(crate) raw: VkVideoSessionParametersKHR,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev VideoSessionParametersKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> Drop for VideoSessionParametersKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyVideoSessionParametersKHR {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> VideoSessionParametersKHR<'dev> {
    #[inline]
    pub fn raw(&self) -> VkVideoSessionParametersKHR {
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
    pub fn table(&self) -> &VideoSessionParametersKHRDispatchTable {
        self.table
    }
    /// [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_video_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `videoSessionParameters`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_video_queue")]
    #[inline(always)]
    pub fn vkDestroyVideoSessionParametersKHR(
        &mut self,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyVideoSessionParametersKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkVideoSessionParametersKHR::NULL;
    }
    /// [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_video_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `videoSessionParameters`
    /// - `pUpdateInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_video_queue")]
    #[inline(always)]
    pub fn vkUpdateVideoSessionParametersKHR(
        &self,
        pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkUpdateVideoSessionParametersKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pUpdateInfo)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_video_encode_queue")]
            VkResult::VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
