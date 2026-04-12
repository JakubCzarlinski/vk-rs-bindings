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
pub struct SemaphoreSciSyncPoolNVDispatchTable {
    #[cfg(feature = "VK_NV_external_sci_sync2")]
    pub vkDestroySemaphoreSciSyncPoolNV: Option<PFN_vkDestroySemaphoreSciSyncPoolNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl SemaphoreSciSyncPoolNVDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NV_external_sci_sync2")]
        vkDestroySemaphoreSciSyncPoolNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_NV_external_sci_sync2")]
        {
            table.vkDestroySemaphoreSciSyncPoolNV = loader(
                    c"vkDestroySemaphoreSciSyncPoolNV".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub struct SemaphoreSciSyncPoolNV<'dev> {
    pub(crate) raw: VkSemaphoreSciSyncPoolNV,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev SemaphoreSciSyncPoolNVDispatchTable,
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl<'dev> Drop for SemaphoreSciSyncPoolNV<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroySemaphoreSciSyncPoolNV {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NV_external_sci_sync2")]
impl<'dev> SemaphoreSciSyncPoolNV<'dev> {
    #[inline]
    pub fn raw(&self) -> VkSemaphoreSciSyncPoolNV {
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
    pub fn table(&self) -> &SemaphoreSciSyncPoolNVDispatchTable {
        self.table
    }
    /// [`vkDestroySemaphoreSciSyncPoolNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphoreSciSyncPoolNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_external_sci_sync2`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `semaphorePool`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NV_external_sci_sync2")]
    #[inline(always)]
    pub fn vkDestroySemaphoreSciSyncPoolNV(
        &mut self,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroySemaphoreSciSyncPoolNV
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkSemaphoreSciSyncPoolNV::NULL;
    }
}
