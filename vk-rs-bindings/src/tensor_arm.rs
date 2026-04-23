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
pub struct TensorARMDispatchTable {
    #[cfg(feature = "VK_ARM_tensors")]
    pub vkDestroyTensorARM: Option<PFN_vkDestroyTensorARM>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl TensorARMDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_ARM_tensors")]
        vkDestroyTensorARM: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_ARM_tensors")]
        {
            table.vkDestroyTensorARM =
                loader(c"vkDestroyTensorARM".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
pub struct TensorARM<'dev> {
    pub(crate) raw: VkTensorARM,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev TensorARMDispatchTable,
}
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
unsafe impl<'dev> Send for TensorARM<'dev> {}
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
unsafe impl<'dev> Sync for TensorARM<'dev> {}
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
impl<'dev> Drop for TensorARM<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyTensorARM {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
impl<'dev> TensorARM<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkTensorARM {
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
    pub const fn table(&self) -> &TensorARMDispatchTable {
        self.table
    }
    /// [`vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_tensors`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `tensor`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_ARM_tensors")]
    #[inline(always)]
    pub fn vkDestroyTensorARM(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyTensorARM.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkTensorARM::NULL;
    }
}
