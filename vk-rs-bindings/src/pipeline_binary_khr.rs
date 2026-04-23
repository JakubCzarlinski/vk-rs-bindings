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
pub struct PipelineBinaryKHRDispatchTable {
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    pub vkDestroyPipelineBinaryKHR: Option<PFN_vkDestroyPipelineBinaryKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl PipelineBinaryKHRDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_KHR_pipeline_binary")]
        vkDestroyPipelineBinaryKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_KHR_pipeline_binary")]
        {
            table.vkDestroyPipelineBinaryKHR = loader(c"vkDestroyPipelineBinaryKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub struct PipelineBinaryKHR<'dev> {
    pub(crate) raw: VkPipelineBinaryKHR,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev PipelineBinaryKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_pipeline_binary")]
unsafe impl<'dev> Send for PipelineBinaryKHR<'dev> {}
#[cfg(feature = "VK_KHR_pipeline_binary")]
unsafe impl<'dev> Sync for PipelineBinaryKHR<'dev> {}
#[cfg(feature = "VK_KHR_pipeline_binary")]
impl<'dev> Drop for PipelineBinaryKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyPipelineBinaryKHR {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_pipeline_binary")]
impl<'dev> PipelineBinaryKHR<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkPipelineBinaryKHR {
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
    pub const fn table(&self) -> &PipelineBinaryKHRDispatchTable {
        self.table
    }
    /// [`vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_pipeline_binary`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipelineBinary`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    #[inline(always)]
    pub fn vkDestroyPipelineBinaryKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyPipelineBinaryKHR.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkPipelineBinaryKHR::NULL;
    }
}
