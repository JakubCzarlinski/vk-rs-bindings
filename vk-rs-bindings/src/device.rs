//! Device-tier dispatch table and safe [`Device`] wrapper.
#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
use crate::commands::*;
use crate::enums::*;
use crate::instance::Instance;
use crate::types::*;
use core::ffi::{c_char, c_void};
/// Raw device-tier function pointer table.
///
/// Fields are `Option<PFN_*>`; `None` means absent at load time.
/// Use [`Device`] for the safe API and [`CommandBuffer`] for `vkCmd*`.
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct DeviceDispatchTable {
  #[cfg(feature = "VKSC_VERSION_1_0")]
  pub vkGetFaultData: Option<PFN_vkGetFaultData>,
  #[cfg(feature = "VK_AMD_anti_lag")]
  pub vkAntiLagUpdateAMD: Option<PFN_vkAntiLagUpdateAMD>,
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  pub vkGetAndroidHardwareBufferPropertiesANDROID:
    Option<PFN_vkGetAndroidHardwareBufferPropertiesANDROID>,
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  pub vkGetMemoryAndroidHardwareBufferANDROID: Option<PFN_vkGetMemoryAndroidHardwareBufferANDROID>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkBindDataGraphPipelineSessionMemoryARM: Option<PFN_vkBindDataGraphPipelineSessionMemoryARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkCreateDataGraphPipelineSessionARM: Option<PFN_vkCreateDataGraphPipelineSessionARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetDataGraphPipelineAvailablePropertiesARM:
    Option<PFN_vkGetDataGraphPipelineAvailablePropertiesARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetDataGraphPipelinePropertiesARM: Option<PFN_vkGetDataGraphPipelinePropertiesARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetDataGraphPipelineSessionBindPointRequirementsARM:
    Option<PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetDataGraphPipelineSessionMemoryRequirementsARM:
    Option<PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM>,
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  pub vkCreateShaderInstrumentationARM: Option<PFN_vkCreateShaderInstrumentationARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkBindTensorMemoryARM: Option<PFN_vkBindTensorMemoryARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkCreateTensorARM: Option<PFN_vkCreateTensorARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkCreateTensorViewARM: Option<PFN_vkCreateTensorViewARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkGetDeviceTensorMemoryRequirementsARM: Option<PFN_vkGetDeviceTensorMemoryRequirementsARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkGetTensorMemoryRequirementsARM: Option<PFN_vkGetTensorMemoryRequirementsARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkGetTensorOpaqueCaptureDescriptorDataARM:
    Option<PFN_vkGetTensorOpaqueCaptureDescriptorDataARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkGetTensorViewOpaqueCaptureDescriptorDataARM:
    Option<PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkAllocateMemory: Option<PFN_vkAllocateMemory>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateBuffer: Option<PFN_vkCreateBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateCommandPool: Option<PFN_vkCreateCommandPool>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateFence: Option<PFN_vkCreateFence>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateImage: Option<PFN_vkCreateImage>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateImageView: Option<PFN_vkCreateImageView>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateQueryPool: Option<PFN_vkCreateQueryPool>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateSemaphore: Option<PFN_vkCreateSemaphore>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkDestroyDevice: Option<PFN_vkDestroyDevice>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkDeviceWaitIdle: Option<PFN_vkDeviceWaitIdle>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkFlushMappedMemoryRanges: Option<PFN_vkFlushMappedMemoryRanges>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetDeviceQueue: Option<PFN_vkGetDeviceQueue>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkInvalidateMappedMemoryRanges: Option<PFN_vkInvalidateMappedMemoryRanges>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkResetFences: Option<PFN_vkResetFences>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkWaitForFences: Option<PFN_vkWaitForFences>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkBindBufferMemory2: Option<PFN_vkBindBufferMemory2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkBindImageMemory2: Option<PFN_vkBindImageMemory2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetBufferMemoryRequirements2: Option<PFN_vkGetBufferMemoryRequirements2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetDeviceGroupPeerMemoryFeatures: Option<PFN_vkGetDeviceGroupPeerMemoryFeatures>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetDeviceQueue2: Option<PFN_vkGetDeviceQueue2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetImageMemoryRequirements2: Option<PFN_vkGetImageMemoryRequirements2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetImageSparseMemoryRequirements2: Option<PFN_vkGetImageSparseMemoryRequirements2>,
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  pub vkGetBufferDeviceAddress: Option<PFN_vkGetBufferDeviceAddress>,
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  pub vkGetBufferOpaqueCaptureAddress: Option<PFN_vkGetBufferOpaqueCaptureAddress>,
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  pub vkGetDeviceMemoryOpaqueCaptureAddress: Option<PFN_vkGetDeviceMemoryOpaqueCaptureAddress>,
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  pub vkSignalSemaphore: Option<PFN_vkSignalSemaphore>,
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  pub vkWaitSemaphores: Option<PFN_vkWaitSemaphores>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCreatePrivateDataSlot: Option<PFN_vkCreatePrivateDataSlot>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkGetDeviceBufferMemoryRequirements: Option<PFN_vkGetDeviceBufferMemoryRequirements>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkGetDeviceImageMemoryRequirements: Option<PFN_vkGetDeviceImageMemoryRequirements>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkGetDeviceImageSparseMemoryRequirements:
    Option<PFN_vkGetDeviceImageSparseMemoryRequirements>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkGetPrivateData: Option<PFN_vkGetPrivateData>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkSetPrivateData: Option<PFN_vkSetPrivateData>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkCopyImageToImage: Option<PFN_vkCopyImageToImage>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkCopyImageToMemory: Option<PFN_vkCopyImageToMemory>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkCopyMemoryToImage: Option<PFN_vkCopyMemoryToImage>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkGetDeviceImageSubresourceLayout: Option<PFN_vkGetDeviceImageSubresourceLayout>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkMapMemory2: Option<PFN_vkMapMemory2>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkTransitionImageLayout: Option<PFN_vkTransitionImageLayout>,
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  pub vkUnmapMemory2: Option<PFN_vkUnmapMemory2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateBufferView: Option<PFN_vkCreateBufferView>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateComputePipelines: Option<PFN_vkCreateComputePipelines>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateDescriptorPool: Option<PFN_vkCreateDescriptorPool>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateDescriptorSetLayout: Option<PFN_vkCreateDescriptorSetLayout>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateEvent: Option<PFN_vkCreateEvent>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreatePipelineCache: Option<PFN_vkCreatePipelineCache>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreatePipelineLayout: Option<PFN_vkCreatePipelineLayout>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateSampler: Option<PFN_vkCreateSampler>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCreateShaderModule: Option<PFN_vkCreateShaderModule>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkUpdateDescriptorSets: Option<PFN_vkUpdateDescriptorSets>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkCreateDescriptorUpdateTemplate: Option<PFN_vkCreateDescriptorUpdateTemplate>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkCreateSamplerYcbcrConversion: Option<PFN_vkCreateSamplerYcbcrConversion>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkGetDescriptorSetLayoutSupport: Option<PFN_vkGetDescriptorSetLayoutSupport>,
  #[cfg(feature = "VK_EXT_buffer_device_address")]
  pub vkGetBufferDeviceAddressEXT: Option<PFN_vkGetBufferDeviceAddressEXT>,
  #[cfg(feature = "VK_EXT_calibrated_timestamps")]
  pub vkGetCalibratedTimestampsEXT: Option<PFN_vkGetCalibratedTimestampsEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkDebugMarkerSetObjectNameEXT: Option<PFN_vkDebugMarkerSetObjectNameEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkDebugMarkerSetObjectTagEXT: Option<PFN_vkDebugMarkerSetObjectTagEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkSetDebugUtilsObjectNameEXT: Option<PFN_vkSetDebugUtilsObjectNameEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkSetDebugUtilsObjectTagEXT: Option<PFN_vkSetDebugUtilsObjectTagEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT:
    Option<PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetBufferOpaqueCaptureDescriptorDataEXT:
    Option<PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetDescriptorEXT: Option<PFN_vkGetDescriptorEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetImageOpaqueCaptureDescriptorDataEXT:
    Option<PFN_vkGetImageOpaqueCaptureDescriptorDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetImageViewOpaqueCaptureDescriptorDataEXT:
    Option<PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkGetSamplerOpaqueCaptureDescriptorDataEXT:
    Option<PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkGetImageOpaqueCaptureDataEXT: Option<PFN_vkGetImageOpaqueCaptureDataEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkGetTensorOpaqueCaptureDataARM: Option<PFN_vkGetTensorOpaqueCaptureDataARM>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkRegisterCustomBorderColorEXT: Option<PFN_vkRegisterCustomBorderColorEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkUnregisterCustomBorderColorEXT: Option<PFN_vkUnregisterCustomBorderColorEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkWriteResourceDescriptorsEXT: Option<PFN_vkWriteResourceDescriptorsEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkWriteSamplerDescriptorsEXT: Option<PFN_vkWriteSamplerDescriptorsEXT>,
  #[cfg(feature = "VK_EXT_device_fault")]
  pub vkGetDeviceFaultInfoEXT: Option<PFN_vkGetDeviceFaultInfoEXT>,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkCreateIndirectCommandsLayoutEXT: Option<PFN_vkCreateIndirectCommandsLayoutEXT>,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkCreateIndirectExecutionSetEXT: Option<PFN_vkCreateIndirectExecutionSetEXT>,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkGetGeneratedCommandsMemoryRequirementsEXT:
    Option<PFN_vkGetGeneratedCommandsMemoryRequirementsEXT>,
  #[cfg(feature = "VK_EXT_display_control")]
  pub vkRegisterDeviceEventEXT: Option<PFN_vkRegisterDeviceEventEXT>,
  #[cfg(feature = "VK_EXT_external_memory_host")]
  pub vkGetMemoryHostPointerPropertiesEXT: Option<PFN_vkGetMemoryHostPointerPropertiesEXT>,
  #[cfg(feature = "VK_EXT_external_memory_metal")]
  pub vkGetMemoryMetalHandleEXT: Option<PFN_vkGetMemoryMetalHandleEXT>,
  #[cfg(feature = "VK_EXT_external_memory_metal")]
  pub vkGetMemoryMetalHandlePropertiesEXT: Option<PFN_vkGetMemoryMetalHandlePropertiesEXT>,
  #[cfg(feature = "VK_EXT_full_screen_exclusive")]
  pub vkGetDeviceGroupSurfacePresentModes2EXT: Option<PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
  #[cfg(feature = "VK_EXT_hdr_metadata")]
  pub vkSetHdrMetadataEXT: Option<PFN_vkSetHdrMetadataEXT>,
  #[cfg(feature = "VK_EXT_host_image_copy")]
  pub vkCopyImageToImageEXT: Option<PFN_vkCopyImageToImageEXT>,
  #[cfg(feature = "VK_EXT_host_image_copy")]
  pub vkCopyImageToMemoryEXT: Option<PFN_vkCopyImageToMemoryEXT>,
  #[cfg(feature = "VK_EXT_host_image_copy")]
  pub vkCopyMemoryToImageEXT: Option<PFN_vkCopyMemoryToImageEXT>,
  #[cfg(feature = "VK_EXT_host_image_copy")]
  pub vkTransitionImageLayoutEXT: Option<PFN_vkTransitionImageLayoutEXT>,
  #[cfg(feature = "VK_EXT_metal_objects")]
  pub vkExportMetalObjectsEXT: Option<PFN_vkExportMetalObjectsEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCreateMicromapEXT: Option<PFN_vkCreateMicromapEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkGetDeviceMicromapCompatibilityEXT: Option<PFN_vkGetDeviceMicromapCompatibilityEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkGetMicromapBuildSizesEXT: Option<PFN_vkGetMicromapBuildSizesEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkWriteMicromapsPropertiesEXT: Option<PFN_vkWriteMicromapsPropertiesEXT>,
  #[cfg(feature = "VK_EXT_pipeline_properties")]
  pub vkGetPipelinePropertiesEXT: Option<PFN_vkGetPipelinePropertiesEXT>,
  #[cfg(feature = "VK_EXT_present_timing")]
  pub vkGetPastPresentationTimingEXT: Option<PFN_vkGetPastPresentationTimingEXT>,
  #[cfg(feature = "VK_EXT_private_data")]
  pub vkCreatePrivateDataSlotEXT: Option<PFN_vkCreatePrivateDataSlotEXT>,
  #[cfg(feature = "VK_EXT_private_data")]
  pub vkGetPrivateDataEXT: Option<PFN_vkGetPrivateDataEXT>,
  #[cfg(feature = "VK_EXT_private_data")]
  pub vkSetPrivateDataEXT: Option<PFN_vkSetPrivateDataEXT>,
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  pub vkGetShaderModuleCreateInfoIdentifierEXT:
    Option<PFN_vkGetShaderModuleCreateInfoIdentifierEXT>,
  #[cfg(feature = "VK_EXT_shader_object")]
  pub vkCreateShadersEXT: Option<PFN_vkCreateShadersEXT>,
  #[cfg(feature = "VK_EXT_swapchain_maintenance1")]
  pub vkReleaseSwapchainImagesEXT: Option<PFN_vkReleaseSwapchainImagesEXT>,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub vkCreateValidationCacheEXT: Option<PFN_vkCreateValidationCacheEXT>,
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  pub vkCreateBufferCollectionFUCHSIA: Option<PFN_vkCreateBufferCollectionFUCHSIA>,
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  pub vkGetMemoryZirconHandleFUCHSIA: Option<PFN_vkGetMemoryZirconHandleFUCHSIA>,
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  pub vkGetMemoryZirconHandlePropertiesFUCHSIA:
    Option<PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA>,
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  pub vkGetSemaphoreZirconHandleFUCHSIA: Option<PFN_vkGetSemaphoreZirconHandleFUCHSIA>,
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  pub vkImportSemaphoreZirconHandleFUCHSIA: Option<PFN_vkImportSemaphoreZirconHandleFUCHSIA>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCreateFramebuffer: Option<PFN_vkCreateFramebuffer>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCreateGraphicsPipelines: Option<PFN_vkCreateGraphicsPipelines>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCreateRenderPass: Option<PFN_vkCreateRenderPass>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCreateRenderPass2: Option<PFN_vkCreateRenderPass2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  pub vkGetRenderingAreaGranularity: Option<PFN_vkGetRenderingAreaGranularity>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkAcquirePerformanceConfigurationINTEL: Option<PFN_vkAcquirePerformanceConfigurationINTEL>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkGetPerformanceParameterINTEL: Option<PFN_vkGetPerformanceParameterINTEL>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkInitializePerformanceApiINTEL: Option<PFN_vkInitializePerformanceApiINTEL>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkUninitializePerformanceApiINTEL: Option<PFN_vkUninitializePerformanceApiINTEL>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCreateAccelerationStructureKHR: Option<PFN_vkCreateAccelerationStructureKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkGetAccelerationStructureBuildSizesKHR: Option<PFN_vkGetAccelerationStructureBuildSizesKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkGetAccelerationStructureDeviceAddressKHR:
    Option<PFN_vkGetAccelerationStructureDeviceAddressKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkGetDeviceAccelerationStructureCompatibilityKHR:
    Option<PFN_vkGetDeviceAccelerationStructureCompatibilityKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkWriteAccelerationStructuresPropertiesKHR:
    Option<PFN_vkWriteAccelerationStructuresPropertiesKHR>,
  #[cfg(feature = "VK_KHR_bind_memory2")]
  pub vkBindBufferMemory2KHR: Option<PFN_vkBindBufferMemory2KHR>,
  #[cfg(feature = "VK_KHR_bind_memory2")]
  pub vkBindImageMemory2KHR: Option<PFN_vkBindImageMemory2KHR>,
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  pub vkGetBufferDeviceAddressKHR: Option<PFN_vkGetBufferDeviceAddressKHR>,
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  pub vkGetBufferOpaqueCaptureAddressKHR: Option<PFN_vkGetBufferOpaqueCaptureAddressKHR>,
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  pub vkGetDeviceMemoryOpaqueCaptureAddressKHR:
    Option<PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>,
  #[cfg(feature = "VK_KHR_calibrated_timestamps")]
  pub vkGetCalibratedTimestampsKHR: Option<PFN_vkGetCalibratedTimestampsKHR>,
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  pub vkCreateRenderPass2KHR: Option<PFN_vkCreateRenderPass2KHR>,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub vkCreateDeferredOperationKHR: Option<PFN_vkCreateDeferredOperationKHR>,
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  pub vkCreateDescriptorUpdateTemplateKHR: Option<PFN_vkCreateDescriptorUpdateTemplateKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCreateAccelerationStructure2KHR: Option<PFN_vkCreateAccelerationStructure2KHR>,
  #[cfg(feature = "VK_KHR_device_fault")]
  pub vkGetDeviceFaultDebugInfoKHR: Option<PFN_vkGetDeviceFaultDebugInfoKHR>,
  #[cfg(feature = "VK_KHR_device_fault")]
  pub vkGetDeviceFaultReportsKHR: Option<PFN_vkGetDeviceFaultReportsKHR>,
  #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
  pub vkAcquireNextImage2KHR: Option<PFN_vkAcquireNextImage2KHR>,
  #[cfg(feature = "VK_KHR_device_group")]
  pub vkGetDeviceGroupPeerMemoryFeaturesKHR: Option<PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR>,
  #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
  pub vkGetDeviceGroupPresentCapabilitiesKHR: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_display_swapchain")]
  pub vkCreateSharedSwapchainsKHR: Option<PFN_vkCreateSharedSwapchainsKHR>,
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  pub vkGetFenceFdKHR: Option<PFN_vkGetFenceFdKHR>,
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  pub vkImportFenceFdKHR: Option<PFN_vkImportFenceFdKHR>,
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  pub vkGetFenceWin32HandleKHR: Option<PFN_vkGetFenceWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  pub vkImportFenceWin32HandleKHR: Option<PFN_vkImportFenceWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  pub vkGetMemoryFdKHR: Option<PFN_vkGetMemoryFdKHR>,
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  pub vkGetMemoryFdPropertiesKHR: Option<PFN_vkGetMemoryFdPropertiesKHR>,
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  pub vkGetMemoryWin32HandleKHR: Option<PFN_vkGetMemoryWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  pub vkGetMemoryWin32HandlePropertiesKHR: Option<PFN_vkGetMemoryWin32HandlePropertiesKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  pub vkGetSemaphoreFdKHR: Option<PFN_vkGetSemaphoreFdKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  pub vkImportSemaphoreFdKHR: Option<PFN_vkImportSemaphoreFdKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  pub vkGetSemaphoreWin32HandleKHR: Option<PFN_vkGetSemaphoreWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  pub vkImportSemaphoreWin32HandleKHR: Option<PFN_vkImportSemaphoreWin32HandleKHR>,
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetBufferMemoryRequirements2KHR: Option<PFN_vkGetBufferMemoryRequirements2KHR>,
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetImageMemoryRequirements2KHR: Option<PFN_vkGetImageMemoryRequirements2KHR>,
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  pub vkGetImageSparseMemoryRequirements2KHR: Option<PFN_vkGetImageSparseMemoryRequirements2KHR>,
  #[cfg(feature = "VK_KHR_maintenance3")]
  pub vkGetDescriptorSetLayoutSupportKHR: Option<PFN_vkGetDescriptorSetLayoutSupportKHR>,
  #[cfg(feature = "VK_KHR_maintenance4")]
  pub vkGetDeviceBufferMemoryRequirementsKHR: Option<PFN_vkGetDeviceBufferMemoryRequirementsKHR>,
  #[cfg(feature = "VK_KHR_maintenance4")]
  pub vkGetDeviceImageMemoryRequirementsKHR: Option<PFN_vkGetDeviceImageMemoryRequirementsKHR>,
  #[cfg(feature = "VK_KHR_maintenance4")]
  pub vkGetDeviceImageSparseMemoryRequirementsKHR:
    Option<PFN_vkGetDeviceImageSparseMemoryRequirementsKHR>,
  #[cfg(feature = "VK_KHR_maintenance5")]
  pub vkGetDeviceImageSubresourceLayoutKHR: Option<PFN_vkGetDeviceImageSubresourceLayoutKHR>,
  #[cfg(feature = "VK_KHR_maintenance5")]
  pub vkGetRenderingAreaGranularityKHR: Option<PFN_vkGetRenderingAreaGranularityKHR>,
  #[cfg(feature = "VK_KHR_map_memory2")]
  pub vkMapMemory2KHR: Option<PFN_vkMapMemory2KHR>,
  #[cfg(feature = "VK_KHR_map_memory2")]
  pub vkUnmapMemory2KHR: Option<PFN_vkUnmapMemory2KHR>,
  #[cfg(feature = "VK_KHR_performance_query")]
  pub vkAcquireProfilingLockKHR: Option<PFN_vkAcquireProfilingLockKHR>,
  #[cfg(feature = "VK_KHR_performance_query")]
  pub vkReleaseProfilingLockKHR: Option<PFN_vkReleaseProfilingLockKHR>,
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  pub vkCreatePipelineBinariesKHR: Option<PFN_vkCreatePipelineBinariesKHR>,
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  pub vkGetPipelineBinaryDataKHR: Option<PFN_vkGetPipelineBinaryDataKHR>,
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  pub vkGetPipelineKeyKHR: Option<PFN_vkGetPipelineKeyKHR>,
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  pub vkReleaseCapturedPipelineDataKHR: Option<PFN_vkReleaseCapturedPipelineDataKHR>,
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  pub vkGetPipelineExecutableInternalRepresentationsKHR:
    Option<PFN_vkGetPipelineExecutableInternalRepresentationsKHR>,
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  pub vkGetPipelineExecutablePropertiesKHR: Option<PFN_vkGetPipelineExecutablePropertiesKHR>,
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  pub vkGetPipelineExecutableStatisticsKHR: Option<PFN_vkGetPipelineExecutableStatisticsKHR>,
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  pub vkCreateSamplerYcbcrConversionKHR: Option<PFN_vkCreateSamplerYcbcrConversionKHR>,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub vkCreateSwapchainKHR: Option<PFN_vkCreateSwapchainKHR>,
  #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
  pub vkReleaseSwapchainImagesKHR: Option<PFN_vkReleaseSwapchainImagesKHR>,
  #[cfg(feature = "VK_KHR_timeline_semaphore")]
  pub vkSignalSemaphoreKHR: Option<PFN_vkSignalSemaphoreKHR>,
  #[cfg(feature = "VK_KHR_timeline_semaphore")]
  pub vkWaitSemaphoresKHR: Option<PFN_vkWaitSemaphoresKHR>,
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  pub vkGetEncodedVideoSessionParametersKHR: Option<PFN_vkGetEncodedVideoSessionParametersKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkCreateVideoSessionKHR: Option<PFN_vkCreateVideoSessionKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkCreateVideoSessionParametersKHR: Option<PFN_vkCreateVideoSessionParametersKHR>,
  #[cfg(feature = "VK_NVX_binary_import")]
  pub vkCreateCuFunctionNVX: Option<PFN_vkCreateCuFunctionNVX>,
  #[cfg(feature = "VK_NVX_binary_import")]
  pub vkCreateCuModuleNVX: Option<PFN_vkCreateCuModuleNVX>,
  #[cfg(feature = "VK_NVX_image_view_handle")]
  pub vkGetDeviceCombinedImageSamplerIndexNVX: Option<PFN_vkGetDeviceCombinedImageSamplerIndexNVX>,
  #[cfg(feature = "VK_NVX_image_view_handle")]
  pub vkGetImageViewHandle64NVX: Option<PFN_vkGetImageViewHandle64NVX>,
  #[cfg(feature = "VK_NVX_image_view_handle")]
  pub vkGetImageViewHandleNVX: Option<PFN_vkGetImageViewHandleNVX>,
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  pub vkGetClusterAccelerationStructureBuildSizesNV:
    Option<PFN_vkGetClusterAccelerationStructureBuildSizesNV>,
  #[cfg(feature = "VK_NV_cooperative_vector")]
  pub vkConvertCooperativeVectorMatrixNV: Option<PFN_vkConvertCooperativeVectorMatrixNV>,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub vkCreateCudaFunctionNV: Option<PFN_vkCreateCudaFunctionNV>,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub vkCreateCudaModuleNV: Option<PFN_vkCreateCudaModuleNV>,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkCreateIndirectCommandsLayoutNV: Option<PFN_vkCreateIndirectCommandsLayoutNV>,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkGetGeneratedCommandsMemoryRequirementsNV:
    Option<PFN_vkGetGeneratedCommandsMemoryRequirementsNV>,
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  pub vkGetPipelineIndirectDeviceAddressNV: Option<PFN_vkGetPipelineIndirectDeviceAddressNV>,
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  pub vkGetPipelineIndirectMemoryRequirementsNV:
    Option<PFN_vkGetPipelineIndirectMemoryRequirementsNV>,
  #[cfg(feature = "VK_NV_external_compute_queue")]
  pub vkCreateExternalComputeQueueNV: Option<PFN_vkCreateExternalComputeQueueNV>,
  #[cfg(feature = "VK_NV_external_memory_rdma")]
  pub vkGetMemoryRemoteAddressNV: Option<PFN_vkGetMemoryRemoteAddressNV>,
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  pub vkGetMemorySciBufNV: Option<PFN_vkGetMemorySciBufNV>,
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  pub vkGetFenceSciSyncFenceNV: Option<PFN_vkGetFenceSciSyncFenceNV>,
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  pub vkGetFenceSciSyncObjNV: Option<PFN_vkGetFenceSciSyncObjNV>,
  #[cfg(feature = "VK_NV_external_sci_sync")]
  pub vkGetSemaphoreSciSyncObjNV: Option<PFN_vkGetSemaphoreSciSyncObjNV>,
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  pub vkImportFenceSciSyncFenceNV: Option<PFN_vkImportFenceSciSyncFenceNV>,
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  pub vkImportFenceSciSyncObjNV: Option<PFN_vkImportFenceSciSyncObjNV>,
  #[cfg(feature = "VK_NV_external_sci_sync")]
  pub vkImportSemaphoreSciSyncObjNV: Option<PFN_vkImportSemaphoreSciSyncObjNV>,
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  pub vkCreateSemaphoreSciSyncPoolNV: Option<PFN_vkCreateSemaphoreSciSyncPoolNV>,
  #[cfg(feature = "VK_NV_optical_flow")]
  pub vkCreateOpticalFlowSessionNV: Option<PFN_vkCreateOpticalFlowSessionNV>,
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  pub vkGetPartitionedAccelerationStructuresBuildSizesNV:
    Option<PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkBindAccelerationStructureMemoryNV: Option<PFN_vkBindAccelerationStructureMemoryNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkCreateAccelerationStructureNV: Option<PFN_vkCreateAccelerationStructureNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkGetAccelerationStructureMemoryRequirementsNV:
    Option<PFN_vkGetAccelerationStructureMemoryRequirementsNV>,
  #[cfg(feature = "VK_OHOS_external_memory")]
  pub vkGetMemoryNativeBufferOHOS: Option<PFN_vkGetMemoryNativeBufferOHOS>,
  #[cfg(feature = "VK_OHOS_external_memory")]
  pub vkGetNativeBufferPropertiesOHOS: Option<PFN_vkGetNativeBufferPropertiesOHOS>,
  #[cfg(feature = "VK_QCOM_tile_properties")]
  pub vkGetDynamicRenderingTilePropertiesQCOM: Option<PFN_vkGetDynamicRenderingTilePropertiesQCOM>,
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  pub vkGetScreenBufferPropertiesQNX: Option<PFN_vkGetScreenBufferPropertiesQNX>,
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  pub vkGetDescriptorSetLayoutHostMappingInfoVALVE:
    Option<PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl DeviceDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VKSC_VERSION_1_0")]
    vkGetFaultData: None,
    #[cfg(feature = "VK_AMD_anti_lag")]
    vkAntiLagUpdateAMD: None,
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    vkGetAndroidHardwareBufferPropertiesANDROID: None,
    #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
    vkGetMemoryAndroidHardwareBufferANDROID: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkBindDataGraphPipelineSessionMemoryARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkCreateDataGraphPipelineSessionARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetDataGraphPipelineAvailablePropertiesARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetDataGraphPipelinePropertiesARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetDataGraphPipelineSessionBindPointRequirementsARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetDataGraphPipelineSessionMemoryRequirementsARM: None,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    vkCreateShaderInstrumentationARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkBindTensorMemoryARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkCreateTensorARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkCreateTensorViewARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkGetDeviceTensorMemoryRequirementsARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkGetTensorMemoryRequirementsARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkGetTensorOpaqueCaptureDescriptorDataARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkGetTensorViewOpaqueCaptureDescriptorDataARM: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkAllocateMemory: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateCommandPool: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateFence: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateImage: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateImageView: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateQueryPool: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateSemaphore: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkDestroyDevice: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkDeviceWaitIdle: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkFlushMappedMemoryRanges: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetDeviceQueue: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkInvalidateMappedMemoryRanges: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkResetFences: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkWaitForFences: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkBindBufferMemory2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkBindImageMemory2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetBufferMemoryRequirements2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetDeviceGroupPeerMemoryFeatures: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetDeviceQueue2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetImageMemoryRequirements2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetImageSparseMemoryRequirements2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    vkGetBufferDeviceAddress: None,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    vkGetBufferOpaqueCaptureAddress: None,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    vkGetDeviceMemoryOpaqueCaptureAddress: None,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    vkSignalSemaphore: None,
    #[cfg(feature = "VK_BASE_VERSION_1_2")]
    vkWaitSemaphores: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCreatePrivateDataSlot: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkGetDeviceBufferMemoryRequirements: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkGetDeviceImageMemoryRequirements: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkGetDeviceImageSparseMemoryRequirements: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkGetPrivateData: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkSetPrivateData: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkCopyImageToImage: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkCopyImageToMemory: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkCopyMemoryToImage: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkGetDeviceImageSubresourceLayout: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkMapMemory2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkTransitionImageLayout: None,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    vkUnmapMemory2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateBufferView: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateComputePipelines: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateDescriptorPool: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateDescriptorSetLayout: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateEvent: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreatePipelineCache: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreatePipelineLayout: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateSampler: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCreateShaderModule: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkUpdateDescriptorSets: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkCreateDescriptorUpdateTemplate: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkCreateSamplerYcbcrConversion: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkGetDescriptorSetLayoutSupport: None,
    #[cfg(feature = "VK_EXT_buffer_device_address")]
    vkGetBufferDeviceAddressEXT: None,
    #[cfg(feature = "VK_EXT_calibrated_timestamps")]
    vkGetCalibratedTimestampsEXT: None,
    #[cfg(feature = "VK_EXT_debug_marker")]
    vkDebugMarkerSetObjectNameEXT: None,
    #[cfg(feature = "VK_EXT_debug_marker")]
    vkDebugMarkerSetObjectTagEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkSetDebugUtilsObjectNameEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkSetDebugUtilsObjectTagEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetBufferOpaqueCaptureDescriptorDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetDescriptorEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetImageOpaqueCaptureDescriptorDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetImageViewOpaqueCaptureDescriptorDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkGetSamplerOpaqueCaptureDescriptorDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkGetImageOpaqueCaptureDataEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkGetTensorOpaqueCaptureDataARM: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkRegisterCustomBorderColorEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkUnregisterCustomBorderColorEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkWriteResourceDescriptorsEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkWriteSamplerDescriptorsEXT: None,
    #[cfg(feature = "VK_EXT_device_fault")]
    vkGetDeviceFaultInfoEXT: None,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkCreateIndirectCommandsLayoutEXT: None,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkCreateIndirectExecutionSetEXT: None,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkGetGeneratedCommandsMemoryRequirementsEXT: None,
    #[cfg(feature = "VK_EXT_display_control")]
    vkRegisterDeviceEventEXT: None,
    #[cfg(feature = "VK_EXT_external_memory_host")]
    vkGetMemoryHostPointerPropertiesEXT: None,
    #[cfg(feature = "VK_EXT_external_memory_metal")]
    vkGetMemoryMetalHandleEXT: None,
    #[cfg(feature = "VK_EXT_external_memory_metal")]
    vkGetMemoryMetalHandlePropertiesEXT: None,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    vkGetDeviceGroupSurfacePresentModes2EXT: None,
    #[cfg(feature = "VK_EXT_hdr_metadata")]
    vkSetHdrMetadataEXT: None,
    #[cfg(feature = "VK_EXT_host_image_copy")]
    vkCopyImageToImageEXT: None,
    #[cfg(feature = "VK_EXT_host_image_copy")]
    vkCopyImageToMemoryEXT: None,
    #[cfg(feature = "VK_EXT_host_image_copy")]
    vkCopyMemoryToImageEXT: None,
    #[cfg(feature = "VK_EXT_host_image_copy")]
    vkTransitionImageLayoutEXT: None,
    #[cfg(feature = "VK_EXT_metal_objects")]
    vkExportMetalObjectsEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCreateMicromapEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkGetDeviceMicromapCompatibilityEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkGetMicromapBuildSizesEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkWriteMicromapsPropertiesEXT: None,
    #[cfg(feature = "VK_EXT_pipeline_properties")]
    vkGetPipelinePropertiesEXT: None,
    #[cfg(feature = "VK_EXT_present_timing")]
    vkGetPastPresentationTimingEXT: None,
    #[cfg(feature = "VK_EXT_private_data")]
    vkCreatePrivateDataSlotEXT: None,
    #[cfg(feature = "VK_EXT_private_data")]
    vkGetPrivateDataEXT: None,
    #[cfg(feature = "VK_EXT_private_data")]
    vkSetPrivateDataEXT: None,
    #[cfg(feature = "VK_EXT_shader_module_identifier")]
    vkGetShaderModuleCreateInfoIdentifierEXT: None,
    #[cfg(feature = "VK_EXT_shader_object")]
    vkCreateShadersEXT: None,
    #[cfg(feature = "VK_EXT_swapchain_maintenance1")]
    vkReleaseSwapchainImagesEXT: None,
    #[cfg(feature = "VK_EXT_validation_cache")]
    vkCreateValidationCacheEXT: None,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    vkCreateBufferCollectionFUCHSIA: None,
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    vkGetMemoryZirconHandleFUCHSIA: None,
    #[cfg(feature = "VK_FUCHSIA_external_memory")]
    vkGetMemoryZirconHandlePropertiesFUCHSIA: None,
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    vkGetSemaphoreZirconHandleFUCHSIA: None,
    #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
    vkImportSemaphoreZirconHandleFUCHSIA: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCreateFramebuffer: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCreateGraphicsPipelines: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCreateRenderPass: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCreateRenderPass2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
    vkGetRenderingAreaGranularity: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkAcquirePerformanceConfigurationINTEL: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkGetPerformanceParameterINTEL: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkInitializePerformanceApiINTEL: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkUninitializePerformanceApiINTEL: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCreateAccelerationStructureKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkGetAccelerationStructureBuildSizesKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkGetAccelerationStructureDeviceAddressKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkGetDeviceAccelerationStructureCompatibilityKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkWriteAccelerationStructuresPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    vkBindBufferMemory2KHR: None,
    #[cfg(feature = "VK_KHR_bind_memory2")]
    vkBindImageMemory2KHR: None,
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    vkGetBufferDeviceAddressKHR: None,
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    vkGetBufferOpaqueCaptureAddressKHR: None,
    #[cfg(feature = "VK_KHR_buffer_device_address")]
    vkGetDeviceMemoryOpaqueCaptureAddressKHR: None,
    #[cfg(feature = "VK_KHR_calibrated_timestamps")]
    vkGetCalibratedTimestampsKHR: None,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    vkCreateRenderPass2KHR: None,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
    vkCreateDeferredOperationKHR: None,
    #[cfg(feature = "VK_KHR_descriptor_update_template")]
    vkCreateDescriptorUpdateTemplateKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCreateAccelerationStructure2KHR: None,
    #[cfg(feature = "VK_KHR_device_fault")]
    vkGetDeviceFaultDebugInfoKHR: None,
    #[cfg(feature = "VK_KHR_device_fault")]
    vkGetDeviceFaultReportsKHR: None,
    #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
    vkAcquireNextImage2KHR: None,
    #[cfg(feature = "VK_KHR_device_group")]
    vkGetDeviceGroupPeerMemoryFeaturesKHR: None,
    #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
    vkGetDeviceGroupPresentCapabilitiesKHR: None,
    #[cfg(feature = "VK_KHR_display_swapchain")]
    vkCreateSharedSwapchainsKHR: None,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    vkGetFenceFdKHR: None,
    #[cfg(feature = "VK_KHR_external_fence_fd")]
    vkImportFenceFdKHR: None,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    vkGetFenceWin32HandleKHR: None,
    #[cfg(feature = "VK_KHR_external_fence_win32")]
    vkImportFenceWin32HandleKHR: None,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    vkGetMemoryFdKHR: None,
    #[cfg(feature = "VK_KHR_external_memory_fd")]
    vkGetMemoryFdPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    vkGetMemoryWin32HandleKHR: None,
    #[cfg(feature = "VK_KHR_external_memory_win32")]
    vkGetMemoryWin32HandlePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    vkGetSemaphoreFdKHR: None,
    #[cfg(feature = "VK_KHR_external_semaphore_fd")]
    vkImportSemaphoreFdKHR: None,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    vkGetSemaphoreWin32HandleKHR: None,
    #[cfg(feature = "VK_KHR_external_semaphore_win32")]
    vkImportSemaphoreWin32HandleKHR: None,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    vkGetBufferMemoryRequirements2KHR: None,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    vkGetImageMemoryRequirements2KHR: None,
    #[cfg(feature = "VK_KHR_get_memory_requirements2")]
    vkGetImageSparseMemoryRequirements2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance3")]
    vkGetDescriptorSetLayoutSupportKHR: None,
    #[cfg(feature = "VK_KHR_maintenance4")]
    vkGetDeviceBufferMemoryRequirementsKHR: None,
    #[cfg(feature = "VK_KHR_maintenance4")]
    vkGetDeviceImageMemoryRequirementsKHR: None,
    #[cfg(feature = "VK_KHR_maintenance4")]
    vkGetDeviceImageSparseMemoryRequirementsKHR: None,
    #[cfg(feature = "VK_KHR_maintenance5")]
    vkGetDeviceImageSubresourceLayoutKHR: None,
    #[cfg(feature = "VK_KHR_maintenance5")]
    vkGetRenderingAreaGranularityKHR: None,
    #[cfg(feature = "VK_KHR_map_memory2")]
    vkMapMemory2KHR: None,
    #[cfg(feature = "VK_KHR_map_memory2")]
    vkUnmapMemory2KHR: None,
    #[cfg(feature = "VK_KHR_performance_query")]
    vkAcquireProfilingLockKHR: None,
    #[cfg(feature = "VK_KHR_performance_query")]
    vkReleaseProfilingLockKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    vkCreatePipelineBinariesKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    vkGetPipelineBinaryDataKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    vkGetPipelineKeyKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    vkReleaseCapturedPipelineDataKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    vkGetPipelineExecutableInternalRepresentationsKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    vkGetPipelineExecutablePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
    vkGetPipelineExecutableStatisticsKHR: None,
    #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
    vkCreateSamplerYcbcrConversionKHR: None,
    #[cfg(feature = "VK_KHR_swapchain")]
    vkCreateSwapchainKHR: None,
    #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
    vkReleaseSwapchainImagesKHR: None,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    vkSignalSemaphoreKHR: None,
    #[cfg(feature = "VK_KHR_timeline_semaphore")]
    vkWaitSemaphoresKHR: None,
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    vkGetEncodedVideoSessionParametersKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkCreateVideoSessionKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkCreateVideoSessionParametersKHR: None,
    #[cfg(feature = "VK_NVX_binary_import")]
    vkCreateCuFunctionNVX: None,
    #[cfg(feature = "VK_NVX_binary_import")]
    vkCreateCuModuleNVX: None,
    #[cfg(feature = "VK_NVX_image_view_handle")]
    vkGetDeviceCombinedImageSamplerIndexNVX: None,
    #[cfg(feature = "VK_NVX_image_view_handle")]
    vkGetImageViewHandle64NVX: None,
    #[cfg(feature = "VK_NVX_image_view_handle")]
    vkGetImageViewHandleNVX: None,
    #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
    vkGetClusterAccelerationStructureBuildSizesNV: None,
    #[cfg(feature = "VK_NV_cooperative_vector")]
    vkConvertCooperativeVectorMatrixNV: None,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    vkCreateCudaFunctionNV: None,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    vkCreateCudaModuleNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkCreateIndirectCommandsLayoutNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkGetGeneratedCommandsMemoryRequirementsNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands_compute")]
    vkGetPipelineIndirectDeviceAddressNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands_compute")]
    vkGetPipelineIndirectMemoryRequirementsNV: None,
    #[cfg(feature = "VK_NV_external_compute_queue")]
    vkCreateExternalComputeQueueNV: None,
    #[cfg(feature = "VK_NV_external_memory_rdma")]
    vkGetMemoryRemoteAddressNV: None,
    #[cfg(feature = "VK_NV_external_memory_sci_buf")]
    vkGetMemorySciBufNV: None,
    #[cfg(any(
      feature = "VK_NV_external_sci_sync",
      feature = "VK_NV_external_sci_sync2"
    ))]
    vkGetFenceSciSyncFenceNV: None,
    #[cfg(any(
      feature = "VK_NV_external_sci_sync",
      feature = "VK_NV_external_sci_sync2"
    ))]
    vkGetFenceSciSyncObjNV: None,
    #[cfg(feature = "VK_NV_external_sci_sync")]
    vkGetSemaphoreSciSyncObjNV: None,
    #[cfg(any(
      feature = "VK_NV_external_sci_sync",
      feature = "VK_NV_external_sci_sync2"
    ))]
    vkImportFenceSciSyncFenceNV: None,
    #[cfg(any(
      feature = "VK_NV_external_sci_sync",
      feature = "VK_NV_external_sci_sync2"
    ))]
    vkImportFenceSciSyncObjNV: None,
    #[cfg(feature = "VK_NV_external_sci_sync")]
    vkImportSemaphoreSciSyncObjNV: None,
    #[cfg(feature = "VK_NV_external_sci_sync2")]
    vkCreateSemaphoreSciSyncPoolNV: None,
    #[cfg(feature = "VK_NV_optical_flow")]
    vkCreateOpticalFlowSessionNV: None,
    #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
    vkGetPartitionedAccelerationStructuresBuildSizesNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkBindAccelerationStructureMemoryNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkCreateAccelerationStructureNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkGetAccelerationStructureMemoryRequirementsNV: None,
    #[cfg(feature = "VK_OHOS_external_memory")]
    vkGetMemoryNativeBufferOHOS: None,
    #[cfg(feature = "VK_OHOS_external_memory")]
    vkGetNativeBufferPropertiesOHOS: None,
    #[cfg(feature = "VK_QCOM_tile_properties")]
    vkGetDynamicRenderingTilePropertiesQCOM: None,
    #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
    vkGetScreenBufferPropertiesQNX: None,
    #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
    vkGetDescriptorSetLayoutHostMappingInfoVALVE: None,
  };
  /// Resolve all device commands from the given loader closure.
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VKSC_VERSION_1_0")]
      vkGetFaultData: loader(c"vkGetFaultData".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_anti_lag")]
      vkAntiLagUpdateAMD: loader(c"vkAntiLagUpdateAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
      vkGetAndroidHardwareBufferPropertiesANDROID: loader(
        c"vkGetAndroidHardwareBufferPropertiesANDROID".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
      vkGetMemoryAndroidHardwareBufferANDROID: loader(
        c"vkGetMemoryAndroidHardwareBufferANDROID".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkBindDataGraphPipelineSessionMemoryARM: loader(
        c"vkBindDataGraphPipelineSessionMemoryARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkCreateDataGraphPipelineSessionARM: loader(c"vkCreateDataGraphPipelineSessionARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetDataGraphPipelineAvailablePropertiesARM: loader(
        c"vkGetDataGraphPipelineAvailablePropertiesARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetDataGraphPipelinePropertiesARM: loader(c"vkGetDataGraphPipelinePropertiesARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetDataGraphPipelineSessionBindPointRequirementsARM: loader(
        c"vkGetDataGraphPipelineSessionBindPointRequirementsARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetDataGraphPipelineSessionMemoryRequirementsARM: loader(
        c"vkGetDataGraphPipelineSessionMemoryRequirementsARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      vkCreateShaderInstrumentationARM: loader(c"vkCreateShaderInstrumentationARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkBindTensorMemoryARM: loader(c"vkBindTensorMemoryARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkCreateTensorARM: loader(c"vkCreateTensorARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkCreateTensorViewARM: loader(c"vkCreateTensorViewARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkGetDeviceTensorMemoryRequirementsARM: loader(
        c"vkGetDeviceTensorMemoryRequirementsARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkGetTensorMemoryRequirementsARM: loader(c"vkGetTensorMemoryRequirementsARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkGetTensorOpaqueCaptureDescriptorDataARM: loader(
        c"vkGetTensorOpaqueCaptureDescriptorDataARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkGetTensorViewOpaqueCaptureDescriptorDataARM: loader(
        c"vkGetTensorViewOpaqueCaptureDescriptorDataARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkAllocateMemory: loader(c"vkAllocateMemory".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateBuffer: loader(c"vkCreateBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateCommandPool: loader(c"vkCreateCommandPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateFence: loader(c"vkCreateFence".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateImage: loader(c"vkCreateImage".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateImageView: loader(c"vkCreateImageView".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateQueryPool: loader(c"vkCreateQueryPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateSemaphore: loader(c"vkCreateSemaphore".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkDestroyDevice: loader(c"vkDestroyDevice".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkDeviceWaitIdle: loader(c"vkDeviceWaitIdle".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkFlushMappedMemoryRanges: loader(c"vkFlushMappedMemoryRanges".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetDeviceQueue: loader(c"vkGetDeviceQueue".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkInvalidateMappedMemoryRanges: loader(c"vkInvalidateMappedMemoryRanges".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkResetFences: loader(c"vkResetFences".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkWaitForFences: loader(c"vkWaitForFences".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkBindBufferMemory2: loader(c"vkBindBufferMemory2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkBindImageMemory2: loader(c"vkBindImageMemory2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetBufferMemoryRequirements2: loader(c"vkGetBufferMemoryRequirements2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetDeviceGroupPeerMemoryFeatures: loader(c"vkGetDeviceGroupPeerMemoryFeatures".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetDeviceQueue2: loader(c"vkGetDeviceQueue2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetImageMemoryRequirements2: loader(c"vkGetImageMemoryRequirements2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetImageSparseMemoryRequirements2: loader(c"vkGetImageSparseMemoryRequirements2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_2")]
      vkGetBufferDeviceAddress: loader(c"vkGetBufferDeviceAddress".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_2")]
      vkGetBufferOpaqueCaptureAddress: loader(c"vkGetBufferOpaqueCaptureAddress".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_2")]
      vkGetDeviceMemoryOpaqueCaptureAddress: loader(
        c"vkGetDeviceMemoryOpaqueCaptureAddress".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_2")]
      vkSignalSemaphore: loader(c"vkSignalSemaphore".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_2")]
      vkWaitSemaphores: loader(c"vkWaitSemaphores".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCreatePrivateDataSlot: loader(c"vkCreatePrivateDataSlot".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkGetDeviceBufferMemoryRequirements: loader(c"vkGetDeviceBufferMemoryRequirements".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkGetDeviceImageMemoryRequirements: loader(c"vkGetDeviceImageMemoryRequirements".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkGetDeviceImageSparseMemoryRequirements: loader(
        c"vkGetDeviceImageSparseMemoryRequirements".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkGetPrivateData: loader(c"vkGetPrivateData".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkSetPrivateData: loader(c"vkSetPrivateData".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkCopyImageToImage: loader(c"vkCopyImageToImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkCopyImageToMemory: loader(c"vkCopyImageToMemory".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkCopyMemoryToImage: loader(c"vkCopyMemoryToImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkGetDeviceImageSubresourceLayout: loader(c"vkGetDeviceImageSubresourceLayout".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkMapMemory2: loader(c"vkMapMemory2".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkTransitionImageLayout: loader(c"vkTransitionImageLayout".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_4")]
      vkUnmapMemory2: loader(c"vkUnmapMemory2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateBufferView: loader(c"vkCreateBufferView".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateComputePipelines: loader(c"vkCreateComputePipelines".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateDescriptorPool: loader(c"vkCreateDescriptorPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateDescriptorSetLayout: loader(c"vkCreateDescriptorSetLayout".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateEvent: loader(c"vkCreateEvent".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreatePipelineCache: loader(c"vkCreatePipelineCache".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreatePipelineLayout: loader(c"vkCreatePipelineLayout".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateSampler: loader(c"vkCreateSampler".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCreateShaderModule: loader(c"vkCreateShaderModule".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkUpdateDescriptorSets: loader(c"vkUpdateDescriptorSets".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkCreateDescriptorUpdateTemplate: loader(c"vkCreateDescriptorUpdateTemplate".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkCreateSamplerYcbcrConversion: loader(c"vkCreateSamplerYcbcrConversion".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkGetDescriptorSetLayoutSupport: loader(c"vkGetDescriptorSetLayoutSupport".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_buffer_device_address")]
      vkGetBufferDeviceAddressEXT: loader(c"vkGetBufferDeviceAddressEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_calibrated_timestamps")]
      vkGetCalibratedTimestampsEXT: loader(c"vkGetCalibratedTimestampsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_marker")]
      vkDebugMarkerSetObjectNameEXT: loader(c"vkDebugMarkerSetObjectNameEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_marker")]
      vkDebugMarkerSetObjectTagEXT: loader(c"vkDebugMarkerSetObjectTagEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkSetDebugUtilsObjectNameEXT: loader(c"vkSetDebugUtilsObjectNameEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkSetDebugUtilsObjectTagEXT: loader(c"vkSetDebugUtilsObjectTagEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT: loader(
        c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetBufferOpaqueCaptureDescriptorDataEXT: loader(
        c"vkGetBufferOpaqueCaptureDescriptorDataEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetDescriptorEXT: loader(c"vkGetDescriptorEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetImageOpaqueCaptureDescriptorDataEXT: loader(
        c"vkGetImageOpaqueCaptureDescriptorDataEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetImageViewOpaqueCaptureDescriptorDataEXT: loader(
        c"vkGetImageViewOpaqueCaptureDescriptorDataEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkGetSamplerOpaqueCaptureDescriptorDataEXT: loader(
        c"vkGetSamplerOpaqueCaptureDescriptorDataEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkGetImageOpaqueCaptureDataEXT: loader(c"vkGetImageOpaqueCaptureDataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkGetTensorOpaqueCaptureDataARM: loader(c"vkGetTensorOpaqueCaptureDataARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkRegisterCustomBorderColorEXT: loader(c"vkRegisterCustomBorderColorEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkUnregisterCustomBorderColorEXT: loader(c"vkUnregisterCustomBorderColorEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkWriteResourceDescriptorsEXT: loader(c"vkWriteResourceDescriptorsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkWriteSamplerDescriptorsEXT: loader(c"vkWriteSamplerDescriptorsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_fault")]
      vkGetDeviceFaultInfoEXT: loader(c"vkGetDeviceFaultInfoEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkCreateIndirectCommandsLayoutEXT: loader(c"vkCreateIndirectCommandsLayoutEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkCreateIndirectExecutionSetEXT: loader(c"vkCreateIndirectExecutionSetEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkGetGeneratedCommandsMemoryRequirementsEXT: loader(
        c"vkGetGeneratedCommandsMemoryRequirementsEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_display_control")]
      vkRegisterDeviceEventEXT: loader(c"vkRegisterDeviceEventEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_external_memory_host")]
      vkGetMemoryHostPointerPropertiesEXT: loader(c"vkGetMemoryHostPointerPropertiesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_external_memory_metal")]
      vkGetMemoryMetalHandleEXT: loader(c"vkGetMemoryMetalHandleEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_external_memory_metal")]
      vkGetMemoryMetalHandlePropertiesEXT: loader(c"vkGetMemoryMetalHandlePropertiesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_full_screen_exclusive")]
      vkGetDeviceGroupSurfacePresentModes2EXT: loader(
        c"vkGetDeviceGroupSurfacePresentModes2EXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_hdr_metadata")]
      vkSetHdrMetadataEXT: loader(c"vkSetHdrMetadataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_host_image_copy")]
      vkCopyImageToImageEXT: loader(c"vkCopyImageToImageEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_host_image_copy")]
      vkCopyImageToMemoryEXT: loader(c"vkCopyImageToMemoryEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_host_image_copy")]
      vkCopyMemoryToImageEXT: loader(c"vkCopyMemoryToImageEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_host_image_copy")]
      vkTransitionImageLayoutEXT: loader(c"vkTransitionImageLayoutEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_metal_objects")]
      vkExportMetalObjectsEXT: loader(c"vkExportMetalObjectsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCreateMicromapEXT: loader(c"vkCreateMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkGetDeviceMicromapCompatibilityEXT: loader(c"vkGetDeviceMicromapCompatibilityEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkGetMicromapBuildSizesEXT: loader(c"vkGetMicromapBuildSizesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkWriteMicromapsPropertiesEXT: loader(c"vkWriteMicromapsPropertiesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_pipeline_properties")]
      vkGetPipelinePropertiesEXT: loader(c"vkGetPipelinePropertiesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_present_timing")]
      vkGetPastPresentationTimingEXT: loader(c"vkGetPastPresentationTimingEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_private_data")]
      vkCreatePrivateDataSlotEXT: loader(c"vkCreatePrivateDataSlotEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_private_data")]
      vkGetPrivateDataEXT: loader(c"vkGetPrivateDataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_private_data")]
      vkSetPrivateDataEXT: loader(c"vkSetPrivateDataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_shader_module_identifier")]
      vkGetShaderModuleCreateInfoIdentifierEXT: loader(
        c"vkGetShaderModuleCreateInfoIdentifierEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_shader_object")]
      vkCreateShadersEXT: loader(c"vkCreateShadersEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_swapchain_maintenance1")]
      vkReleaseSwapchainImagesEXT: loader(c"vkReleaseSwapchainImagesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_validation_cache")]
      vkCreateValidationCacheEXT: loader(c"vkCreateValidationCacheEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
      vkCreateBufferCollectionFUCHSIA: loader(c"vkCreateBufferCollectionFUCHSIA".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_FUCHSIA_external_memory")]
      vkGetMemoryZirconHandleFUCHSIA: loader(c"vkGetMemoryZirconHandleFUCHSIA".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_FUCHSIA_external_memory")]
      vkGetMemoryZirconHandlePropertiesFUCHSIA: loader(
        c"vkGetMemoryZirconHandlePropertiesFUCHSIA".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
      vkGetSemaphoreZirconHandleFUCHSIA: loader(c"vkGetSemaphoreZirconHandleFUCHSIA".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
      vkImportSemaphoreZirconHandleFUCHSIA: loader(
        c"vkImportSemaphoreZirconHandleFUCHSIA".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCreateFramebuffer: loader(c"vkCreateFramebuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCreateGraphicsPipelines: loader(c"vkCreateGraphicsPipelines".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCreateRenderPass: loader(c"vkCreateRenderPass".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCreateRenderPass2: loader(c"vkCreateRenderPass2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
      vkGetRenderingAreaGranularity: loader(c"vkGetRenderingAreaGranularity".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkAcquirePerformanceConfigurationINTEL: loader(
        c"vkAcquirePerformanceConfigurationINTEL".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkGetPerformanceParameterINTEL: loader(c"vkGetPerformanceParameterINTEL".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkInitializePerformanceApiINTEL: loader(c"vkInitializePerformanceApiINTEL".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkUninitializePerformanceApiINTEL: loader(c"vkUninitializePerformanceApiINTEL".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCreateAccelerationStructureKHR: loader(c"vkCreateAccelerationStructureKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkGetAccelerationStructureBuildSizesKHR: loader(
        c"vkGetAccelerationStructureBuildSizesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkGetAccelerationStructureDeviceAddressKHR: loader(
        c"vkGetAccelerationStructureDeviceAddressKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkGetDeviceAccelerationStructureCompatibilityKHR: loader(
        c"vkGetDeviceAccelerationStructureCompatibilityKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkWriteAccelerationStructuresPropertiesKHR: loader(
        c"vkWriteAccelerationStructuresPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_bind_memory2")]
      vkBindBufferMemory2KHR: loader(c"vkBindBufferMemory2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_bind_memory2")]
      vkBindImageMemory2KHR: loader(c"vkBindImageMemory2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_buffer_device_address")]
      vkGetBufferDeviceAddressKHR: loader(c"vkGetBufferDeviceAddressKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_buffer_device_address")]
      vkGetBufferOpaqueCaptureAddressKHR: loader(c"vkGetBufferOpaqueCaptureAddressKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_buffer_device_address")]
      vkGetDeviceMemoryOpaqueCaptureAddressKHR: loader(
        c"vkGetDeviceMemoryOpaqueCaptureAddressKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_calibrated_timestamps")]
      vkGetCalibratedTimestampsKHR: loader(c"vkGetCalibratedTimestampsKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_create_renderpass2")]
      vkCreateRenderPass2KHR: loader(c"vkCreateRenderPass2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      vkCreateDeferredOperationKHR: loader(c"vkCreateDeferredOperationKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_descriptor_update_template")]
      vkCreateDescriptorUpdateTemplateKHR: loader(c"vkCreateDescriptorUpdateTemplateKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCreateAccelerationStructure2KHR: loader(c"vkCreateAccelerationStructure2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_fault")]
      vkGetDeviceFaultDebugInfoKHR: loader(c"vkGetDeviceFaultDebugInfoKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_fault")]
      vkGetDeviceFaultReportsKHR: loader(c"vkGetDeviceFaultReportsKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
      vkAcquireNextImage2KHR: loader(c"vkAcquireNextImage2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_group")]
      vkGetDeviceGroupPeerMemoryFeaturesKHR: loader(
        c"vkGetDeviceGroupPeerMemoryFeaturesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
      vkGetDeviceGroupPresentCapabilitiesKHR: loader(
        c"vkGetDeviceGroupPresentCapabilitiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display_swapchain")]
      vkCreateSharedSwapchainsKHR: loader(c"vkCreateSharedSwapchainsKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_fence_fd")]
      vkGetFenceFdKHR: loader(c"vkGetFenceFdKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_fence_fd")]
      vkImportFenceFdKHR: loader(c"vkImportFenceFdKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_fence_win32")]
      vkGetFenceWin32HandleKHR: loader(c"vkGetFenceWin32HandleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_fence_win32")]
      vkImportFenceWin32HandleKHR: loader(c"vkImportFenceWin32HandleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_memory_fd")]
      vkGetMemoryFdKHR: loader(c"vkGetMemoryFdKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_memory_fd")]
      vkGetMemoryFdPropertiesKHR: loader(c"vkGetMemoryFdPropertiesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_memory_win32")]
      vkGetMemoryWin32HandleKHR: loader(c"vkGetMemoryWin32HandleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_memory_win32")]
      vkGetMemoryWin32HandlePropertiesKHR: loader(c"vkGetMemoryWin32HandlePropertiesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_semaphore_fd")]
      vkGetSemaphoreFdKHR: loader(c"vkGetSemaphoreFdKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_semaphore_fd")]
      vkImportSemaphoreFdKHR: loader(c"vkImportSemaphoreFdKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_semaphore_win32")]
      vkGetSemaphoreWin32HandleKHR: loader(c"vkGetSemaphoreWin32HandleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_semaphore_win32")]
      vkImportSemaphoreWin32HandleKHR: loader(c"vkImportSemaphoreWin32HandleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_memory_requirements2")]
      vkGetBufferMemoryRequirements2KHR: loader(c"vkGetBufferMemoryRequirements2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_memory_requirements2")]
      vkGetImageMemoryRequirements2KHR: loader(c"vkGetImageMemoryRequirements2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_memory_requirements2")]
      vkGetImageSparseMemoryRequirements2KHR: loader(
        c"vkGetImageSparseMemoryRequirements2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance3")]
      vkGetDescriptorSetLayoutSupportKHR: loader(c"vkGetDescriptorSetLayoutSupportKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance4")]
      vkGetDeviceBufferMemoryRequirementsKHR: loader(
        c"vkGetDeviceBufferMemoryRequirementsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance4")]
      vkGetDeviceImageMemoryRequirementsKHR: loader(
        c"vkGetDeviceImageMemoryRequirementsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance4")]
      vkGetDeviceImageSparseMemoryRequirementsKHR: loader(
        c"vkGetDeviceImageSparseMemoryRequirementsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance5")]
      vkGetDeviceImageSubresourceLayoutKHR: loader(
        c"vkGetDeviceImageSubresourceLayoutKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance5")]
      vkGetRenderingAreaGranularityKHR: loader(c"vkGetRenderingAreaGranularityKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_map_memory2")]
      vkMapMemory2KHR: loader(c"vkMapMemory2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_map_memory2")]
      vkUnmapMemory2KHR: loader(c"vkUnmapMemory2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_performance_query")]
      vkAcquireProfilingLockKHR: loader(c"vkAcquireProfilingLockKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_performance_query")]
      vkReleaseProfilingLockKHR: loader(c"vkReleaseProfilingLockKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      vkCreatePipelineBinariesKHR: loader(c"vkCreatePipelineBinariesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      vkGetPipelineBinaryDataKHR: loader(c"vkGetPipelineBinaryDataKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      vkGetPipelineKeyKHR: loader(c"vkGetPipelineKeyKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      vkReleaseCapturedPipelineDataKHR: loader(c"vkReleaseCapturedPipelineDataKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
      vkGetPipelineExecutableInternalRepresentationsKHR: loader(
        c"vkGetPipelineExecutableInternalRepresentationsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
      vkGetPipelineExecutablePropertiesKHR: loader(
        c"vkGetPipelineExecutablePropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
      vkGetPipelineExecutableStatisticsKHR: loader(
        c"vkGetPipelineExecutableStatisticsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
      vkCreateSamplerYcbcrConversionKHR: loader(c"vkCreateSamplerYcbcrConversionKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_swapchain")]
      vkCreateSwapchainKHR: loader(c"vkCreateSwapchainKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
      vkReleaseSwapchainImagesKHR: loader(c"vkReleaseSwapchainImagesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_timeline_semaphore")]
      vkSignalSemaphoreKHR: loader(c"vkSignalSemaphoreKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_timeline_semaphore")]
      vkWaitSemaphoresKHR: loader(c"vkWaitSemaphoresKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_encode_queue")]
      vkGetEncodedVideoSessionParametersKHR: loader(
        c"vkGetEncodedVideoSessionParametersKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkCreateVideoSessionKHR: loader(c"vkCreateVideoSessionKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkCreateVideoSessionParametersKHR: loader(c"vkCreateVideoSessionParametersKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_binary_import")]
      vkCreateCuFunctionNVX: loader(c"vkCreateCuFunctionNVX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_binary_import")]
      vkCreateCuModuleNVX: loader(c"vkCreateCuModuleNVX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_image_view_handle")]
      vkGetDeviceCombinedImageSamplerIndexNVX: loader(
        c"vkGetDeviceCombinedImageSamplerIndexNVX".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_image_view_handle")]
      vkGetImageViewHandle64NVX: loader(c"vkGetImageViewHandle64NVX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_image_view_handle")]
      vkGetImageViewHandleNVX: loader(c"vkGetImageViewHandleNVX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
      vkGetClusterAccelerationStructureBuildSizesNV: loader(
        c"vkGetClusterAccelerationStructureBuildSizesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cooperative_vector")]
      vkConvertCooperativeVectorMatrixNV: loader(c"vkConvertCooperativeVectorMatrixNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      vkCreateCudaFunctionNV: loader(c"vkCreateCudaFunctionNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      vkCreateCudaModuleNV: loader(c"vkCreateCudaModuleNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkCreateIndirectCommandsLayoutNV: loader(c"vkCreateIndirectCommandsLayoutNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkGetGeneratedCommandsMemoryRequirementsNV: loader(
        c"vkGetGeneratedCommandsMemoryRequirementsNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands_compute")]
      vkGetPipelineIndirectDeviceAddressNV: loader(
        c"vkGetPipelineIndirectDeviceAddressNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands_compute")]
      vkGetPipelineIndirectMemoryRequirementsNV: loader(
        c"vkGetPipelineIndirectMemoryRequirementsNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_compute_queue")]
      vkCreateExternalComputeQueueNV: loader(c"vkCreateExternalComputeQueueNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_memory_rdma")]
      vkGetMemoryRemoteAddressNV: loader(c"vkGetMemoryRemoteAddressNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_memory_sci_buf")]
      vkGetMemorySciBufNV: loader(c"vkGetMemorySciBufNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_NV_external_sci_sync",
        feature = "VK_NV_external_sci_sync2"
      ))]
      vkGetFenceSciSyncFenceNV: loader(c"vkGetFenceSciSyncFenceNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_NV_external_sci_sync",
        feature = "VK_NV_external_sci_sync2"
      ))]
      vkGetFenceSciSyncObjNV: loader(c"vkGetFenceSciSyncObjNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_sci_sync")]
      vkGetSemaphoreSciSyncObjNV: loader(c"vkGetSemaphoreSciSyncObjNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_NV_external_sci_sync",
        feature = "VK_NV_external_sci_sync2"
      ))]
      vkImportFenceSciSyncFenceNV: loader(c"vkImportFenceSciSyncFenceNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_NV_external_sci_sync",
        feature = "VK_NV_external_sci_sync2"
      ))]
      vkImportFenceSciSyncObjNV: loader(c"vkImportFenceSciSyncObjNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_sci_sync")]
      vkImportSemaphoreSciSyncObjNV: loader(c"vkImportSemaphoreSciSyncObjNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_sci_sync2")]
      vkCreateSemaphoreSciSyncPoolNV: loader(c"vkCreateSemaphoreSciSyncPoolNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_optical_flow")]
      vkCreateOpticalFlowSessionNV: loader(c"vkCreateOpticalFlowSessionNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
      vkGetPartitionedAccelerationStructuresBuildSizesNV: loader(
        c"vkGetPartitionedAccelerationStructuresBuildSizesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkBindAccelerationStructureMemoryNV: loader(c"vkBindAccelerationStructureMemoryNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkCreateAccelerationStructureNV: loader(c"vkCreateAccelerationStructureNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkGetAccelerationStructureMemoryRequirementsNV: loader(
        c"vkGetAccelerationStructureMemoryRequirementsNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_OHOS_external_memory")]
      vkGetMemoryNativeBufferOHOS: loader(c"vkGetMemoryNativeBufferOHOS".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_OHOS_external_memory")]
      vkGetNativeBufferPropertiesOHOS: loader(c"vkGetNativeBufferPropertiesOHOS".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_properties")]
      vkGetDynamicRenderingTilePropertiesQCOM: loader(
        c"vkGetDynamicRenderingTilePropertiesQCOM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
      vkGetScreenBufferPropertiesQNX: loader(c"vkGetScreenBufferPropertiesQNX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
      vkGetDescriptorSetLayoutHostMappingInfoVALVE: loader(
        c"vkGetDescriptorSetLayoutHostMappingInfoVALVE".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
  /// Resolve all device commands via `vkGetDeviceProcAddr(device, …)`.
  pub fn load_for_device<F>(device: VkDevice, get_proc: F) -> Self
  where
    F: Fn(VkDevice, *const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self::load(|name| get_proc(device, name))
  }
}
/// Safe `VkDevice` wrapper.
///
/// Lifetime `'inst` ties this device to the [`Instance`] that created it;
/// it cannot outlive it.
///
/// Holds [`DeviceDispatchTable`] by value.
///
/// # Cleanup
/// On drop, if the raw `VkDevice` is non-null and `vkDestroyDevice` is
/// present in the dispatch table, it is called with `self.raw` and
/// `pAllocator = null`.
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Device<'inst> {
  pub(crate) raw: VkDevice,
  pub(crate) instance: &'inst Instance<'inst>,
  pub(crate) table: DeviceDispatchTable,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub(crate) acceleration_structure_khr_table:
    crate::acceleration_structure_khr::AccelerationStructureKHRDispatchTable,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub(crate) acceleration_structure_nv_table:
    crate::acceleration_structure_nv::AccelerationStructureNVDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) buffer_table: crate::buffer::BufferDispatchTable,
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  pub(crate) buffer_collection_fuchsia_table:
    crate::buffer_collection_fuchsia::BufferCollectionFUCHSIADispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) buffer_view_table: crate::buffer_view::BufferViewDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) command_buffer_table: crate::command_buffer::CommandBufferDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) command_pool_table: crate::command_pool::CommandPoolDispatchTable,
  #[cfg(feature = "VK_NVX_binary_import")]
  pub(crate) cu_function_nvx_table: crate::cu_function_nvx::CuFunctionNVXDispatchTable,
  #[cfg(feature = "VK_NVX_binary_import")]
  pub(crate) cu_module_nvx_table: crate::cu_module_nvx::CuModuleNVXDispatchTable,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub(crate) cuda_function_nv_table: crate::cuda_function_nv::CudaFunctionNVDispatchTable,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub(crate) cuda_module_nv_table: crate::cuda_module_nv::CudaModuleNVDispatchTable,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub(crate) data_graph_pipeline_session_arm_table:
    crate::data_graph_pipeline_session_arm::DataGraphPipelineSessionARMDispatchTable,
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  pub(crate) deferred_operation_khr_table:
    crate::deferred_operation_khr::DeferredOperationKHRDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) descriptor_pool_table: crate::descriptor_pool::DescriptorPoolDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) descriptor_set_table: crate::descriptor_set::DescriptorSetDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) descriptor_set_layout_table:
    crate::descriptor_set_layout::DescriptorSetLayoutDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub(crate) descriptor_update_template_table:
    crate::descriptor_update_template::DescriptorUpdateTemplateDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) device_memory_table: crate::device_memory::DeviceMemoryDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) event_table: crate::event::EventDispatchTable,
  #[cfg(feature = "VK_NV_external_compute_queue")]
  pub(crate) external_compute_queue_nv_table:
    crate::external_compute_queue_nv::ExternalComputeQueueNVDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) fence_table: crate::fence::FenceDispatchTable,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub(crate) framebuffer_table: crate::framebuffer::FramebufferDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) image_table: crate::image::ImageDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) image_view_table: crate::image_view::ImageViewDispatchTable,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub(crate) indirect_commands_layout_ext_table:
    crate::indirect_commands_layout_ext::IndirectCommandsLayoutEXTDispatchTable,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub(crate) indirect_commands_layout_nv_table:
    crate::indirect_commands_layout_nv::IndirectCommandsLayoutNVDispatchTable,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub(crate) indirect_execution_set_ext_table:
    crate::indirect_execution_set_ext::IndirectExecutionSetEXTDispatchTable,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub(crate) micromap_ext_table: crate::micromap_ext::MicromapEXTDispatchTable,
  #[cfg(feature = "VK_NV_optical_flow")]
  pub(crate) optical_flow_session_nv_table:
    crate::optical_flow_session_nv::OpticalFlowSessionNVDispatchTable,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub(crate) performance_configuration_intel_table:
    crate::performance_configuration_intel::PerformanceConfigurationINTELDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) pipeline_table: crate::pipeline::PipelineDispatchTable,
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  pub(crate) pipeline_binary_khr_table: crate::pipeline_binary_khr::PipelineBinaryKHRDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) pipeline_cache_table: crate::pipeline_cache::PipelineCacheDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) pipeline_layout_table: crate::pipeline_layout::PipelineLayoutDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub(crate) private_data_slot_table: crate::private_data_slot::PrivateDataSlotDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) query_pool_table: crate::query_pool::QueryPoolDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) queue_table: crate::queue::QueueDispatchTable,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub(crate) render_pass_table: crate::render_pass::RenderPassDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) sampler_table: crate::sampler::SamplerDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub(crate) sampler_ycbcr_conversion_table:
    crate::sampler_ycbcr_conversion::SamplerYcbcrConversionDispatchTable,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub(crate) semaphore_table: crate::semaphore::SemaphoreDispatchTable,
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  pub(crate) semaphore_sci_sync_pool_nv_table:
    crate::semaphore_sci_sync_pool_nv::SemaphoreSciSyncPoolNVDispatchTable,
  #[cfg(feature = "VK_EXT_shader_object")]
  pub(crate) shader_ext_table: crate::shader_ext::ShaderEXTDispatchTable,
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  pub(crate) shader_instrumentation_arm_table:
    crate::shader_instrumentation_arm::ShaderInstrumentationARMDispatchTable,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub(crate) shader_module_table: crate::shader_module::ShaderModuleDispatchTable,
  #[cfg(feature = "VK_KHR_swapchain")]
  pub(crate) swapchain_khr_table: crate::swapchain_khr::SwapchainKHRDispatchTable,
  #[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
  pub(crate) tensor_arm_table: crate::tensor_arm::TensorARMDispatchTable,
  #[cfg(feature = "VK_ARM_tensors")]
  pub(crate) tensor_view_arm_table: crate::tensor_view_arm::TensorViewARMDispatchTable,
  #[cfg(feature = "VK_EXT_validation_cache")]
  pub(crate) validation_cache_ext_table:
    crate::validation_cache_ext::ValidationCacheEXTDispatchTable,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub(crate) video_session_khr_table: crate::video_session_khr::VideoSessionKHRDispatchTable,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub(crate) video_session_parameters_khr_table:
    crate::video_session_parameters_khr::VideoSessionParametersKHRDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'inst> Send for Device<'inst> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'inst> Sync for Device<'inst> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'inst> Device<'inst> {
  /// Wrap a raw `VkDevice` with a pre-loaded dispatch table.
  ///
  /// # Safety
  /// `raw` must be a valid live `VkDevice` for `'inst`.
  #[inline]
  pub const unsafe fn from_raw(
    raw: VkDevice,
    instance: &'inst Instance<'inst>,
    table: DeviceDispatchTable,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
        acceleration_structure_khr_table: crate::acceleration_structure_khr::AccelerationStructureKHRDispatchTable,
    #[cfg(feature = "VK_NV_ray_tracing")]
        acceleration_structure_nv_table: crate::acceleration_structure_nv::AccelerationStructureNVDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")] buffer_table: crate::buffer::BufferDispatchTable,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        buffer_collection_fuchsia_table: crate::buffer_collection_fuchsia::BufferCollectionFUCHSIADispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    buffer_view_table: crate::buffer_view::BufferViewDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    command_buffer_table: crate::command_buffer::CommandBufferDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    command_pool_table: crate::command_pool::CommandPoolDispatchTable,
    #[cfg(feature = "VK_NVX_binary_import")]
    cu_function_nvx_table: crate::cu_function_nvx::CuFunctionNVXDispatchTable,
    #[cfg(feature = "VK_NVX_binary_import")]
    cu_module_nvx_table: crate::cu_module_nvx::CuModuleNVXDispatchTable,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    cuda_function_nv_table: crate::cuda_function_nv::CudaFunctionNVDispatchTable,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    cuda_module_nv_table: crate::cuda_module_nv::CudaModuleNVDispatchTable,
    #[cfg(feature = "VK_ARM_data_graph")]
        data_graph_pipeline_session_arm_table: crate::data_graph_pipeline_session_arm::DataGraphPipelineSessionARMDispatchTable,
    #[cfg(feature = "VK_KHR_deferred_host_operations")]
        deferred_operation_khr_table: crate::deferred_operation_khr::DeferredOperationKHRDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    descriptor_pool_table: crate::descriptor_pool::DescriptorPoolDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    descriptor_set_table: crate::descriptor_set::DescriptorSetDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    descriptor_set_layout_table: crate::descriptor_set_layout::DescriptorSetLayoutDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        descriptor_update_template_table: crate::descriptor_update_template::DescriptorUpdateTemplateDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    device_memory_table: crate::device_memory::DeviceMemoryDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")] event_table: crate::event::EventDispatchTable,
    #[cfg(feature = "VK_NV_external_compute_queue")]
        external_compute_queue_nv_table: crate::external_compute_queue_nv::ExternalComputeQueueNVDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")] fence_table: crate::fence::FenceDispatchTable,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    framebuffer_table: crate::framebuffer::FramebufferDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")] image_table: crate::image::ImageDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    image_view_table: crate::image_view::ImageViewDispatchTable,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
        indirect_commands_layout_ext_table: crate::indirect_commands_layout_ext::IndirectCommandsLayoutEXTDispatchTable,
    #[cfg(feature = "VK_NV_device_generated_commands")]
        indirect_commands_layout_nv_table: crate::indirect_commands_layout_nv::IndirectCommandsLayoutNVDispatchTable,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
        indirect_execution_set_ext_table: crate::indirect_execution_set_ext::IndirectExecutionSetEXTDispatchTable,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    micromap_ext_table: crate::micromap_ext::MicromapEXTDispatchTable,
    #[cfg(feature = "VK_NV_optical_flow")]
        optical_flow_session_nv_table: crate::optical_flow_session_nv::OpticalFlowSessionNVDispatchTable,
    #[cfg(feature = "VK_INTEL_performance_query")]
        performance_configuration_intel_table: crate::performance_configuration_intel::PerformanceConfigurationINTELDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline_table: crate::pipeline::PipelineDispatchTable,
    #[cfg(feature = "VK_KHR_pipeline_binary")]
    pipeline_binary_khr_table: crate::pipeline_binary_khr::PipelineBinaryKHRDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline_cache_table: crate::pipeline_cache::PipelineCacheDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pipeline_layout_table: crate::pipeline_layout::PipelineLayoutDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    private_data_slot_table: crate::private_data_slot::PrivateDataSlotDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    query_pool_table: crate::query_pool::QueryPoolDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")] queue_table: crate::queue::QueueDispatchTable,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    render_pass_table: crate::render_pass::RenderPassDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")] sampler_table: crate::sampler::SamplerDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
        sampler_ycbcr_conversion_table: crate::sampler_ycbcr_conversion::SamplerYcbcrConversionDispatchTable,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    semaphore_table: crate::semaphore::SemaphoreDispatchTable,
    #[cfg(feature = "VK_NV_external_sci_sync2")]
        semaphore_sci_sync_pool_nv_table: crate::semaphore_sci_sync_pool_nv::SemaphoreSciSyncPoolNVDispatchTable,
    #[cfg(feature = "VK_EXT_shader_object")]
    shader_ext_table: crate::shader_ext::ShaderEXTDispatchTable,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
        shader_instrumentation_arm_table: crate::shader_instrumentation_arm::ShaderInstrumentationARMDispatchTable,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    shader_module_table: crate::shader_module::ShaderModuleDispatchTable,
    #[cfg(feature = "VK_KHR_swapchain")]
    swapchain_khr_table: crate::swapchain_khr::SwapchainKHRDispatchTable,
    #[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
    tensor_arm_table: crate::tensor_arm::TensorARMDispatchTable,
    #[cfg(feature = "VK_ARM_tensors")]
    tensor_view_arm_table: crate::tensor_view_arm::TensorViewARMDispatchTable,
    #[cfg(feature = "VK_EXT_validation_cache")]
    validation_cache_ext_table: crate::validation_cache_ext::ValidationCacheEXTDispatchTable,
    #[cfg(feature = "VK_KHR_video_queue")]
    video_session_khr_table: crate::video_session_khr::VideoSessionKHRDispatchTable,
    #[cfg(feature = "VK_KHR_video_queue")]
        video_session_parameters_khr_table: crate::video_session_parameters_khr::VideoSessionParametersKHRDispatchTable,
  ) -> Self {
    Self {
      raw,
      instance,
      table,
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      acceleration_structure_khr_table,
      #[cfg(feature = "VK_NV_ray_tracing")]
      acceleration_structure_nv_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      buffer_table,
      #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
      buffer_collection_fuchsia_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      buffer_view_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      command_buffer_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      command_pool_table,
      #[cfg(feature = "VK_NVX_binary_import")]
      cu_function_nvx_table,
      #[cfg(feature = "VK_NVX_binary_import")]
      cu_module_nvx_table,
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      cuda_function_nv_table,
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      cuda_module_nv_table,
      #[cfg(feature = "VK_ARM_data_graph")]
      data_graph_pipeline_session_arm_table,
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      deferred_operation_khr_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_pool_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_set_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_set_layout_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      descriptor_update_template_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      device_memory_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      event_table,
      #[cfg(feature = "VK_NV_external_compute_queue")]
      external_compute_queue_nv_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      fence_table,
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      framebuffer_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      image_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      image_view_table,
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      indirect_commands_layout_ext_table,
      #[cfg(feature = "VK_NV_device_generated_commands")]
      indirect_commands_layout_nv_table,
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      indirect_execution_set_ext_table,
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      micromap_ext_table,
      #[cfg(feature = "VK_NV_optical_flow")]
      optical_flow_session_nv_table,
      #[cfg(feature = "VK_INTEL_performance_query")]
      performance_configuration_intel_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_table,
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      pipeline_binary_khr_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_cache_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_layout_table,
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      private_data_slot_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      query_pool_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      queue_table,
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      render_pass_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      sampler_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      sampler_ycbcr_conversion_table,
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      semaphore_table,
      #[cfg(feature = "VK_NV_external_sci_sync2")]
      semaphore_sci_sync_pool_nv_table,
      #[cfg(feature = "VK_EXT_shader_object")]
      shader_ext_table,
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      shader_instrumentation_arm_table,
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      shader_module_table,
      #[cfg(feature = "VK_KHR_swapchain")]
      swapchain_khr_table,
      #[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
      tensor_arm_table,
      #[cfg(feature = "VK_ARM_tensors")]
      tensor_view_arm_table,
      #[cfg(feature = "VK_EXT_validation_cache")]
      validation_cache_ext_table,
      #[cfg(feature = "VK_KHR_video_queue")]
      video_session_khr_table,
      #[cfg(feature = "VK_KHR_video_queue")]
      video_session_parameters_khr_table,
    }
  }
  /// The raw `VkDevice` handle.
  #[inline(always)]
  pub const fn raw(&self) -> VkDevice {
    self.raw
  }
  /// The underlying dispatch table.
  #[inline(always)]
  pub const fn table(&self) -> &DeviceDispatchTable {
    &self.table
  }
  /// The instance that created this device.
  #[inline(always)]
  pub const fn instance(&self) -> &'inst Instance<'inst> {
    self.instance
  }
  /// [`vkGetFaultData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFaultData.html)
  ///
  /// Provided by:
  /// - `VKSC_VERSION_1_0`
  ///
  /// - **Export Scopes:** VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `faultQueryBehavior`
  /// - `pUnrecordedFaults`
  /// - `pFaultCount`: optional: pointer required, values optional if pointer not null
  /// - `pFaults`: optional: true, len: pFaultCount
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
  #[cfg(feature = "VKSC_VERSION_1_0")]
  #[inline(always)]
  pub fn vkGetFaultData(
    &self,
    faultQueryBehavior: VkFaultQueryBehavior,
    pUnrecordedFaults: &mut VkBool32,
    pFaultCount: *mut u32,
    pFaults: *mut VkFaultData,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetFaultData.unwrap_unchecked()(
        self.raw,
        faultQueryBehavior,
        pUnrecordedFaults,
        pFaultCount,
        pFaults,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html)
  ///
  /// Provided by:
  /// - `VK_AMD_anti_lag`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pData`
  #[cfg(feature = "VK_AMD_anti_lag")]
  #[inline(always)]
  pub fn vkAntiLagUpdateAMD(&self, pData: &VkAntiLagDataAMD) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkAntiLagUpdateAMD.unwrap_unchecked()(self.raw, pData)
    }
  }
  /// [`vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html)
  ///
  /// Provided by:
  /// - `VK_ANDROID_external_memory_android_hardware_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `buffer`
  /// - `pProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  #[inline(always)]
  pub fn vkGetAndroidHardwareBufferPropertiesANDROID(
    &self,
    buffer: &AHardwareBuffer,
    pProperties: &mut VkAndroidHardwareBufferPropertiesANDROID,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetAndroidHardwareBufferPropertiesANDROID
        .unwrap_unchecked()(self.raw, buffer, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html)
  ///
  /// Provided by:
  /// - `VK_ANDROID_external_memory_android_hardware_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pBuffer`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
  #[inline(always)]
  pub fn vkGetMemoryAndroidHardwareBufferANDROID(
    &self,
    pInfo: &VkMemoryGetAndroidHardwareBufferInfoANDROID,
    pBuffer: *mut *mut AHardwareBuffer,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryAndroidHardwareBufferANDROID
        .unwrap_unchecked()(self.raw, pInfo, pBuffer)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkBindDataGraphPipelineSessionMemoryARM(
    &self,
    pBindInfos: &[VkBindDataGraphPipelineSessionMemoryInfoARM],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkBindDataGraphPipelineSessionMemoryARM
        .unwrap_unchecked()(self.raw, pBindInfos.len() as u32, pBindInfos.as_ptr())
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSession`
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline]
  pub fn vkCreateDataGraphPipelineSessionARM<'ret>(
    &'ret self,
    pCreateInfo: &VkDataGraphPipelineSessionCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::data_graph_pipeline_session_arm::DataGraphPipelineSessionARM<'ret>, VkResult>
  {
    let mut handle = VkDataGraphPipelineSessionARM::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateDataGraphPipelineSessionARM
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::data_graph_pipeline_session_arm::DataGraphPipelineSessionARM {
          raw: handle,
          parent: self,
          table: &self.data_graph_pipeline_session_arm_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPipelineInfo`
  /// - `pPropertiesCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertiesCount
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkGetDataGraphPipelineAvailablePropertiesARM(
    &self,
    pPipelineInfo: &VkDataGraphPipelineInfoARM,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkDataGraphPipelinePropertyARM,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDataGraphPipelineAvailablePropertiesARM
        .unwrap_unchecked()(self.raw, pPipelineInfo, pPropertiesCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPipelineInfo`
  /// - `propertiesCount`
  /// - `pProperties`: len: propertiesCount
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkGetDataGraphPipelinePropertiesARM(
    &self,
    pPipelineInfo: &VkDataGraphPipelineInfoARM,
    pProperties: &mut [VkDataGraphPipelinePropertyQueryResultARM],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDataGraphPipelinePropertiesARM
        .unwrap_unchecked()(
        self.raw,
        pPipelineInfo,
        pProperties.len() as u32,
        pProperties.as_mut_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pBindPointRequirementCount`: optional: pointer required, values optional if pointer not null
  /// - `pBindPointRequirements`: optional: true, len: pBindPointRequirementCount
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
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkGetDataGraphPipelineSessionBindPointRequirementsARM(
    &self,
    pInfo: &VkDataGraphPipelineSessionBindPointRequirementsInfoARM,
    pBindPointRequirementCount: *mut u32,
    pBindPointRequirements: *mut VkDataGraphPipelineSessionBindPointRequirementARM,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDataGraphPipelineSessionBindPointRequirementsARM
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        pBindPointRequirementCount,
        pBindPointRequirements,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkGetDataGraphPipelineSessionMemoryRequirementsARM(
    &self,
    pInfo: &VkDataGraphPipelineSessionMemoryRequirementsInfoARM,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDataGraphPipelineSessionMemoryRequirementsARM
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_shader_instrumentation`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pInstrumentation`
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
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  #[inline]
  pub fn vkCreateShaderInstrumentationARM<'ret>(
    &'ret self,
    pCreateInfo: &VkShaderInstrumentationCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::shader_instrumentation_arm::ShaderInstrumentationARM<'ret>, VkResult> {
    let mut handle = VkShaderInstrumentationARM::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateShaderInstrumentationARM
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::shader_instrumentation_arm::ShaderInstrumentationARM {
          raw: handle,
          parent: self,
          table: &self.shader_instrumentation_arm_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
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
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkBindTensorMemoryARM(
    &self,
    pBindInfos: &[VkBindTensorMemoryInfoARM],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkBindTensorMemoryARM.unwrap_unchecked()(
        self.raw,
        pBindInfos.len() as u32,
        pBindInfos.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pTensor`
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
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline]
  pub fn vkCreateTensorARM<'ret>(
    &'ret self,
    pCreateInfo: &VkTensorCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::tensor_arm::TensorARM<'ret>, VkResult> {
    let mut handle = VkTensorARM::NULL;
    let r = unsafe {
      (&self.table).vkCreateTensorARM.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::tensor_arm::TensorARM {
        raw: handle,
        parent: self,
        table: &self.tensor_arm_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pView`
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
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline]
  pub fn vkCreateTensorViewARM<'ret>(
    &'ret self,
    pCreateInfo: &VkTensorViewCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::tensor_view_arm::TensorViewARM<'ret>, VkResult> {
    let mut handle = VkTensorViewARM::NULL;
    let r = unsafe {
      (&self.table).vkCreateTensorViewARM.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::tensor_view_arm::TensorViewARM {
        raw: handle,
        parent: self,
        table: &self.tensor_view_arm_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkGetDeviceTensorMemoryRequirementsARM(
    &self,
    pInfo: &VkDeviceTensorMemoryRequirementsARM,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceTensorMemoryRequirementsARM
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkGetTensorMemoryRequirementsARM(
    &self,
    pInfo: &VkTensorMemoryRequirementsInfoARM,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetTensorMemoryRequirementsARM
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkGetTensorOpaqueCaptureDescriptorDataARM(
    &self,
    pInfo: &VkTensorCaptureDescriptorDataInfoARM,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetTensorOpaqueCaptureDescriptorDataARM
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkGetTensorViewOpaqueCaptureDescriptorDataARM(
    &self,
    pInfo: &VkTensorViewCaptureDescriptorDataInfoARM,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetTensorViewOpaqueCaptureDescriptorDataARM
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pAllocateInfo`
  /// - `pAllocator`: optional: true
  /// - `pMemory`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkAllocateMemory<'ret>(
    &'ret self,
    pAllocateInfo: &VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::device_memory::DeviceMemory<'ret>, VkResult> {
    let mut handle = VkDeviceMemory::NULL;
    let r = unsafe {
      (&self.table).vkAllocateMemory.unwrap_unchecked()(
        self.raw,
        pAllocateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::device_memory::DeviceMemory {
        raw: handle,
        parent: self,
        table: &self.device_memory_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pBuffer`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateBuffer<'ret>(
    &'ret self,
    pCreateInfo: &VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::buffer::Buffer<'ret>, VkResult> {
    let mut handle = VkBuffer::NULL;
    let r = unsafe {
      (&self.table).vkCreateBuffer.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::buffer::Buffer {
        raw: handle,
        parent: self,
        table: &self.buffer_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pCommandPool`
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateCommandPool<'dev>(
    &'dev self,
    pCreateInfo: &VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::command_pool::CommandPool<'dev>, VkResult> {
    let mut raw = VkCommandPool::NULL;
    let fp = unsafe { self.table.vkCreateCommandPool.unwrap_unchecked() };
    let r = unsafe { fp(self.raw, pCreateInfo, pAllocator, &mut raw) };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::command_pool::CommandPool {
        raw,
        parent: self,
        table: &self.command_pool_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pFence`
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateFence<'ret>(
    &'ret self,
    pCreateInfo: &VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::fence::Fence<'ret>, VkResult> {
    let mut handle = VkFence::NULL;
    let r = unsafe {
      (&self.table).vkCreateFence.unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::fence::Fence {
        raw: handle,
        parent: self,
        table: &self.fence_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pImage`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_COMPRESSION_EXHAUSTED_EXT`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateImage<'ret>(
    &'ret self,
    pCreateInfo: &VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::image::Image<'ret>, VkResult> {
    let mut handle = VkImage::NULL;
    let r = unsafe {
      (&self.table).vkCreateImage.unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::image::Image {
        raw: handle,
        parent: self,
        table: &self.image_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pView`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateImageView<'ret>(
    &'ret self,
    pCreateInfo: &VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::image_view::ImageView<'ret>, VkResult> {
    let mut handle = VkImageView::NULL;
    let r = unsafe {
      (&self.table).vkCreateImageView.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::image_view::ImageView {
        raw: handle,
        parent: self,
        table: &self.image_view_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pQueryPool`
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateQueryPool<'ret>(
    &'ret self,
    pCreateInfo: &VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::query_pool::QueryPool<'ret>, VkResult> {
    let mut handle = VkQueryPool::NULL;
    let r = unsafe {
      (&self.table).vkCreateQueryPool.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::query_pool::QueryPool {
        raw: handle,
        parent: self,
        table: &self.query_pool_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSemaphore`
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateSemaphore<'ret>(
    &'ret self,
    pCreateInfo: &VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::semaphore::Semaphore<'ret>, VkResult> {
    let mut handle = VkSemaphore::NULL;
    let r = unsafe {
      (&self.table).vkCreateSemaphore.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::semaphore::Semaphore {
        raw: handle,
        parent: self,
        table: &self.semaphore_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`: optional: true
  /// - `pAllocator`: optional: true
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkDestroyDevice(&mut self, pAllocator: *const VkAllocationCallbacks) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkDestroyDevice.unwrap_unchecked()(self.raw, pAllocator)
    }
    self.raw = VkDevice::NULL;
  }
  /// [`vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkDeviceWaitIdle(&self) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkDeviceWaitIdle.unwrap_unchecked()(self.raw) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `memoryRangeCount`
  /// - `pMemoryRanges`: len: memoryRangeCount
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkFlushMappedMemoryRanges(
    &self,
    pMemoryRanges: &[VkMappedMemoryRange],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkFlushMappedMemoryRanges.unwrap_unchecked()(
        self.raw,
        pMemoryRanges.len() as u32,
        pMemoryRanges.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `queueFamilyIndex`
  /// - `queueIndex`
  /// - `pQueue`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkGetDeviceQueue<'dev>(
    &'dev self,
    queueFamilyIndex: u32,
    queueIndex: u32,
  ) -> crate::queue::Queue<'dev> {
    let mut raw = VkQueue::NULL;
    let fp = unsafe { self.table.vkGetDeviceQueue.unwrap_unchecked() };
    unsafe { fp(self.raw, queueFamilyIndex, queueIndex, &mut raw) };
    crate::queue::Queue {
      raw,
      parent: self,
      table: &self.queue_table,
    }
  }
  /// [`vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `memoryRangeCount`
  /// - `pMemoryRanges`: len: memoryRangeCount
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
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkInvalidateMappedMemoryRanges(
    &self,
    pMemoryRanges: &[VkMappedMemoryRange],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkInvalidateMappedMemoryRanges
        .unwrap_unchecked()(self.raw, pMemoryRanges.len() as u32, pMemoryRanges.as_ptr())
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `fenceCount`
  /// - `pFences`: len: fenceCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkResetFences(&self, pFences: &[VkFence]) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkResetFences.unwrap_unchecked()(
        self.raw,
        pFences.len() as u32,
        pFences.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `fenceCount`
  /// - `pFences`: len: fenceCount
  /// - `waitAll`
  /// - `timeout`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_TIMEOUT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkWaitForFences(
    &self,
    pFences: &[VkFence],
    waitAll: VkBool32,
    timeout: u64,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkWaitForFences.unwrap_unchecked()(
        self.raw,
        pFences.len() as u32,
        pFences.as_ptr(),
        waitAll,
        timeout,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkBindBufferMemory2(
    &self,
    pBindInfos: &[VkBindBufferMemoryInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkBindBufferMemory2.unwrap_unchecked()(
        self.raw,
        pBindInfos.len() as u32,
        pBindInfos.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
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
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkBindImageMemory2(
    &self,
    pBindInfos: &[VkBindImageMemoryInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkBindImageMemory2.unwrap_unchecked()(
        self.raw,
        pBindInfos.len() as u32,
        pBindInfos.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetBufferMemoryRequirements2(
    &self,
    pInfo: &VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetBufferMemoryRequirements2
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `heapIndex`
  /// - `localDeviceIndex`
  /// - `remoteDeviceIndex`
  /// - `pPeerMemoryFeatures`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetDeviceGroupPeerMemoryFeatures(
    &self,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: &mut VkPeerMemoryFeatureFlags,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceGroupPeerMemoryFeatures
        .unwrap_unchecked()(
        self.raw,
        heapIndex,
        localDeviceIndex,
        remoteDeviceIndex,
        pPeerMemoryFeatures,
      )
    }
  }
  /// [`vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pQueueInfo`
  /// - `pQueue`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetDeviceQueue2(&self, pQueueInfo: &VkDeviceQueueInfo2, pQueue: &mut VkQueue) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetDeviceQueue2.unwrap_unchecked()(self.raw, pQueueInfo, pQueue)
    }
  }
  /// [`vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetImageMemoryRequirements2(
    &self,
    pInfo: &VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetImageMemoryRequirements2
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
  /// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetImageSparseMemoryRequirements2(
    &self,
    pInfo: &VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetImageSparseMemoryRequirements2
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
      )
    }
  }
  /// [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  #[inline(always)]
  pub fn vkGetBufferDeviceAddress(&self, pInfo: &VkBufferDeviceAddressInfo) -> VkDeviceAddress {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetBufferDeviceAddress.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  #[inline(always)]
  pub fn vkGetBufferOpaqueCaptureAddress(&self, pInfo: &VkBufferDeviceAddressInfo) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetBufferOpaqueCaptureAddress
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  #[inline(always)]
  pub fn vkGetDeviceMemoryOpaqueCaptureAddress(
    &self,
    pInfo: &VkDeviceMemoryOpaqueCaptureAddressInfo,
  ) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceMemoryOpaqueCaptureAddress
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pSignalInfo`
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
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  #[inline(always)]
  pub fn vkSignalSemaphore(
    &self,
    pSignalInfo: &VkSemaphoreSignalInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkSignalSemaphore.unwrap_unchecked()(self.raw, pSignalInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pWaitInfo`
  /// - `timeout`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_TIMEOUT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_2")]
  #[inline(always)]
  pub fn vkWaitSemaphores(
    &self,
    pWaitInfo: &VkSemaphoreWaitInfo,
    timeout: u64,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkWaitSemaphores.unwrap_unchecked()(self.raw, pWaitInfo, timeout) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pPrivateDataSlot`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline]
  pub fn vkCreatePrivateDataSlot<'ret>(
    &'ret self,
    pCreateInfo: &VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::private_data_slot::PrivateDataSlot<'ret>, VkResult> {
    let mut handle = VkPrivateDataSlot::NULL;
    let r = unsafe {
      (&self.table).vkCreatePrivateDataSlot.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::private_data_slot::PrivateDataSlot {
        raw: handle,
        parent: self,
        table: &self.private_data_slot_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkGetDeviceBufferMemoryRequirements(
    &self,
    pInfo: &VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceBufferMemoryRequirements
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkGetDeviceImageMemoryRequirements(
    &self,
    pInfo: &VkDeviceImageMemoryRequirements,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageMemoryRequirements
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
  /// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkGetDeviceImageSparseMemoryRequirements(
    &self,
    pInfo: &VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageSparseMemoryRequirements
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
      )
    }
  }
  /// [`vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `objectType`
  /// - `objectHandle`: object_type: objectType
  /// - `privateDataSlot`
  /// - `pData`
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkGetPrivateData(
    &self,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: &mut u64,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetPrivateData.unwrap_unchecked()(
        self.raw,
        objectType,
        objectHandle,
        privateDataSlot,
        pData,
      )
    }
  }
  /// [`vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `objectType`
  /// - `objectHandle`: object_type: objectType
  /// - `privateDataSlot`
  /// - `data`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkSetPrivateData(
    &self,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkSetPrivateData.unwrap_unchecked()(
        self.raw,
        objectType,
        objectHandle,
        privateDataSlot,
        data,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyImageToImageInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkCopyImageToImage(
    &self,
    pCopyImageToImageInfo: &VkCopyImageToImageInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyImageToImage.unwrap_unchecked()(self.raw, pCopyImageToImageInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyImageToMemoryInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkCopyImageToMemory(
    &self,
    pCopyImageToMemoryInfo: &VkCopyImageToMemoryInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyImageToMemory.unwrap_unchecked()(self.raw, pCopyImageToMemoryInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyMemoryToImageInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkCopyMemoryToImage(
    &self,
    pCopyMemoryToImageInfo: &VkCopyMemoryToImageInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyMemoryToImage.unwrap_unchecked()(self.raw, pCopyMemoryToImageInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pLayout`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkGetDeviceImageSubresourceLayout(
    &self,
    pInfo: &VkDeviceImageSubresourceInfo,
    pLayout: &mut VkSubresourceLayout2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageSubresourceLayout
        .unwrap_unchecked()(self.raw, pInfo, pLayout)
    }
  }
  /// [`vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pMemoryMapInfo`
  /// - `ppData`: optional: pointer required, values optional if pointer not null
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkMapMemory2(
    &self,
    pMemoryMapInfo: &VkMemoryMapInfo,
    ppData: *mut *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkMapMemory2.unwrap_unchecked()(self.raw, pMemoryMapInfo, ppData) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `transitionCount`
  /// - `pTransitions`: len: transitionCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkTransitionImageLayout(
    &self,
    pTransitions: &[VkHostImageLayoutTransitionInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkTransitionImageLayout.unwrap_unchecked()(
        self.raw,
        pTransitions.len() as u32,
        pTransitions.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pMemoryUnmapInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_4")]
  #[inline(always)]
  pub fn vkUnmapMemory2(&self, pMemoryUnmapInfo: &VkMemoryUnmapInfo) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkUnmapMemory2.unwrap_unchecked()(self.raw, pMemoryUnmapInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pView`
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
  #[inline]
  pub fn vkCreateBufferView<'ret>(
    &'ret self,
    pCreateInfo: &VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::buffer_view::BufferView<'ret>, VkResult> {
    let mut handle = VkBufferView::NULL;
    let r = unsafe {
      (&self.table).vkCreateBufferView.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::buffer_view::BufferView {
        raw: handle,
        parent: self,
        table: &self.buffer_view_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** VulkanSC
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateComputePipelines<'dev>(
    &'dev self,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<alloc::boxed::Box<[crate::pipeline::Pipeline<'dev>]>, VkResult> {
    let count = createInfoCount;
    let mut raw_pipelines = alloc::boxed::Box::<[VkPipeline]>::new_uninit_slice(count as usize);
    let fp = unsafe { self.table.vkCreateComputePipelines.unwrap_unchecked() };
    let r = unsafe {
      fp(
        self.raw,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        raw_pipelines.as_mut_ptr().cast(),
      )
    };
    if r < VkResult::VK_SUCCESS {
      return Err(r);
    }
    let raw_pipelines = unsafe { raw_pipelines.assume_init() };
    Ok(
      raw_pipelines
        .into_iter()
        .map(|raw| crate::pipeline::Pipeline {
          raw,
          parent: self,
          table: &self.pipeline_table,
        })
        .collect(),
    )
  }
  /// [`vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pDescriptorPool`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_FRAGMENTATION_EXT`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateDescriptorPool<'ret>(
    &'ret self,
    pCreateInfo: &VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::descriptor_pool::DescriptorPool<'ret>, VkResult> {
    let mut handle = VkDescriptorPool::NULL;
    let r = unsafe {
      (&self.table).vkCreateDescriptorPool.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::descriptor_pool::DescriptorPool {
        raw: handle,
        parent: self,
        table: &self.descriptor_pool_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSetLayout`
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
  #[inline]
  pub fn vkCreateDescriptorSetLayout<'ret>(
    &'ret self,
    pCreateInfo: &VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::descriptor_set_layout::DescriptorSetLayout<'ret>, VkResult> {
    let mut handle = VkDescriptorSetLayout::NULL;
    let r = unsafe {
      (&self.table).vkCreateDescriptorSetLayout.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::descriptor_set_layout::DescriptorSetLayout {
        raw: handle,
        parent: self,
        table: &self.descriptor_set_layout_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pEvent`
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
  #[inline]
  pub fn vkCreateEvent<'ret>(
    &'ret self,
    pCreateInfo: &VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::event::Event<'ret>, VkResult> {
    let mut handle = VkEvent::NULL;
    let r = unsafe {
      (&self.table).vkCreateEvent.unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::event::Event {
        raw: handle,
        parent: self,
        table: &self.event_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pPipelineCache`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_PIPELINE_CACHE_DATA`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub fn vkCreatePipelineCache<'ret>(
    &'ret self,
    pCreateInfo: &VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::pipeline_cache::PipelineCache<'ret>, VkResult> {
    let mut handle = VkPipelineCache::NULL;
    let r = unsafe {
      (&self.table).vkCreatePipelineCache.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::pipeline_cache::PipelineCache {
        raw: handle,
        parent: self,
        table: &self.pipeline_cache_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pPipelineLayout`
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
  #[inline]
  pub fn vkCreatePipelineLayout<'ret>(
    &'ret self,
    pCreateInfo: &VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::pipeline_layout::PipelineLayout<'ret>, VkResult> {
    let mut handle = VkPipelineLayout::NULL;
    let r = unsafe {
      (&self.table).vkCreatePipelineLayout.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::pipeline_layout::PipelineLayout {
        raw: handle,
        parent: self,
        table: &self.pipeline_layout_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSampler`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateSampler<'ret>(
    &'ret self,
    pCreateInfo: &VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::sampler::Sampler<'ret>, VkResult> {
    let mut handle = VkSampler::NULL;
    let r = unsafe {
      (&self.table).vkCreateSampler.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::sampler::Sampler {
        raw: handle,
        parent: self,
        table: &self.sampler_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pShaderModule`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_SHADER_NV`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateShaderModule<'ret>(
    &'ret self,
    pCreateInfo: &VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::shader_module::ShaderModule<'ret>, VkResult> {
    let mut handle = VkShaderModule::NULL;
    let r = unsafe {
      (&self.table).vkCreateShaderModule.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::shader_module::ShaderModule {
        raw: handle,
        parent: self,
        table: &self.shader_module_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `descriptorWriteCount`: optional: true
  /// - `pDescriptorWrites`: len: descriptorWriteCount
  /// - `descriptorCopyCount`: optional: true
  /// - `pDescriptorCopies`: len: descriptorCopyCount
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkUpdateDescriptorSets(
    &self,
    pDescriptorWrites: &[VkWriteDescriptorSet],
    pDescriptorCopies: &[VkCopyDescriptorSet],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkUpdateDescriptorSets.unwrap_unchecked()(
        self.raw,
        pDescriptorWrites.len() as u32,
        pDescriptorWrites.as_ptr(),
        pDescriptorCopies.len() as u32,
        pDescriptorCopies.as_ptr(),
      )
    }
  }
  /// [`vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pDescriptorUpdateTemplate`
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  #[inline]
  pub fn vkCreateDescriptorUpdateTemplate<'ret>(
    &'ret self,
    pCreateInfo: &VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::descriptor_update_template::DescriptorUpdateTemplate<'ret>, VkResult> {
    let mut handle = VkDescriptorUpdateTemplate::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateDescriptorUpdateTemplate
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::descriptor_update_template::DescriptorUpdateTemplate {
          raw: handle,
          parent: self,
          table: &self.descriptor_update_template_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_1`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pYcbcrConversion`
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
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  #[inline]
  pub fn vkCreateSamplerYcbcrConversion<'ret>(
    &'ret self,
    pCreateInfo: &VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::sampler_ycbcr_conversion::SamplerYcbcrConversion<'ret>, VkResult> {
    let mut handle = VkSamplerYcbcrConversion::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateSamplerYcbcrConversion
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::sampler_ycbcr_conversion::SamplerYcbcrConversion {
        raw: handle,
        parent: self,
        table: &self.sampler_ycbcr_conversion_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html)
  ///
  /// Provided by:
  /// - `VK_COMPUTE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pSupport`
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetDescriptorSetLayoutSupport(
    &self,
    pCreateInfo: &VkDescriptorSetLayoutCreateInfo,
    pSupport: &mut VkDescriptorSetLayoutSupport,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDescriptorSetLayoutSupport
        .unwrap_unchecked()(self.raw, pCreateInfo, pSupport)
    }
  }
  /// [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)
  ///
  /// Provided by:
  /// - `VK_EXT_buffer_device_address`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_EXT_buffer_device_address")]
  #[inline(always)]
  pub fn vkGetBufferDeviceAddressEXT(&self, pInfo: &VkBufferDeviceAddressInfo) -> VkDeviceAddress {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetBufferDeviceAddressEXT.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html)
  ///
  /// Provided by:
  /// - `VK_EXT_calibrated_timestamps`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `timestampCount`
  /// - `pTimestampInfos`: len: timestampCount
  /// - `pTimestamps`: len: timestampCount
  /// - `pMaxDeviation`
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
  #[cfg(feature = "VK_EXT_calibrated_timestamps")]
  #[inline(always)]
  pub fn vkGetCalibratedTimestampsEXT(
    &self,
    pTimestampInfos: &[VkCalibratedTimestampInfoKHR],
    pTimestamps: &mut [u64],
    pMaxDeviation: &mut u64,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetCalibratedTimestampsEXT
        .unwrap_unchecked()(
        self.raw,
        pTimestamps.len() as u32,
        pTimestampInfos.as_ptr(),
        pTimestamps.as_mut_ptr(),
        pMaxDeviation,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_marker`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pNameInfo`
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
  #[cfg(feature = "VK_EXT_debug_marker")]
  #[inline(always)]
  pub fn vkDebugMarkerSetObjectNameEXT(
    &self,
    pNameInfo: &VkDebugMarkerObjectNameInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkDebugMarkerSetObjectNameEXT
        .unwrap_unchecked()(self.raw, pNameInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_marker`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pTagInfo`
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
  #[cfg(feature = "VK_EXT_debug_marker")]
  #[inline(always)]
  pub fn vkDebugMarkerSetObjectTagEXT(
    &self,
    pTagInfo: &VkDebugMarkerObjectTagInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkDebugMarkerSetObjectTagEXT
        .unwrap_unchecked()(self.raw, pTagInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_utils`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pNameInfo`
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
  #[cfg(feature = "VK_EXT_debug_utils")]
  #[inline(always)]
  pub fn vkSetDebugUtilsObjectNameEXT(
    &self,
    pNameInfo: &VkDebugUtilsObjectNameInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkSetDebugUtilsObjectNameEXT
        .unwrap_unchecked()(self.raw, pNameInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_debug_utils`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pTagInfo`
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
  #[cfg(feature = "VK_EXT_debug_utils")]
  #[inline(always)]
  pub fn vkSetDebugUtilsObjectTagEXT(
    &self,
    pTagInfo: &VkDebugUtilsObjectTagInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkSetDebugUtilsObjectTagEXT.unwrap_unchecked()(self.raw, pTagInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT(
    &self,
    pInfo: &VkAccelerationStructureCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetBufferOpaqueCaptureDescriptorDataEXT(
    &self,
    pInfo: &VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetBufferOpaqueCaptureDescriptorDataEXT
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pDescriptorInfo`
  /// - `dataSize`
  /// - `pDescriptor`: len: dataSize
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetDescriptorEXT(
    &self,
    pDescriptorInfo: &VkDescriptorGetInfoEXT,
    dataSize: usize,
    pDescriptor: *mut core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetDescriptorEXT.unwrap_unchecked()(
        self.raw,
        pDescriptorInfo,
        dataSize,
        pDescriptor,
      )
    }
  }
  /// [`vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetImageOpaqueCaptureDescriptorDataEXT(
    &self,
    pInfo: &VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetImageOpaqueCaptureDescriptorDataEXT
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetImageViewOpaqueCaptureDescriptorDataEXT(
    &self,
    pInfo: &VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetImageViewOpaqueCaptureDescriptorDataEXT
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pData`
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
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  #[inline(always)]
  pub fn vkGetSamplerOpaqueCaptureDescriptorDataEXT(
    &self,
    pInfo: &VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetSamplerOpaqueCaptureDescriptorDataEXT
        .unwrap_unchecked()(self.raw, pInfo, pData)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `imageCount`
  /// - `pImages`: len: imageCount
  /// - `pDatas`: len: imageCount
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
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkGetImageOpaqueCaptureDataEXT(
    &self,
    pImages: &[VkImage],
    pDatas: &mut [VkHostAddressRangeEXT],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetImageOpaqueCaptureDataEXT
        .unwrap_unchecked()(
        self.raw,
        pDatas.len() as u32,
        pImages.as_ptr(),
        pDatas.as_mut_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `tensorCount`
  /// - `pTensors`: len: tensorCount
  /// - `pDatas`: len: tensorCount
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
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkGetTensorOpaqueCaptureDataARM(
    &self,
    pTensors: &[VkTensorARM],
    pDatas: &mut [VkHostAddressRangeEXT],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetTensorOpaqueCaptureDataARM
        .unwrap_unchecked()(
        self.raw,
        pDatas.len() as u32,
        pTensors.as_ptr(),
        pDatas.as_mut_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pBorderColor`
  /// - `requestIndex`
  /// - `pIndex`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkRegisterCustomBorderColorEXT(
    &self,
    pBorderColor: &VkSamplerCustomBorderColorCreateInfoEXT,
    requestIndex: VkBool32,
    pIndex: &mut u32,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkRegisterCustomBorderColorEXT
        .unwrap_unchecked()(self.raw, pBorderColor, requestIndex, pIndex)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `index`
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkUnregisterCustomBorderColorEXT(&self, index: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkUnregisterCustomBorderColorEXT
        .unwrap_unchecked()(self.raw, index)
    }
  }
  /// [`vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `resourceCount`
  /// - `pResources`: len: resourceCount
  /// - `pDescriptors`: len: resourceCount
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
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkWriteResourceDescriptorsEXT(
    &self,
    pResources: &[VkResourceDescriptorInfoEXT],
    pDescriptors: &[VkHostAddressRangeEXT],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkWriteResourceDescriptorsEXT
        .unwrap_unchecked()(
        self.raw,
        pDescriptors.len() as u32,
        pResources.as_ptr(),
        pDescriptors.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `samplerCount`
  /// - `pSamplers`: len: samplerCount
  /// - `pDescriptors`: len: samplerCount
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
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkWriteSamplerDescriptorsEXT(
    &self,
    pSamplers: &[VkSamplerCreateInfo],
    pDescriptors: &[VkHostAddressRangeEXT],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkWriteSamplerDescriptorsEXT
        .unwrap_unchecked()(
        self.raw,
        pDescriptors.len() as u32,
        pSamplers.as_ptr(),
        pDescriptors.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_device_fault`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pFaultCounts`
  /// - `pFaultInfo`: optional: true
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_device_fault")]
  #[inline(always)]
  pub fn vkGetDeviceFaultInfoEXT(
    &self,
    pFaultCounts: &mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetDeviceFaultInfoEXT.unwrap_unchecked()(self.raw, pFaultCounts, pFaultInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pIndirectCommandsLayout`
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
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  #[inline]
  pub fn vkCreateIndirectCommandsLayoutEXT<'ret>(
    &'ret self,
    pCreateInfo: &VkIndirectCommandsLayoutCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::indirect_commands_layout_ext::IndirectCommandsLayoutEXT<'ret>, VkResult> {
    let mut handle = VkIndirectCommandsLayoutEXT::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateIndirectCommandsLayoutEXT
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::indirect_commands_layout_ext::IndirectCommandsLayoutEXT {
          raw: handle,
          parent: self,
          table: &self.indirect_commands_layout_ext_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pIndirectExecutionSet`
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
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  #[inline]
  pub fn vkCreateIndirectExecutionSetEXT<'ret>(
    &'ret self,
    pCreateInfo: &VkIndirectExecutionSetCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::indirect_execution_set_ext::IndirectExecutionSetEXT<'ret>, VkResult> {
    let mut handle = VkIndirectExecutionSetEXT::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateIndirectExecutionSetEXT
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::indirect_execution_set_ext::IndirectExecutionSetEXT {
        raw: handle,
        parent: self,
        table: &self.indirect_execution_set_ext_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  #[inline(always)]
  pub fn vkGetGeneratedCommandsMemoryRequirementsEXT(
    &self,
    pInfo: &VkGeneratedCommandsMemoryRequirementsInfoEXT,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetGeneratedCommandsMemoryRequirementsEXT
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_display_control`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pDeviceEventInfo`
  /// - `pAllocator`: optional: true
  /// - `pFence`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_display_control")]
  #[inline]
  pub fn vkRegisterDeviceEventEXT<'ret>(
    &'ret self,
    pDeviceEventInfo: &VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::fence::Fence<'ret>, VkResult> {
    let mut handle = VkFence::NULL;
    let r = unsafe {
      (&self.table).vkRegisterDeviceEventEXT.unwrap_unchecked()(
        self.raw,
        pDeviceEventInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::fence::Fence {
        raw: handle,
        parent: self,
        table: &self.fence_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_external_memory_host`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `handleType`
  /// - `pHostPointer`
  /// - `pMemoryHostPointerProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_external_memory_host")]
  #[inline(always)]
  pub fn vkGetMemoryHostPointerPropertiesEXT(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const core::ffi::c_void,
    pMemoryHostPointerProperties: &mut VkMemoryHostPointerPropertiesEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryHostPointerPropertiesEXT
        .unwrap_unchecked()(
        self.raw,
        handleType,
        pHostPointer,
        pMemoryHostPointerProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_external_memory_metal`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetMetalHandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_external_memory_metal")]
  #[inline(always)]
  pub fn vkGetMemoryMetalHandleEXT(
    &self,
    pGetMetalHandleInfo: &VkMemoryGetMetalHandleInfoEXT,
    pHandle: *mut *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemoryMetalHandleEXT.unwrap_unchecked()(
        self.raw,
        pGetMetalHandleInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_external_memory_metal`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `handleType`
  /// - `pHandle`
  /// - `pMemoryMetalHandleProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_external_memory_metal")]
  #[inline(always)]
  pub fn vkGetMemoryMetalHandlePropertiesEXT(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHandle: *const core::ffi::c_void,
    pMemoryMetalHandleProperties: &mut VkMemoryMetalHandlePropertiesEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryMetalHandlePropertiesEXT
        .unwrap_unchecked()(self.raw, handleType, pHandle, pMemoryMetalHandleProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_full_screen_exclusive`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pSurfaceInfo`
  /// - `pModes`: optional: pointer required, values optional if pointer not null
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_full_screen_exclusive")]
  #[inline(always)]
  pub fn vkGetDeviceGroupSurfacePresentModes2EXT(
    &self,
    pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDeviceGroupSurfacePresentModes2EXT
        .unwrap_unchecked()(self.raw, pSurfaceInfo, pModes)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_hdr_metadata`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `swapchainCount`
  /// - `pSwapchains`: len: swapchainCount
  /// - `pMetadata`: len: swapchainCount
  #[cfg(feature = "VK_EXT_hdr_metadata")]
  #[inline(always)]
  pub fn vkSetHdrMetadataEXT(
    &self,
    pSwapchains: &[VkSwapchainKHR],
    pMetadata: &[VkHdrMetadataEXT],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkSetHdrMetadataEXT.unwrap_unchecked()(
        self.raw,
        pMetadata.len() as u32,
        pSwapchains.as_ptr(),
        pMetadata.as_ptr(),
      )
    }
  }
  /// [`vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html)
  ///
  /// Provided by:
  /// - `VK_EXT_host_image_copy`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyImageToImageInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_host_image_copy")]
  #[inline(always)]
  pub fn vkCopyImageToImageEXT(
    &self,
    pCopyImageToImageInfo: &VkCopyImageToImageInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyImageToImageEXT.unwrap_unchecked()(self.raw, pCopyImageToImageInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html)
  ///
  /// Provided by:
  /// - `VK_EXT_host_image_copy`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyImageToMemoryInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_host_image_copy")]
  #[inline(always)]
  pub fn vkCopyImageToMemoryEXT(
    &self,
    pCopyImageToMemoryInfo: &VkCopyImageToMemoryInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyImageToMemoryEXT.unwrap_unchecked()(self.raw, pCopyImageToMemoryInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html)
  ///
  /// Provided by:
  /// - `VK_EXT_host_image_copy`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCopyMemoryToImageInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_host_image_copy")]
  #[inline(always)]
  pub fn vkCopyMemoryToImageEXT(
    &self,
    pCopyMemoryToImageInfo: &VkCopyMemoryToImageInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCopyMemoryToImageEXT.unwrap_unchecked()(self.raw, pCopyMemoryToImageInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html)
  ///
  /// Provided by:
  /// - `VK_EXT_host_image_copy`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `transitionCount`
  /// - `pTransitions`: len: transitionCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_host_image_copy")]
  #[inline(always)]
  pub fn vkTransitionImageLayoutEXT(
    &self,
    pTransitions: &[VkHostImageLayoutTransitionInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkTransitionImageLayoutEXT.unwrap_unchecked()(
        self.raw,
        pTransitions.len() as u32,
        pTransitions.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_metal_objects`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pMetalObjectsInfo`
  #[cfg(feature = "VK_EXT_metal_objects")]
  #[inline(always)]
  pub fn vkExportMetalObjectsEXT(&self, pMetalObjectsInfo: &mut VkExportMetalObjectsInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkExportMetalObjectsEXT.unwrap_unchecked()(self.raw, pMetalObjectsInfo)
    }
  }
  /// [`vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pMicromap`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline]
  pub fn vkCreateMicromapEXT<'ret>(
    &'ret self,
    pCreateInfo: &VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::micromap_ext::MicromapEXT<'ret>, VkResult> {
    let mut handle = VkMicromapEXT::NULL;
    let r = unsafe {
      (&self.table).vkCreateMicromapEXT.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::micromap_ext::MicromapEXT {
        raw: handle,
        parent: self,
        table: &self.micromap_ext_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pVersionInfo`
  /// - `pCompatibility`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkGetDeviceMicromapCompatibilityEXT(
    &self,
    pVersionInfo: &VkMicromapVersionInfoEXT,
    pCompatibility: &mut VkAccelerationStructureCompatibilityKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceMicromapCompatibilityEXT
        .unwrap_unchecked()(self.raw, pVersionInfo, pCompatibility)
    }
  }
  /// [`vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `buildType`
  /// - `pBuildInfo`
  /// - `pSizeInfo`
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkGetMicromapBuildSizesEXT(
    &self,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: &VkMicromapBuildInfoEXT,
    pSizeInfo: &mut VkMicromapBuildSizesInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetMicromapBuildSizesEXT.unwrap_unchecked()(
        self.raw, buildType, pBuildInfo, pSizeInfo,
      )
    }
  }
  /// [`vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_opacity_micromap`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `micromapCount`
  /// - `pMicromaps`: len: micromapCount
  /// - `queryType`
  /// - `dataSize`
  /// - `pData`: len: dataSize
  /// - `stride`
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
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  #[inline(always)]
  pub fn vkWriteMicromapsPropertiesEXT(
    &self,
    pMicromaps: &[VkMicromapEXT],
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
    stride: usize,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkWriteMicromapsPropertiesEXT
        .unwrap_unchecked()(
        self.raw,
        pMicromaps.len() as u32,
        pMicromaps.as_ptr(),
        queryType,
        dataSize,
        pData,
        stride,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_pipeline_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPipelineInfo`
  /// - `pPipelineProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_pipeline_properties")]
  #[inline(always)]
  pub fn vkGetPipelinePropertiesEXT(
    &self,
    pPipelineInfo: &VkPipelineInfoKHR,
    pPipelineProperties: &mut VkBaseOutStructure,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetPipelinePropertiesEXT.unwrap_unchecked()(
        self.raw,
        pPipelineInfo,
        pPipelineProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_present_timing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPastPresentationTimingInfo`
  /// - `pPastPresentationTimingProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_OUT_OF_DATE_KHR`
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_present_timing")]
  #[inline(always)]
  pub fn vkGetPastPresentationTimingEXT(
    &self,
    pPastPresentationTimingInfo: &VkPastPresentationTimingInfoEXT,
    pPastPresentationTimingProperties: &mut VkPastPresentationTimingPropertiesEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetPastPresentationTimingEXT
        .unwrap_unchecked()(
        self.raw,
        pPastPresentationTimingInfo,
        pPastPresentationTimingProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html)
  ///
  /// Provided by:
  /// - `VK_EXT_private_data`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pPrivateDataSlot`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_private_data")]
  #[inline]
  pub fn vkCreatePrivateDataSlotEXT<'ret>(
    &'ret self,
    pCreateInfo: &VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::private_data_slot::PrivateDataSlot<'ret>, VkResult> {
    let mut handle = VkPrivateDataSlot::NULL;
    let r = unsafe {
      (&self.table).vkCreatePrivateDataSlotEXT.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::private_data_slot::PrivateDataSlot {
        raw: handle,
        parent: self,
        table: &self.private_data_slot_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html)
  ///
  /// Provided by:
  /// - `VK_EXT_private_data`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `objectType`
  /// - `objectHandle`: object_type: objectType
  /// - `privateDataSlot`
  /// - `pData`
  #[cfg(feature = "VK_EXT_private_data")]
  #[inline(always)]
  pub fn vkGetPrivateDataEXT(
    &self,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: &mut u64,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetPrivateDataEXT.unwrap_unchecked()(
        self.raw,
        objectType,
        objectHandle,
        privateDataSlot,
        pData,
      )
    }
  }
  /// [`vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html)
  ///
  /// Provided by:
  /// - `VK_EXT_private_data`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `objectType`
  /// - `objectHandle`: object_type: objectType
  /// - `privateDataSlot`
  /// - `data`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_private_data")]
  #[inline(always)]
  pub fn vkSetPrivateDataEXT(
    &self,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkSetPrivateDataEXT.unwrap_unchecked()(
        self.raw,
        objectType,
        objectHandle,
        privateDataSlot,
        data,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_shader_module_identifier`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pIdentifier`
  #[cfg(feature = "VK_EXT_shader_module_identifier")]
  #[inline(always)]
  pub fn vkGetShaderModuleCreateInfoIdentifierEXT(
    &self,
    pCreateInfo: &VkShaderModuleCreateInfo,
    pIdentifier: &mut VkShaderModuleIdentifierEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetShaderModuleCreateInfoIdentifierEXT
        .unwrap_unchecked()(self.raw, pCreateInfo, pIdentifier)
    }
  }
  /// [`vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_shader_object`
  ///
  /// - **Allow No Queues:** True
  ///
  /// # Parameters
  /// - `device`
  /// - `createInfoCount`
  /// - `pCreateInfos`: len: createInfoCount
  /// - `pAllocator`: optional: true
  /// - `pShaders`: len: createInfoCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPATIBLE_SHADER_BINARY_EXT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_shader_object")]
  #[inline(always)]
  pub fn vkCreateShadersEXT(
    &self,
    pCreateInfos: &[VkShaderCreateInfoEXT],
    pAllocator: *const VkAllocationCallbacks,
    pShaders: &mut [VkShaderEXT],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCreateShadersEXT.unwrap_unchecked()(
        self.raw,
        pShaders.len() as u32,
        pCreateInfos.as_ptr(),
        pAllocator,
        pShaders.as_mut_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html)
  ///
  /// Provided by:
  /// - `VK_EXT_swapchain_maintenance1`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pReleaseInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_swapchain_maintenance1")]
  #[inline(always)]
  pub fn vkReleaseSwapchainImagesEXT(
    &self,
    pReleaseInfo: &VkReleaseSwapchainImagesInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkReleaseSwapchainImagesEXT.unwrap_unchecked()(self.raw, pReleaseInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_validation_cache`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pValidationCache`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_validation_cache")]
  #[inline]
  pub fn vkCreateValidationCacheEXT<'ret>(
    &'ret self,
    pCreateInfo: &VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::validation_cache_ext::ValidationCacheEXT<'ret>, VkResult> {
    let mut handle = VkValidationCacheEXT::NULL;
    let r = unsafe {
      (&self.table).vkCreateValidationCacheEXT.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::validation_cache_ext::ValidationCacheEXT {
        raw: handle,
        parent: self,
        table: &self.validation_cache_ext_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html)
  ///
  /// Provided by:
  /// - `VK_FUCHSIA_buffer_collection`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pCollection`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
  #[inline]
  pub fn vkCreateBufferCollectionFUCHSIA<'ret>(
    &'ret self,
    pCreateInfo: &VkBufferCollectionCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::buffer_collection_fuchsia::BufferCollectionFUCHSIA<'ret>, VkResult> {
    let mut handle = VkBufferCollectionFUCHSIA::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateBufferCollectionFUCHSIA
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::buffer_collection_fuchsia::BufferCollectionFUCHSIA {
        raw: handle,
        parent: self,
        table: &self.buffer_collection_fuchsia_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html)
  ///
  /// Provided by:
  /// - `VK_FUCHSIA_external_memory`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetZirconHandleInfo`
  /// - `pZirconHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  #[inline(always)]
  pub fn vkGetMemoryZirconHandleFUCHSIA(
    &self,
    pGetZirconHandleInfo: &VkMemoryGetZirconHandleInfoFUCHSIA,
    pZirconHandle: &mut zx_handle_t,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryZirconHandleFUCHSIA
        .unwrap_unchecked()(self.raw, pGetZirconHandleInfo, pZirconHandle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html)
  ///
  /// Provided by:
  /// - `VK_FUCHSIA_external_memory`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `handleType`
  /// - `zirconHandle`
  /// - `pMemoryZirconHandleProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_FUCHSIA_external_memory")]
  #[inline(always)]
  pub fn vkGetMemoryZirconHandlePropertiesFUCHSIA(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: zx_handle_t,
    pMemoryZirconHandleProperties: &mut VkMemoryZirconHandlePropertiesFUCHSIA,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryZirconHandlePropertiesFUCHSIA
        .unwrap_unchecked()(
        self.raw,
        handleType,
        zirconHandle,
        pMemoryZirconHandleProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html)
  ///
  /// Provided by:
  /// - `VK_FUCHSIA_external_semaphore`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetZirconHandleInfo`
  /// - `pZirconHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  #[inline(always)]
  pub fn vkGetSemaphoreZirconHandleFUCHSIA(
    &self,
    pGetZirconHandleInfo: &VkSemaphoreGetZirconHandleInfoFUCHSIA,
    pZirconHandle: &mut zx_handle_t,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetSemaphoreZirconHandleFUCHSIA
        .unwrap_unchecked()(self.raw, pGetZirconHandleInfo, pZirconHandle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html)
  ///
  /// Provided by:
  /// - `VK_FUCHSIA_external_semaphore`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportSemaphoreZirconHandleInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_FUCHSIA_external_semaphore")]
  #[inline(always)]
  pub fn vkImportSemaphoreZirconHandleFUCHSIA(
    &self,
    pImportSemaphoreZirconHandleInfo: &VkImportSemaphoreZirconHandleInfoFUCHSIA,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkImportSemaphoreZirconHandleFUCHSIA
        .unwrap_unchecked()(self.raw, pImportSemaphoreZirconHandleInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pFramebuffer`
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline]
  pub fn vkCreateFramebuffer<'ret>(
    &'ret self,
    pCreateInfo: &VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::framebuffer::Framebuffer<'ret>, VkResult> {
    let mut handle = VkFramebuffer::NULL;
    let r = unsafe {
      (&self.table).vkCreateFramebuffer.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::framebuffer::Framebuffer {
        raw: handle,
        parent: self,
        table: &self.framebuffer_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** VulkanSC
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[inline]
  pub fn vkCreateGraphicsPipelines<'dev>(
    &'dev self,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<alloc::boxed::Box<[crate::pipeline::Pipeline<'dev>]>, VkResult> {
    let count = createInfoCount;
    let mut raw_pipelines = alloc::boxed::Box::<[VkPipeline]>::new_uninit_slice(count as usize);
    let fp = unsafe { self.table.vkCreateGraphicsPipelines.unwrap_unchecked() };
    let r = unsafe {
      fp(
        self.raw,
        pipelineCache,
        createInfoCount,
        pCreateInfos,
        pAllocator,
        raw_pipelines.as_mut_ptr().cast(),
      )
    };
    if r < VkResult::VK_SUCCESS {
      return Err(r);
    }
    let raw_pipelines = unsafe { raw_pipelines.assume_init() };
    Ok(
      raw_pipelines
        .into_iter()
        .map(|raw| crate::pipeline::Pipeline {
          raw,
          parent: self,
          table: &self.pipeline_table,
        })
        .collect(),
    )
  }
  /// [`vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_0`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pRenderPass`
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkCreateRenderPass2`")]
  #[inline]
  pub fn vkCreateRenderPass<'ret>(
    &'ret self,
    pCreateInfo: &VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::render_pass::RenderPass<'ret>, VkResult> {
    let mut handle = VkRenderPass::NULL;
    let r = unsafe {
      (&self.table).vkCreateRenderPass.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::render_pass::RenderPass {
        raw: handle,
        parent: self,
        table: &self.render_pass_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_2`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pRenderPass`
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
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  #[inline]
  pub fn vkCreateRenderPass2<'ret>(
    &'ret self,
    pCreateInfo: &VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::render_pass::RenderPass<'ret>, VkResult> {
    let mut handle = VkRenderPass::NULL;
    let r = unsafe {
      (&self.table).vkCreateRenderPass2.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::render_pass::RenderPass {
        raw: handle,
        parent: self,
        table: &self.render_pass_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html)
  ///
  /// Provided by:
  /// - `VK_GRAPHICS_VERSION_1_4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pRenderingAreaInfo`
  /// - `pGranularity`
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  #[inline(always)]
  pub fn vkGetRenderingAreaGranularity(
    &self,
    pRenderingAreaInfo: &VkRenderingAreaInfo,
    pGranularity: &mut VkExtent2D,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetRenderingAreaGranularity
        .unwrap_unchecked()(self.raw, pRenderingAreaInfo, pGranularity)
    }
  }
  /// [`vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html)
  ///
  /// Provided by:
  /// - `VK_INTEL_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pAcquireInfo`
  /// - `pConfiguration`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_INTEL_performance_query")]
  #[inline]
  pub fn vkAcquirePerformanceConfigurationINTEL<'ret>(
    &'ret self,
    pAcquireInfo: &VkPerformanceConfigurationAcquireInfoINTEL,
  ) -> Result<crate::performance_configuration_intel::PerformanceConfigurationINTEL<'ret>, VkResult>
  {
    let mut handle = VkPerformanceConfigurationINTEL::NULL;
    let r = unsafe {
      (&self.table)
        .vkAcquirePerformanceConfigurationINTEL
        .unwrap_unchecked()(self.raw, pAcquireInfo, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::performance_configuration_intel::PerformanceConfigurationINTEL {
          raw: handle,
          parent: self,
          table: &self.performance_configuration_intel_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html)
  ///
  /// Provided by:
  /// - `VK_INTEL_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `parameter`
  /// - `pValue`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_INTEL_performance_query")]
  #[inline(always)]
  pub fn vkGetPerformanceParameterINTEL(
    &self,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: &mut VkPerformanceValueINTEL,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetPerformanceParameterINTEL
        .unwrap_unchecked()(self.raw, parameter, pValue)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html)
  ///
  /// Provided by:
  /// - `VK_INTEL_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInitializeInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_INTEL_performance_query")]
  #[inline(always)]
  pub fn vkInitializePerformanceApiINTEL(
    &self,
    pInitializeInfo: &VkInitializePerformanceApiInfoINTEL,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkInitializePerformanceApiINTEL
        .unwrap_unchecked()(self.raw, pInitializeInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html)
  ///
  /// Provided by:
  /// - `VK_INTEL_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  #[cfg(feature = "VK_INTEL_performance_query")]
  #[inline(always)]
  pub fn vkUninitializePerformanceApiINTEL(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkUninitializePerformanceApiINTEL
        .unwrap_unchecked()(self.raw)
    }
  }
  /// [`vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pAccelerationStructure`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[deprecated(note = "superseded by `vkCreateAccelerationStructure2KHR`")]
  #[inline]
  pub fn vkCreateAccelerationStructureKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::acceleration_structure_khr::AccelerationStructureKHR<'ret>, VkResult> {
    let mut handle = VkAccelerationStructureKHR::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateAccelerationStructureKHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::acceleration_structure_khr::AccelerationStructureKHR {
          raw: handle,
          parent: self,
          table: &self.acceleration_structure_khr_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `buildType`
  /// - `pBuildInfo`
  /// - `pMaxPrimitiveCounts`: optional: true, len: pBuildInfo->geometryCount
  /// - `pSizeInfo`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkGetAccelerationStructureBuildSizesKHR(
    &self,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: &VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: &mut VkAccelerationStructureBuildSizesInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetAccelerationStructureBuildSizesKHR
        .unwrap_unchecked()(
        self.raw,
        buildType,
        pBuildInfo,
        pMaxPrimitiveCounts,
        pSizeInfo,
      )
    }
  }
  /// [`vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkGetAccelerationStructureDeviceAddressKHR(
    &self,
    pInfo: &VkAccelerationStructureDeviceAddressInfoKHR,
  ) -> VkDeviceAddress {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetAccelerationStructureDeviceAddressKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pVersionInfo`
  /// - `pCompatibility`
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkGetDeviceAccelerationStructureCompatibilityKHR(
    &self,
    pVersionInfo: &VkAccelerationStructureVersionInfoKHR,
    pCompatibility: &mut VkAccelerationStructureCompatibilityKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceAccelerationStructureCompatibilityKHR
        .unwrap_unchecked()(self.raw, pVersionInfo, pCompatibility)
    }
  }
  /// [`vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `accelerationStructureCount`
  /// - `pAccelerationStructures`: len: accelerationStructureCount
  /// - `queryType`
  /// - `dataSize`
  /// - `pData`: len: dataSize
  /// - `stride`
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
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  #[inline(always)]
  pub fn vkWriteAccelerationStructuresPropertiesKHR(
    &self,
    pAccelerationStructures: &[VkAccelerationStructureKHR],
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
    stride: usize,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkWriteAccelerationStructuresPropertiesKHR
        .unwrap_unchecked()(
        self.raw,
        pAccelerationStructures.len() as u32,
        pAccelerationStructures.as_ptr(),
        queryType,
        dataSize,
        pData,
        stride,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_bind_memory2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_bind_memory2")]
  #[inline(always)]
  pub fn vkBindBufferMemory2KHR(
    &self,
    pBindInfos: &[VkBindBufferMemoryInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkBindBufferMemory2KHR.unwrap_unchecked()(
        self.raw,
        pBindInfos.len() as u32,
        pBindInfos.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_bind_memory2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
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
  #[cfg(feature = "VK_KHR_bind_memory2")]
  #[inline(always)]
  pub fn vkBindImageMemory2KHR(
    &self,
    pBindInfos: &[VkBindImageMemoryInfo],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkBindImageMemory2KHR.unwrap_unchecked()(
        self.raw,
        pBindInfos.len() as u32,
        pBindInfos.as_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html)
  ///
  /// Provided by:
  /// - `VK_KHR_buffer_device_address`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  #[inline(always)]
  pub fn vkGetBufferDeviceAddressKHR(&self, pInfo: &VkBufferDeviceAddressInfo) -> VkDeviceAddress {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetBufferDeviceAddressKHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html)
  ///
  /// Provided by:
  /// - `VK_KHR_buffer_device_address`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  #[inline(always)]
  pub fn vkGetBufferOpaqueCaptureAddressKHR(&self, pInfo: &VkBufferDeviceAddressInfo) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetBufferOpaqueCaptureAddressKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html)
  ///
  /// Provided by:
  /// - `VK_KHR_buffer_device_address`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_KHR_buffer_device_address")]
  #[inline(always)]
  pub fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(
    &self,
    pInfo: &VkDeviceMemoryOpaqueCaptureAddressInfo,
  ) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceMemoryOpaqueCaptureAddressKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_calibrated_timestamps`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `timestampCount`
  /// - `pTimestampInfos`: len: timestampCount
  /// - `pTimestamps`: len: timestampCount
  /// - `pMaxDeviation`
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
  #[cfg(feature = "VK_KHR_calibrated_timestamps")]
  #[inline(always)]
  pub fn vkGetCalibratedTimestampsKHR(
    &self,
    pTimestampInfos: &[VkCalibratedTimestampInfoKHR],
    pTimestamps: &mut [u64],
    pMaxDeviation: &mut u64,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetCalibratedTimestampsKHR
        .unwrap_unchecked()(
        self.raw,
        pTimestamps.len() as u32,
        pTimestampInfos.as_ptr(),
        pTimestamps.as_mut_ptr(),
        pMaxDeviation,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_create_renderpass2`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pRenderPass`
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
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  #[inline]
  pub fn vkCreateRenderPass2KHR<'ret>(
    &'ret self,
    pCreateInfo: &VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::render_pass::RenderPass<'ret>, VkResult> {
    let mut handle = VkRenderPass::NULL;
    let r = unsafe {
      (&self.table).vkCreateRenderPass2KHR.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::render_pass::RenderPass {
        raw: handle,
        parent: self,
        table: &self.render_pass_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_deferred_host_operations`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pAllocator`: optional: true
  /// - `pDeferredOperation`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_deferred_host_operations")]
  #[inline]
  pub fn vkCreateDeferredOperationKHR<'ret>(
    &'ret self,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::deferred_operation_khr::DeferredOperationKHR<'ret>, VkResult> {
    let mut handle = VkDeferredOperationKHR::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateDeferredOperationKHR
        .unwrap_unchecked()(self.raw, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::deferred_operation_khr::DeferredOperationKHR {
        raw: handle,
        parent: self,
        table: &self.deferred_operation_khr_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html)
  ///
  /// Provided by:
  /// - `VK_KHR_descriptor_update_template`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pDescriptorUpdateTemplate`
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
  #[cfg(feature = "VK_KHR_descriptor_update_template")]
  #[inline]
  pub fn vkCreateDescriptorUpdateTemplateKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::descriptor_update_template::DescriptorUpdateTemplate<'ret>, VkResult> {
    let mut handle = VkDescriptorUpdateTemplate::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateDescriptorUpdateTemplateKHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::descriptor_update_template::DescriptorUpdateTemplate {
          raw: handle,
          parent: self,
          table: &self.descriptor_update_template_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_address_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pAccelerationStructure`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  #[cfg(feature = "VK_KHR_device_address_commands")]
  #[inline]
  pub fn vkCreateAccelerationStructure2KHR<'ret>(
    &'ret self,
    pCreateInfo: &VkAccelerationStructureCreateInfo2KHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::acceleration_structure_khr::AccelerationStructureKHR<'ret>, VkResult> {
    let mut handle = VkAccelerationStructureKHR::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateAccelerationStructure2KHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::acceleration_structure_khr::AccelerationStructureKHR {
          raw: handle,
          parent: self,
          table: &self.acceleration_structure_khr_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_fault`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pDebugInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_NOT_ENOUGH_SPACE_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_device_fault")]
  #[inline(always)]
  pub fn vkGetDeviceFaultDebugInfoKHR(
    &self,
    pDebugInfo: &mut VkDeviceFaultDebugInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDeviceFaultDebugInfoKHR
        .unwrap_unchecked()(self.raw, pDebugInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_fault`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `timeout`
  /// - `pFaultCounts`
  /// - `pFaultInfo`: optional: true, len: pFaultCounts
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///   - `VK_TIMEOUT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_device_fault")]
  #[inline(always)]
  pub fn vkGetDeviceFaultReportsKHR(
    &self,
    timeout: u64,
    pFaultCounts: &mut u32,
    pFaultInfo: *mut VkDeviceFaultInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetDeviceFaultReportsKHR.unwrap_unchecked()(
        self.raw,
        timeout,
        pFaultCounts,
        pFaultInfo,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
  /// - `VK_KHR_swapchain`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pAcquireInfo`
  /// - `pImageIndex`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_TIMEOUT`
  ///   - `VK_NOT_READY`
  ///   - `VK_SUBOPTIMAL_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_OUT_OF_DATE_KHR`
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
  #[inline(always)]
  pub fn vkAcquireNextImage2KHR(
    &self,
    pAcquireInfo: &VkAcquireNextImageInfoKHR,
    pImageIndex: &mut u32,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkAcquireNextImage2KHR.unwrap_unchecked()(self.raw, pAcquireInfo, pImageIndex)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `heapIndex`
  /// - `localDeviceIndex`
  /// - `remoteDeviceIndex`
  /// - `pPeerMemoryFeatures`
  #[cfg(feature = "VK_KHR_device_group")]
  #[inline(always)]
  pub fn vkGetDeviceGroupPeerMemoryFeaturesKHR(
    &self,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: &mut VkPeerMemoryFeatureFlags,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceGroupPeerMemoryFeaturesKHR
        .unwrap_unchecked()(
        self.raw,
        heapIndex,
        localDeviceIndex,
        remoteDeviceIndex,
        pPeerMemoryFeatures,
      )
    }
  }
  /// [`vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
  /// - `VK_KHR_swapchain`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pDeviceGroupPresentCapabilities`
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
  #[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
  #[inline(always)]
  pub fn vkGetDeviceGroupPresentCapabilitiesKHR(
    &self,
    pDeviceGroupPresentCapabilities: &mut VkDeviceGroupPresentCapabilitiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDeviceGroupPresentCapabilitiesKHR
        .unwrap_unchecked()(self.raw, pDeviceGroupPresentCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display_swapchain`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `swapchainCount`
  /// - `pCreateInfos`: len: swapchainCount
  /// - `pAllocator`: optional: true
  /// - `pSwapchains`: len: swapchainCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_display_swapchain")]
  #[inline(always)]
  pub fn vkCreateSharedSwapchainsKHR(
    &self,
    pCreateInfos: &[VkSwapchainCreateInfoKHR],
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: &mut [VkSwapchainKHR],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCreateSharedSwapchainsKHR.unwrap_unchecked()(
        self.raw,
        pSwapchains.len() as u32,
        pCreateInfos.as_ptr(),
        pAllocator,
        pSwapchains.as_mut_ptr(),
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_fence_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetFdInfo`
  /// - `pFd`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  #[inline(always)]
  pub fn vkGetFenceFdKHR(
    &self,
    pGetFdInfo: &VkFenceGetFdInfoKHR,
    pFd: &mut core::ffi::c_int,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkGetFenceFdKHR.unwrap_unchecked()(self.raw, pGetFdInfo, pFd) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_fence_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportFenceFdInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_fence_fd")]
  #[inline(always)]
  pub fn vkImportFenceFdKHR(
    &self,
    pImportFenceFdInfo: &VkImportFenceFdInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkImportFenceFdKHR.unwrap_unchecked()(self.raw, pImportFenceFdInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_fence_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetWin32HandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  #[inline(always)]
  pub fn vkGetFenceWin32HandleKHR(
    &self,
    pGetWin32HandleInfo: &VkFenceGetWin32HandleInfoKHR,
    pHandle: &mut HANDLE,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetFenceWin32HandleKHR.unwrap_unchecked()(
        self.raw,
        pGetWin32HandleInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_fence_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportFenceWin32HandleInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_fence_win32")]
  #[inline(always)]
  pub fn vkImportFenceWin32HandleKHR(
    &self,
    pImportFenceWin32HandleInfo: &VkImportFenceWin32HandleInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkImportFenceWin32HandleKHR.unwrap_unchecked()(
        self.raw,
        pImportFenceWin32HandleInfo,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_memory_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetFdInfo`
  /// - `pFd`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  #[inline(always)]
  pub fn vkGetMemoryFdKHR(
    &self,
    pGetFdInfo: &VkMemoryGetFdInfoKHR,
    pFd: &mut core::ffi::c_int,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkGetMemoryFdKHR.unwrap_unchecked()(self.raw, pGetFdInfo, pFd) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_memory_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `handleType`
  /// - `fd`
  /// - `pMemoryFdProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_memory_fd")]
  #[inline(always)]
  pub fn vkGetMemoryFdPropertiesKHR(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: core::ffi::c_int,
    pMemoryFdProperties: &mut VkMemoryFdPropertiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemoryFdPropertiesKHR.unwrap_unchecked()(
        self.raw,
        handleType,
        fd,
        pMemoryFdProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_memory_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetWin32HandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  #[inline(always)]
  pub fn vkGetMemoryWin32HandleKHR(
    &self,
    pGetWin32HandleInfo: &VkMemoryGetWin32HandleInfoKHR,
    pHandle: &mut HANDLE,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemoryWin32HandleKHR.unwrap_unchecked()(
        self.raw,
        pGetWin32HandleInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_memory_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `handleType`
  /// - `handle`
  /// - `pMemoryWin32HandleProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_memory_win32")]
  #[inline(always)]
  pub fn vkGetMemoryWin32HandlePropertiesKHR(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: &mut VkMemoryWin32HandlePropertiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetMemoryWin32HandlePropertiesKHR
        .unwrap_unchecked()(self.raw, handleType, handle, pMemoryWin32HandleProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_semaphore_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetFdInfo`
  /// - `pFd`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  #[inline(always)]
  pub fn vkGetSemaphoreFdKHR(
    &self,
    pGetFdInfo: &VkSemaphoreGetFdInfoKHR,
    pFd: &mut core::ffi::c_int,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkGetSemaphoreFdKHR.unwrap_unchecked()(self.raw, pGetFdInfo, pFd) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_semaphore_fd`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportSemaphoreFdInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_semaphore_fd")]
  #[inline(always)]
  pub fn vkImportSemaphoreFdKHR(
    &self,
    pImportSemaphoreFdInfo: &VkImportSemaphoreFdInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkImportSemaphoreFdKHR.unwrap_unchecked()(self.raw, pImportSemaphoreFdInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_semaphore_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetWin32HandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  #[inline(always)]
  pub fn vkGetSemaphoreWin32HandleKHR(
    &self,
    pGetWin32HandleInfo: &VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: &mut HANDLE,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetSemaphoreWin32HandleKHR
        .unwrap_unchecked()(self.raw, pGetWin32HandleInfo, pHandle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_semaphore_win32`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportSemaphoreWin32HandleInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_external_semaphore_win32")]
  #[inline(always)]
  pub fn vkImportSemaphoreWin32HandleKHR(
    &self,
    pImportSemaphoreWin32HandleInfo: &VkImportSemaphoreWin32HandleInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkImportSemaphoreWin32HandleKHR
        .unwrap_unchecked()(self.raw, pImportSemaphoreWin32HandleInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_memory_requirements2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  #[inline(always)]
  pub fn vkGetBufferMemoryRequirements2KHR(
    &self,
    pInfo: &VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetBufferMemoryRequirements2KHR
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_memory_requirements2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  #[inline(always)]
  pub fn vkGetImageMemoryRequirements2KHR(
    &self,
    pInfo: &VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetImageMemoryRequirements2KHR
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_memory_requirements2`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
  /// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
  #[cfg(feature = "VK_KHR_get_memory_requirements2")]
  #[inline(always)]
  pub fn vkGetImageSparseMemoryRequirements2KHR(
    &self,
    pInfo: &VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetImageSparseMemoryRequirements2KHR
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
      )
    }
  }
  /// [`vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance3`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pSupport`
  #[cfg(feature = "VK_KHR_maintenance3")]
  #[inline(always)]
  pub fn vkGetDescriptorSetLayoutSupportKHR(
    &self,
    pCreateInfo: &VkDescriptorSetLayoutCreateInfo,
    pSupport: &mut VkDescriptorSetLayoutSupport,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDescriptorSetLayoutSupportKHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pSupport)
    }
  }
  /// [`vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance4`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_KHR_maintenance4")]
  #[inline(always)]
  pub fn vkGetDeviceBufferMemoryRequirementsKHR(
    &self,
    pInfo: &VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceBufferMemoryRequirementsKHR
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance4`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_KHR_maintenance4")]
  #[inline(always)]
  pub fn vkGetDeviceImageMemoryRequirementsKHR(
    &self,
    pInfo: &VkDeviceImageMemoryRequirements,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageMemoryRequirementsKHR
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance4`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
  /// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
  #[cfg(feature = "VK_KHR_maintenance4")]
  #[inline(always)]
  pub fn vkGetDeviceImageSparseMemoryRequirementsKHR(
    &self,
    pInfo: &VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageSparseMemoryRequirementsKHR
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        pSparseMemoryRequirementCount,
        pSparseMemoryRequirements,
      )
    }
  }
  /// [`vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance5`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pLayout`
  #[cfg(feature = "VK_KHR_maintenance5")]
  #[inline(always)]
  pub fn vkGetDeviceImageSubresourceLayoutKHR(
    &self,
    pInfo: &VkDeviceImageSubresourceInfo,
    pLayout: &mut VkSubresourceLayout2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceImageSubresourceLayoutKHR
        .unwrap_unchecked()(self.raw, pInfo, pLayout)
    }
  }
  /// [`vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance5`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pRenderingAreaInfo`
  /// - `pGranularity`
  #[cfg(feature = "VK_KHR_maintenance5")]
  #[inline(always)]
  pub fn vkGetRenderingAreaGranularityKHR(
    &self,
    pRenderingAreaInfo: &VkRenderingAreaInfo,
    pGranularity: &mut VkExtent2D,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetRenderingAreaGranularityKHR
        .unwrap_unchecked()(self.raw, pRenderingAreaInfo, pGranularity)
    }
  }
  /// [`vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_map_memory2`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pMemoryMapInfo`
  /// - `ppData`: optional: pointer required, values optional if pointer not null
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_map_memory2")]
  #[inline(always)]
  pub fn vkMapMemory2KHR(
    &self,
    pMemoryMapInfo: &VkMemoryMapInfo,
    ppData: *mut *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkMapMemory2KHR.unwrap_unchecked()(self.raw, pMemoryMapInfo, ppData) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_map_memory2`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `device`
  /// - `pMemoryUnmapInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_MEMORY_MAP_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_map_memory2")]
  #[inline(always)]
  pub fn vkUnmapMemory2KHR(
    &self,
    pMemoryUnmapInfo: &VkMemoryUnmapInfo,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkUnmapMemory2KHR.unwrap_unchecked()(self.raw, pMemoryUnmapInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_TIMEOUT`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_performance_query")]
  #[inline(always)]
  pub fn vkAcquireProfilingLockKHR(
    &self,
    pInfo: &VkAcquireProfilingLockInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkAcquireProfilingLockKHR.unwrap_unchecked()(self.raw, pInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_performance_query`
  ///
  ///
  /// # Parameters
  /// - `device`
  #[cfg(feature = "VK_KHR_performance_query")]
  #[inline(always)]
  pub fn vkReleaseProfilingLockKHR(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkReleaseProfilingLockKHR.unwrap_unchecked()(self.raw)
    }
  }
  /// [`vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_binary`
  ///
  /// - **Allow No Queues:** True
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pBinaries`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///   - `VK_PIPELINE_BINARY_MISSING_KHR`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  #[inline(always)]
  pub fn vkCreatePipelineBinariesKHR(
    &self,
    pCreateInfo: &VkPipelineBinaryCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pBinaries: &mut VkPipelineBinaryHandlesInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkCreatePipelineBinariesKHR.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        pBinaries,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_binary`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pPipelineBinaryKey`
  /// - `pPipelineBinaryDataSize`: optional: pointer required, values optional if pointer not null
  /// - `pPipelineBinaryData`: optional: true, len: pPipelineBinaryDataSize
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_NOT_ENOUGH_SPACE_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  #[inline(always)]
  pub fn vkGetPipelineBinaryDataKHR(
    &self,
    pInfo: &VkPipelineBinaryDataInfoKHR,
    pPipelineBinaryKey: &mut VkPipelineBinaryKeyKHR,
    pPipelineBinaryDataSize: *mut usize,
    pPipelineBinaryData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetPipelineBinaryDataKHR.unwrap_unchecked()(
        self.raw,
        pInfo,
        pPipelineBinaryKey,
        pPipelineBinaryDataSize,
        pPipelineBinaryData,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_binary`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPipelineCreateInfo`: optional: true
  /// - `pPipelineKey`
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
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  #[inline(always)]
  pub fn vkGetPipelineKeyKHR(
    &self,
    pPipelineCreateInfo: *const VkPipelineCreateInfoKHR,
    pPipelineKey: &mut VkPipelineBinaryKeyKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetPipelineKeyKHR.unwrap_unchecked()(
        self.raw,
        pPipelineCreateInfo,
        pPipelineKey,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_binary`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pAllocator`: optional: true
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_pipeline_binary")]
  #[inline(always)]
  pub fn vkReleaseCapturedPipelineDataKHR(
    &self,
    pInfo: &VkReleaseCapturedPipelineDataInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkReleaseCapturedPipelineDataKHR
        .unwrap_unchecked()(self.raw, pInfo, pAllocator)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_executable_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pExecutableInfo`
  /// - `pInternalRepresentationCount`: optional: pointer required, values optional if pointer not null
  /// - `pInternalRepresentations`: optional: true, len: pInternalRepresentationCount
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
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  #[inline(always)]
  pub fn vkGetPipelineExecutableInternalRepresentationsKHR(
    &self,
    pExecutableInfo: &VkPipelineExecutableInfoKHR,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetPipelineExecutableInternalRepresentationsKHR
        .unwrap_unchecked()(
        self.raw,
        pExecutableInfo,
        pInternalRepresentationCount,
        pInternalRepresentations,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_executable_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pPipelineInfo`
  /// - `pExecutableCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pExecutableCount
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
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  #[inline(always)]
  pub fn vkGetPipelineExecutablePropertiesKHR(
    &self,
    pPipelineInfo: &VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetPipelineExecutablePropertiesKHR
        .unwrap_unchecked()(self.raw, pPipelineInfo, pExecutableCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_pipeline_executable_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pExecutableInfo`
  /// - `pStatisticCount`: optional: pointer required, values optional if pointer not null
  /// - `pStatistics`: optional: true, len: pStatisticCount
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
  #[cfg(feature = "VK_KHR_pipeline_executable_properties")]
  #[inline(always)]
  pub fn vkGetPipelineExecutableStatisticsKHR(
    &self,
    pExecutableInfo: &VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetPipelineExecutableStatisticsKHR
        .unwrap_unchecked()(self.raw, pExecutableInfo, pStatisticCount, pStatistics)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html)
  ///
  /// Provided by:
  /// - `VK_KHR_sampler_ycbcr_conversion`
  ///
  /// - **Allow No Queues:** True
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pYcbcrConversion`
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
  #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
  #[inline]
  pub fn vkCreateSamplerYcbcrConversionKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::sampler_ycbcr_conversion::SamplerYcbcrConversion<'ret>, VkResult> {
    let mut handle = VkSamplerYcbcrConversion::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateSamplerYcbcrConversionKHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::sampler_ycbcr_conversion::SamplerYcbcrConversion {
        raw: handle,
        parent: self,
        table: &self.sampler_ycbcr_conversion_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_swapchain`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSwapchain`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_COMPRESSION_EXHAUSTED_EXT`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_swapchain")]
  #[inline]
  pub fn vkCreateSwapchainKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::swapchain_khr::SwapchainKHR<'ret>, VkResult> {
    let mut handle = VkSwapchainKHR::NULL;
    let r = unsafe {
      (&self.table).vkCreateSwapchainKHR.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::swapchain_khr::SwapchainKHR {
        raw: handle,
        parent: self,
        table: &self.swapchain_khr_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_swapchain_maintenance1`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pReleaseInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_swapchain_maintenance1")]
  #[inline(always)]
  pub fn vkReleaseSwapchainImagesKHR(
    &self,
    pReleaseInfo: &VkReleaseSwapchainImagesInfoKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkReleaseSwapchainImagesKHR.unwrap_unchecked()(self.raw, pReleaseInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html)
  ///
  /// Provided by:
  /// - `VK_KHR_timeline_semaphore`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pSignalInfo`
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
  #[cfg(feature = "VK_KHR_timeline_semaphore")]
  #[inline(always)]
  pub fn vkSignalSemaphoreKHR(
    &self,
    pSignalInfo: &VkSemaphoreSignalInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (&self.table).vkSignalSemaphoreKHR.unwrap_unchecked()(self.raw, pSignalInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html)
  ///
  /// Provided by:
  /// - `VK_KHR_timeline_semaphore`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `device`
  /// - `pWaitInfo`
  /// - `timeout`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_TIMEOUT`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_timeline_semaphore")]
  #[inline(always)]
  pub fn vkWaitSemaphoresKHR(
    &self,
    pWaitInfo: &VkSemaphoreWaitInfo,
    timeout: u64,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (&self.table).vkWaitSemaphoresKHR.unwrap_unchecked()(self.raw, pWaitInfo, timeout) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_encode_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pVideoSessionParametersInfo`
  /// - `pFeedbackInfo`: optional: true
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
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  #[inline(always)]
  pub fn vkGetEncodedVideoSessionParametersKHR(
    &self,
    pVideoSessionParametersInfo: &VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetEncodedVideoSessionParametersKHR
        .unwrap_unchecked()(
        self.raw,
        pVideoSessionParametersInfo,
        pFeedbackInfo,
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
  /// [`vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pVideoSession`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline]
  pub fn vkCreateVideoSessionKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::video_session_khr::VideoSessionKHR<'ret>, VkResult> {
    let mut handle = VkVideoSessionKHR::NULL;
    let r = unsafe {
      (&self.table).vkCreateVideoSessionKHR.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::video_session_khr::VideoSessionKHR {
        raw: handle,
        parent: self,
        table: &self.video_session_khr_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pVideoSessionParameters`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline]
  pub fn vkCreateVideoSessionParametersKHR<'ret>(
    &'ret self,
    pCreateInfo: &VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::video_session_parameters_khr::VideoSessionParametersKHR<'ret>, VkResult> {
    let mut handle = VkVideoSessionParametersKHR::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateVideoSessionParametersKHR
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::video_session_parameters_khr::VideoSessionParametersKHR {
          raw: handle,
          parent: self,
          table: &self.video_session_parameters_khr_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html)
  ///
  /// Provided by:
  /// - `VK_NVX_binary_import`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pFunction`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NVX_binary_import")]
  #[inline]
  pub fn vkCreateCuFunctionNVX<'ret>(
    &'ret self,
    pCreateInfo: &VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::cu_function_nvx::CuFunctionNVX<'ret>, VkResult> {
    let mut handle = VkCuFunctionNVX::NULL;
    let r = unsafe {
      (&self.table).vkCreateCuFunctionNVX.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::cu_function_nvx::CuFunctionNVX {
        raw: handle,
        parent: self,
        table: &self.cu_function_nvx_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html)
  ///
  /// Provided by:
  /// - `VK_NVX_binary_import`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pModule`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NVX_binary_import")]
  #[inline]
  pub fn vkCreateCuModuleNVX<'ret>(
    &'ret self,
    pCreateInfo: &VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::cu_module_nvx::CuModuleNVX<'ret>, VkResult> {
    let mut handle = VkCuModuleNVX::NULL;
    let r = unsafe {
      (&self.table).vkCreateCuModuleNVX.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::cu_module_nvx::CuModuleNVX {
        raw: handle,
        parent: self,
        table: &self.cu_module_nvx_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html)
  ///
  /// Provided by:
  /// - `VK_NVX_image_view_handle`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `imageViewIndex`
  /// - `samplerIndex`
  #[cfg(feature = "VK_NVX_image_view_handle")]
  #[inline(always)]
  pub fn vkGetDeviceCombinedImageSamplerIndexNVX(
    &self,
    imageViewIndex: u64,
    samplerIndex: u64,
  ) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDeviceCombinedImageSamplerIndexNVX
        .unwrap_unchecked()(self.raw, imageViewIndex, samplerIndex)
    }
  }
  /// [`vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html)
  ///
  /// Provided by:
  /// - `VK_NVX_image_view_handle`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_NVX_image_view_handle")]
  #[inline(always)]
  pub fn vkGetImageViewHandle64NVX(&self, pInfo: &VkImageViewHandleInfoNVX) -> u64 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetImageViewHandle64NVX.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html)
  ///
  /// Provided by:
  /// - `VK_NVX_image_view_handle`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_NVX_image_view_handle")]
  #[inline(always)]
  pub fn vkGetImageViewHandleNVX(&self, pInfo: &VkImageViewHandleInfoNVX) -> u32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table).vkGetImageViewHandleNVX.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cluster_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSizeInfo`
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  #[inline(always)]
  pub fn vkGetClusterAccelerationStructureBuildSizesNV(
    &self,
    pInfo: &VkClusterAccelerationStructureInputInfoNV,
    pSizeInfo: &mut VkAccelerationStructureBuildSizesInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetClusterAccelerationStructureBuildSizesNV
        .unwrap_unchecked()(self.raw, pInfo, pSizeInfo)
    }
  }
  /// [`vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cooperative_vector`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_cooperative_vector")]
  #[inline(always)]
  pub fn vkConvertCooperativeVectorMatrixNV(
    &self,
    pInfo: &VkConvertCooperativeVectorMatrixInfoNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkConvertCooperativeVectorMatrixNV
        .unwrap_unchecked()(self.raw, pInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cuda_kernel_launch`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pFunction`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  #[inline]
  pub fn vkCreateCudaFunctionNV<'ret>(
    &'ret self,
    pCreateInfo: &VkCudaFunctionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::cuda_function_nv::CudaFunctionNV<'ret>, VkResult> {
    let mut handle = VkCudaFunctionNV::NULL;
    let r = unsafe {
      (&self.table).vkCreateCudaFunctionNV.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::cuda_function_nv::CudaFunctionNV {
        raw: handle,
        parent: self,
        table: &self.cuda_function_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cuda_kernel_launch`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pModule`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  #[inline]
  pub fn vkCreateCudaModuleNV<'ret>(
    &'ret self,
    pCreateInfo: &VkCudaModuleCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::cuda_module_nv::CudaModuleNV<'ret>, VkResult> {
    let mut handle = VkCudaModuleNV::NULL;
    let r = unsafe {
      (&self.table).vkCreateCudaModuleNV.unwrap_unchecked()(
        self.raw,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::cuda_module_nv::CudaModuleNV {
        raw: handle,
        parent: self,
        table: &self.cuda_module_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pIndirectCommandsLayout`
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
  #[cfg(feature = "VK_NV_device_generated_commands")]
  #[inline]
  pub fn vkCreateIndirectCommandsLayoutNV<'ret>(
    &'ret self,
    pCreateInfo: &VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::indirect_commands_layout_nv::IndirectCommandsLayoutNV<'ret>, VkResult> {
    let mut handle = VkIndirectCommandsLayoutNV::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateIndirectCommandsLayoutNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(
        crate::indirect_commands_layout_nv::IndirectCommandsLayoutNV {
          raw: handle,
          parent: self,
          table: &self.indirect_commands_layout_nv_table,
        },
      )
    } else {
      Err(r)
    }
  }
  /// [`vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_device_generated_commands`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_NV_device_generated_commands")]
  #[inline(always)]
  pub fn vkGetGeneratedCommandsMemoryRequirementsNV(
    &self,
    pInfo: &VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetGeneratedCommandsMemoryRequirementsNV
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_device_generated_commands_compute`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  #[inline(always)]
  pub fn vkGetPipelineIndirectDeviceAddressNV(
    &self,
    pInfo: &VkPipelineIndirectDeviceAddressInfoNV,
  ) -> VkDeviceAddress {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetPipelineIndirectDeviceAddressNV
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_device_generated_commands_compute`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  #[inline(always)]
  pub fn vkGetPipelineIndirectMemoryRequirementsNV(
    &self,
    pCreateInfo: &VkComputePipelineCreateInfo,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetPipelineIndirectMemoryRequirementsNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pMemoryRequirements)
    }
  }
  /// [`vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_compute_queue`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pExternalQueue`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_compute_queue")]
  #[inline]
  pub fn vkCreateExternalComputeQueueNV<'ret>(
    &'ret self,
    pCreateInfo: &VkExternalComputeQueueCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::external_compute_queue_nv::ExternalComputeQueueNV<'ret>, VkResult> {
    let mut handle = VkExternalComputeQueueNV::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateExternalComputeQueueNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::external_compute_queue_nv::ExternalComputeQueueNV {
        raw: handle,
        parent: self,
        table: &self.external_compute_queue_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_memory_rdma`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pMemoryGetRemoteAddressInfo`
  /// - `pAddress`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_memory_rdma")]
  #[inline(always)]
  pub fn vkGetMemoryRemoteAddressNV(
    &self,
    pMemoryGetRemoteAddressInfo: &VkMemoryGetRemoteAddressInfoNV,
    pAddress: &mut VkRemoteAddressNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemoryRemoteAddressNV.unwrap_unchecked()(
        self.raw,
        pMemoryGetRemoteAddressInfo,
        pAddress,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetMemorySciBufNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemorySciBufNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_memory_sci_buf`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetSciBufInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  #[inline(always)]
  pub fn vkGetMemorySciBufNV(
    &self,
    pGetSciBufInfo: &VkMemoryGetSciBufInfoNV,
    pHandle: &mut NvSciBufObj,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemorySciBufNV.unwrap_unchecked()(self.raw, pGetSciBufInfo, pHandle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetFenceSciSyncFenceNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceSciSyncFenceNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetSciSyncHandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  #[inline(always)]
  pub fn vkGetFenceSciSyncFenceNV(
    &self,
    pGetSciSyncHandleInfo: &VkFenceGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetFenceSciSyncFenceNV.unwrap_unchecked()(
        self.raw,
        pGetSciSyncHandleInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetFenceSciSyncObjNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceSciSyncObjNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetSciSyncHandleInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  #[inline(always)]
  pub fn vkGetFenceSciSyncObjNV(
    &self,
    pGetSciSyncHandleInfo: &VkFenceGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetFenceSciSyncObjNV.unwrap_unchecked()(
        self.raw,
        pGetSciSyncHandleInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetSemaphoreSciSyncObjNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreSciSyncObjNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pGetSciSyncInfo`
  /// - `pHandle`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_sci_sync")]
  #[inline(always)]
  pub fn vkGetSemaphoreSciSyncObjNV(
    &self,
    pGetSciSyncInfo: &VkSemaphoreGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetSemaphoreSciSyncObjNV.unwrap_unchecked()(
        self.raw,
        pGetSciSyncInfo,
        pHandle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportFenceSciSyncFenceNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceSciSyncFenceNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportFenceSciSyncInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  #[inline(always)]
  pub fn vkImportFenceSciSyncFenceNV(
    &self,
    pImportFenceSciSyncInfo: &VkImportFenceSciSyncInfoNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkImportFenceSciSyncFenceNV.unwrap_unchecked()(
        self.raw,
        pImportFenceSciSyncInfo,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportFenceSciSyncObjNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceSciSyncObjNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportFenceSciSyncInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  #[inline(always)]
  pub fn vkImportFenceSciSyncObjNV(
    &self,
    pImportFenceSciSyncInfo: &VkImportFenceSciSyncInfoNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkImportFenceSciSyncObjNV.unwrap_unchecked()(self.raw, pImportFenceSciSyncInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkImportSemaphoreSciSyncObjNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreSciSyncObjNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pImportSemaphoreSciSyncInfo`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_NOT_PERMITTED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_sci_sync")]
  #[inline(always)]
  pub fn vkImportSemaphoreSciSyncObjNV(
    &self,
    pImportSemaphoreSciSyncInfo: &VkImportSemaphoreSciSyncInfoNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkImportSemaphoreSciSyncObjNV
        .unwrap_unchecked()(self.raw, pImportSemaphoreSciSyncInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateSemaphoreSciSyncPoolNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphoreSciSyncPoolNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSemaphorePool`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_sci_sync2")]
  #[inline]
  pub fn vkCreateSemaphoreSciSyncPoolNV<'ret>(
    &'ret self,
    pCreateInfo: &VkSemaphoreSciSyncPoolCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::semaphore_sci_sync_pool_nv::SemaphoreSciSyncPoolNV<'ret>, VkResult> {
    let mut handle = VkSemaphoreSciSyncPoolNV::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateSemaphoreSciSyncPoolNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::semaphore_sci_sync_pool_nv::SemaphoreSciSyncPoolNV {
        raw: handle,
        parent: self,
        table: &self.semaphore_sci_sync_pool_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pSession`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_optical_flow")]
  #[inline]
  pub fn vkCreateOpticalFlowSessionNV<'ret>(
    &'ret self,
    pCreateInfo: &VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::optical_flow_session_nv::OpticalFlowSessionNV<'ret>, VkResult> {
    let mut handle = VkOpticalFlowSessionNV::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateOpticalFlowSessionNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::optical_flow_session_nv::OpticalFlowSessionNV {
        raw: handle,
        parent: self,
        table: &self.optical_flow_session_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_partitioned_acceleration_structure`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pSizeInfo`
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  #[inline(always)]
  pub fn vkGetPartitionedAccelerationStructuresBuildSizesNV(
    &self,
    pInfo: &VkPartitionedAccelerationStructureInstancesInputNV,
    pSizeInfo: &mut VkAccelerationStructureBuildSizesInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetPartitionedAccelerationStructuresBuildSizesNV
        .unwrap_unchecked()(self.raw, pInfo, pSizeInfo)
    }
  }
  /// [`vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_ray_tracing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `bindInfoCount`
  /// - `pBindInfos`: len: bindInfoCount
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
  #[cfg(feature = "VK_NV_ray_tracing")]
  #[inline(always)]
  pub fn vkBindAccelerationStructureMemoryNV(
    &self,
    pBindInfos: &[VkBindAccelerationStructureMemoryInfoNV],
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkBindAccelerationStructureMemoryNV
        .unwrap_unchecked()(self.raw, pBindInfos.len() as u32, pBindInfos.as_ptr())
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_ray_tracing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pAccelerationStructure`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_ray_tracing")]
  #[inline]
  pub fn vkCreateAccelerationStructureNV<'ret>(
    &'ret self,
    pCreateInfo: &VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::acceleration_structure_nv::AccelerationStructureNV<'ret>, VkResult> {
    let mut handle = VkAccelerationStructureNV::NULL;
    let r = unsafe {
      (&self.table)
        .vkCreateAccelerationStructureNV
        .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(crate::acceleration_structure_nv::AccelerationStructureNV {
        raw: handle,
        parent: self,
        table: &self.acceleration_structure_nv_table,
      })
    } else {
      Err(r)
    }
  }
  /// [`vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_ray_tracing`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pMemoryRequirements`
  #[cfg(feature = "VK_NV_ray_tracing")]
  #[inline(always)]
  pub fn vkGetAccelerationStructureMemoryRequirementsNV(
    &self,
    pInfo: &VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: &mut VkMemoryRequirements2,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetAccelerationStructureMemoryRequirementsNV
        .unwrap_unchecked()(self.raw, pInfo, pMemoryRequirements)
    }
  }
  /// [`vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html)
  ///
  /// Provided by:
  /// - `VK_OHOS_external_memory`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pInfo`
  /// - `pBuffer`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_OHOS_external_memory")]
  #[inline(always)]
  pub fn vkGetMemoryNativeBufferOHOS(
    &self,
    pInfo: &VkMemoryGetNativeBufferInfoOHOS,
    pBuffer: *mut *mut OH_NativeBuffer,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table).vkGetMemoryNativeBufferOHOS.unwrap_unchecked()(self.raw, pInfo, pBuffer)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html)
  ///
  /// Provided by:
  /// - `VK_OHOS_external_memory`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `buffer`
  /// - `pProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_OHOS_external_memory")]
  #[inline(always)]
  pub fn vkGetNativeBufferPropertiesOHOS(
    &self,
    buffer: &OH_NativeBuffer,
    pProperties: &mut VkNativeBufferPropertiesOHOS,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetNativeBufferPropertiesOHOS
        .unwrap_unchecked()(self.raw, buffer, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html)
  ///
  /// Provided by:
  /// - `VK_QCOM_tile_properties`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pRenderingInfo`
  /// - `pProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_QCOM_tile_properties")]
  #[inline(always)]
  pub fn vkGetDynamicRenderingTilePropertiesQCOM(
    &self,
    pRenderingInfo: &VkRenderingInfo,
    pProperties: &mut VkTilePropertiesQCOM,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetDynamicRenderingTilePropertiesQCOM
        .unwrap_unchecked()(self.raw, pRenderingInfo, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html)
  ///
  /// Provided by:
  /// - `VK_QNX_external_memory_screen_buffer`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `buffer`
  /// - `pProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
  #[inline(always)]
  pub fn vkGetScreenBufferPropertiesQNX(
    &self,
    buffer: &_screen_buffer,
    pProperties: &mut VkScreenBufferPropertiesQNX,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (&self.table)
        .vkGetScreenBufferPropertiesQNX
        .unwrap_unchecked()(self.raw, buffer, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
  /// [`vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html)
  ///
  /// Provided by:
  /// - `VK_VALVE_descriptor_set_host_mapping`
  ///
  ///
  /// # Parameters
  /// - `device`
  /// - `pBindingReference`
  /// - `pHostMapping`
  #[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
  #[inline(always)]
  pub fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(
    &self,
    pBindingReference: &VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: &mut VkDescriptorSetLayoutHostMappingInfoVALVE,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (&self.table)
        .vkGetDescriptorSetLayoutHostMappingInfoVALVE
        .unwrap_unchecked()(self.raw, pBindingReference, pHostMapping)
    }
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'inst> Drop for Device<'inst> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe { self.table.vkDestroyDevice.unwrap_unchecked()(self.raw, core::ptr::null()) };
  }
}
