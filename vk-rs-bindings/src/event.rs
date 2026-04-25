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
pub struct EventDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyEvent: Option<PFN_vkDestroyEvent>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkGetEventStatus: Option<PFN_vkGetEventStatus>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkResetEvent: Option<PFN_vkResetEvent>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkSetEvent: Option<PFN_vkSetEvent>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl EventDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyEvent: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkGetEventStatus: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkResetEvent: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkSetEvent: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkDestroyEvent: loader(c"vkDestroyEvent".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkGetEventStatus: loader(c"vkGetEventStatus".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkResetEvent: loader(c"vkResetEvent".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkSetEvent: loader(c"vkSetEvent".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct Event<'dev> {
    pub(crate) raw: VkEvent,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev EventDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for Event<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for Event<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for Event<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyEvent {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Event<'dev> {
    #[inline(always)]
    pub const fn raw(&self) -> VkEvent {
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
    pub const fn table(&self) -> &EventDispatchTable {
        self.table
    }
    /// [`vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `event`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyEvent(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyEvent.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkEvent::NULL;
    }
    /// [`vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `event`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_EVENT_SET`
    ///   - `VK_EVENT_RESET`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_DEVICE_LOST`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetEventStatus(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetEventStatus.unwrap_unchecked()(self.device().raw(), self.raw)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `event`
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
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkResetEvent(&self) -> Result<VkResult, VkResult> {
        let r =
            unsafe { (self.table).vkResetEvent.unwrap_unchecked()(self.device().raw(), self.raw) };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `event`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkSetEvent(&self) -> Result<VkResult, VkResult> {
        let r =
            unsafe { (self.table).vkSetEvent.unwrap_unchecked()(self.device().raw(), self.raw) };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
}
