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
pub struct DataGraphPipelineSessionARMDispatchTable {
    #[cfg(feature = "VK_ARM_data_graph")]
    pub vkDestroyDataGraphPipelineSessionARM: Option<PFN_vkDestroyDataGraphPipelineSessionARM>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DataGraphPipelineSessionARMDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_ARM_data_graph")]
        vkDestroyDataGraphPipelineSessionARM: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_ARM_data_graph")]
        {
            table.vkDestroyDataGraphPipelineSessionARM =
                loader(c"vkDestroyDataGraphPipelineSessionARM".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_ARM_data_graph")]
pub struct DataGraphPipelineSessionARM<'dev> {
    pub(crate) raw: VkDataGraphPipelineSessionARM,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev DataGraphPipelineSessionARMDispatchTable,
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'dev> Drop for DataGraphPipelineSessionARM<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyDataGraphPipelineSessionARM {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_ARM_data_graph")]
impl<'dev> DataGraphPipelineSessionARM<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDataGraphPipelineSessionARM {
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
    pub fn table(&self) -> &DataGraphPipelineSessionARMDispatchTable {
        self.table
    }
    /// [`vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_data_graph`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `session`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_ARM_data_graph")]
    #[inline(always)]
    pub fn vkDestroyDataGraphPipelineSessionARM(
        &mut self,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyDataGraphPipelineSessionARM
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkDataGraphPipelineSessionARM::NULL;
    }
}
