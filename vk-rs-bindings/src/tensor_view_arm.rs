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
pub struct TensorViewARMDispatchTable {
    #[cfg(feature = "VK_ARM_tensors")]
    pub vkDestroyTensorViewARM: Option<PFN_vkDestroyTensorViewARM>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl TensorViewARMDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_ARM_tensors")]
        vkDestroyTensorViewARM: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_ARM_tensors")]
        {
            table.vkDestroyTensorViewARM = loader(c"vkDestroyTensorViewARM".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_ARM_tensors")]
pub struct TensorViewARM<'dev> {
    pub(crate) raw: VkTensorViewARM,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev TensorViewARMDispatchTable,
}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'dev> Send for TensorViewARM<'dev> {}
#[cfg(feature = "VK_ARM_tensors")]
unsafe impl<'dev> Sync for TensorViewARM<'dev> {}
#[cfg(feature = "VK_ARM_tensors")]
impl<'dev> Drop for TensorViewARM<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyTensorViewARM {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_ARM_tensors")]
impl<'dev> TensorViewARM<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkTensorViewARM {
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
    pub const fn table(&self) -> &TensorViewARMDispatchTable {
        self.table
    }
    /// [`vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_tensors`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `tensorView`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_ARM_tensors")]
    #[inline(always)]
    pub fn vkDestroyTensorViewARM(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyTensorViewARM.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkTensorViewARM::NULL;
    }
}
