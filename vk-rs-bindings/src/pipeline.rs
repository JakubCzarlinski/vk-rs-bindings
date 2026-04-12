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
pub struct PipelineDispatchTable {
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    pub vkGetExecutionGraphPipelineNodeIndexAMDX:
        Option<PFN_vkGetExecutionGraphPipelineNodeIndexAMDX>,
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    pub vkGetExecutionGraphPipelineScratchSizeAMDX:
        Option<PFN_vkGetExecutionGraphPipelineScratchSizeAMDX>,
    #[cfg(feature = "VK_AMD_shader_info")]
    pub vkGetShaderInfoAMD: Option<PFN_vkGetShaderInfoAMD>,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyPipeline: Option<PFN_vkDestroyPipeline>,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub vkGetRayTracingCaptureReplayShaderGroupHandlesKHR:
        Option<PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub vkGetRayTracingShaderGroupHandlesKHR: Option<PFN_vkGetRayTracingShaderGroupHandlesKHR>,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    pub vkGetRayTracingShaderGroupStackSizeKHR: Option<PFN_vkGetRayTracingShaderGroupStackSizeKHR>,
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub vkCompileDeferredNV: Option<PFN_vkCompileDeferredNV>,
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub vkGetRayTracingShaderGroupHandlesNV: Option<PFN_vkGetRayTracingShaderGroupHandlesNV>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl PipelineDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_AMDX_shader_enqueue")]
        vkGetExecutionGraphPipelineNodeIndexAMDX: None,
        #[cfg(feature = "VK_AMDX_shader_enqueue")]
        vkGetExecutionGraphPipelineScratchSizeAMDX: None,
        #[cfg(feature = "VK_AMD_shader_info")]
        vkGetShaderInfoAMD: None,
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyPipeline: None,
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        vkGetRayTracingCaptureReplayShaderGroupHandlesKHR: None,
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        vkGetRayTracingShaderGroupHandlesKHR: None,
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        vkGetRayTracingShaderGroupStackSizeKHR: None,
        #[cfg(feature = "VK_NV_ray_tracing")]
        vkCompileDeferredNV: None,
        #[cfg(feature = "VK_NV_ray_tracing")]
        vkGetRayTracingShaderGroupHandlesNV: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_AMDX_shader_enqueue")]
        {
            table.vkGetExecutionGraphPipelineNodeIndexAMDX =
                loader(c"vkGetExecutionGraphPipelineNodeIndexAMDX".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_AMDX_shader_enqueue")]
        {
            table.vkGetExecutionGraphPipelineScratchSizeAMDX =
                loader(c"vkGetExecutionGraphPipelineScratchSizeAMDX".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_AMD_shader_info")]
        {
            table.vkGetShaderInfoAMD =
                loader(c"vkGetShaderInfoAMD".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkDestroyPipeline =
                loader(c"vkDestroyPipeline".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        {
            table.vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
                loader(c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        {
            table.vkGetRayTracingShaderGroupHandlesKHR =
                loader(c"vkGetRayTracingShaderGroupHandlesKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
        {
            table.vkGetRayTracingShaderGroupStackSizeKHR =
                loader(c"vkGetRayTracingShaderGroupStackSizeKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            table.vkCompileDeferredNV =
                loader(c"vkCompileDeferredNV".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            table.vkGetRayTracingShaderGroupHandlesNV =
                loader(c"vkGetRayTracingShaderGroupHandlesNV".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct Pipeline<'dev> {
    pub(crate) raw: VkPipeline,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev PipelineDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for Pipeline<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyPipeline {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Pipeline<'dev> {
    #[inline]
    pub fn raw(&self) -> VkPipeline {
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
    pub fn table(&self) -> &PipelineDispatchTable {
        self.table
    }
    /// [`vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html)
    ///
    /// Provided by:
    /// - `VK_AMDX_shader_enqueue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `executionGraph`
    /// - `pNodeInfo`
    /// - `pNodeIndex`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    #[inline(always)]
    pub fn vkGetExecutionGraphPipelineNodeIndexAMDX(
        &self,
        pNodeInfo: *const VkPipelineShaderStageNodeCreateInfoAMDX,
        pNodeIndex: *mut u32,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetExecutionGraphPipelineNodeIndexAMDX
                .unwrap_unchecked()(self.device().raw(), self.raw, pNodeInfo, pNodeIndex)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html)
    ///
    /// Provided by:
    /// - `VK_AMDX_shader_enqueue`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `executionGraph`
    /// - `pSizeInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    #[inline(always)]
    pub fn vkGetExecutionGraphPipelineScratchSizeAMDX(
        &self,
        pSizeInfo: *mut VkExecutionGraphPipelineScratchSizeAMDX,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetExecutionGraphPipelineScratchSizeAMDX
                .unwrap_unchecked()(self.device().raw(), self.raw, pSizeInfo)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html)
    ///
    /// Provided by:
    /// - `VK_AMD_shader_info`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `shaderStage`
    /// - `infoType`
    /// - `pInfoSize`: optional: pointer required, values optional if pointer not null
    /// - `pInfo`: optional: true, len: pInfoSize
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///   - VK_INCOMPLETE
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_FEATURE_NOT_PRESENT
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_AMD_shader_info")]
    #[inline(always)]
    pub fn vkGetShaderInfoAMD(
        &self,
        shaderStage: VkShaderStageFlagBits,
        infoType: VkShaderInfoTypeAMD,
        pInfoSize: *mut usize,
        pInfo: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetShaderInfoAMD.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                shaderStage,
                infoType,
                pInfoSize,
                pInfo,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_FEATURE_NOT_PRESENT
            | VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyPipeline(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyPipeline.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkPipeline::NULL;
    }
    /// [`vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_ray_tracing_pipeline`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `firstGroup`
    /// - `groupCount`
    /// - `dataSize`
    /// - `pData`: len: dataSize
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    #[inline(always)]
    pub fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(
        &self,
        firstGroup: u32,
        groupCount: u32,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetRayTracingCaptureReplayShaderGroupHandlesKHR
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstGroup,
                groupCount,
                dataSize,
                pData,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_ray_tracing_pipeline`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `firstGroup`
    /// - `groupCount`
    /// - `dataSize`
    /// - `pData`: len: dataSize
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    #[inline(always)]
    pub fn vkGetRayTracingShaderGroupHandlesKHR(
        &self,
        firstGroup: u32,
        groupCount: u32,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetRayTracingShaderGroupHandlesKHR
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstGroup,
                groupCount,
                dataSize,
                pData,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_ray_tracing_pipeline`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `group`
    /// - `groupShader`
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    #[inline(always)]
    pub fn vkGetRayTracingShaderGroupStackSizeKHR(
        &self,
        group: u32,
        groupShader: VkShaderGroupShaderKHR,
    ) -> VkDeviceSize {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetRayTracingShaderGroupStackSizeKHR
                .unwrap_unchecked()(self.device().raw(), self.raw, group, groupShader)
        }
    }
    /// [`vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html)
    ///
    /// Provided by:
    /// - `VK_NV_ray_tracing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `shader`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_NV_ray_tracing")]
    #[inline(always)]
    pub fn vkCompileDeferredNV(&self, shader: u32) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkCompileDeferredNV.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                shader,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
    /// [`vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html)
    ///
    /// Provided by:
    /// - `VK_NV_ray_tracing`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `pipeline`
    /// - `firstGroup`
    /// - `groupCount`
    /// - `dataSize`
    /// - `pData`: len: dataSize
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_NV_ray_tracing")]
    #[inline(always)]
    pub fn vkGetRayTracingShaderGroupHandlesNV(
        &self,
        firstGroup: u32,
        groupCount: u32,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetRayTracingShaderGroupHandlesNV
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                firstGroup,
                groupCount,
                dataSize,
                pData,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
}
