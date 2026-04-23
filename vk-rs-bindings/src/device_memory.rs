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
pub struct DeviceMemoryDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkFreeMemory: Option<PFN_vkFreeMemory>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetDeviceMemoryCommitment: Option<PFN_vkGetDeviceMemoryCommitment>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkMapMemory: Option<PFN_vkMapMemory>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkUnmapMemory: Option<PFN_vkUnmapMemory>,
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    pub vkSetDeviceMemoryPriorityEXT: Option<PFN_vkSetDeviceMemoryPriorityEXT>,
    #[cfg(feature = "VK_NV_external_memory_win32")]
    pub vkGetMemoryWin32HandleNV: Option<PFN_vkGetMemoryWin32HandleNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DeviceMemoryDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkFreeMemory: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetDeviceMemoryCommitment: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkMapMemory: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkUnmapMemory: None,
        #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
        vkSetDeviceMemoryPriorityEXT: None,
        #[cfg(feature = "VK_NV_external_memory_win32")]
        vkGetMemoryWin32HandleNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkFreeMemory =
                loader(c"vkFreeMemory".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetDeviceMemoryCommitment = loader(c"vkGetDeviceMemoryCommitment".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkMapMemory =
                loader(c"vkMapMemory".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkUnmapMemory =
                loader(c"vkUnmapMemory".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
        {
            table.vkSetDeviceMemoryPriorityEXT = loader(c"vkSetDeviceMemoryPriorityEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_external_memory_win32")]
        {
            table.vkGetMemoryWin32HandleNV = loader(c"vkGetMemoryWin32HandleNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct DeviceMemory<'dev> {
    pub(crate) raw: VkDeviceMemory,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev DeviceMemoryDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for DeviceMemory<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for DeviceMemory<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for DeviceMemory<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(free_fn) = self.table.vkFreeMemory {
            unsafe { free_fn(self.device().raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> DeviceMemory<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkDeviceMemory {
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
    pub const fn table(&self) -> &DeviceMemoryDispatchTable {
        self.table
    }
    /// [`vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkFreeMemory(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkFreeMemory.unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkDeviceMemory::NULL;
    }
    /// [`vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`
    /// - `pCommittedMemoryInBytes`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetDeviceMemoryCommitment(&self, pCommittedMemoryInBytes: *mut VkDeviceSize) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkGetDeviceMemoryCommitment.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pCommittedMemoryInBytes,
            )
        }
    }
    /// [`vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`
    /// - `offset`
    /// - `size`
    /// - `flags`: optional: true
    /// - `ppData`: optional: pointer required, values optional if pointer not null
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_MEMORY_MAP_FAILED
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkMapMemory(
        &self,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkMapMemory.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                offset,
                size,
                flags,
                ppData,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_MEMORY_MAP_FAILED
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
    /// [`vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkUnmapMemory(&self) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkUnmapMemory.unwrap_unchecked()(self.device().raw(), self.raw)
        }
    }
    /// [`vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_pageable_device_local_memory`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`
    /// - `priority`
    #[cfg(feature = "VK_EXT_pageable_device_local_memory")]
    #[inline(always)]
    pub fn vkSetDeviceMemoryPriorityEXT(&self, priority: f32) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkSetDeviceMemoryPriorityEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                priority,
            )
        }
    }
    /// [`vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_external_memory_win32`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `memory`
    /// - `handleType`
    /// - `pHandle`
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
    #[cfg(feature = "VK_NV_external_memory_win32")]
    #[inline(always)]
    pub fn vkGetMemoryWin32HandleNV(
        &self,
        handleType: VkExternalMemoryHandleTypeFlagsNV,
        pHandle: *mut HANDLE,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetMemoryWin32HandleNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                handleType,
                pHandle,
            )
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
}
