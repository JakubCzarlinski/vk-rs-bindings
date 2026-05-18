//! PhysicalDevice-tier dispatch table and safe wrapper.
#![allow(
  non_snake_case,
  unused_imports,
  clippy::too_many_arguments,
  clippy::missing_safety_doc
)]
#[cfg(feature = "VK_EXT_acquire_drm_display")]
use crate::commands::PFN_vkAcquireDrmDisplayEXT;
#[cfg(feature = "VK_NV_acquire_winrt_display")]
use crate::commands::PFN_vkAcquireWinrtDisplayNV;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
use crate::commands::PFN_vkAcquireXlibDisplayEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkCreateDevice;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkCreateDisplayModeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkEnumerateDeviceExtensionProperties;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkEnumerateDeviceLayerProperties;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::commands::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::commands::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::commands::PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::commands::PFN_vkGetDisplayModeProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkGetDisplayModePropertiesKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::commands::PFN_vkGetDisplayPlaneCapabilities2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkGetDisplayPlaneCapabilitiesKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkGetDisplayPlaneSupportedDisplaysKHR;
#[cfg(feature = "VK_EXT_acquire_drm_display")]
use crate::commands::PFN_vkGetDrmDisplayEXT;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::commands::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT;
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
use crate::commands::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::commands::PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::commands::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::commands::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::commands::PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::commands::PFN_vkGetPhysicalDeviceDescriptorSizeEXT;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceDisplayProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::commands::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalBufferProperties;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalFenceProperties;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::commands::PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceFeatures2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceFeatures2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceFormatProperties2KHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::commands::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceMemoryProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceMemoryProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceMemoryProperties2KHR;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::commands::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::commands::PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
use crate::commands::PFN_vkGetPhysicalDevicePresentRectanglesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceProperties2KHR;
#[cfg(any(
  feature = "VK_ARM_data_graph_instruction_set_tosa",
  feature = "VK_ARM_data_graph_optical_flow"
))]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR;
#[cfg(feature = "VK_KHR_object_refresh")]
use crate::commands::PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::commands::PFN_vkGetPhysicalDeviceSciBufAttributesNV;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::commands::PFN_vkGetPhysicalDeviceSciSyncAttributesNV;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::commands::PFN_vkGetPhysicalDeviceSparseImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::commands::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::commands::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::commands::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT;
#[cfg(feature = "VK_KHR_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceSurfaceSupportKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::commands::PFN_vkGetPhysicalDeviceToolProperties;
#[cfg(feature = "VK_EXT_tooling_info")]
use crate::commands::PFN_vkGetPhysicalDeviceToolPropertiesEXT;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::commands::PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::commands::PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::commands::PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR;
#[cfg(feature = "VK_KHR_win32_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::commands::PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
use crate::commands::PFN_vkGetRandROutputDisplayEXT;
#[cfg(feature = "VK_NV_acquire_winrt_display")]
use crate::commands::PFN_vkGetWinrtDisplayNV;
#[cfg(feature = "VK_EXT_direct_mode_display")]
use crate::commands::PFN_vkReleaseDisplayEXT;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_inline_uniform_block",
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_QCOM_image_processing",
  feature = "VK_EXT_mutable_descriptor_type"
))]
use crate::enums::VkDescriptorType;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_memory_capabilities",
  feature = "VK_EXT_external_memory_dma_buf",
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  feature = "VK_EXT_external_memory_host",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_NV_external_memory_rdma",
  feature = "VK_OHOS_external_memory",
  feature = "VK_QNX_external_memory_screen_buffer",
  feature = "VK_EXT_external_memory_metal"
))]
use crate::enums::VkExternalMemoryHandleTypeFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageTiling;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkObjectType;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkPresentModeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::enums::VkTimeDomainEXT;
#[cfg(any(
  feature = "VK_KHR_calibrated_timestamps",
  feature = "VK_EXT_calibrated_timestamps"
))]
use crate::enums::VkTimeDomainKHR;
use crate::instance::Instance;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::types::_screen_window;
#[cfg(any(
  feature = "VK_KHR_xlib_surface",
  feature = "VK_EXT_acquire_xlib_display"
))]
use crate::types::Display;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::types::IDirectFB;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::NvSciBufAttrList;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::NvSciBufObj;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::NvSciSyncAttrList;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
use crate::types::RROutput;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::types::VisualID;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAllocationCallbacks;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBaseOutStructure;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkCooperativeMatrixFlexibleDimensionsPropertiesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkCooperativeMatrixPropertiesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkCooperativeVectorPropertiesNV;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatPropertiesARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeCreateInfoKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayModeProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModePropertiesKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayPlaneCapabilities2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayPlaneCapabilitiesKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayPlaneInfo2KHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayPlaneProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayPlanePropertiesKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtensionProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalBufferProperties;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalBufferPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalFenceProperties;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkExternalFencePropertiesKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::types::VkExternalImageFormatPropertiesNV;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::types::VkExternalMemoryHandleTypeFlagsNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalSemaphoreProperties;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkExternalSemaphorePropertiesKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkExternalTensorPropertiesARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkFormatProperties2KHR;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkFramebufferMixedSamplesCombinationNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkImageFormatProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkLayerProperties;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkMemorySciBufPropertiesNV;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkMultisamplePropertiesEXT;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatInfoNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatPropertiesNV;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPerformanceCounterARM;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPerformanceCounterDescriptionARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterDescriptionKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDevice;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalBufferInfo;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkPhysicalDeviceExternalBufferInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalFenceInfo;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkPhysicalDeviceExternalFenceInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalSemaphoreInfo;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkPhysicalDeviceExternalSemaphoreInfoKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceExternalTensorInfoARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceFeatures2KHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceImageFormatInfo2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceImageFormatInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceMemoryProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMemoryProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceMemoryProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceProperties2KHR;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSparseImageFormatInfo2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceSparseImageFormatInfo2KHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkPhysicalDeviceSurfaceInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceToolProperties;
#[cfg(feature = "VK_EXT_tooling_info")]
use crate::types::VkPhysicalDeviceToolPropertiesEXT;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkPhysicalDeviceVideoFormatInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkQueryPoolPerformanceCreateInfoKHR;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkQueueFamilyDataGraphProcessingEnginePropertiesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkQueueFamilyDataGraphPropertiesARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFamilyProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkQueueFamilyProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkQueueFamilyProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkSciSyncAttributesInfoNV;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationMetricDescriptionARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSparseImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkSparseImageFormatProperties2KHR;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::types::VkSurfaceCapabilities2EXT;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceCapabilities2KHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceCapabilitiesKHR;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceFormat2KHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceFormatKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeQualityLevelPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileInfoKHR;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::types::ubm_device;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::types::wl_display;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::xcb_connection_t;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::xcb_visualid_t;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct PhysicalDeviceDispatchTable {
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM>,
  #[cfg(any(
    feature = "VK_ARM_data_graph_instruction_set_tosa",
    feature = "VK_ARM_data_graph_optical_flow"
  ))]
  pub vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM>,
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  pub vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM>,
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  pub vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM:
    Option<PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM>,
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  pub vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM:
    Option<PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkGetPhysicalDeviceExternalTensorPropertiesARM:
    Option<PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCreateDevice: Option<PFN_vkCreateDevice>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkEnumerateDeviceExtensionProperties: Option<PFN_vkEnumerateDeviceExtensionProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkEnumerateDeviceLayerProperties: Option<PFN_vkEnumerateDeviceLayerProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceFeatures: Option<PFN_vkGetPhysicalDeviceFeatures>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceFormatProperties: Option<PFN_vkGetPhysicalDeviceFormatProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceImageFormatProperties:
    Option<PFN_vkGetPhysicalDeviceImageFormatProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceMemoryProperties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceProperties: Option<PFN_vkGetPhysicalDeviceProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceQueueFamilyProperties:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkGetPhysicalDeviceSparseImageFormatProperties:
    Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceExternalBufferProperties:
    Option<PFN_vkGetPhysicalDeviceExternalBufferProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceExternalFenceProperties:
    Option<PFN_vkGetPhysicalDeviceExternalFenceProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceExternalSemaphoreProperties:
    Option<PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceFeatures2: Option<PFN_vkGetPhysicalDeviceFeatures2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceFormatProperties2: Option<PFN_vkGetPhysicalDeviceFormatProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceImageFormatProperties2:
    Option<PFN_vkGetPhysicalDeviceImageFormatProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceMemoryProperties2: Option<PFN_vkGetPhysicalDeviceMemoryProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceProperties2: Option<PFN_vkGetPhysicalDeviceProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceQueueFamilyProperties2:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkGetPhysicalDeviceSparseImageFormatProperties2:
    Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkGetPhysicalDeviceToolProperties: Option<PFN_vkGetPhysicalDeviceToolProperties>,
  #[cfg(feature = "VK_EXT_acquire_drm_display")]
  pub vkAcquireDrmDisplayEXT: Option<PFN_vkAcquireDrmDisplayEXT>,
  #[cfg(feature = "VK_EXT_acquire_drm_display")]
  pub vkGetDrmDisplayEXT: Option<PFN_vkGetDrmDisplayEXT>,
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  pub vkAcquireXlibDisplayEXT: Option<PFN_vkAcquireXlibDisplayEXT>,
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  pub vkGetRandROutputDisplayEXT: Option<PFN_vkGetRandROutputDisplayEXT>,
  #[cfg(feature = "VK_EXT_calibrated_timestamps")]
  pub vkGetPhysicalDeviceCalibrateableTimeDomainsEXT:
    Option<PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkGetPhysicalDeviceDescriptorSizeEXT: Option<PFN_vkGetPhysicalDeviceDescriptorSizeEXT>,
  #[cfg(feature = "VK_EXT_direct_mode_display")]
  pub vkReleaseDisplayEXT: Option<PFN_vkReleaseDisplayEXT>,
  #[cfg(feature = "VK_EXT_directfb_surface")]
  pub vkGetPhysicalDeviceDirectFBPresentationSupportEXT:
    Option<PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT>,
  #[cfg(feature = "VK_EXT_display_surface_counter")]
  pub vkGetPhysicalDeviceSurfaceCapabilities2EXT:
    Option<PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>,
  #[cfg(feature = "VK_EXT_full_screen_exclusive")]
  pub vkGetPhysicalDeviceSurfacePresentModes2EXT:
    Option<PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT>,
  #[cfg(feature = "VK_EXT_sample_locations")]
  pub vkGetPhysicalDeviceMultisamplePropertiesEXT:
    Option<PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>,
  #[cfg(feature = "VK_EXT_tooling_info")]
  pub vkGetPhysicalDeviceToolPropertiesEXT: Option<PFN_vkGetPhysicalDeviceToolPropertiesEXT>,
  #[cfg(feature = "VK_KHR_calibrated_timestamps")]
  pub vkGetPhysicalDeviceCalibrateableTimeDomainsKHR:
    Option<PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR>,
  #[cfg(feature = "VK_KHR_cooperative_matrix")]
  pub vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR>,
  #[cfg(any(
    all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
    all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
  ))]
  pub vkGetPhysicalDevicePresentRectanglesKHR: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkCreateDisplayModeKHR: Option<PFN_vkCreateDisplayModeKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayModePropertiesKHR: Option<PFN_vkGetDisplayModePropertiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayPlaneCapabilitiesKHR: Option<PFN_vkGetDisplayPlaneCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetDisplayPlaneSupportedDisplaysKHR: Option<PFN_vkGetDisplayPlaneSupportedDisplaysKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetPhysicalDeviceDisplayPlanePropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
  #[cfg(feature = "VK_KHR_display")]
  pub vkGetPhysicalDeviceDisplayPropertiesKHR: Option<PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>,
  #[cfg(feature = "VK_KHR_external_fence_capabilities")]
  pub vkGetPhysicalDeviceExternalFencePropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>,
  #[cfg(feature = "VK_KHR_external_memory_capabilities")]
  pub vkGetPhysicalDeviceExternalBufferPropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
  #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
  pub vkGetPhysicalDeviceExternalSemaphorePropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  pub vkGetPhysicalDeviceFragmentShadingRatesKHR:
    Option<PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>,
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  pub vkGetDisplayModeProperties2KHR: Option<PFN_vkGetDisplayModeProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  pub vkGetDisplayPlaneCapabilities2KHR: Option<PFN_vkGetDisplayPlaneCapabilities2KHR>,
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  pub vkGetPhysicalDeviceDisplayPlaneProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  pub vkGetPhysicalDeviceDisplayProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceDisplayProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceFeatures2KHR: Option<PFN_vkGetPhysicalDeviceFeatures2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceFormatProperties2KHR: Option<PFN_vkGetPhysicalDeviceFormatProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceImageFormatProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceMemoryProperties2KHR: Option<PFN_vkGetPhysicalDeviceMemoryProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceProperties2KHR: Option<PFN_vkGetPhysicalDeviceProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceQueueFamilyProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  pub vkGetPhysicalDeviceSparseImageFormatProperties2KHR:
    Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  pub vkGetPhysicalDeviceSurfaceCapabilities2KHR:
    Option<PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  pub vkGetPhysicalDeviceSurfaceFormats2KHR: Option<PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>,
  #[cfg(feature = "VK_KHR_object_refresh")]
  pub vkGetPhysicalDeviceRefreshableObjectTypesKHR:
    Option<PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR>,
  #[cfg(feature = "VK_KHR_performance_query")]
  pub vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR:
    Option<PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
  #[cfg(feature = "VK_KHR_performance_query")]
  pub vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR:
    Option<PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceCapabilitiesKHR:
    Option<PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceFormatsKHR: Option<PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfacePresentModesKHR:
    Option<PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>,
  #[cfg(feature = "VK_KHR_surface")]
  pub vkGetPhysicalDeviceSurfaceSupportKHR: Option<PFN_vkGetPhysicalDeviceSurfaceSupportKHR>,
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  pub vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkGetPhysicalDeviceVideoCapabilitiesKHR: Option<PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkGetPhysicalDeviceVideoFormatPropertiesKHR:
    Option<PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR>,
  #[cfg(feature = "VK_KHR_wayland_surface")]
  pub vkGetPhysicalDeviceWaylandPresentationSupportKHR:
    Option<PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
  #[cfg(feature = "VK_KHR_win32_surface")]
  pub vkGetPhysicalDeviceWin32PresentationSupportKHR:
    Option<PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR>,
  #[cfg(feature = "VK_KHR_xcb_surface")]
  pub vkGetPhysicalDeviceXcbPresentationSupportKHR:
    Option<PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>,
  #[cfg(feature = "VK_KHR_xlib_surface")]
  pub vkGetPhysicalDeviceXlibPresentationSupportKHR:
    Option<PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR>,
  #[cfg(feature = "VK_NV_acquire_winrt_display")]
  pub vkAcquireWinrtDisplayNV: Option<PFN_vkAcquireWinrtDisplayNV>,
  #[cfg(feature = "VK_NV_acquire_winrt_display")]
  pub vkGetWinrtDisplayNV: Option<PFN_vkGetWinrtDisplayNV>,
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  pub vkGetPhysicalDeviceCooperativeMatrixPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  pub vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV>,
  #[cfg(feature = "VK_NV_cooperative_vector")]
  pub vkGetPhysicalDeviceCooperativeVectorPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV>,
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  pub vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV:
    Option<PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
  #[cfg(feature = "VK_NV_external_memory_capabilities")]
  pub vkGetPhysicalDeviceExternalImageFormatPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  pub vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV:
    Option<PFN_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV>,
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  pub vkGetPhysicalDeviceSciBufAttributesNV: Option<PFN_vkGetPhysicalDeviceSciBufAttributesNV>,
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  pub vkGetPhysicalDeviceSciSyncAttributesNV: Option<PFN_vkGetPhysicalDeviceSciSyncAttributesNV>,
  #[cfg(feature = "VK_NV_optical_flow")]
  pub vkGetPhysicalDeviceOpticalFlowImageFormatsNV:
    Option<PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV>,
  #[cfg(feature = "VK_QNX_screen_surface")]
  pub vkGetPhysicalDeviceScreenPresentationSupportQNX:
    Option<PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX>,
  #[cfg(feature = "VK_SEC_ubm_surface")]
  pub vkGetPhysicalDeviceUbmPresentationSupportSEC:
    Option<PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl PhysicalDeviceDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM: None,
    #[cfg(any(
      feature = "VK_ARM_data_graph_instruction_set_tosa",
      feature = "VK_ARM_data_graph_optical_flow"
    ))]
    vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM: None,
    #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
    vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM: None,
    #[cfg(feature = "VK_ARM_performance_counters_by_region")]
    vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM: None,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkGetPhysicalDeviceExternalTensorPropertiesARM: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCreateDevice: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkEnumerateDeviceExtensionProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkEnumerateDeviceLayerProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceFeatures: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceFormatProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceImageFormatProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceMemoryProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceQueueFamilyProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkGetPhysicalDeviceSparseImageFormatProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceExternalBufferProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceExternalFenceProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceExternalSemaphoreProperties: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceFeatures2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceFormatProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceImageFormatProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceMemoryProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceQueueFamilyProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkGetPhysicalDeviceSparseImageFormatProperties2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkGetPhysicalDeviceToolProperties: None,
    #[cfg(feature = "VK_EXT_acquire_drm_display")]
    vkAcquireDrmDisplayEXT: None,
    #[cfg(feature = "VK_EXT_acquire_drm_display")]
    vkGetDrmDisplayEXT: None,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    vkAcquireXlibDisplayEXT: None,
    #[cfg(feature = "VK_EXT_acquire_xlib_display")]
    vkGetRandROutputDisplayEXT: None,
    #[cfg(feature = "VK_EXT_calibrated_timestamps")]
    vkGetPhysicalDeviceCalibrateableTimeDomainsEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkGetPhysicalDeviceDescriptorSizeEXT: None,
    #[cfg(feature = "VK_EXT_direct_mode_display")]
    vkReleaseDisplayEXT: None,
    #[cfg(feature = "VK_EXT_directfb_surface")]
    vkGetPhysicalDeviceDirectFBPresentationSupportEXT: None,
    #[cfg(feature = "VK_EXT_display_surface_counter")]
    vkGetPhysicalDeviceSurfaceCapabilities2EXT: None,
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    vkGetPhysicalDeviceSurfacePresentModes2EXT: None,
    #[cfg(feature = "VK_EXT_sample_locations")]
    vkGetPhysicalDeviceMultisamplePropertiesEXT: None,
    #[cfg(feature = "VK_EXT_tooling_info")]
    vkGetPhysicalDeviceToolPropertiesEXT: None,
    #[cfg(feature = "VK_KHR_calibrated_timestamps")]
    vkGetPhysicalDeviceCalibrateableTimeDomainsKHR: None,
    #[cfg(feature = "VK_KHR_cooperative_matrix")]
    vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR: None,
    #[cfg(any(
      all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
      all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
    ))]
    vkGetPhysicalDevicePresentRectanglesKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkCreateDisplayModeKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkGetDisplayModePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkGetDisplayPlaneCapabilitiesKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkGetDisplayPlaneSupportedDisplaysKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkGetPhysicalDeviceDisplayPlanePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_display")]
    vkGetPhysicalDeviceDisplayPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_external_fence_capabilities")]
    vkGetPhysicalDeviceExternalFencePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_external_memory_capabilities")]
    vkGetPhysicalDeviceExternalBufferPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
    vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: None,
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    vkGetPhysicalDeviceFragmentShadingRatesKHR: None,
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    vkGetDisplayModeProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    vkGetDisplayPlaneCapabilities2KHR: None,
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    vkGetPhysicalDeviceDisplayPlaneProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_display_properties2")]
    vkGetPhysicalDeviceDisplayProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceFeatures2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceFormatProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceImageFormatProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceMemoryProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceQueueFamilyProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
    vkGetPhysicalDeviceSparseImageFormatProperties2KHR: None,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    vkGetPhysicalDeviceSurfaceCapabilities2KHR: None,
    #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
    vkGetPhysicalDeviceSurfaceFormats2KHR: None,
    #[cfg(feature = "VK_KHR_object_refresh")]
    vkGetPhysicalDeviceRefreshableObjectTypesKHR: None,
    #[cfg(feature = "VK_KHR_performance_query")]
    vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR: None,
    #[cfg(feature = "VK_KHR_performance_query")]
    vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR: None,
    #[cfg(feature = "VK_KHR_surface")]
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR: None,
    #[cfg(feature = "VK_KHR_surface")]
    vkGetPhysicalDeviceSurfaceFormatsKHR: None,
    #[cfg(feature = "VK_KHR_surface")]
    vkGetPhysicalDeviceSurfacePresentModesKHR: None,
    #[cfg(feature = "VK_KHR_surface")]
    vkGetPhysicalDeviceSurfaceSupportKHR: None,
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkGetPhysicalDeviceVideoCapabilitiesKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkGetPhysicalDeviceVideoFormatPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_wayland_surface")]
    vkGetPhysicalDeviceWaylandPresentationSupportKHR: None,
    #[cfg(feature = "VK_KHR_win32_surface")]
    vkGetPhysicalDeviceWin32PresentationSupportKHR: None,
    #[cfg(feature = "VK_KHR_xcb_surface")]
    vkGetPhysicalDeviceXcbPresentationSupportKHR: None,
    #[cfg(feature = "VK_KHR_xlib_surface")]
    vkGetPhysicalDeviceXlibPresentationSupportKHR: None,
    #[cfg(feature = "VK_NV_acquire_winrt_display")]
    vkAcquireWinrtDisplayNV: None,
    #[cfg(feature = "VK_NV_acquire_winrt_display")]
    vkGetWinrtDisplayNV: None,
    #[cfg(feature = "VK_NV_cooperative_matrix")]
    vkGetPhysicalDeviceCooperativeMatrixPropertiesNV: None,
    #[cfg(feature = "VK_NV_cooperative_matrix2")]
    vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV: None,
    #[cfg(feature = "VK_NV_cooperative_vector")]
    vkGetPhysicalDeviceCooperativeVectorPropertiesNV: None,
    #[cfg(feature = "VK_NV_coverage_reduction_mode")]
    vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV: None,
    #[cfg(feature = "VK_NV_external_memory_capabilities")]
    vkGetPhysicalDeviceExternalImageFormatPropertiesNV: None,
    #[cfg(feature = "VK_NV_external_memory_sci_buf")]
    vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV: None,
    #[cfg(feature = "VK_NV_external_memory_sci_buf")]
    vkGetPhysicalDeviceSciBufAttributesNV: None,
    #[cfg(any(
      feature = "VK_NV_external_sci_sync",
      feature = "VK_NV_external_sci_sync2"
    ))]
    vkGetPhysicalDeviceSciSyncAttributesNV: None,
    #[cfg(feature = "VK_NV_optical_flow")]
    vkGetPhysicalDeviceOpticalFlowImageFormatsNV: None,
    #[cfg(feature = "VK_QNX_screen_surface")]
    vkGetPhysicalDeviceScreenPresentationSupportQNX: None,
    #[cfg(feature = "VK_SEC_ubm_surface")]
    vkGetPhysicalDeviceUbmPresentationSupportSEC: None,
  };
  #[inline(always)]
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM: loader(
        c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM: loader(
        c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_ARM_data_graph_instruction_set_tosa",
        feature = "VK_ARM_data_graph_optical_flow"
      ))]
      vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM: loader(
        c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
      vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM: loader(
        c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_performance_counters_by_region")]
      vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM: loader(
        c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM: loader(
        c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkGetPhysicalDeviceExternalTensorPropertiesARM: loader(
        c"vkGetPhysicalDeviceExternalTensorPropertiesARM".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCreateDevice: loader(c"vkCreateDevice".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkEnumerateDeviceExtensionProperties: loader(
        c"vkEnumerateDeviceExtensionProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkEnumerateDeviceLayerProperties: loader(c"vkEnumerateDeviceLayerProperties".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceFeatures: loader(c"vkGetPhysicalDeviceFeatures".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceFormatProperties: loader(c"vkGetPhysicalDeviceFormatProperties".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceImageFormatProperties: loader(
        c"vkGetPhysicalDeviceImageFormatProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceMemoryProperties: loader(c"vkGetPhysicalDeviceMemoryProperties".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceProperties: loader(c"vkGetPhysicalDeviceProperties".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceQueueFamilyProperties: loader(
        c"vkGetPhysicalDeviceQueueFamilyProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkGetPhysicalDeviceSparseImageFormatProperties: loader(
        c"vkGetPhysicalDeviceSparseImageFormatProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceExternalBufferProperties: loader(
        c"vkGetPhysicalDeviceExternalBufferProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceExternalFenceProperties: loader(
        c"vkGetPhysicalDeviceExternalFenceProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceExternalSemaphoreProperties: loader(
        c"vkGetPhysicalDeviceExternalSemaphoreProperties".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceFeatures2: loader(c"vkGetPhysicalDeviceFeatures2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceFormatProperties2: loader(
        c"vkGetPhysicalDeviceFormatProperties2".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceImageFormatProperties2: loader(
        c"vkGetPhysicalDeviceImageFormatProperties2".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceMemoryProperties2: loader(
        c"vkGetPhysicalDeviceMemoryProperties2".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceProperties2: loader(c"vkGetPhysicalDeviceProperties2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceQueueFamilyProperties2: loader(
        c"vkGetPhysicalDeviceQueueFamilyProperties2".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkGetPhysicalDeviceSparseImageFormatProperties2: loader(
        c"vkGetPhysicalDeviceSparseImageFormatProperties2".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkGetPhysicalDeviceToolProperties: loader(c"vkGetPhysicalDeviceToolProperties".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_acquire_drm_display")]
      vkAcquireDrmDisplayEXT: loader(c"vkAcquireDrmDisplayEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_acquire_drm_display")]
      vkGetDrmDisplayEXT: loader(c"vkGetDrmDisplayEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_acquire_xlib_display")]
      vkAcquireXlibDisplayEXT: loader(c"vkAcquireXlibDisplayEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_acquire_xlib_display")]
      vkGetRandROutputDisplayEXT: loader(c"vkGetRandROutputDisplayEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_calibrated_timestamps")]
      vkGetPhysicalDeviceCalibrateableTimeDomainsEXT: loader(
        c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkGetPhysicalDeviceDescriptorSizeEXT: loader(
        c"vkGetPhysicalDeviceDescriptorSizeEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_direct_mode_display")]
      vkReleaseDisplayEXT: loader(c"vkReleaseDisplayEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_directfb_surface")]
      vkGetPhysicalDeviceDirectFBPresentationSupportEXT: loader(
        c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_display_surface_counter")]
      vkGetPhysicalDeviceSurfaceCapabilities2EXT: loader(
        c"vkGetPhysicalDeviceSurfaceCapabilities2EXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_full_screen_exclusive")]
      vkGetPhysicalDeviceSurfacePresentModes2EXT: loader(
        c"vkGetPhysicalDeviceSurfacePresentModes2EXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_sample_locations")]
      vkGetPhysicalDeviceMultisamplePropertiesEXT: loader(
        c"vkGetPhysicalDeviceMultisamplePropertiesEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_tooling_info")]
      vkGetPhysicalDeviceToolPropertiesEXT: loader(
        c"vkGetPhysicalDeviceToolPropertiesEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_calibrated_timestamps")]
      vkGetPhysicalDeviceCalibrateableTimeDomainsKHR: loader(
        c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_cooperative_matrix")]
      vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR: loader(
        c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
        all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
      ))]
      vkGetPhysicalDevicePresentRectanglesKHR: loader(
        c"vkGetPhysicalDevicePresentRectanglesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkCreateDisplayModeKHR: loader(c"vkCreateDisplayModeKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkGetDisplayModePropertiesKHR: loader(c"vkGetDisplayModePropertiesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkGetDisplayPlaneCapabilitiesKHR: loader(c"vkGetDisplayPlaneCapabilitiesKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkGetDisplayPlaneSupportedDisplaysKHR: loader(
        c"vkGetDisplayPlaneSupportedDisplaysKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkGetPhysicalDeviceDisplayPlanePropertiesKHR: loader(
        c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_display")]
      vkGetPhysicalDeviceDisplayPropertiesKHR: loader(
        c"vkGetPhysicalDeviceDisplayPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_fence_capabilities")]
      vkGetPhysicalDeviceExternalFencePropertiesKHR: loader(
        c"vkGetPhysicalDeviceExternalFencePropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_memory_capabilities")]
      vkGetPhysicalDeviceExternalBufferPropertiesKHR: loader(
        c"vkGetPhysicalDeviceExternalBufferPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
      vkGetPhysicalDeviceExternalSemaphorePropertiesKHR: loader(
        c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_fragment_shading_rate")]
      vkGetPhysicalDeviceFragmentShadingRatesKHR: loader(
        c"vkGetPhysicalDeviceFragmentShadingRatesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_display_properties2")]
      vkGetDisplayModeProperties2KHR: loader(c"vkGetDisplayModeProperties2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_display_properties2")]
      vkGetDisplayPlaneCapabilities2KHR: loader(c"vkGetDisplayPlaneCapabilities2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_display_properties2")]
      vkGetPhysicalDeviceDisplayPlaneProperties2KHR: loader(
        c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_display_properties2")]
      vkGetPhysicalDeviceDisplayProperties2KHR: loader(
        c"vkGetPhysicalDeviceDisplayProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceFeatures2KHR: loader(c"vkGetPhysicalDeviceFeatures2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceFormatProperties2KHR: loader(
        c"vkGetPhysicalDeviceFormatProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceImageFormatProperties2KHR: loader(
        c"vkGetPhysicalDeviceImageFormatProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceMemoryProperties2KHR: loader(
        c"vkGetPhysicalDeviceMemoryProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceProperties2KHR: loader(c"vkGetPhysicalDeviceProperties2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceQueueFamilyProperties2KHR: loader(
        c"vkGetPhysicalDeviceQueueFamilyProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
      vkGetPhysicalDeviceSparseImageFormatProperties2KHR: loader(
        c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
      vkGetPhysicalDeviceSurfaceCapabilities2KHR: loader(
        c"vkGetPhysicalDeviceSurfaceCapabilities2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
      vkGetPhysicalDeviceSurfaceFormats2KHR: loader(
        c"vkGetPhysicalDeviceSurfaceFormats2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_object_refresh")]
      vkGetPhysicalDeviceRefreshableObjectTypesKHR: loader(
        c"vkGetPhysicalDeviceRefreshableObjectTypesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_performance_query")]
      vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR: loader(
        c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_performance_query")]
      vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR: loader(
        c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_surface")]
      vkGetPhysicalDeviceSurfaceCapabilitiesKHR: loader(
        c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_surface")]
      vkGetPhysicalDeviceSurfaceFormatsKHR: loader(
        c"vkGetPhysicalDeviceSurfaceFormatsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_surface")]
      vkGetPhysicalDeviceSurfacePresentModesKHR: loader(
        c"vkGetPhysicalDeviceSurfacePresentModesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_surface")]
      vkGetPhysicalDeviceSurfaceSupportKHR: loader(
        c"vkGetPhysicalDeviceSurfaceSupportKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_encode_queue")]
      vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR: loader(
        c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkGetPhysicalDeviceVideoCapabilitiesKHR: loader(
        c"vkGetPhysicalDeviceVideoCapabilitiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkGetPhysicalDeviceVideoFormatPropertiesKHR: loader(
        c"vkGetPhysicalDeviceVideoFormatPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_wayland_surface")]
      vkGetPhysicalDeviceWaylandPresentationSupportKHR: loader(
        c"vkGetPhysicalDeviceWaylandPresentationSupportKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_win32_surface")]
      vkGetPhysicalDeviceWin32PresentationSupportKHR: loader(
        c"vkGetPhysicalDeviceWin32PresentationSupportKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_xcb_surface")]
      vkGetPhysicalDeviceXcbPresentationSupportKHR: loader(
        c"vkGetPhysicalDeviceXcbPresentationSupportKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_xlib_surface")]
      vkGetPhysicalDeviceXlibPresentationSupportKHR: loader(
        c"vkGetPhysicalDeviceXlibPresentationSupportKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_acquire_winrt_display")]
      vkAcquireWinrtDisplayNV: loader(c"vkAcquireWinrtDisplayNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_acquire_winrt_display")]
      vkGetWinrtDisplayNV: loader(c"vkGetWinrtDisplayNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cooperative_matrix")]
      vkGetPhysicalDeviceCooperativeMatrixPropertiesNV: loader(
        c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cooperative_matrix2")]
      vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV: loader(
        c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cooperative_vector")]
      vkGetPhysicalDeviceCooperativeVectorPropertiesNV: loader(
        c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_coverage_reduction_mode")]
      vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV: loader(
        c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_memory_capabilities")]
      vkGetPhysicalDeviceExternalImageFormatPropertiesNV: loader(
        c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_memory_sci_buf")]
      vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV: loader(
        c"vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_external_memory_sci_buf")]
      vkGetPhysicalDeviceSciBufAttributesNV: loader(
        c"vkGetPhysicalDeviceSciBufAttributesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_NV_external_sci_sync",
        feature = "VK_NV_external_sci_sync2"
      ))]
      vkGetPhysicalDeviceSciSyncAttributesNV: loader(
        c"vkGetPhysicalDeviceSciSyncAttributesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_optical_flow")]
      vkGetPhysicalDeviceOpticalFlowImageFormatsNV: loader(
        c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QNX_screen_surface")]
      vkGetPhysicalDeviceScreenPresentationSupportQNX: loader(
        c"vkGetPhysicalDeviceScreenPresentationSupportQNX".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_SEC_ubm_surface")]
      vkGetPhysicalDeviceUbmPresentationSupportSEC: loader(
        c"vkGetPhysicalDeviceUbmPresentationSupportSEC".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct PhysicalDevice<'inst> {
  pub(crate) raw: VkPhysicalDevice,
  pub(crate) instance: &'inst Instance<'inst>,
  pub(crate) table: &'inst PhysicalDeviceDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'inst> Send for PhysicalDevice<'inst> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'inst> Sync for PhysicalDevice<'inst> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'inst> PhysicalDevice<'inst> {
  #[inline]
  pub fn raw(&self) -> VkPhysicalDevice {
    self.raw
  }
  #[inline]
  pub fn instance(&self) -> &Instance<'inst> {
    self.instance
  }
  /// [`vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pQueueFamilyDataGraphProcessingEngineInfo`
  /// - `pQueueFamilyDataGraphProcessingEngineProperties`
  #[cfg(feature = "VK_ARM_data_graph")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM(
    &self,
    pQueueFamilyDataGraphProcessingEngineInfo: &VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<
            '_,
        >,
    pQueueFamilyDataGraphProcessingEngineProperties: &mut VkQueueFamilyDataGraphProcessingEnginePropertiesARM<
            '_,
        >,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM
        .unwrap_unchecked()(
        self.raw,
        pQueueFamilyDataGraphProcessingEngineInfo,
        pQueueFamilyDataGraphProcessingEngineProperties,
      )
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `pQueueFamilyDataGraphPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pQueueFamilyDataGraphProperties`: optional: true, len: pQueueFamilyDataGraphPropertyCount
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
  pub fn vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM(
    &self,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphPropertyCount: &mut u32,
    pQueueFamilyDataGraphProperties: *mut VkQueueFamilyDataGraphPropertiesARM<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM
        .unwrap_unchecked()(
        self.raw,
        queueFamilyIndex,
        pQueueFamilyDataGraphPropertyCount,
        pQueueFamilyDataGraphProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph_instruction_set_tosa`
  /// - `VK_ARM_data_graph_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `pQueueFamilyDataGraphProperties`
  /// - `pProperties`
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
  #[cfg(any(
    feature = "VK_ARM_data_graph_instruction_set_tosa",
    feature = "VK_ARM_data_graph_optical_flow"
  ))]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM(
    &self,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: &VkQueueFamilyDataGraphPropertiesARM<'_>,
    pProperties: &mut VkBaseOutStructure<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM
        .unwrap_unchecked()(
        self.raw,
        queueFamilyIndex,
        pQueueFamilyDataGraphProperties,
        pProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_data_graph_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `pQueueFamilyDataGraphProperties`
  /// - `pOpticalFlowImageFormatInfo`
  /// - `pFormatCount`: optional: pointer required, values optional if pointer not null
  /// - `pImageFormatProperties`: optional: true, len: pFormatCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_EXTENSION_NOT_PRESENT`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_ARM_data_graph_optical_flow")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM(
    &self,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: &VkQueueFamilyDataGraphPropertiesARM<'_>,
    pOpticalFlowImageFormatInfo: &VkDataGraphOpticalFlowImageFormatInfoARM<'_>,
    pFormatCount: &mut u32,
    pImageFormatProperties: *mut VkDataGraphOpticalFlowImageFormatPropertiesARM<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM
        .unwrap_unchecked()(
        self.raw,
        queueFamilyIndex,
        pQueueFamilyDataGraphProperties,
        pOpticalFlowImageFormatInfo,
        pFormatCount,
        pImageFormatProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_performance_counters_by_region`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `pCounterCount`: optional: pointer required, values optional if pointer not null
  /// - `pCounters`: optional: true, len: pCounterCount
  /// - `pCounterDescriptions`: optional: true, len: pCounterCount
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
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_ARM_performance_counters_by_region")]
  #[inline(always)]
  pub fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM(
    &self,
    queueFamilyIndex: u32,
    pCounterCount: &mut u32,
    pCounters: *mut VkPerformanceCounterARM<'_>,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionARM<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM
        .unwrap_unchecked()(
        self.raw,
        queueFamilyIndex,
        pCounterCount,
        pCounters,
        pCounterDescriptions,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_shader_instrumentation`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pDescriptionCount`: optional: pointer required, values optional if pointer not null
  /// - `pDescriptions`: optional: true, len: pDescriptionCount
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
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  #[inline(always)]
  pub fn vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM(
    &self,
    pDescriptionCount: &mut u32,
    pDescriptions: *mut VkShaderInstrumentationMetricDescriptionARM<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM
        .unwrap_unchecked()(self.raw, pDescriptionCount, pDescriptions)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html)
  ///
  /// Provided by:
  /// - `VK_ARM_tensors`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalTensorInfo`
  /// - `pExternalTensorProperties`
  #[cfg(feature = "VK_ARM_tensors")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalTensorPropertiesARM(
    &self,
    pExternalTensorInfo: &VkPhysicalDeviceExternalTensorInfoARM<'_>,
    pExternalTensorProperties: &mut VkExternalTensorPropertiesARM<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalTensorPropertiesARM
        .unwrap_unchecked()(self.raw, pExternalTensorInfo, pExternalTensorProperties)
    }
  }
  /// [`vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html)
  ///
  /// Provided by:
  /// - `VKSC_VERSION_1_0`
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pDevice`
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
  ///   - `VK_ERROR_EXTENSION_NOT_PRESENT`
  ///   - `VK_ERROR_FEATURE_NOT_PRESENT`
  ///   - `VK_ERROR_TOO_MANY_OBJECTS`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline]
  pub fn vkCreateDevice(
    &self,
    pCreateInfo: &VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
  ) -> Result<crate::device::Device<'inst>, VkResult> {
    use crate::device::{Device, DeviceDispatchTable};
    let mut raw = VkDevice::NULL;
    {
      let r = unsafe {
        (self.table.vkCreateDevice.unwrap_unchecked())(self.raw, pCreateInfo, pAllocator, &mut raw)
      };
      if r < VkResult::VK_SUCCESS {
        core::hint::cold_path();
        return Err(r);
      }
    }
    let gdpa = unsafe { self.instance.table.vkGetDeviceProcAddr.unwrap_unchecked() };
    let load_lambda = |name: *const c_char| unsafe { gdpa(raw, name) };
    Ok(Device {
      raw,
      instance: self.instance,
      table: DeviceDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      acceleration_structure_khr_table:
        crate::acceleration_structure_khr::AccelerationStructureKHRDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NV_ray_tracing")]
      acceleration_structure_nv_table:
        crate::acceleration_structure_nv::AccelerationStructureNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      buffer_table: crate::buffer::BufferDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
      buffer_collection_fuchsia_table:
        crate::buffer_collection_fuchsia::BufferCollectionFUCHSIADispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      buffer_view_table: crate::buffer_view::BufferViewDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      command_buffer_table: crate::command_buffer::CommandBufferDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      command_pool_table: crate::command_pool::CommandPoolDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NVX_binary_import")]
      cu_function_nvx_table: crate::cu_function_nvx::CuFunctionNVXDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NVX_binary_import")]
      cu_module_nvx_table: crate::cu_module_nvx::CuModuleNVXDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      cuda_function_nv_table: crate::cuda_function_nv::CudaFunctionNVDispatchTable::load(
        load_lambda,
      ),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      cuda_module_nv_table: crate::cuda_module_nv::CudaModuleNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_ARM_data_graph")]
      data_graph_pipeline_session_arm_table:
        crate::data_graph_pipeline_session_arm::DataGraphPipelineSessionARMDispatchTable::load(
          load_lambda,
        ),
      #[cfg(feature = "VK_KHR_deferred_host_operations")]
      deferred_operation_khr_table:
        crate::deferred_operation_khr::DeferredOperationKHRDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_pool_table: crate::descriptor_pool::DescriptorPoolDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_set_table: crate::descriptor_set::DescriptorSetDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      descriptor_set_layout_table:
        crate::descriptor_set_layout::DescriptorSetLayoutDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      descriptor_update_template_table:
        crate::descriptor_update_template::DescriptorUpdateTemplateDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_descriptor_update_template")]
      descriptor_update_template_khr_table:
        crate::descriptor_update_template_khr::DescriptorUpdateTemplateKHRDispatchTable::load(
          load_lambda,
        ),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      device_memory_table: crate::device_memory::DeviceMemoryDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      event_table: crate::event::EventDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NV_external_compute_queue")]
      external_compute_queue_nv_table:
        crate::external_compute_queue_nv::ExternalComputeQueueNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      fence_table: crate::fence::FenceDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      framebuffer_table: crate::framebuffer::FramebufferDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_AMD_gpa_interface")]
      gpa_session_amd_table: crate::gpa_session_amd::GpaSessionAMDDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      image_table: crate::image::ImageDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      image_view_table: crate::image_view::ImageViewDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      indirect_commands_layout_ext_table:
        crate::indirect_commands_layout_ext::IndirectCommandsLayoutEXTDispatchTable::load(
          load_lambda,
        ),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      indirect_commands_layout_nv_table:
        crate::indirect_commands_layout_nv::IndirectCommandsLayoutNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      indirect_execution_set_ext_table:
        crate::indirect_execution_set_ext::IndirectExecutionSetEXTDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      micromap_ext_table: crate::micromap_ext::MicromapEXTDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NV_optical_flow")]
      optical_flow_session_nv_table:
        crate::optical_flow_session_nv::OpticalFlowSessionNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_INTEL_performance_query")]
      performance_configuration_intel_table:
        crate::performance_configuration_intel::PerformanceConfigurationINTELDispatchTable::load(
          load_lambda,
        ),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_table: crate::pipeline::PipelineDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_pipeline_binary")]
      pipeline_binary_khr_table: crate::pipeline_binary_khr::PipelineBinaryKHRDispatchTable::load(
        load_lambda,
      ),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_cache_table: crate::pipeline_cache::PipelineCacheDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      pipeline_layout_table: crate::pipeline_layout::PipelineLayoutDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      private_data_slot_table: crate::private_data_slot::PrivateDataSlotDispatchTable::load(
        load_lambda,
      ),
      #[cfg(feature = "VK_EXT_private_data")]
      private_data_slot_ext_table:
        crate::private_data_slot_ext::PrivateDataSlotEXTDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      query_pool_table: crate::query_pool::QueryPoolDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      queue_table: crate::queue::QueueDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      render_pass_table: crate::render_pass::RenderPassDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      sampler_table: crate::sampler::SamplerDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      sampler_ycbcr_conversion_table:
        crate::sampler_ycbcr_conversion::SamplerYcbcrConversionDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
      sampler_ycbcr_conversion_khr_table:
        crate::sampler_ycbcr_conversion_khr::SamplerYcbcrConversionKHRDispatchTable::load(
          load_lambda,
        ),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      semaphore_table: crate::semaphore::SemaphoreDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_NV_external_sci_sync2")]
      semaphore_sci_sync_pool_nv_table:
        crate::semaphore_sci_sync_pool_nv::SemaphoreSciSyncPoolNVDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_EXT_shader_object")]
      shader_ext_table: crate::shader_ext::ShaderEXTDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      shader_instrumentation_arm_table:
        crate::shader_instrumentation_arm::ShaderInstrumentationARMDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      shader_module_table: crate::shader_module::ShaderModuleDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_swapchain")]
      swapchain_khr_table: crate::swapchain_khr::SwapchainKHRDispatchTable::load(load_lambda),
      #[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
      tensor_arm_table: crate::tensor_arm::TensorARMDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_ARM_tensors")]
      tensor_view_arm_table: crate::tensor_view_arm::TensorViewARMDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_EXT_validation_cache")]
      validation_cache_ext_table:
        crate::validation_cache_ext::ValidationCacheEXTDispatchTable::load(load_lambda),
      #[cfg(feature = "VK_KHR_video_queue")]
      video_session_khr_table: crate::video_session_khr::VideoSessionKHRDispatchTable::load(
        load_lambda,
      ),
      #[cfg(feature = "VK_KHR_video_queue")]
      video_session_parameters_khr_table:
        crate::video_session_parameters_khr::VideoSessionParametersKHRDispatchTable::load(
          load_lambda,
        ),
    })
  }
  /// [`vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pLayerName`: optional: true, len: null-terminated
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  ///   - `VK_ERROR_LAYER_NOT_PRESENT`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkEnumerateDeviceExtensionProperties(
    &self,
    pLayerName: *const core::ffi::c_char,
    pPropertyCount: &mut u32,
    pProperties: *mut VkExtensionProperties,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkEnumerateDeviceExtensionProperties
        .unwrap_unchecked()(self.raw, pLayerName, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html)
  ///
  /// Provided by:
  /// - `VKSC_VERSION_1_0`
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[inline(always)]
  pub fn vkEnumerateDeviceLayerProperties(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkLayerProperties,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkEnumerateDeviceLayerProperties
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFeatures`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceFeatures2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFeatures(&self, pFeatures: &mut VkPhysicalDeviceFeatures) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkGetPhysicalDeviceFeatures.unwrap_unchecked()(self.raw, pFeatures)
    }
  }
  /// [`vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `pFormatProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceFormatProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFormatProperties(
    &self,
    format: VkFormat,
    pFormatProperties: &mut VkFormatProperties,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceFormatProperties
        .unwrap_unchecked()(self.raw, format, pFormatProperties)
    }
  }
  /// [`vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `type`
  /// - `tiling`
  /// - `usage`
  /// - `flags`: optional: true
  /// - `pImageFormatProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceImageFormatProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceImageFormatProperties(
    &self,
    format: VkFormat,
    ty: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: &mut VkImageFormatProperties,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceImageFormatProperties
        .unwrap_unchecked()(
        self.raw,
        format,
        ty,
        tiling,
        usage,
        flags,
        pImageFormatProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pMemoryProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceMemoryProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceMemoryProperties(
    &self,
    pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceMemoryProperties
        .unwrap_unchecked()(self.raw, pMemoryProperties)
    }
  }
  /// [`vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceProperties(&self, pProperties: &mut VkPhysicalDeviceProperties) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceProperties
        .unwrap_unchecked()(self.raw, pProperties)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pQueueFamilyPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pQueueFamilyProperties`: optional: true, len: pQueueFamilyPropertyCount
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceQueueFamilyProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyProperties(
    &self,
    pQueueFamilyPropertyCount: &mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyProperties
        .unwrap_unchecked()(self.raw, pQueueFamilyPropertyCount, pQueueFamilyProperties)
    }
  }
  /// [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_0`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `type`
  /// - `samples`
  /// - `usage`
  /// - `tiling`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  #[deprecated(note = "superseded by `vkGetPhysicalDeviceSparseImageFormatProperties2`")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSparseImageFormatProperties(
    &self,
    format: VkFormat,
    ty: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: &mut u32,
    pProperties: *mut VkSparseImageFormatProperties,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceSparseImageFormatProperties
        .unwrap_unchecked()(
        self.raw,
        format,
        ty,
        samples,
        usage,
        tiling,
        pPropertyCount,
        pProperties,
      )
    }
  }
  /// [`vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalBufferInfo`
  /// - `pExternalBufferProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalBufferProperties(
    &self,
    pExternalBufferInfo: &VkPhysicalDeviceExternalBufferInfo<'_>,
    pExternalBufferProperties: &mut VkExternalBufferProperties<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalBufferProperties
        .unwrap_unchecked()(self.raw, pExternalBufferInfo, pExternalBufferProperties)
    }
  }
  /// [`vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalFenceInfo`
  /// - `pExternalFenceProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalFenceProperties(
    &self,
    pExternalFenceInfo: &VkPhysicalDeviceExternalFenceInfo<'_>,
    pExternalFenceProperties: &mut VkExternalFenceProperties<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalFenceProperties
        .unwrap_unchecked()(self.raw, pExternalFenceInfo, pExternalFenceProperties)
    }
  }
  /// [`vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalSemaphoreInfo`
  /// - `pExternalSemaphoreProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalSemaphoreProperties(
    &self,
    pExternalSemaphoreInfo: &VkPhysicalDeviceExternalSemaphoreInfo<'_>,
    pExternalSemaphoreProperties: &mut VkExternalSemaphoreProperties<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalSemaphoreProperties
        .unwrap_unchecked()(
        self.raw,
        pExternalSemaphoreInfo,
        pExternalSemaphoreProperties,
      )
    }
  }
  /// [`vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFeatures`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFeatures2(&self, pFeatures: &mut VkPhysicalDeviceFeatures2<'_>) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkGetPhysicalDeviceFeatures2.unwrap_unchecked()(self.raw, pFeatures)
    }
  }
  /// [`vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `pFormatProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFormatProperties2(
    &self,
    format: VkFormat,
    pFormatProperties: &mut VkFormatProperties2<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceFormatProperties2
        .unwrap_unchecked()(self.raw, format, pFormatProperties)
    }
  }
  /// [`vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pImageFormatInfo`
  /// - `pImageFormatProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceImageFormatProperties2(
    &self,
    pImageFormatInfo: &VkPhysicalDeviceImageFormatInfo2<'_>,
    pImageFormatProperties: &mut VkImageFormatProperties2<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceImageFormatProperties2
        .unwrap_unchecked()(self.raw, pImageFormatInfo, pImageFormatProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pMemoryProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceMemoryProperties2(
    &self,
    pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties2<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceMemoryProperties2
        .unwrap_unchecked()(self.raw, pMemoryProperties)
    }
  }
  /// [`vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pProperties`
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceProperties2(&self, pProperties: &mut VkPhysicalDeviceProperties2<'_>) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceProperties2
        .unwrap_unchecked()(self.raw, pProperties)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pQueueFamilyPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pQueueFamilyProperties`: optional: true, len: pQueueFamilyPropertyCount
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyProperties2(
    &self,
    pQueueFamilyPropertyCount: &mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyProperties2
        .unwrap_unchecked()(self.raw, pQueueFamilyPropertyCount, pQueueFamilyProperties)
    }
  }
  /// [`vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_1`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFormatInfo`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSparseImageFormatProperties2(
    &self,
    pFormatInfo: &VkPhysicalDeviceSparseImageFormatInfo2<'_>,
    pPropertyCount: &mut u32,
    pProperties: *mut VkSparseImageFormatProperties2<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceSparseImageFormatProperties2
        .unwrap_unchecked()(self.raw, pFormatInfo, pPropertyCount, pProperties)
    }
  }
  /// [`vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html)
  ///
  /// Provided by:
  /// - `VK_BASE_VERSION_1_3`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pToolCount`: optional: pointer required, values optional if pointer not null
  /// - `pToolProperties`: optional: true, len: pToolCount
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
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceToolProperties(
    &self,
    pToolCount: &mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceToolProperties
        .unwrap_unchecked()(self.raw, pToolCount, pToolProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_acquire_drm_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `drmFd`
  /// - `display`
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
  #[cfg(feature = "VK_EXT_acquire_drm_display")]
  #[inline(always)]
  pub fn vkAcquireDrmDisplayEXT(
    &self,
    drmFd: i32,
    display: VkDisplayKHR,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (self.table).vkAcquireDrmDisplayEXT.unwrap_unchecked()(self.raw, drmFd, display) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_acquire_drm_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `drmFd`
  /// - `connectorId`
  /// - `display`
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
  #[cfg(feature = "VK_EXT_acquire_drm_display")]
  #[inline]
  pub fn vkGetDrmDisplayEXT(&self, drmFd: i32, connectorId: u32) -> Result<VkDisplayKHR, VkResult> {
    let mut handle = VkDisplayKHR::NULL;
    let r = unsafe {
      (self.table).vkGetDrmDisplayEXT.unwrap_unchecked()(self.raw, drmFd, connectorId, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(handle)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_acquire_xlib_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `dpy`
  /// - `display`
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
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  #[inline(always)]
  pub fn vkAcquireXlibDisplayEXT(
    &self,
    dpy: &mut Display,
    display: VkDisplayKHR,
  ) -> Result<VkResult, VkResult> {
    let r =
      unsafe { (self.table).vkAcquireXlibDisplayEXT.unwrap_unchecked()(self.raw, dpy, display) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_acquire_xlib_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `dpy`
  /// - `rrOutput`
  /// - `pDisplay`
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
  #[cfg(feature = "VK_EXT_acquire_xlib_display")]
  #[inline]
  pub fn vkGetRandROutputDisplayEXT(
    &self,
    dpy: &mut Display,
    rrOutput: RROutput,
  ) -> Result<VkDisplayKHR, VkResult> {
    let mut handle = VkDisplayKHR::NULL;
    let r = unsafe {
      (self.table).vkGetRandROutputDisplayEXT.unwrap_unchecked()(
        self.raw,
        dpy,
        rrOutput,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(handle)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html)
  ///
  /// Provided by:
  /// - `VK_EXT_calibrated_timestamps`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pTimeDomainCount`: optional: pointer required, values optional if pointer not null
  /// - `pTimeDomains`: optional: true, len: pTimeDomainCount
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
  #[cfg(feature = "VK_EXT_calibrated_timestamps")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    &self,
    pTimeDomainCount: &mut u32,
    pTimeDomains: *mut VkTimeDomainEXT,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCalibrateableTimeDomainsEXT
        .unwrap_unchecked()(self.raw, pTimeDomainCount, pTimeDomains)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_descriptor_heap`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `descriptorType`
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDescriptorSizeEXT(
    &self,
    descriptorType: VkDescriptorType,
  ) -> VkDeviceSize {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceDescriptorSizeEXT
        .unwrap_unchecked()(self.raw, descriptorType)
    }
  }
  /// [`vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_direct_mode_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `display`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_direct_mode_display")]
  #[inline(always)]
  pub fn vkReleaseDisplayEXT(&self, display: VkDisplayKHR) -> Result<VkResult, VkResult> {
    let r = unsafe { (self.table).vkReleaseDisplayEXT.unwrap_unchecked()(self.raw, display) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_directfb_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `dfb`
  #[cfg(feature = "VK_EXT_directfb_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDirectFBPresentationSupportEXT(
    &self,
    queueFamilyIndex: u32,
    dfb: &mut IDirectFB,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceDirectFBPresentationSupportEXT
        .unwrap_unchecked()(self.raw, queueFamilyIndex, dfb)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_display_surface_counter`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `surface`
  /// - `pSurfaceCapabilities`
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
  #[cfg(feature = "VK_EXT_display_surface_counter")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(
    &self,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: &mut VkSurfaceCapabilities2EXT<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceCapabilities2EXT
        .unwrap_unchecked()(self.raw, surface, pSurfaceCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_full_screen_exclusive`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pSurfaceInfo`
  /// - `pPresentModeCount`: optional: pointer required, values optional if pointer not null
  /// - `pPresentModes`: optional: true, len: pPresentModeCount
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
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_EXT_full_screen_exclusive")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfacePresentModes2EXT(
    &self,
    pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pPresentModeCount: &mut u32,
    pPresentModes: *mut VkPresentModeKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfacePresentModes2EXT
        .unwrap_unchecked()(self.raw, pSurfaceInfo, pPresentModeCount, pPresentModes)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html)
  ///
  /// Provided by:
  /// - `VK_EXT_sample_locations`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `samples`
  /// - `pMultisampleProperties`
  #[cfg(feature = "VK_EXT_sample_locations")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceMultisamplePropertiesEXT(
    &self,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: &mut VkMultisamplePropertiesEXT<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceMultisamplePropertiesEXT
        .unwrap_unchecked()(self.raw, samples, pMultisampleProperties)
    }
  }
  /// [`vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html)
  ///
  /// Provided by:
  /// - `VK_EXT_tooling_info`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pToolCount`: optional: pointer required, values optional if pointer not null
  /// - `pToolProperties`: optional: true, len: pToolCount
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
  #[cfg(feature = "VK_EXT_tooling_info")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceToolPropertiesEXT(
    &self,
    pToolCount: &mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolPropertiesEXT<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceToolPropertiesEXT
        .unwrap_unchecked()(self.raw, pToolCount, pToolProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_calibrated_timestamps`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pTimeDomainCount`: optional: pointer required, values optional if pointer not null
  /// - `pTimeDomains`: optional: true, len: pTimeDomainCount
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
  #[cfg(feature = "VK_KHR_calibrated_timestamps")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCalibrateableTimeDomainsKHR(
    &self,
    pTimeDomainCount: &mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCalibrateableTimeDomainsKHR
        .unwrap_unchecked()(self.raw, pTimeDomainCount, pTimeDomains)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_cooperative_matrix`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_cooperative_matrix")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
  /// - `VK_KHR_swapchain`
  ///
  /// - **Availability:** depends on `VK_VERSION_1_1`
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `surface`
  /// - `pRectCount`: optional: pointer required, values optional if pointer not null
  /// - `pRects`: optional: true, len: pRectCount
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
  #[cfg(any(
    all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
    all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
  ))]
  #[inline(always)]
  pub fn vkGetPhysicalDevicePresentRectanglesKHR(
    &self,
    surface: VkSurfaceKHR,
    pRectCount: &mut u32,
    pRects: *mut VkRect2D,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDevicePresentRectanglesKHR
        .unwrap_unchecked()(self.raw, surface, pRectCount, pRects)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `display`
  /// - `pCreateInfo`
  /// - `pAllocator`: optional: true
  /// - `pMode`
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
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_display")]
  #[inline]
  pub fn vkCreateDisplayModeKHR(
    &self,
    display: VkDisplayKHR,
    pCreateInfo: &VkDisplayModeCreateInfoKHR<'_>,
    pAllocator: *const VkAllocationCallbacks<'_>,
  ) -> Result<VkDisplayModeKHR, VkResult> {
    let mut handle = VkDisplayModeKHR::NULL;
    let r = unsafe {
      (self.table).vkCreateDisplayModeKHR.unwrap_unchecked()(
        self.raw,
        display,
        pCreateInfo,
        pAllocator,
        &mut handle,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(handle)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `display`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_display")]
  #[inline(always)]
  pub fn vkGetDisplayModePropertiesKHR(
    &self,
    display: VkDisplayKHR,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDisplayModePropertiesKHR
        .unwrap_unchecked()(self.raw, display, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `mode`
  /// - `planeIndex`
  /// - `pCapabilities`
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
  #[cfg(feature = "VK_KHR_display")]
  #[inline(always)]
  pub fn vkGetDisplayPlaneCapabilitiesKHR(
    &self,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: &mut VkDisplayPlaneCapabilitiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDisplayPlaneCapabilitiesKHR
        .unwrap_unchecked()(self.raw, mode, planeIndex, pCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `planeIndex`
  /// - `pDisplayCount`: optional: pointer required, values optional if pointer not null
  /// - `pDisplays`: optional: true, len: pDisplayCount
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
  #[cfg(feature = "VK_KHR_display")]
  #[inline(always)]
  pub fn vkGetDisplayPlaneSupportedDisplaysKHR(
    &self,
    planeIndex: u32,
    pDisplayCount: &mut u32,
    pDisplays: *mut VkDisplayKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDisplayPlaneSupportedDisplaysKHR
        .unwrap_unchecked()(self.raw, planeIndex, pDisplayCount, pDisplays)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_display")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceDisplayPlanePropertiesKHR
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_display")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayPropertiesKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceDisplayPropertiesKHR
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_fence_capabilities`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalFenceInfo`
  /// - `pExternalFenceProperties`
  #[cfg(feature = "VK_KHR_external_fence_capabilities")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(
    &self,
    pExternalFenceInfo: &VkPhysicalDeviceExternalFenceInfoKHR<'_>,
    pExternalFenceProperties: &mut VkExternalFencePropertiesKHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalFencePropertiesKHR
        .unwrap_unchecked()(self.raw, pExternalFenceInfo, pExternalFenceProperties)
    }
  }
  /// [`vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_memory_capabilities`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalBufferInfo`
  /// - `pExternalBufferProperties`
  #[cfg(feature = "VK_KHR_external_memory_capabilities")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
    &self,
    pExternalBufferInfo: &VkPhysicalDeviceExternalBufferInfoKHR<'_>,
    pExternalBufferProperties: &mut VkExternalBufferPropertiesKHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalBufferPropertiesKHR
        .unwrap_unchecked()(self.raw, pExternalBufferInfo, pExternalBufferProperties)
    }
  }
  /// [`vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html)
  ///
  /// Provided by:
  /// - `VK_KHR_external_semaphore_capabilities`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pExternalSemaphoreInfo`
  /// - `pExternalSemaphoreProperties`
  #[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
    &self,
    pExternalSemaphoreInfo: &VkPhysicalDeviceExternalSemaphoreInfoKHR<'_>,
    pExternalSemaphoreProperties: &mut VkExternalSemaphorePropertiesKHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceExternalSemaphorePropertiesKHR
        .unwrap_unchecked()(
        self.raw,
        pExternalSemaphoreInfo,
        pExternalSemaphoreProperties,
      )
    }
  }
  /// [`vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_fragment_shading_rate`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFragmentShadingRateCount`: optional: pointer required, values optional if pointer not null
  /// - `pFragmentShadingRates`: optional: true, len: pFragmentShadingRateCount
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
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
    &self,
    pFragmentShadingRateCount: &mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceFragmentShadingRatesKHR
        .unwrap_unchecked()(self.raw, pFragmentShadingRateCount, pFragmentShadingRates)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_display_properties2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `display`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  #[inline(always)]
  pub fn vkGetDisplayModeProperties2KHR(
    &self,
    display: VkDisplayKHR,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDisplayModeProperties2KHR
        .unwrap_unchecked()(self.raw, display, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_display_properties2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pDisplayPlaneInfo`
  /// - `pCapabilities`
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
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  #[inline(always)]
  pub fn vkGetDisplayPlaneCapabilities2KHR(
    &self,
    pDisplayPlaneInfo: &VkDisplayPlaneInfo2KHR<'_>,
    pCapabilities: &mut VkDisplayPlaneCapabilities2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetDisplayPlaneCapabilities2KHR
        .unwrap_unchecked()(self.raw, pDisplayPlaneInfo, pCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_display_properties2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceDisplayPlaneProperties2KHR
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_display_properties2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_KHR_get_display_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceDisplayProperties2KHR(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkDisplayProperties2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceDisplayProperties2KHR
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFeatures`
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFeatures2KHR(&self, pFeatures: &mut VkPhysicalDeviceFeatures2KHR<'_>) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceFeatures2KHR
        .unwrap_unchecked()(self.raw, pFeatures)
    }
  }
  /// [`vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `pFormatProperties`
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceFormatProperties2KHR(
    &self,
    format: VkFormat,
    pFormatProperties: &mut VkFormatProperties2KHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceFormatProperties2KHR
        .unwrap_unchecked()(self.raw, format, pFormatProperties)
    }
  }
  /// [`vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pImageFormatInfo`
  /// - `pImageFormatProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(
    &self,
    pImageFormatInfo: &VkPhysicalDeviceImageFormatInfo2KHR<'_>,
    pImageFormatProperties: &mut VkImageFormatProperties2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceImageFormatProperties2KHR
        .unwrap_unchecked()(self.raw, pImageFormatInfo, pImageFormatProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pMemoryProperties`
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceMemoryProperties2KHR(
    &self,
    pMemoryProperties: &mut VkPhysicalDeviceMemoryProperties2KHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceMemoryProperties2KHR
        .unwrap_unchecked()(self.raw, pMemoryProperties)
    }
  }
  /// [`vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pProperties`
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceProperties2KHR(
    &self,
    pProperties: &mut VkPhysicalDeviceProperties2KHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceProperties2KHR
        .unwrap_unchecked()(self.raw, pProperties)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pQueueFamilyPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pQueueFamilyProperties`: optional: true, len: pQueueFamilyPropertyCount
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(
    &self,
    pQueueFamilyPropertyCount: &mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyProperties2KHR
        .unwrap_unchecked()(self.raw, pQueueFamilyPropertyCount, pQueueFamilyProperties)
    }
  }
  /// [`vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_physical_device_properties2`
  ///
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pFormatInfo`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
  #[cfg(feature = "VK_KHR_get_physical_device_properties2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
    &self,
    pFormatInfo: &VkPhysicalDeviceSparseImageFormatInfo2KHR<'_>,
    pPropertyCount: &mut u32,
    pProperties: *mut VkSparseImageFormatProperties2KHR<'_>,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceSparseImageFormatProperties2KHR
        .unwrap_unchecked()(self.raw, pFormatInfo, pPropertyCount, pProperties)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_surface_capabilities2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pSurfaceInfo`
  /// - `pSurfaceCapabilities`
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
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
    &self,
    pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pSurfaceCapabilities: &mut VkSurfaceCapabilities2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceCapabilities2KHR
        .unwrap_unchecked()(self.raw, pSurfaceInfo, pSurfaceCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_get_surface_capabilities2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pSurfaceInfo`
  /// - `pSurfaceFormatCount`: optional: pointer required, values optional if pointer not null
  /// - `pSurfaceFormats`: optional: true, len: pSurfaceFormatCount
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
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_get_surface_capabilities2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(
    &self,
    pSurfaceInfo: &VkPhysicalDeviceSurfaceInfo2KHR<'_>,
    pSurfaceFormatCount: &mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceFormats2KHR
        .unwrap_unchecked()(self.raw, pSurfaceInfo, pSurfaceFormatCount, pSurfaceFormats)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceRefreshableObjectTypesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_object_refresh`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pRefreshableObjectTypeCount`: optional: pointer required, values optional if pointer not null
  /// - `pRefreshableObjectTypes`: optional: true, len: pRefreshableObjectTypeCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_object_refresh")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceRefreshableObjectTypesKHR(
    &self,
    pRefreshableObjectTypeCount: &mut u32,
    pRefreshableObjectTypes: *mut VkObjectType,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceRefreshableObjectTypesKHR
        .unwrap_unchecked()(
        self.raw,
        pRefreshableObjectTypeCount,
        pRefreshableObjectTypes,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_performance_query`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `pCounterCount`: optional: pointer required, values optional if pointer not null
  /// - `pCounters`: optional: true, len: pCounterCount
  /// - `pCounterDescriptions`: optional: true, len: pCounterCount
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
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_performance_query")]
  #[inline(always)]
  pub fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
    &self,
    queueFamilyIndex: u32,
    pCounterCount: &mut u32,
    pCounters: *mut VkPerformanceCounterKHR<'_>,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR
        .unwrap_unchecked()(
        self.raw,
        queueFamilyIndex,
        pCounterCount,
        pCounters,
        pCounterDescriptions,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_performance_query`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPerformanceQueryCreateInfo`
  /// - `pNumPasses`
  #[cfg(feature = "VK_KHR_performance_query")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(
    &self,
    pPerformanceQueryCreateInfo: &VkQueryPoolPerformanceCreateInfoKHR<'_>,
    pNumPasses: &mut u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR
        .unwrap_unchecked()(self.raw, pPerformanceQueryCreateInfo, pNumPasses)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `surface`
  /// - `pSurfaceCapabilities`
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
  #[cfg(feature = "VK_KHR_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
    &self,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: &mut VkSurfaceCapabilitiesKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceCapabilitiesKHR
        .unwrap_unchecked()(self.raw, surface, pSurfaceCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `surface`: optional: true
  /// - `pSurfaceFormatCount`: optional: pointer required, values optional if pointer not null
  /// - `pSurfaceFormats`: optional: true, len: pSurfaceFormatCount
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
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(
    &self,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: &mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceFormatsKHR
        .unwrap_unchecked()(self.raw, surface, pSurfaceFormatCount, pSurfaceFormats)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `surface`: optional: true
  /// - `pPresentModeCount`: optional: pointer required, values optional if pointer not null
  /// - `pPresentModes`: optional: true, len: pPresentModeCount
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
  ///   - `VK_ERROR_SURFACE_LOST_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(
    &self,
    surface: VkSurfaceKHR,
    pPresentModeCount: &mut u32,
    pPresentModes: *mut VkPresentModeKHR,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfacePresentModesKHR
        .unwrap_unchecked()(self.raw, surface, pPresentModeCount, pPresentModes)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `surface`
  /// - `pSupported`
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
  #[cfg(feature = "VK_KHR_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSurfaceSupportKHR(
    &self,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: &mut VkBool32,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSurfaceSupportKHR
        .unwrap_unchecked()(self.raw, queueFamilyIndex, surface, pSupported)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_encode_queue`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pQualityLevelInfo`
  /// - `pQualityLevelProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR(
    &self,
    pQualityLevelInfo: &VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
    pQualityLevelProperties: &mut VkVideoEncodeQualityLevelPropertiesKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR
        .unwrap_unchecked()(self.raw, pQualityLevelInfo, pQualityLevelProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pVideoProfile`
  /// - `pCapabilities`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceVideoCapabilitiesKHR(
    &self,
    pVideoProfile: &VkVideoProfileInfoKHR<'_>,
    pCapabilities: &mut VkVideoCapabilitiesKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceVideoCapabilitiesKHR
        .unwrap_unchecked()(self.raw, pVideoProfile, pCapabilities)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_video_queue`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pVideoFormatInfo`
  /// - `pVideoFormatPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pVideoFormatProperties`: optional: true, len: pVideoFormatPropertyCount
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
  ///   - `VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_KHR_video_queue")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceVideoFormatPropertiesKHR(
    &self,
    pVideoFormatInfo: &VkPhysicalDeviceVideoFormatInfoKHR<'_>,
    pVideoFormatPropertyCount: &mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceVideoFormatPropertiesKHR
        .unwrap_unchecked()(
        self.raw,
        pVideoFormatInfo,
        pVideoFormatPropertyCount,
        pVideoFormatProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_wayland_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `display`
  #[cfg(feature = "VK_KHR_wayland_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceWaylandPresentationSupportKHR(
    &self,
    queueFamilyIndex: u32,
    display: &mut wl_display,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceWaylandPresentationSupportKHR
        .unwrap_unchecked()(self.raw, queueFamilyIndex, display)
    }
  }
  /// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_win32_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  #[cfg(feature = "VK_KHR_win32_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceWin32PresentationSupportKHR(&self, queueFamilyIndex: u32) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceWin32PresentationSupportKHR
        .unwrap_unchecked()(self.raw, queueFamilyIndex)
    }
  }
  /// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_xcb_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `connection`
  /// - `visual_id`
  #[cfg(feature = "VK_KHR_xcb_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceXcbPresentationSupportKHR(
    &self,
    queueFamilyIndex: u32,
    connection: &mut xcb_connection_t,
    visual_id: xcb_visualid_t,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceXcbPresentationSupportKHR
        .unwrap_unchecked()(self.raw, queueFamilyIndex, connection, visual_id)
    }
  }
  /// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html)
  ///
  /// Provided by:
  /// - `VK_KHR_xlib_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `dpy`
  /// - `visualID`
  #[cfg(feature = "VK_KHR_xlib_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceXlibPresentationSupportKHR(
    &self,
    queueFamilyIndex: u32,
    dpy: &mut Display,
    visualID: VisualID,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceXlibPresentationSupportKHR
        .unwrap_unchecked()(self.raw, queueFamilyIndex, dpy, visualID)
    }
  }
  /// [`vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_acquire_winrt_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `display`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_acquire_winrt_display")]
  #[inline(always)]
  pub fn vkAcquireWinrtDisplayNV(&self, display: VkDisplayKHR) -> Result<VkResult, VkResult> {
    let r = unsafe { (self.table).vkAcquireWinrtDisplayNV.unwrap_unchecked()(self.raw, display) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_acquire_winrt_display`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `deviceRelativeId`
  /// - `pDisplay`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_DEVICE_LOST`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_acquire_winrt_display")]
  #[inline]
  pub fn vkGetWinrtDisplayNV(&self, deviceRelativeId: u32) -> Result<VkDisplayKHR, VkResult> {
    let mut handle = VkDisplayKHR::NULL;
    let r = unsafe {
      (self.table).vkGetWinrtDisplayNV.unwrap_unchecked()(self.raw, deviceRelativeId, &mut handle)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(handle)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cooperative_matrix`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_NV_cooperative_matrix")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCooperativeMatrixPropertiesNV
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cooperative_matrix2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_NV_cooperative_matrix2")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_cooperative_vector`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
  /// - `pProperties`: optional: true, len: pPropertyCount
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
  #[cfg(feature = "VK_NV_cooperative_vector")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceCooperativeVectorPropertiesNV(
    &self,
    pPropertyCount: &mut u32,
    pProperties: *mut VkCooperativeVectorPropertiesNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceCooperativeVectorPropertiesNV
        .unwrap_unchecked()(self.raw, pPropertyCount, pProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_coverage_reduction_mode`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pCombinationCount`: optional: pointer required, values optional if pointer not null
  /// - `pCombinations`: optional: true, len: pCombinationCount
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
  #[cfg(feature = "VK_NV_coverage_reduction_mode")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
    &self,
    pCombinationCount: &mut u32,
    pCombinations: *mut VkFramebufferMixedSamplesCombinationNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV
        .unwrap_unchecked()(self.raw, pCombinationCount, pCombinations)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_memory_capabilities`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `format`
  /// - `type`
  /// - `tiling`
  /// - `usage`
  /// - `flags`: optional: true
  /// - `externalHandleType`: optional: true
  /// - `pExternalImageFormatProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
  ///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_memory_capabilities")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
    &self,
    format: VkFormat,
    ty: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
    pExternalImageFormatProperties: &mut VkExternalImageFormatPropertiesNV,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceExternalImageFormatPropertiesNV
        .unwrap_unchecked()(
        self.raw,
        format,
        ty,
        tiling,
        usage,
        flags,
        externalHandleType,
        pExternalImageFormatProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_memory_sci_buf`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `handleType`
  /// - `handle`
  /// - `pMemorySciBufProperties`
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV(
    &self,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: NvSciBufObj,
    pMemorySciBufProperties: &mut VkMemorySciBufPropertiesNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV
        .unwrap_unchecked()(self.raw, handleType, handle, pMemorySciBufProperties)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSciBufAttributesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSciBufAttributesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_memory_sci_buf`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pAttributes`
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
  #[cfg(feature = "VK_NV_external_memory_sci_buf")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSciBufAttributesNV(
    &self,
    pAttributes: NvSciBufAttrList,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSciBufAttributesNV
        .unwrap_unchecked()(self.raw, pAttributes)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceSciSyncAttributesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSciSyncAttributesNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_external_sci_sync`
  /// - `VK_NV_external_sci_sync2`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pSciSyncAttributesInfo`
  /// - `pAttributes`
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
  #[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
  ))]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceSciSyncAttributesNV(
    &self,
    pSciSyncAttributesInfo: &VkSciSyncAttributesInfoNV<'_>,
    pAttributes: NvSciSyncAttrList,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceSciSyncAttributesNV
        .unwrap_unchecked()(self.raw, pSciSyncAttributesInfo, pAttributes)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html)
  ///
  /// Provided by:
  /// - `VK_NV_optical_flow`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `pOpticalFlowImageFormatInfo`
  /// - `pFormatCount`: optional: pointer required, values optional if pointer not null
  /// - `pImageFormatProperties`: optional: true, len: pFormatCount
  ///
  /// # Returns
  ///
  /// **Success Codes:**
  ///   - `VK_SUCCESS`
  ///   - `VK_INCOMPLETE`
  ///
  /// **Error Codes:**
  ///   - `VK_ERROR_EXTENSION_NOT_PRESENT`
  ///   - `VK_ERROR_INITIALIZATION_FAILED`
  ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
  ///   - `VK_ERROR_UNKNOWN`
  ///   - `VK_ERROR_VALIDATION_FAILED`
  #[cfg(feature = "VK_NV_optical_flow")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceOpticalFlowImageFormatsNV(
    &self,
    pOpticalFlowImageFormatInfo: &VkOpticalFlowImageFormatInfoNV<'_>,
    pFormatCount: &mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV<'_>,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkGetPhysicalDeviceOpticalFlowImageFormatsNV
        .unwrap_unchecked()(
        self.raw,
        pOpticalFlowImageFormatInfo,
        pFormatCount,
        pImageFormatProperties,
      )
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      core::hint::cold_path();
      Err(r)
    }
  }
  /// [`vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html)
  ///
  /// Provided by:
  /// - `VK_QNX_screen_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `window`
  #[cfg(feature = "VK_QNX_screen_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceScreenPresentationSupportQNX(
    &self,
    queueFamilyIndex: u32,
    window: &mut _screen_window,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceScreenPresentationSupportQNX
        .unwrap_unchecked()(self.raw, queueFamilyIndex, window)
    }
  }
  /// [`vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html)
  ///
  /// Provided by:
  /// - `VK_SEC_ubm_surface`
  ///
  ///
  /// # Parameters
  /// - `physicalDevice`
  /// - `queueFamilyIndex`
  /// - `device`
  #[cfg(feature = "VK_SEC_ubm_surface")]
  #[inline(always)]
  pub fn vkGetPhysicalDeviceUbmPresentationSupportSEC(
    &self,
    queueFamilyIndex: u32,
    device: &mut ubm_device,
  ) -> VkBool32 {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkGetPhysicalDeviceUbmPresentationSupportSEC
        .unwrap_unchecked()(self.raw, queueFamilyIndex, device)
    }
  }
}
