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
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct PipelineCacheDispatchTable {
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    pub vkCreateExecutionGraphPipelinesAMDX: Option<PFN_vkCreateExecutionGraphPipelinesAMDX>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyPipelineCache: Option<PFN_vkDestroyPipelineCache>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkGetPipelineCacheData: Option<PFN_vkGetPipelineCacheData>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkMergePipelineCaches: Option<PFN_vkMergePipelineCaches>,
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub vkCreateRayTracingPipelinesNV: Option<PFN_vkCreateRayTracingPipelinesNV>,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl PipelineCacheDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_AMDX_shader_enqueue")]
        vkCreateExecutionGraphPipelinesAMDX: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyPipelineCache: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkGetPipelineCacheData: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkMergePipelineCaches: None,
        #[cfg(feature = "VK_NV_ray_tracing")]
        vkCreateRayTracingPipelinesNV: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_AMDX_shader_enqueue")]
            vkCreateExecutionGraphPipelinesAMDX: loader(
                c"vkCreateExecutionGraphPipelinesAMDX".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkDestroyPipelineCache: loader(c"vkDestroyPipelineCache".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkGetPipelineCacheData: loader(c"vkGetPipelineCacheData".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
            vkMergePipelineCaches: loader(c"vkMergePipelineCaches".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_NV_ray_tracing")]
            vkCreateRayTracingPipelinesNV: loader(c"vkCreateRayTracingPipelinesNV".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct PipelineCache<'dev> {
    pub(crate) raw: VkPipelineCache,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev PipelineCacheDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Send for PipelineCache<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
unsafe impl<'dev> Sync for PipelineCache<'dev> {}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for PipelineCache<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyPipelineCache {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> PipelineCache<'dev> {
    #[inline(always)]
    pub const fn raw(&self) -> VkPipelineCache {
        self.raw
    }
    #[inline(always)]
    pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline(always)]
    pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline(always)]
    pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline(always)]
    pub const fn table(&self) -> &PipelineCacheDispatchTable {
        self.table
    }
    /// [`vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html)
    ///
    /// Provided by:
    /// - `VK_AMDX_shader_enqueue`
    ///
    /// - **Allow No Queues:** True
    ///
    /// # Parameters
    /// - `device`
    /// - `pipelineCache`: optional: true
    /// - `createInfoCount`
    /// - `pCreateInfos`: len: createInfoCount
    /// - `pAllocator`: optional: true
    /// - `pPipelines`: len: createInfoCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    #[inline(always)]
    pub fn vkCreateExecutionGraphPipelinesAMDX(
        &self,
        createInfoCount: u32,
        pCreateInfos: *const VkExecutionGraphPipelineCreateInfoAMDX,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkCreateExecutionGraphPipelinesAMDX
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                createInfoCount,
                pCreateInfos,
                pAllocator,
                pPipelines,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `pipelineCache`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyPipelineCache(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyPipelineCache.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkPipelineCache::NULL;
    }
    /// [`vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `pipelineCache`
    /// - `pDataSize`: optional: pointer required, values optional if pointer not null
    /// - `pData`: optional: true, len: pDataSize
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_INCOMPLETE`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetPipelineCacheData(
        &self,
        pDataSize: *mut usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetPipelineCacheData.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pDataSize,
                pData,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `dstCache`
    /// - `srcCacheCount`
    /// - `pSrcCaches`: len: srcCacheCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkMergePipelineCaches(
        &self,
        srcCacheCount: u32,
        pSrcCaches: *const VkPipelineCache,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkMergePipelineCaches.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                srcCacheCount,
                pSrcCaches,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_ray_tracing`
    ///
    /// - **Allow No Queues:** True
    ///
    /// # Parameters
    /// - `device`
    /// - `pipelineCache`
    /// - `createInfoCount`
    /// - `pCreateInfos`: len: createInfoCount
    /// - `pAllocator`: optional: true
    /// - `pPipelines`: len: createInfoCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///   - `VK_ERROR_NO_PIPELINE_MATCH`
    ///   - `VK_ERROR_OUT_OF_POOL_MEMORY`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_NV_ray_tracing")]
    #[inline(always)]
    pub fn vkCreateRayTracingPipelinesNV(
        &self,
        createInfoCount: u32,
        pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkCreateRayTracingPipelinesNV
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                createInfoCount,
                pCreateInfos,
                pAllocator,
                pPipelines,
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
}
