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
pub struct AccelerationStructureKHRDispatchTable {
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    pub vkDestroyAccelerationStructureKHR: Option<PFN_vkDestroyAccelerationStructureKHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl AccelerationStructureKHRDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_KHR_acceleration_structure")]
        vkDestroyAccelerationStructureKHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_KHR_acceleration_structure")]
        {
            table.vkDestroyAccelerationStructureKHR =
                loader(c"vkDestroyAccelerationStructureKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub struct AccelerationStructureKHR<'dev> {
    pub(crate) raw: VkAccelerationStructureKHR,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev AccelerationStructureKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl<'dev> Drop for AccelerationStructureKHR<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyAccelerationStructureKHR {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_KHR_acceleration_structure")]
impl<'dev> AccelerationStructureKHR<'dev> {
    #[inline]
    pub fn raw(&self) -> VkAccelerationStructureKHR {
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
    pub fn table(&self) -> &AccelerationStructureKHRDispatchTable {
        self.table
    }
    /// [`vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_acceleration_structure`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `accelerationStructure`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    #[inline(always)]
    pub fn vkDestroyAccelerationStructureKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyAccelerationStructureKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkAccelerationStructureKHR::NULL;
    }
}
