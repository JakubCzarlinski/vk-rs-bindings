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
pub struct DescriptorPoolDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkAllocateDescriptorSets: Option<PFN_vkAllocateDescriptorSets>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyDescriptorPool: Option<PFN_vkDestroyDescriptorPool>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkFreeDescriptorSets: Option<PFN_vkFreeDescriptorSets>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkResetDescriptorPool: Option<PFN_vkResetDescriptorPool>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DescriptorPoolDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkAllocateDescriptorSets: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyDescriptorPool: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkFreeDescriptorSets: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkResetDescriptorPool: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkAllocateDescriptorSets = loader(c"vkAllocateDescriptorSets".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkDestroyDescriptorPool = loader(c"vkDestroyDescriptorPool".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkFreeDescriptorSets = loader(c"vkFreeDescriptorSets".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkResetDescriptorPool = loader(c"vkResetDescriptorPool".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct DescriptorPool<'dev> {
    pub(crate) raw: VkDescriptorPool,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev DescriptorPoolDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for DescriptorPool<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyDescriptorPool {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> DescriptorPool<'dev> {
    #[inline]
    pub fn raw(&self) -> VkDescriptorPool {
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
    pub fn table(&self) -> &DescriptorPoolDispatchTable {
        self.table
    }
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline]
    pub fn vkAllocateDescriptorSets<'pool>(
        &'pool self,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    ) -> Result<alloc::vec::Vec<crate::descriptor_set::DescriptorSet<'pool>>, VkResult> {
        let count = unsafe { (*pAllocateInfo).descriptorSetCount };
        let mut raw_sets = alloc::vec::Vec::with_capacity(count as usize);
        let fp = unsafe { self.table.vkAllocateDescriptorSets.unwrap_unchecked() };
        let r = unsafe { fp(self.device().raw, pAllocateInfo, raw_sets.as_mut_ptr()) };
        if let Err(e) = {
            match r {
                VkResult::VK_SUCCESS => Ok(r),
                VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
                | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
                | VkResult::VK_ERROR_FRAGMENTED_POOL
                | VkResult::VK_ERROR_UNKNOWN => Err(r),
                #[cfg(feature = "VK_BASE_VERSION_1_0")]
                VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
                #[cfg(feature = "VK_BASE_VERSION_1_1")]
                VkResult::VK_ERROR_OUT_OF_POOL_MEMORY => Err(r),
                _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
            }
        } {
            return Err(e);
        }
        unsafe {
            raw_sets.set_len(count as usize);
        }
        Ok(
            raw_sets
                .into_iter()
                .map(|raw| crate::descriptor_set::DescriptorSet {
                    raw,
                    parent: self,
                    table: &self.device().descriptor_set_table,
                })
                .collect(),
        )
    }
    /// [`vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorPool`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyDescriptorPool(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyDescriptorPool
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkDescriptorPool::NULL;
    }
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline]
    pub fn vkFreeDescriptorSets(
        &self,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
    ) -> Result<VkResult, VkResult> {
        let fp = unsafe { self.table.vkFreeDescriptorSets.unwrap_unchecked() };
        let r = unsafe {
            fp(self.device().raw, self.raw, descriptorSetCount, pDescriptorSets)
        };
        if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
    }
    /// [`vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `descriptorPool`
    /// - `flags`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkResetDescriptorPool(
        &self,
        flags: VkDescriptorPoolResetFlags,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkResetDescriptorPool
                .unwrap_unchecked()(self.device().raw(), self.raw, flags)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
