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
pub struct AccelerationStructureNVDispatchTable {
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub vkDestroyAccelerationStructureNV: Option<PFN_vkDestroyAccelerationStructureNV>,
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub vkGetAccelerationStructureHandleNV: Option<
        PFN_vkGetAccelerationStructureHandleNV,
    >,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl AccelerationStructureNVDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NV_ray_tracing")]
        vkDestroyAccelerationStructureNV: None,
        #[cfg(feature = "VK_NV_ray_tracing")]
        vkGetAccelerationStructureHandleNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            table.vkDestroyAccelerationStructureNV = loader(
                    c"vkDestroyAccelerationStructureNV".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            table.vkGetAccelerationStructureHandleNV = loader(
                    c"vkGetAccelerationStructureHandleNV".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
pub struct AccelerationStructureNV<'dev> {
    pub(crate) raw: VkAccelerationStructureNV,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev AccelerationStructureNVDispatchTable,
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'dev> Drop for AccelerationStructureNV<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyAccelerationStructureNV {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NV_ray_tracing")]
impl<'dev> AccelerationStructureNV<'dev> {
    #[inline]
    pub fn raw(&self) -> VkAccelerationStructureNV {
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
    pub fn table(&self) -> &AccelerationStructureNVDispatchTable {
        self.table
    }
    /// [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_ray_tracing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `accelerationStructure`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NV_ray_tracing")]
    #[inline(always)]
    pub fn vkDestroyAccelerationStructureNV(
        &mut self,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyAccelerationStructureNV
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkAccelerationStructureNV::NULL;
    }
    /// [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_ray_tracing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `accelerationStructure`
    /// - `dataSize`
    /// - `pData`: len: dataSize
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
    #[cfg(feature = "VK_NV_ray_tracing")]
    #[inline(always)]
    pub fn vkGetAccelerationStructureHandleNV(
        &self,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetAccelerationStructureHandleNV
                .unwrap_unchecked()(self.device().raw(), self.raw, dataSize, pData)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
