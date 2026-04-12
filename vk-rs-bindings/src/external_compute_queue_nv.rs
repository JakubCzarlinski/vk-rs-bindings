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
pub struct ExternalComputeQueueNVDispatchTable {
    #[cfg(feature = "VK_NV_external_compute_queue")]
    pub vkDestroyExternalComputeQueueNV: Option<PFN_vkDestroyExternalComputeQueueNV>,
    #[cfg(feature = "VK_NV_external_compute_queue")]
    pub vkGetExternalComputeQueueDataNV: Option<PFN_vkGetExternalComputeQueueDataNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ExternalComputeQueueNVDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NV_external_compute_queue")]
        vkDestroyExternalComputeQueueNV: None,
        #[cfg(feature = "VK_NV_external_compute_queue")]
        vkGetExternalComputeQueueDataNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_NV_external_compute_queue")]
        {
            table.vkDestroyExternalComputeQueueNV =
                loader(c"vkDestroyExternalComputeQueueNV".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_external_compute_queue")]
        {
            table.vkGetExternalComputeQueueDataNV =
                loader(c"vkGetExternalComputeQueueDataNV".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_NV_external_compute_queue")]
pub struct ExternalComputeQueueNV<'dev> {
    pub(crate) raw: VkExternalComputeQueueNV,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ExternalComputeQueueNVDispatchTable,
}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'dev> Drop for ExternalComputeQueueNV<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyExternalComputeQueueNV {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NV_external_compute_queue")]
impl<'dev> ExternalComputeQueueNV<'dev> {
    #[inline]
    pub fn raw(&self) -> VkExternalComputeQueueNV {
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
    pub fn table(&self) -> &ExternalComputeQueueNVDispatchTable {
        self.table
    }
    /// [`vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_external_compute_queue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `externalQueue`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NV_external_compute_queue")]
    #[inline(always)]
    pub fn vkDestroyExternalComputeQueueNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyExternalComputeQueueNV
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkExternalComputeQueueNV::NULL;
    }
    /// [`vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_external_compute_queue`
    ///
    ///
    /// # Parameters
    /// - `externalQueue`
    /// - `params`
    /// - `pData`
    #[cfg(feature = "VK_NV_external_compute_queue")]
    #[inline(always)]
    pub fn vkGetExternalComputeQueueDataNV(
        &self,
        params: *mut VkExternalComputeQueueDataParamsNV,
        pData: *mut core::ffi::c_void,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetExternalComputeQueueDataNV
                .unwrap_unchecked()(self.raw, params, pData)
        }
    }
}
