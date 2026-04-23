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
pub struct SemaphoreDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroySemaphore: Option<PFN_vkDestroySemaphore>,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    pub vkGetSemaphoreCounterValue: Option<PFN_vkGetSemaphoreCounterValue>,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    pub vkGetSemaphoreCounterValueKHR: Option<PFN_vkGetSemaphoreCounterValueKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl SemaphoreDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroySemaphore: None,
        #[cfg(feature = "VK_BASE_VERSION_1_2")]
        vkGetSemaphoreCounterValue: None,
        #[cfg(feature = "VK_KHR_timeline_semaphore")]
        vkGetSemaphoreCounterValueKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroySemaphore =
                loader(c"vkDestroySemaphore".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_2")]
        {
            table.vkGetSemaphoreCounterValue = loader(c"vkGetSemaphoreCounterValue".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_timeline_semaphore")]
        {
            table.vkGetSemaphoreCounterValueKHR = loader(c"vkGetSemaphoreCounterValueKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Semaphore<'dev> {
    pub(crate) raw: VkSemaphore,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev SemaphoreDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for Semaphore<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for Semaphore<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for Semaphore<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroySemaphore {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Semaphore<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkSemaphore {
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
    pub const fn table(&self) -> &SemaphoreDispatchTable {
        self.table
    }
    /// [`vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `semaphore`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroySemaphore(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroySemaphore.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkSemaphore::NULL;
    }
    /// [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_2`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `semaphore`
    /// - `pValue`
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
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    #[inline(always)]
    pub fn vkGetSemaphoreCounterValue(&self, pValue: *mut u64) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetSemaphoreCounterValue.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pValue,
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
    /// [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)
    ///
    /// Provided by:
    /// - `VK_KHR_timeline_semaphore`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `semaphore`
    /// - `pValue`
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
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    #[inline(always)]
    pub fn vkGetSemaphoreCounterValueKHR(&self, pValue: *mut u64) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetSemaphoreCounterValueKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pValue)
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
}
