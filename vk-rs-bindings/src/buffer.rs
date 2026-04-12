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
pub struct BufferDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkBindBufferMemory: Option<PFN_vkBindBufferMemory>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyBuffer: Option<PFN_vkDestroyBuffer>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetBufferMemoryRequirements: Option<PFN_vkGetBufferMemoryRequirements>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl BufferDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkBindBufferMemory: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyBuffer: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetBufferMemoryRequirements: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkBindBufferMemory =
                loader(c"vkBindBufferMemory".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroyBuffer =
                loader(c"vkDestroyBuffer".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetBufferMemoryRequirements = loader(c"vkGetBufferMemoryRequirements".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Buffer<'dev> {
    pub(crate) raw: VkBuffer,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev BufferDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for Buffer<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyBuffer {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Buffer<'dev> {
    #[inline]
    pub fn raw(&self) -> VkBuffer {
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
    pub fn table(&self) -> &BufferDispatchTable {
        self.table
    }
    /// [`vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `buffer`
    /// - `memory`
    /// - `memoryOffset`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkBindBufferMemory(
        &self,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkBindBufferMemory.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                memory,
                memoryOffset,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_buffer_device_address")]
            VkResult::VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `buffer`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyBuffer(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyBuffer.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkBuffer::NULL;
    }
    /// [`vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `buffer`
    /// - `pMemoryRequirements`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetBufferMemoryRequirements(&self, pMemoryRequirements: *mut VkMemoryRequirements) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetBufferMemoryRequirements
                .unwrap_unchecked()(self.device().raw(), self.raw, pMemoryRequirements)
        }
    }
}
