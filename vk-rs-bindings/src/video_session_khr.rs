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
pub struct VideoSessionKHRDispatchTable {
    #[cfg(feature = "VK_KHR_video_queue")]
    pub vkBindVideoSessionMemoryKHR: Option<PFN_vkBindVideoSessionMemoryKHR>,
    #[cfg(feature = "VK_KHR_video_queue")]
    pub vkDestroyVideoSessionKHR: Option<PFN_vkDestroyVideoSessionKHR>,
    #[cfg(feature = "VK_KHR_video_queue")]
    pub vkGetVideoSessionMemoryRequirementsKHR: Option<PFN_vkGetVideoSessionMemoryRequirementsKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl VideoSessionKHRDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_KHR_video_queue")]
        vkBindVideoSessionMemoryKHR: None,
        #[cfg(feature = "VK_KHR_video_queue")]
        vkDestroyVideoSessionKHR: None,
        #[cfg(feature = "VK_KHR_video_queue")]
        vkGetVideoSessionMemoryRequirementsKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            table.vkBindVideoSessionMemoryKHR = loader(c"vkBindVideoSessionMemoryKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            table.vkDestroyVideoSessionKHR = loader(c"vkDestroyVideoSessionKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_video_queue")]
        {
            table.vkGetVideoSessionMemoryRequirementsKHR =
                loader(c"vkGetVideoSessionMemoryRequirementsKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
pub struct VideoSessionKHR<'dev> {
    pub(crate) raw: VkVideoSessionKHR,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev VideoSessionKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_video_queue")]
unsafe impl<'dev> Send for VideoSessionKHR<'dev> {}
#[cfg(feature = "VK_KHR_video_queue")]
unsafe impl<'dev> Sync for VideoSessionKHR<'dev> {}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> Drop for VideoSessionKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyVideoSessionKHR {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_video_queue")]
impl<'dev> VideoSessionKHR<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkVideoSessionKHR {
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
    pub const fn table(&self) -> &VideoSessionKHRDispatchTable {
        self.table
    }
    /// [`vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_video_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `videoSession`
    /// - `bindSessionMemoryInfoCount`
    /// - `pBindSessionMemoryInfos`: len: bindSessionMemoryInfoCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_video_queue")]
    #[inline(always)]
    pub fn vkBindVideoSessionMemoryKHR(
        &self,
        bindSessionMemoryInfoCount: u32,
        pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkBindVideoSessionMemoryKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                bindSessionMemoryInfoCount,
                pBindSessionMemoryInfos,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
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
    /// [`vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_video_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `videoSession`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_video_queue")]
    #[inline(always)]
    pub fn vkDestroyVideoSessionKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyVideoSessionKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkVideoSessionKHR::NULL;
    }
    /// [`vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_video_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `videoSession`
    /// - `pMemoryRequirementsCount`: optional: pointer required, values optional if pointer not null
    /// - `pMemoryRequirements`: optional: true, len: pMemoryRequirementsCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///   - VK_INCOMPLETE
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_video_queue")]
    #[inline(always)]
    pub fn vkGetVideoSessionMemoryRequirementsKHR(
        &self,
        pMemoryRequirementsCount: *mut u32,
        pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetVideoSessionMemoryRequirementsKHR
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pMemoryRequirementsCount,
                pMemoryRequirements,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_UNKNOWN => Err(r),
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
}
