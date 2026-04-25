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
#[cfg(feature = "VK_KHR_deferred_host_operations")]
#[derive(Debug, Clone)]
pub struct DeferredOperationKHRDispatchTable {
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkCreateDataGraphPipelinesARM: Option<PFN_vkCreateDataGraphPipelinesARM>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkBuildMicromapsEXT: Option<PFN_vkBuildMicromapsEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCopyMemoryToMicromapEXT: Option<PFN_vkCopyMemoryToMicromapEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCopyMicromapEXT: Option<PFN_vkCopyMicromapEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCopyMicromapToMemoryEXT: Option<PFN_vkCopyMicromapToMemoryEXT>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkBuildAccelerationStructuresKHR: Option<PFN_vkBuildAccelerationStructuresKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCopyAccelerationStructureKHR: Option<PFN_vkCopyAccelerationStructureKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCopyAccelerationStructureToMemoryKHR: Option<PFN_vkCopyAccelerationStructureToMemoryKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCopyMemoryToAccelerationStructureKHR: Option<PFN_vkCopyMemoryToAccelerationStructureKHR>,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub vkDeferredOperationJoinKHR: Option<PFN_vkDeferredOperationJoinKHR>,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub vkDestroyDeferredOperationKHR: Option<PFN_vkDestroyDeferredOperationKHR>,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub vkGetDeferredOperationMaxConcurrencyKHR: Option<PFN_vkGetDeferredOperationMaxConcurrencyKHR>,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub vkGetDeferredOperationResultKHR: Option<PFN_vkGetDeferredOperationResultKHR>,
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  pub vkCreateRayTracingPipelinesKHR: Option<PFN_vkCreateRayTracingPipelinesKHR>,
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl DeferredOperationKHRDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_ARM_data_graph")]
    vkCreateDataGraphPipelinesARM: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkBuildMicromapsEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCopyMemoryToMicromapEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCopyMicromapEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCopyMicromapToMemoryEXT: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkBuildAccelerationStructuresKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCopyAccelerationStructureKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCopyAccelerationStructureToMemoryKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCopyMemoryToAccelerationStructureKHR: None,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    vkDeferredOperationJoinKHR: None,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    vkDestroyDeferredOperationKHR: None,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    vkGetDeferredOperationMaxConcurrencyKHR: None,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    vkGetDeferredOperationResultKHR: None,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    vkCreateRayTracingPipelinesKHR: None,
  };
  pub fn load<F>(mut loader: F) -> Self
  where
    F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_ARM_data_graph")]
      vkCreateDataGraphPipelinesARM: loader(c"vkCreateDataGraphPipelinesARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkBuildMicromapsEXT: loader(c"vkBuildMicromapsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCopyMemoryToMicromapEXT: loader(c"vkCopyMemoryToMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCopyMicromapEXT: loader(c"vkCopyMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCopyMicromapToMemoryEXT: loader(c"vkCopyMicromapToMemoryEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkBuildAccelerationStructuresKHR: loader(c"vkBuildAccelerationStructuresKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCopyAccelerationStructureKHR: loader(c"vkCopyAccelerationStructureKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCopyAccelerationStructureToMemoryKHR: loader(
        c"vkCopyAccelerationStructureToMemoryKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCopyMemoryToAccelerationStructureKHR: loader(
        c"vkCopyMemoryToAccelerationStructureKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      vkDeferredOperationJoinKHR: loader(c"vkDeferredOperationJoinKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      vkDestroyDeferredOperationKHR: loader(c"vkDestroyDeferredOperationKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      vkGetDeferredOperationMaxConcurrencyKHR: loader(
        c"vkGetDeferredOperationMaxConcurrencyKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      vkGetDeferredOperationResultKHR: loader(c"vkGetDeferredOperationResultKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
      vkCreateRayTracingPipelinesKHR: loader(c"vkCreateRayTracingPipelinesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub struct DeferredOperationKHR<'dev> {
  pub(crate) raw: VkDeferredOperationKHR,
  pub(crate) parent: &'dev crate::device::Device<'dev>,
  pub(crate) table: &'dev DeferredOperationKHRDispatchTable,
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
unsafe impl<'dev> Send for DeferredOperationKHR<'dev> {}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
unsafe impl<'dev> Sync for DeferredOperationKHR<'dev> {}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl<'dev> Drop for DeferredOperationKHR<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    if let Some(destroy_fn) = self.table.vkDestroyDeferredOperationKHR {
      unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
    }
  }
}
#[cfg(feature = "VK_KHR_deferred_host_operations")]
impl<'dev> DeferredOperationKHR<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkDeferredOperationKHR {
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
  pub const fn table(&self) -> &DeferredOperationKHRDispatchTable {
    self.table
  }
  /// [`vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkCreateDataGraphPipelinesARM(
    &self,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkDataGraphPipelineCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCreateDataGraphPipelinesARM
        .unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pipelineCache,
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
  /// [`vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `infoCount`
  /// - `pInfos`: len: infoCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkBuildMicromapsEXT(
    &self,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkBuildMicromapsEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        infoCount,
        pInfos,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkCopyMemoryToMicromapEXT(
    &self,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkCopyMemoryToMicromapEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pInfo,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkCopyMicromapEXT(
    &self,
    pInfo: *const VkCopyMicromapInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkCopyMicromapEXT.unwrap_unchecked()(self.device().raw(), self.raw, pInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkCopyMicromapToMemoryEXT(
    &self,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkCopyMicromapToMemoryEXT.unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pInfo,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `infoCount`
  /// - `pInfos`: len: infoCount
  /// - `ppBuildRangeInfos`: len: infoCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkBuildAccelerationStructuresKHR(
    &self,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkBuildAccelerationStructuresKHR
        .unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        infoCount,
        pInfos,
        ppBuildRangeInfos,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkCopyAccelerationStructureKHR(
    &self,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCopyAccelerationStructureKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkCopyAccelerationStructureToMemoryKHR(
    &self,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCopyAccelerationStructureToMemoryKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkCopyMemoryToAccelerationStructureKHR(
    &self,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCopyMemoryToAccelerationStructureKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_deferred_host_operations`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `operation`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_THREAD_DONE_KHR`
  ///   - `VK_THREAD_IDLE_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  #[inline(always)]
  pub fn vkDeferredOperationJoinKHR(&self) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table).vkDeferredOperationJoinKHR.unwrap_unchecked()(self.device().raw(), self.raw)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_deferred_host_operations`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `operation`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  #[inline(always)]
  pub fn vkDestroyDeferredOperationKHR(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkDestroyDeferredOperationKHR
        .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
    }
    self.raw = VkDeferredOperationKHR::NULL;
  }
  /// [`vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_deferred_host_operations`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `operation`
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  #[inline(always)]
  pub fn vkGetDeferredOperationMaxConcurrencyKHR(&self) -> u32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetDeferredOperationMaxConcurrencyKHR
        .unwrap_unchecked()(self.device().raw(), self.raw)
    }
  }
  /// [`vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_deferred_host_operations`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `operation`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_NOT_READY`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  #[inline(always)]
  pub fn vkGetDeferredOperationResultKHR(&self) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDeferredOperationResultKHR
        .unwrap_unchecked()(self.device().raw(), self.raw)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_ray_tracing_pipeline`
  ///
  /// - **Allow No Queues:** True
  ///
  /// # Parameters
  /// - `device`
  /// - `deferredOperation`: optional: true
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
  ///   - `VK_OPERATION_DEFERRED_KHR`
  ///   - `VK_OPERATION_NOT_DEFERRED_KHR`
  ///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
  ///   - `VK_ERROR_NO_PIPELINE_MATCH`
  ///   - `VK_ERROR_OUT_OF_POOL_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  #[inline(always)]
  pub fn vkCreateRayTracingPipelinesKHR(
    &self,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCreateRayTracingPipelinesKHR
        .unwrap_unchecked()(
        self.device().raw(),
        self.raw,
        pipelineCache,
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
