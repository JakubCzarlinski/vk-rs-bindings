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
pub struct CudaFunctionNVDispatchTable {
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    pub vkDestroyCudaFunctionNV: Option<PFN_vkDestroyCudaFunctionNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl CudaFunctionNVDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NV_cuda_kernel_launch")]
        vkDestroyCudaFunctionNV: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_NV_cuda_kernel_launch")]
            vkDestroyCudaFunctionNV: loader(c"vkDestroyCudaFunctionNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub struct CudaFunctionNV<'dev> {
    pub(crate) raw: VkCudaFunctionNV,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev CudaFunctionNVDispatchTable,
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'dev> Send for CudaFunctionNV<'dev> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
unsafe impl<'dev> Sync for CudaFunctionNV<'dev> {}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'dev> Drop for CudaFunctionNV<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyCudaFunctionNV {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
impl<'dev> CudaFunctionNV<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkCudaFunctionNV {
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
    pub const fn table(&self) -> &CudaFunctionNVDispatchTable {
        self.table
    }
    /// [`vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_cuda_kernel_launch`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `function`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    #[inline(always)]
    pub fn vkDestroyCudaFunctionNV(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyCudaFunctionNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkCudaFunctionNV::NULL;
    }
}
