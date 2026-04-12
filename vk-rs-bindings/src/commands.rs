//! Vulkan command function pointer types (`PFN_vk*`).
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_void};
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_acquire_drm_display")]
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    display: VkDisplayKHR,
) -> VkResult;
/// [`vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_full_screen_exclusive`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_TIMEOUT
///   - VK_NOT_READY
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: VkDevice,
    pAcquireInfo: *const VkAcquireNextImageInfoKHR,
    pImageIndex: *mut u32,
) -> VkResult;
/// [`vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html)
///
/// Provided by:
/// - `VK_KHR_swapchain`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `timeout`
/// - `semaphore`: optional: true
/// - `fence`: optional: true
/// - `pImageIndex`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_TIMEOUT
///   - VK_NOT_READY
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    pImageIndex: *mut u32,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: VkDevice,
    pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL,
    pConfiguration: *mut VkPerformanceConfigurationINTEL,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_TIMEOUT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_performance_query")]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkAcquireProfilingLockInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_acquire_winrt_display")]
pub type PFN_vkAcquireWinrtDisplayNV =
    unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    display: VkDisplayKHR,
) -> VkResult;
/// [`vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `pAllocateInfo`
/// - `pCommandBuffers`: len: pAllocateInfo->commandBufferCount
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    pAllocateInfo: *const VkCommandBufferAllocateInfo,
    pCommandBuffers: *mut VkCommandBuffer,
) -> VkResult;
/// [`vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `pAllocateInfo`
/// - `pDescriptorSets`: len: pAllocateInfo->descriptorSetCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_FRAGMENTED_POOL
///   - VK_ERROR_OUT_OF_POOL_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    pAllocateInfo: *const VkDescriptorSetAllocateInfo,
    pDescriptorSets: *mut VkDescriptorSet,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: VkDevice,
    pAllocateInfo: *const VkMemoryAllocateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pMemory: *mut VkDeviceMemory,
) -> VkResult;
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
pub type PFN_vkAntiLagUpdateAMD =
    unsafe extern "system" fn(device: VkDevice, pData: *const VkAntiLagDataAMD);
/// [`vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pBeginInfo`
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkCommandBufferBeginInfo,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV,
) -> VkResult;
/// [`vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `buffer`
/// - `memory`
/// - `memoryOffset`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindBufferMemoryInfo,
) -> VkResult;
/// [`vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_bind_memory2`
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindDataGraphPipelineSessionMemoryInfoARM,
) -> VkResult;
/// [`vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `image`
/// - `memory`
/// - `memoryOffset`
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
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memoryOffset: VkDeviceSize,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindImageMemoryInfo,
) -> VkResult;
/// [`vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_bind_memory2`
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;
/// [`vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html)
///
/// Provided by:
/// - `VK_NV_optical_flow`
///
///
/// # Parameters
/// - `device`
/// - `session`
/// - `bindingPoint`
/// - `view`: optional: true
/// - `layout`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    bindingPoint: VkOpticalFlowSessionBindingPointNV,
    view: VkImageView,
    layout: VkImageLayout,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: VkDevice,
    bindInfoCount: u32,
    pBindInfos: *const VkBindTensorMemoryInfoARM,
) -> VkResult;
/// [`vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
///
/// # Parameters
/// - `device`
/// - `videoSession`
/// - `bindSessionMemoryInfoCount`
/// - `pBindSessionMemoryInfos`: len: bindSessionMemoryInfoCount
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
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    bindSessionMemoryInfoCount: u32,
    pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
) -> VkResult;
/// [`vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html)
///
/// Provided by:
/// - `VK_ARM_shader_instrumentation`
///
///
/// # Parameters
/// - `device`
/// - `instrumentation`
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkClearShaderInstrumentationMetricsARM =
    unsafe extern "system" fn(device: VkDevice, instrumentation: VkShaderInstrumentationARM);
/// [`vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pConditionalRenderingBegin`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdBeginConditionalRendering2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfo2EXT,
);
/// [`vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html)
///
/// Provided by:
/// - `VK_EXT_conditional_rendering`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pConditionalRenderingBegin`
#[cfg(feature = "VK_EXT_conditional_rendering")]
#[deprecated(note = "superseded by `vkCmdBeginConditionalRendering2EXT`")]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT,
);
/// [`vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html)
///
/// Provided by:
/// - `VK_EXT_custom_resolve`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pBeginCustomResolveInfo`: optional: true
#[cfg(feature = "VK_EXT_custom_resolve")]
pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBeginCustomResolveInfo: *const VkBeginCustomResolveInfoEXT,
);
/// [`vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pLabelInfo`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
);
/// [`vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_tile_shading`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pPerTileBeginInfo`
#[cfg(feature = "VK_QCOM_tile_shading")]
pub type PFN_vkCmdBeginPerTileExecutionQCOM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPerTileBeginInfo: *const VkPerTileBeginInfoQCOM,
);
/// [`vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `query`
/// - `flags`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
);
/// [`vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `query`
/// - `flags`: optional: true
/// - `index`
#[cfg(feature = "VK_EXT_transform_feedback")]
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
);
/// [`vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pRenderPassBegin`
/// - `contents`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdBeginRenderPass2`")]
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
);
/// [`vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_2`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pRenderPassBegin`
/// - `pSubpassBeginInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRenderPassBegin: *const VkRenderPassBeginInfo,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
);
/// [`vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;
/// [`vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action, State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pRenderingInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRenderingInfo: *const VkRenderingInfo,
);
/// [`vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering`
///
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;
/// [`vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html)
///
/// Provided by:
/// - `VK_ARM_shader_instrumentation`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `instrumentation`
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkCmdBeginShaderInstrumentationARM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    instrumentation: VkShaderInstrumentationARM,
);
/// [`vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterRange`
/// - `counterRangeCount`: optional: true
/// - `pCounterInfos`: optional: true, len: counterRangeCount
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdBeginTransformFeedback2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
);
/// [`vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterBuffer`
/// - `counterBufferCount`: optional: true
/// - `pCounterBuffers`: len: counterBufferCount
/// - `pCounterBufferOffsets`: optional: true, len: counterBufferCount
#[cfg(feature = "VK_EXT_transform_feedback")]
#[deprecated(note = "superseded by `vkCmdBeginTransformFeedback2EXT`")]
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
);
/// [`vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
/// - **Queues:** VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pBeginInfo`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBeginInfo: *const VkVideoBeginCodingInfoKHR,
);
/// [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pBindDescriptorBufferEmbeddedSamplersInfo`
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorBufferEmbeddedSamplersInfo: *const VkBindDescriptorBufferEmbeddedSamplersInfoEXT,
);
/// [`vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `layout`
/// - `set`
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
);
/// [`vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `bufferCount`
/// - `pBindingInfos`: len: bufferCount
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    bufferCount: u32,
    pBindingInfos: *const VkDescriptorBufferBindingInfoEXT,
);
/// [`vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `layout`
/// - `firstSet`
/// - `descriptorSetCount`
/// - `pDescriptorSets`: optional: pointer required, values optional if pointer not null, len: descriptorSetCount
/// - `dynamicOffsetCount`: optional: true
/// - `pDynamicOffsets`: len: dynamicOffsetCount
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
    dynamicOffsetCount: u32,
    pDynamicOffsets: *const u32,
);
/// [`vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pBindDescriptorSetsInfo`
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo,
);
/// [`vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
/// [`vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`: optional: true
/// - `offset`
/// - `indexType`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
);
/// [`vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_4`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`: optional: true
/// - `offset`
/// - `size`
/// - `indexType`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
#[deprecated(note = "superseded by `vkCmdBindIndexBuffer3KHR`")]
pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
);
/// [`vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;
/// [`vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdBindIndexBuffer3KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkBindIndexBuffer3InfoKHR,
);
/// [`vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html)
///
/// Provided by:
/// - `VK_HUAWEI_invocation_mask`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `imageView`: optional: true
/// - `imageLayout`
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
);
/// [`vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `pipeline`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
);
/// [`vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html)
///
/// Provided by:
/// - `VK_NV_device_generated_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `pipeline`
/// - `groupIndex`
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
);
/// [`vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pBindInfo`
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkCmdBindResourceHeapEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBindInfo: *const VkBindHeapInfoEXT);
/// [`vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pBindInfo`
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkCmdBindSamplerHeapEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pBindInfo: *const VkBindHeapInfoEXT);
/// [`vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html)
///
/// Provided by:
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `stageCount`
/// - `pStages`: len: stageCount
/// - `pShaders`: optional: pointer, values optional, len: stageCount
#[cfg(feature = "VK_EXT_shader_object")]
pub type PFN_vkCmdBindShadersEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    stageCount: u32,
    pStages: *const VkShaderStageFlagBits,
    pShaders: *const VkShaderEXT,
);
/// [`vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html)
///
/// Provided by:
/// - `VK_NV_shading_rate_image`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `imageView`: optional: true
/// - `imageLayout`
#[cfg(feature = "VK_NV_shading_rate_image")]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    imageView: VkImageView,
    imageLayout: VkImageLayout,
);
/// [`vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_tile_memory_heap`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pTileMemoryBindInfo`: optional: true
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pTileMemoryBindInfo: *const VkTileMemoryBindInfoQCOM,
);
/// [`vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBindingInfos`: optional: true, len: bindingCount
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBindingInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
);
/// [`vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBuffers`: len: bindingCount
/// - `pOffsets`: len: bindingCount
/// - `pSizes`: optional: true, len: bindingCount
#[cfg(feature = "VK_EXT_transform_feedback")]
#[deprecated(note = "superseded by `vkCmdBindTransformFeedbackBuffers2EXT`")]
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
);
/// [`vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBuffers`: optional: pointer required, values optional if pointer not null, len: bindingCount
/// - `pOffsets`: len: bindingCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
);
/// [`vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBuffers`: optional: pointer required, values optional if pointer not null, len: bindingCount
/// - `pOffsets`: len: bindingCount
/// - `pSizes`: optional: true, len: bindingCount
/// - `pStrides`: optional: true, len: bindingCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBuffers: *const VkBuffer,
    pOffsets: *const VkDeviceSize,
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
);
/// [`vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2EXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
#[deprecated(note = "superseded by `vkCmdBindVertexBuffers3KHR`")]
pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;
/// [`vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBindingInfos`: len: bindingCount
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdBindVertexBuffers3KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstBinding: u32,
    bindingCount: u32,
    pBindingInfos: *const VkBindVertexBuffer3InfoKHR,
);
/// [`vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcImage`
/// - `srcImageLayout`
/// - `dstImage`
/// - `dstImageLayout`
/// - `regionCount`
/// - `pRegions`: len: regionCount
/// - `filter`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageBlit,
    filter: VkFilter,
);
/// [`vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pBlitImageInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBlitImageInfo: *const VkBlitImageInfo2,
);
/// [`vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdBlitImage2KHR = PFN_vkCmdBlitImage2;
/// [`vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
/// - `instanceData`: optional: true
/// - `instanceOffset`
/// - `update`
/// - `dst`
/// - `src`: optional: true
/// - `scratch`
/// - `scratchOffset`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkAccelerationStructureInfoNV,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
);
/// [`vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `infoCount`
/// - `pInfos`: len: infoCount
/// - `pIndirectDeviceAddresses`: len: infoCount
/// - `pIndirectStrides`: len: infoCount
/// - `ppMaxPrimitiveCounts`: len: infoCount
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pIndirectDeviceAddresses: *const VkDeviceAddress,
    pIndirectStrides: *const u32,
    ppMaxPrimitiveCounts: *const *const u32,
);
/// [`vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `infoCount`
/// - `pInfos`: len: infoCount
/// - `ppBuildRangeInfos`: len: infoCount
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
);
/// [`vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html)
///
/// Provided by:
/// - `VK_NV_cluster_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCommandInfos`
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCommandInfos: *const VkClusterAccelerationStructureCommandsInfoNV,
);
/// [`vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `infoCount`
/// - `pInfos`: len: infoCount
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkMicromapBuildInfoEXT,
);
/// [`vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html)
///
/// Provided by:
/// - `VK_NV_partitioned_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pBuildInfo`
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pBuildInfo: *const VkBuildPartitionedAccelerationStructureInfoNV,
);
/// [`vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `attachmentCount`
/// - `pAttachments`: len: attachmentCount
/// - `rectCount`
/// - `pRects`: len: rectCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pAttachments: *const VkClearAttachment,
    rectCount: u32,
    pRects: *const VkClearRect,
);
/// [`vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `image`
/// - `imageLayout`
/// - `pColor`
/// - `rangeCount`
/// - `pRanges`: len: rangeCount
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: *const VkClearColorValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
);
/// [`vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `image`
/// - `imageLayout`
/// - `pDepthStencil`
/// - `rangeCount`
/// - `pRanges`: len: rangeCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: *const VkClearDepthStencilValue,
    rangeCount: u32,
    pRanges: *const VkImageSubresourceRange,
);
/// [`vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
/// - **Queues:** VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCodingControlInfo`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCodingControlInfo: *const VkVideoCodingControlInfoKHR,
);
/// [`vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html)
///
/// Provided by:
/// - `VK_NV_cooperative_vector`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `infoCount`
/// - `pInfos`: len: infoCount
#[cfg(feature = "VK_NV_cooperative_vector")]
pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    infoCount: u32,
    pInfos: *const VkConvertCooperativeVectorMatrixInfoNV,
);
/// [`vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
);
/// [`vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `dst`
/// - `src`
/// - `mode`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
);
/// [`vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
);
/// [`vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcBuffer`
/// - `dstBuffer`
/// - `regionCount`
/// - `pRegions`: len: regionCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferCopy,
);
/// [`vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyBufferInfo`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `vkCmdCopyMemoryKHR`")]
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyBufferInfo: *const VkCopyBufferInfo2,
);
/// [`vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyBuffer2KHR = PFN_vkCmdCopyBuffer2;
/// [`vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcBuffer`
/// - `dstImage`
/// - `dstImageLayout`
/// - `regionCount`
/// - `pRegions`: len: regionCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
);
/// [`vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyBufferToImageInfo`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `vkCmdCopyMemoryToImageKHR`")]
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2,
);
/// [`vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyBufferToImage2KHR = PFN_vkCmdCopyBufferToImage2;
/// [`vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcImage`
/// - `srcImageLayout`
/// - `dstImage`
/// - `dstImageLayout`
/// - `regionCount`
/// - `pRegions`: len: regionCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageCopy,
);
/// [`vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyImageInfo`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyImageInfo: *const VkCopyImageInfo2,
);
/// [`vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyImage2KHR = PFN_vkCmdCopyImage2;
/// [`vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcImage`
/// - `srcImageLayout`
/// - `dstBuffer`
/// - `regionCount`
/// - `pRegions`: len: regionCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    regionCount: u32,
    pRegions: *const VkBufferImageCopy,
);
/// [`vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyImageToBufferInfo`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
#[deprecated(note = "superseded by `vkCmdCopyImageToMemoryKHR`")]
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2,
);
/// [`vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyImageToBuffer2KHR = PFN_vkCmdCopyImageToBuffer2;
/// [`vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyMemoryInfo`: optional: true
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdCopyImageToMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR,
);
/// [`vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_memory_indirect`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyMemoryIndirectInfo`
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
pub type PFN_vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyMemoryIndirectInfo: *const VkCopyMemoryIndirectInfoKHR,
);
/// [`vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html)
///
/// Provided by:
/// - `VK_NV_copy_memory_indirect`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `copyBufferAddress`
/// - `copyCount`
/// - `stride`
#[cfg(feature = "VK_NV_copy_memory_indirect")]
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
);
/// [`vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyMemoryInfo`: optional: true
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdCopyMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyMemoryInfo: *const VkCopyDeviceMemoryInfoKHR,
);
/// [`vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
);
/// [`vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_memory_indirect`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyMemoryToImageIndirectInfo`
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyMemoryToImageIndirectInfo: *const VkCopyMemoryToImageIndirectInfoKHR,
);
/// [`vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html)
///
/// Provided by:
/// - `VK_NV_copy_memory_indirect`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `copyBufferAddress`
/// - `copyCount`
/// - `stride`
/// - `dstImage`
/// - `dstImageLayout`
/// - `pImageSubresources`: len: copyCount
#[cfg(feature = "VK_NV_copy_memory_indirect")]
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: *const VkImageSubresourceLayers,
);
/// [`vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyMemoryInfo`: optional: true
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdCopyMemoryToImageKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR,
);
/// [`vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
);
/// [`vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCmdCopyMicromapEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyMicromapInfoEXT);
/// [`vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
);
/// [`vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `firstQuery`
/// - `queryCount`
/// - `dstBuffer`
/// - `dstOffset`
/// - `stride`
/// - `flags`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdCopyQueryPoolResultsToMemoryKHR`")]
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
);
/// [`vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `firstQuery`
/// - `queryCount`
/// - `pDstRange`
/// - `dstFlags`: optional: true
/// - `queryResultFlags`: optional: true
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    pDstRange: *const VkStridedDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    queryResultFlags: VkQueryResultFlags,
);
/// [`vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html)
///
/// Provided by:
/// - `VK_ARM_tensors`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCopyTensorInfo`
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCopyTensorInfo: *const VkCopyTensorInfoARM,
);
/// [`vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html)
///
/// Provided by:
/// - `VK_NVX_binary_import`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pLaunchInfo`
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCuLaunchInfoNVX,
);
/// [`vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html)
///
/// Provided by:
/// - `VK_NV_cuda_kernel_launch`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pLaunchInfo`
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkCmdCudaLaunchKernelNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pLaunchInfo: *const VkCudaLaunchInfoNV,
);
/// [`vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_marker`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pMarkerInfo`
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
);
/// [`vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_marker`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_marker`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pMarkerInfo`
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT,
);
/// [`vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_decode_queue`
///
/// - **Queues:** VideoDecodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pDecodeInfo`
#[cfg(feature = "VK_KHR_video_decode_queue")]
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDecodeInfo: *const VkVideoDecodeInfoKHR,
);
/// [`vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html)
///
/// Provided by:
/// - `VK_EXT_memory_decompression`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pDecompressMemoryInfoEXT`
#[cfg(feature = "VK_EXT_memory_decompression")]
pub type PFN_vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDecompressMemoryInfoEXT: *const VkDecompressMemoryInfoEXT,
);
/// [`vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html)
///
/// Provided by:
/// - `VK_EXT_memory_decompression`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `decompressionMethod`
/// - `indirectCommandsAddress`
/// - `indirectCommandsCountAddress`
/// - `maxDecompressionCount`
/// - `stride`
#[cfg(feature = "VK_EXT_memory_decompression")]
pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    decompressionMethod: VkMemoryDecompressionMethodFlagsEXT,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    maxDecompressionCount: u32,
    stride: u32,
);
/// [`vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html)
///
/// Provided by:
/// - `VK_NV_memory_decompression`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `indirectCommandsAddress`
/// - `indirectCommandsCountAddress`
/// - `stride`
#[cfg(feature = "VK_NV_memory_decompression")]
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
);
/// [`vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html)
///
/// Provided by:
/// - `VK_NV_memory_decompression`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `decompressRegionCount`
/// - `pDecompressMemoryRegions`: len: decompressRegionCount
#[cfg(feature = "VK_NV_memory_decompression")]
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    decompressRegionCount: u32,
    pDecompressMemoryRegions: *const VkDecompressMemoryRegionNV,
);
/// [`vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `groupCountX`
/// - `groupCountY`
/// - `groupCountZ`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
/// [`vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_1`
///
/// - **Queues:** Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `baseGroupX`
/// - `baseGroupY`
/// - `baseGroupZ`
/// - `groupCountX`
/// - `groupCountY`
/// - `groupCountZ`
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
/// [`vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBaseKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
///
#[cfg(feature = "VK_KHR_device_group")]
pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;
/// [`vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html)
///
/// Provided by:
/// - `VK_ARM_data_graph`
///
/// - **Queues:** DataGraphArm
/// - **Render Pass:** Outside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `session`
/// - `pInfo`: optional: true
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    session: VkDataGraphPipelineSessionARM,
    pInfo: *const VkDataGraphPipelineDispatchInfoARM,
);
/// [`vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html)
///
/// Provided by:
/// - `VK_AMDX_shader_enqueue`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `scratch`
/// - `scratchSize`
/// - `pCountInfo`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    pCountInfo: *const VkDispatchGraphCountInfoAMDX,
);
/// [`vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html)
///
/// Provided by:
/// - `VK_AMDX_shader_enqueue`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `scratch`
/// - `scratchSize`
/// - `pCountInfo`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    pCountInfo: *const VkDispatchGraphCountInfoAMDX,
);
/// [`vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html)
///
/// Provided by:
/// - `VK_AMDX_shader_enqueue`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `scratch`
/// - `scratchSize`
/// - `countInfo`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    countInfo: VkDeviceAddress,
);
/// [`vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdDispatchIndirect2KHR`")]
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
);
/// [`vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDispatchIndirect2KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkDispatchIndirect2InfoKHR,
);
/// [`vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_tile_shading`
///
/// - **Queues:** Compute
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pDispatchTileInfo`
#[cfg(feature = "VK_QCOM_tile_shading")]
pub type PFN_vkCmdDispatchTileQCOM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDispatchTileInfo: *const VkDispatchTileInfoQCOM,
);
/// [`vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `vertexCount`
/// - `instanceCount`
/// - `firstVertex`
/// - `firstInstance`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
);
/// [`vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html)
///
/// Provided by:
/// - `VK_HUAWEI_cluster_culling_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `groupCountX`
/// - `groupCountY`
/// - `groupCountZ`
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
/// [`vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html)
///
/// Provided by:
/// - `VK_HUAWEI_cluster_culling_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
pub type PFN_vkCmdDrawClusterIndirectHUAWEI = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
);
/// [`vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `indexCount`
/// - `instanceCount`
/// - `firstIndex`
/// - `vertexOffset`
/// - `firstInstance`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
);
/// [`vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `drawCount`
/// - `stride`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdDrawIndexedIndirect2KHR`")]
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawIndexedIndirect2KHR =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkDrawIndirect2InfoKHR);
/// [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_2`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[deprecated(note = "superseded by `vkCmdDrawIndexedIndirectCount2KHR`")]
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkDrawIndirectCount2InfoKHR,
);
/// [`vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html)
///
/// Provided by:
/// - `VK_AMD_draw_indirect_count`
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = PFN_vkCmdDrawIndexedIndirectCount;
/// [`vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html)
///
/// Provided by:
/// - `VK_KHR_draw_indirect_count`
///
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = PFN_vkCmdDrawIndexedIndirectCount;
/// [`vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `drawCount`
/// - `stride`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdDrawIndirect2KHR`")]
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawIndirect2KHR =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkDrawIndirect2InfoKHR);
/// [`vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `instanceCount`
/// - `firstInstance`
/// - `pCounterInfo`
/// - `counterOffset`
/// - `vertexStride`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawIndirectByteCount2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    pCounterInfo: *const VkBindTransformFeedbackBuffer2InfoEXT,
    counterOffset: u32,
    vertexStride: u32,
);
/// [`vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `instanceCount`
/// - `firstInstance`
/// - `counterBuffer`
/// - `counterBufferOffset`
/// - `counterOffset`
/// - `vertexStride`
#[cfg(feature = "VK_EXT_transform_feedback")]
#[deprecated(note = "superseded by `vkCmdDrawIndirectByteCount2EXT`")]
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
);
/// [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_2`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
#[deprecated(note = "superseded by `vkCmdDrawIndirectCount2KHR`")]
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawIndirectCount2KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkDrawIndirectCount2InfoKHR,
);
/// [`vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html)
///
/// Provided by:
/// - `VK_AMD_draw_indirect_count`
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndirectCountAMD = PFN_vkCmdDrawIndirectCount;
/// [`vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html)
///
/// Provided by:
/// - `VK_KHR_draw_indirect_count`
///
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub type PFN_vkCmdDrawIndirectCountKHR = PFN_vkCmdDrawIndirectCount;
/// [`vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html)
///
/// Provided by:
/// - `VK_EXT_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `groupCountX`
/// - `groupCountY`
/// - `groupCountZ`
#[cfg(feature = "VK_EXT_mesh_shader")]
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
);
/// [`vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawMeshTasksIndirect2EXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkDrawIndirect2InfoKHR);
/// [`vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInfo: *const VkDrawIndirectCount2InfoKHR,
);
/// [`vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html)
///
/// Provided by:
/// - `VK_EXT_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(feature = "VK_EXT_mesh_shader")]
#[deprecated(note = "superseded by `vkCmdDrawMeshTasksIndirectCount2EXT`")]
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html)
///
/// Provided by:
/// - `VK_NV_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(feature = "VK_NV_mesh_shader")]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html)
///
/// Provided by:
/// - `VK_EXT_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `drawCount`
/// - `stride`
#[cfg(feature = "VK_EXT_mesh_shader")]
#[deprecated(note = "superseded by `vkCmdDrawMeshTasksIndirect2EXT`")]
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html)
///
/// Provided by:
/// - `VK_NV_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `drawCount`
/// - `stride`
#[cfg(feature = "VK_NV_mesh_shader")]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
);
/// [`vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html)
///
/// Provided by:
/// - `VK_NV_mesh_shader`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `taskCount`
/// - `firstTask`
#[cfg(feature = "VK_NV_mesh_shader")]
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, taskCount: u32, firstTask: u32);
/// [`vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html)
///
/// Provided by:
/// - `VK_EXT_multi_draw`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `drawCount`: optional: true
/// - `pVertexInfo`: len: drawCount
/// - `instanceCount`
/// - `firstInstance`
/// - `stride`
#[cfg(feature = "VK_EXT_multi_draw")]
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pVertexInfo: *const VkMultiDrawInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
);
/// [`vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html)
///
/// Provided by:
/// - `VK_EXT_multi_draw`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `drawCount`: optional: true
/// - `pIndexInfo`: len: drawCount
/// - `instanceCount`
/// - `firstInstance`
/// - `stride`
/// - `pVertexOffset`: optional: true
#[cfg(feature = "VK_EXT_multi_draw")]
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    drawCount: u32,
    pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
);
/// [`vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_encode_queue`
///
/// - **Queues:** VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pEncodeInfo`
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pEncodeInfo: *const VkVideoEncodeInfoKHR,
);
/// [`vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html)
///
/// Provided by:
/// - `VK_EXT_conditional_rendering`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_EXT_conditional_rendering")]
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_tile_shading`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pPerTileEndInfo`
#[cfg(feature = "VK_QCOM_tile_shading")]
pub type PFN_vkCmdEndPerTileExecutionQCOM = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPerTileEndInfo: *const VkPerTileEndInfoQCOM,
);
/// [`vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `query`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdEndQuery =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
/// [`vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `query`
/// - `index`
#[cfg(feature = "VK_EXT_transform_feedback")]
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    query: u32,
    index: u32,
);
/// [`vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdEndRenderPass2`")]
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_2`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pSubpassEndInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pSubpassEndInfo: *const VkSubpassEndInfo,
);
/// [`vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;
/// [`vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2EXT.html)
///
/// Provided by:
/// - `VK_EXT_fragment_density_map_offset`
///
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;
/// [`vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance10`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pRenderingEndInfo`: optional: true
#[cfg(feature = "VK_KHR_maintenance10")]
pub type PFN_vkCmdEndRendering2KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRenderingEndInfo: *const VkRenderingEndInfoKHR,
);
/// [`vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering`
///
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;
/// [`vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html)
///
/// Provided by:
/// - `VK_ARM_shader_instrumentation`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkCmdEndShaderInstrumentationARM =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterRange`
/// - `counterRangeCount`: optional: true
/// - `pCounterInfos`: optional: true, len: counterRangeCount
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdEndTransformFeedback2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
);
/// [`vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html)
///
/// Provided by:
/// - `VK_EXT_transform_feedback`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterBuffer`
/// - `counterBufferCount`: optional: true
/// - `pCounterBuffers`: len: counterBufferCount
/// - `pCounterBufferOffsets`: optional: true, len: counterBufferCount
#[cfg(feature = "VK_EXT_transform_feedback")]
#[deprecated(note = "superseded by `vkCmdEndTransformFeedback2EXT`")]
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstCounterBuffer: u32,
    counterBufferCount: u32,
    pCounterBuffers: *const VkBuffer,
    pCounterBufferOffsets: *const VkDeviceSize,
);
/// [`vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
/// - **Queues:** VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pEndCodingInfo`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pEndCodingInfo: *const VkVideoEndCodingInfoKHR,
);
/// [`vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Indirection
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `commandBufferCount`
/// - `pCommandBuffers`: len: commandBufferCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
);
/// [`vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action, Indirection
///
/// # Parameters
/// - `commandBuffer`
/// - `isPreprocessed`
/// - `pGeneratedCommandsInfo`
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT,
);
/// [`vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html)
///
/// Provided by:
/// - `VK_NV_device_generated_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action, Indirection
///
/// # Parameters
/// - `commandBuffer`
/// - `isPreprocessed`
/// - `pGeneratedCommandsInfo`
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
);
/// [`vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///transfer support is only available when VK_KHR_maintenance1 is enabled, as documented in valid usage language in the specification
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `dstBuffer`
/// - `dstOffset`
/// - `size`
/// - `data`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdFillMemoryKHR`")]
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
);
/// [`vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pDstRange`
/// - `dstFlags`: optional: true
/// - `data`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdFillMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDstRange: *const VkDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    data: u32,
);
/// [`vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html)
///
/// Provided by:
/// - `VK_AMDX_shader_enqueue`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `executionGraph`
/// - `scratch`
/// - `scratchSize`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    executionGraph: VkPipeline,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
);
/// [`vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pLabelInfo`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pLabelInfo: *const VkDebugUtilsLabelEXT,
);
/// [`vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `contents`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdNextSubpass2`")]
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
/// [`vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_2`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action, State, Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pSubpassBeginInfo`
/// - `pSubpassEndInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pSubpassBeginInfo: *const VkSubpassBeginInfo,
    pSubpassEndInfo: *const VkSubpassEndInfo,
);
/// [`vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;
/// [`vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html)
///
/// Provided by:
/// - `VK_NV_optical_flow`
///
/// - **Queues:** OpticalFlowNV
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `session`
/// - `pExecuteInfo`
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: *const VkOpticalFlowExecuteInfoNV,
);
/// [`vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcStageMask`: optional: true
/// - `dstStageMask`: optional: true
/// - `dependencyFlags`: optional: true
/// - `memoryBarrierCount`: optional: true
/// - `pMemoryBarriers`: len: memoryBarrierCount
/// - `bufferMemoryBarrierCount`: optional: true
/// - `pBufferMemoryBarriers`: len: bufferMemoryBarrierCount
/// - `imageMemoryBarrierCount`: optional: true
/// - `pImageMemoryBarriers`: len: imageMemoryBarrierCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdPipelineBarrier2`")]
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
);
/// [`vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pDependencyInfo`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDependencyInfo: *const VkDependencyInfo,
);
/// [`vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdPipelineBarrier2KHR = PFN_vkCmdPipelineBarrier2;
/// [`vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pGeneratedCommandsInfo`
/// - `stateCommandBuffer`
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT,
    stateCommandBuffer: VkCommandBuffer,
);
/// [`vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html)
///
/// Provided by:
/// - `VK_NV_device_generated_commands`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pGeneratedCommandsInfo`
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV,
);
/// [`vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `layout`
/// - `stageFlags`
/// - `offset`
/// - `size`
/// - `pValues`: len: size
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const core::ffi::c_void,
);
/// [`vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pPushConstantsInfo`
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPushConstantsInfo: *const VkPushConstantsInfo,
);
/// [`vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
/// [`vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pPushDataInfo`
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkCmdPushDataEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPushDataInfo: *const VkPushDataInfoEXT,
);
/// [`vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `layout`
/// - `set`
/// - `descriptorWriteCount`
/// - `pDescriptorWrites`: len: descriptorWriteCount
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
);
/// [`vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pPushDescriptorSetInfo`
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo,
);
/// [`vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
/// [`vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html)
///
/// Provided by:
/// - `VK_KHR_push_descriptor`
///
#[cfg(feature = "VK_KHR_push_descriptor")]
pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;
/// [`vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `descriptorUpdateTemplate`
/// - `layout`
/// - `set`
/// - `pData`
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const core::ffi::c_void,
);
/// [`vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_4`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pPushDescriptorSetWithTemplateInfo`
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo,
);
/// [`vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;
/// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
/// - `VK_KHR_push_descriptor`
///
#[cfg(any(
    feature = "VK_KHR_descriptor_update_template",
    feature = "VK_KHR_push_descriptor"
))]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;
/// [`vkCmdRefreshObjectsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdRefreshObjectsKHR.html)
///
/// Provided by:
/// - `VK_KHR_object_refresh`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pRefreshObjects`
#[cfg(feature = "VK_KHR_object_refresh")]
pub type PFN_vkCmdRefreshObjectsKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRefreshObjects: *const VkRefreshObjectListKHR,
);
/// [`vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `event`
/// - `stageMask`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdResetEvent2`")]
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
);
/// [`vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_3`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `event`
/// - `stageMask`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags2,
);
/// [`vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdResetEvent2KHR = PFN_vkCmdResetEvent2;
/// [`vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `queryPool`
/// - `firstQuery`
/// - `queryCount`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
);
/// [`vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `srcImage`
/// - `srcImageLayout`
/// - `dstImage`
/// - `dstImageLayout`
/// - `regionCount`
/// - `pRegions`: len: regionCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    regionCount: u32,
    pRegions: *const VkImageResolve,
);
/// [`vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pResolveImageInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pResolveImageInfo: *const VkResolveImageInfo2,
);
/// [`vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdResolveImage2KHR = PFN_vkCmdResolveImage2;
/// [`vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `alphaToCoverageEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, alphaToCoverageEnable: VkBool32);
/// [`vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `alphaToOneEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, alphaToOneEnable: VkBool32);
/// [`vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_attachment_feedback_loop_dynamic_state`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `aspectMask`: optional: true
#[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, aspectMask: VkImageAspectFlags);
/// [`vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `blendConstants`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]);
/// [`vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html)
///
/// Provided by:
/// - `VK_NV_device_diagnostic_checkpoints`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pCheckpointMarker`
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub type PFN_vkCmdSetCheckpointNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pCheckpointMarker: *const core::ffi::c_void,
);
/// [`vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html)
///
/// Provided by:
/// - `VK_NV_shading_rate_image`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `sampleOrderType`
/// - `customSampleOrderCount`: optional: true
/// - `pCustomSampleOrders`: len: customSampleOrderCount
#[cfg(feature = "VK_NV_shading_rate_image")]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    customSampleOrderCount: u32,
    pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
);
/// [`vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstAttachment`
/// - `attachmentCount`
/// - `pColorBlendAdvanced`: len: attachmentCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendAdvanced: *const VkColorBlendAdvancedEXT,
);
/// [`vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstAttachment`
/// - `attachmentCount`
/// - `pColorBlendEnables`: len: attachmentCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEnables: *const VkBool32,
);
/// [`vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstAttachment`
/// - `attachmentCount`
/// - `pColorBlendEquations`: len: attachmentCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorBlendEquations: *const VkColorBlendEquationEXT,
);
/// [`vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_color_write_enable`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `attachmentCount`
/// - `pColorWriteEnables`: len: attachmentCount
#[cfg(feature = "VK_EXT_color_write_enable")]
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    attachmentCount: u32,
    pColorWriteEnables: *const VkBool32,
);
/// [`vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstAttachment`
/// - `attachmentCount`
/// - `pColorWriteMasks`: optional: pointer required, values optional if pointer not null, len: attachmentCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
);
/// [`vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html)
///
/// Provided by:
/// - `VK_NV_compute_occupancy_priority`
///
/// - **Queues:** Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pParameters`
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pParameters: *const VkComputeOccupancyPriorityParametersNV,
);
/// [`vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `conservativeRasterizationMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
);
/// [`vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    coverageModulationMode: VkCoverageModulationModeNV,
);
/// [`vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationTableEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableEnable: VkBool32,
);
/// [`vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationTableCount`
/// - `pCoverageModulationTable`: len: coverageModulationTableCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    coverageModulationTableCount: u32,
    pCoverageModulationTable: *const f32,
);
/// [`vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageReductionMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    coverageReductionMode: VkCoverageReductionModeNV,
);
/// [`vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageToColorEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageToColorEnableNV =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, coverageToColorEnable: VkBool32);
/// [`vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageToColorLocation`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCoverageToColorLocationNV =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, coverageToColorLocation: u32);
/// [`vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `cullMode`: optional: true
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
/// [`vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;
/// [`vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `depthBiasConstantFactor`
/// - `depthBiasClamp`
/// - `depthBiasSlopeFactor`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
);
/// [`vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html)
///
/// Provided by:
/// - `VK_EXT_depth_bias_control`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pDepthBiasInfo`
#[cfg(feature = "VK_EXT_depth_bias_control")]
pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDepthBiasInfo: *const VkDepthBiasInfoEXT,
);
/// [`vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `depthBiasEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
/// [`vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state2`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;
/// [`vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `minDepthBounds`
/// - `maxDepthBounds`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    minDepthBounds: f32,
    maxDepthBounds: f32,
);
/// [`vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `depthBoundsTestEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
/// [`vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;
/// [`vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `depthClampEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthClampEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthClampEnable: VkBool32);
/// [`vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html)
///
/// Provided by:
/// - `VK_EXT_depth_clamp_control`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `depthClampMode`
/// - `pDepthClampRange`: optional: true
#[cfg(any(
    feature = "VK_EXT_depth_clamp_control",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    depthClampMode: VkDepthClampModeEXT,
    pDepthClampRange: *const VkDepthClampRangeEXT,
);
/// [`vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `depthClipEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthClipEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthClipEnable: VkBool32);
/// [`vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `negativeOneToOne`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, negativeOneToOne: VkBool32);
/// [`vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `depthCompareOp`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
/// [`vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOpEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;
/// [`vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `depthTestEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
/// [`vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;
/// [`vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `depthWriteEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
/// [`vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;
/// [`vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pSetDescriptorBufferOffsetsInfo`
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pSetDescriptorBufferOffsetsInfo: *const VkSetDescriptorBufferOffsetsInfoEXT,
);
/// [`vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `layout`
/// - `firstSet`
/// - `setCount`
/// - `pBufferIndices`: len: setCount
/// - `pOffsets`: len: setCount
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    setCount: u32,
    pBufferIndices: *const u32,
    pOffsets: *const VkDeviceSize,
);
/// [`vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_1`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `deviceMask`
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
/// [`vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMaskKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
///
#[cfg(feature = "VK_KHR_device_group")]
pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;
/// [`vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html)
///
/// Provided by:
/// - `VK_EXT_discard_rectangles`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstDiscardRectangle`
/// - `discardRectangleCount`
/// - `pDiscardRectangles`: len: discardRectangleCount
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstDiscardRectangle: u32,
    discardRectangleCount: u32,
    pDiscardRectangles: *const VkRect2D,
);
/// [`vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_discard_rectangles`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `discardRectangleEnable`
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, discardRectangleEnable: VkBool32);
/// [`vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_discard_rectangles`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `discardRectangleMode`
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    discardRectangleMode: VkDiscardRectangleModeEXT,
);
/// [`vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `event`
/// - `stageMask`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdSetEvent2`")]
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    stageMask: VkPipelineStageFlags,
);
/// [`vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_3`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Outside
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `event`
/// - `pDependencyInfo`
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    event: VkEvent,
    pDependencyInfo: *const VkDependencyInfo,
);
/// [`vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdSetEvent2KHR = PFN_vkCmdSetEvent2;
/// [`vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html)
///
/// Provided by:
/// - `VK_NV_scissor_exclusive`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstExclusiveScissor`
/// - `exclusiveScissorCount`
/// - `pExclusiveScissorEnables`: len: exclusiveScissorCount
#[cfg(feature = "VK_NV_scissor_exclusive")]
pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissorEnables: *const VkBool32,
);
/// [`vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html)
///
/// Provided by:
/// - `VK_NV_scissor_exclusive`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstExclusiveScissor`
/// - `exclusiveScissorCount`
/// - `pExclusiveScissors`: len: exclusiveScissorCount
#[cfg(feature = "VK_NV_scissor_exclusive")]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstExclusiveScissor: u32,
    exclusiveScissorCount: u32,
    pExclusiveScissors: *const VkRect2D,
);
/// [`vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `extraPrimitiveOverestimationSize`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    extraPrimitiveOverestimationSize: f32,
);
/// [`vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html)
///
/// Provided by:
/// - `VK_NV_fragment_shading_rate_enums`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `shadingRate`
/// - `combinerOps`
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
);
/// [`vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html)
///
/// Provided by:
/// - `VK_KHR_fragment_shading_rate`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pFragmentSize`
/// - `combinerOps`
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pFragmentSize: *const VkExtent2D,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
);
/// [`vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `frontFace`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
/// [`vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFaceEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;
/// [`vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `lineRasterizationMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    lineRasterizationMode: VkLineRasterizationModeEXT,
);
/// [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_4`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `lineStippleFactor`
/// - `lineStipplePattern`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    lineStippleFactor: u32,
    lineStipplePattern: u16,
);
/// [`vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEXT.html)
///
/// Provided by:
/// - `VK_EXT_line_rasterization`
///
#[cfg(feature = "VK_EXT_line_rasterization")]
pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;
/// [`vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `stippledLineEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetLineStippleEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stippledLineEnable: VkBool32);
/// [`vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html)
///
/// Provided by:
/// - `VK_KHR_line_rasterization`
///
#[cfg(feature = "VK_KHR_line_rasterization")]
pub type PFN_vkCmdSetLineStippleKHR = PFN_vkCmdSetLineStipple;
/// [`vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `lineWidth`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);
/// [`vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state2`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `logicOp`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp);
/// [`vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `logicOpEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetLogicOpEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, logicOpEnable: VkBool32);
/// [`vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state2`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `patchControlPoints`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, patchControlPoints: u32);
/// [`vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pMarkerInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceMarkerInfoINTEL,
) -> VkResult;
/// [`vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pOverrideInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pOverrideInfo: *const VkPerformanceOverrideInfoINTEL,
) -> VkResult;
/// [`vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `pMarkerInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL,
) -> VkResult;
/// [`vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `polygonMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetPolygonModeEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, polygonMode: VkPolygonMode);
/// [`vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `primitiveRestartEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
/// [`vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state2`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;
/// [`vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `primitiveTopology`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    primitiveTopology: VkPrimitiveTopology,
);
/// [`vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopologyEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;
/// [`vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `provokingVertexMode`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    provokingVertexMode: VkProvokingVertexModeEXT,
);
/// [`vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `rasterizationSamples`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    rasterizationSamples: VkSampleCountFlagBits,
);
/// [`vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `rasterizationStream`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetRasterizationStreamEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, rasterizationStream: u32);
/// [`vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `rasterizerDiscardEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
/// [`vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state2`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;
/// [`vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html)
///
/// Provided by:
/// - `VK_KHR_ray_tracing_pipeline`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineStackSize`
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pipelineStackSize: u32);
/// [`vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_4`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pLocationInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pLocationInfo: *const VkRenderingAttachmentLocationInfo,
);
/// [`vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering_local_read`
///
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;
/// [`vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_4`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `pInputAttachmentIndexInfo`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo,
);
/// [`vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering_local_read`
///
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
    PFN_vkCmdSetRenderingInputAttachmentIndices;
/// [`vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `representativeFragmentTestEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    representativeFragmentTestEnable: VkBool32,
);
/// [`vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html)
///
/// Provided by:
/// - `VK_EXT_sample_locations`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pSampleLocationsInfo`
#[cfg(feature = "VK_EXT_sample_locations")]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pSampleLocationsInfo: *const VkSampleLocationsInfoEXT,
);
/// [`vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `sampleLocationsEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, sampleLocationsEnable: VkBool32);
/// [`vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `samples`
/// - `pSampleMask`: optional: true, len: latexmath:[\lceil{\mathit{samples} \over 32}\rceil]
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
);
/// [`vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `firstScissor`
/// - `scissorCount`
/// - `pScissors`: len: scissorCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstScissor: u32,
    scissorCount: u32,
    pScissors: *const VkRect2D,
);
/// [`vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `scissorCount`
/// - `pScissors`: len: scissorCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    scissorCount: u32,
    pScissors: *const VkRect2D,
);
/// [`vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCountEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;
/// [`vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `shadingRateImageEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetShadingRateImageEnableNV =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, shadingRateImageEnable: VkBool32);
/// [`vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `faceMask`
/// - `compareMask`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    compareMask: u32,
);
/// [`vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `faceMask`
/// - `failOp`
/// - `passOp`
/// - `depthFailOp`
/// - `compareOp`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
);
/// [`vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOpEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;
/// [`vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `faceMask`
/// - `reference`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    reference: u32,
);
/// [`vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `stencilTestEnable`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
/// [`vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;
/// [`vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `faceMask`
/// - `writeMask`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    faceMask: VkStencilFaceFlags,
    writeMask: u32,
);
/// [`vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `domainOrigin`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    domainOrigin: VkTessellationDomainOrigin,
);
/// [`vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html)
///
/// Provided by:
/// - `VK_EXT_shader_object`
/// - `VK_EXT_vertex_input_dynamic_state`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `vertexBindingDescriptionCount`: optional: true
/// - `pVertexBindingDescriptions`: len: vertexBindingDescriptionCount
/// - `vertexAttributeDescriptionCount`: optional: true
/// - `pVertexAttributeDescriptions`: len: vertexAttributeDescriptionCount
#[cfg(any(
    feature = "VK_EXT_shader_object",
    feature = "VK_EXT_vertex_input_dynamic_state"
))]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    vertexBindingDescriptionCount: u32,
    pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
    vertexAttributeDescriptionCount: u32,
    pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
);
/// [`vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `firstViewport`
/// - `viewportCount`
/// - `pViewports`: len: viewportCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewports: *const VkViewport,
);
/// [`vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html)
///
/// Provided by:
/// - `VK_NV_shading_rate_image`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstViewport`
/// - `viewportCount`
/// - `pShadingRatePalettes`: len: viewportCount
#[cfg(feature = "VK_NV_shading_rate_image")]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pShadingRatePalettes: *const VkShadingRatePaletteNV,
);
/// [`vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstViewport`
/// - `viewportCount`
/// - `pViewportSwizzles`: len: viewportCount
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportSwizzles: *const VkViewportSwizzleNV,
);
/// [`vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `viewportWScalingEnable`
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetViewportWScalingEnableNV =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, viewportWScalingEnable: VkBool32);
/// [`vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html)
///
/// Provided by:
/// - `VK_NV_clip_space_w_scaling`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `firstViewport`
/// - `viewportCount`
/// - `pViewportWScalings`: len: viewportCount
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    firstViewport: u32,
    viewportCount: u32,
    pViewportWScalings: *const VkViewportWScalingNV,
);
/// [`vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_3`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `viewportCount`
/// - `pViewports`: len: viewportCount
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    viewportCount: u32,
    pViewports: *const VkViewport,
);
/// [`vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCountEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state`
/// - `VK_EXT_shader_object`
///
#[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;
/// [`vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html)
///
/// Provided by:
/// - `VK_HUAWEI_subpass_shading`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
/// [`vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html)
///
/// Provided by:
/// - `VK_KHR_ray_tracing_maintenance1`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `indirectDeviceAddress`
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    indirectDeviceAddress: VkDeviceAddress,
);
/// [`vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html)
///
/// Provided by:
/// - `VK_KHR_ray_tracing_pipeline`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pRaygenShaderBindingTable`
/// - `pMissShaderBindingTable`
/// - `pHitShaderBindingTable`
/// - `pCallableShaderBindingTable`
/// - `indirectDeviceAddress`
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
);
/// [`vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html)
///
/// Provided by:
/// - `VK_KHR_ray_tracing_pipeline`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pRaygenShaderBindingTable`
/// - `pMissShaderBindingTable`
/// - `pHitShaderBindingTable`
/// - `pCallableShaderBindingTable`
/// - `width`
/// - `height`
/// - `depth`
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
/// [`vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `raygenShaderBindingTableBuffer`
/// - `raygenShaderBindingOffset`
/// - `missShaderBindingTableBuffer`: optional: true
/// - `missShaderBindingOffset`
/// - `missShaderBindingStride`
/// - `hitShaderBindingTableBuffer`: optional: true
/// - `hitShaderBindingOffset`
/// - `hitShaderBindingStride`
/// - `callableShaderBindingTableBuffer`: optional: true
/// - `callableShaderBindingOffset`
/// - `callableShaderBindingStride`
/// - `width`
/// - `height`
/// - `depth`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    raygenShaderBindingTableBuffer: VkBuffer,
    raygenShaderBindingOffset: VkDeviceSize,
    missShaderBindingTableBuffer: VkBuffer,
    missShaderBindingOffset: VkDeviceSize,
    missShaderBindingStride: VkDeviceSize,
    hitShaderBindingTableBuffer: VkBuffer,
    hitShaderBindingOffset: VkDeviceSize,
    hitShaderBindingStride: VkDeviceSize,
    callableShaderBindingTableBuffer: VkBuffer,
    callableShaderBindingOffset: VkDeviceSize,
    callableShaderBindingStride: VkDeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
/// [`vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `dstBuffer`
/// - `dstOffset`
/// - `dataSize`
/// - `pData`: len: dataSize
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdUpdateMemoryKHR`")]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const core::ffi::c_void,
);
/// [`vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Transfer
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pDstRange`
/// - `dstFlags`: optional: true
/// - `dataSize`
/// - `pData`: len: dataSize
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdUpdateMemoryKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pDstRange: *const VkDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    dataSize: VkDeviceSize,
    pData: *const core::ffi::c_void,
);
/// [`vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html)
///
/// Provided by:
/// - `VK_NV_device_generated_commands_compute`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineBindPoint`
/// - `pipeline`
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
);
/// [`vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `eventCount`
/// - `pEvents`: len: eventCount
/// - `srcStageMask`: optional: true
/// - `dstStageMask`: optional: true
/// - `memoryBarrierCount`: optional: true
/// - `pMemoryBarriers`: len: memoryBarrierCount
/// - `bufferMemoryBarrierCount`: optional: true
/// - `pBufferMemoryBarriers`: len: bufferMemoryBarrierCount
/// - `imageMemoryBarrierCount`: optional: true
/// - `pImageMemoryBarriers`: len: imageMemoryBarrierCount
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdWaitEvents2`")]
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    memoryBarrierCount: u32,
    pMemoryBarriers: *const VkMemoryBarrier,
    bufferMemoryBarrierCount: u32,
    pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
    imageMemoryBarrierCount: u32,
    pImageMemoryBarriers: *const VkImageMemoryBarrier,
);
/// [`vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_3`
///
/// - **Queues:** Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Synchronization
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `eventCount`
/// - `pEvents`: len: eventCount
/// - `pDependencyInfos`: len: eventCount
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    eventCount: u32,
    pEvents: *const VkEvent,
    pDependencyInfos: *const VkDependencyInfo,
);
/// [`vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdWaitEvents2KHR = PFN_vkCmdWaitEvents2;
/// [`vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_acceleration_structure`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `accelerationStructureCount`
/// - `pAccelerationStructures`: len: accelerationStructureCount
/// - `queryType`
/// - `queryPool`
/// - `firstQuery`
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
/// [`vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `accelerationStructureCount`
/// - `pAccelerationStructures`: len: accelerationStructureCount
/// - `queryType`
/// - `queryPool`
/// - `firstQuery`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureNV,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
/// [`vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html)
///
/// Provided by:
/// - `VK_AMD_buffer_marker`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `stage`: optional: true
/// - `dstBuffer`
/// - `dstOffset`
/// - `marker`
#[cfg(feature = "VK_AMD_buffer_marker")]
#[deprecated(note = "superseded by `vkCmdWriteMarkerToMemoryAMD`")]
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
);
/// [`vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html)
///
/// Provided by:
/// - `VK_AMD_buffer_marker`
///
/// - **Queues:** Transfer, Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineStage`: optional: true
/// - `dstBuffer`
/// - `dstOffset`
/// - `marker`
#[cfg(feature = "VK_AMD_buffer_marker")]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
);
/// [`vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCmdWriteMarkerToMemoryAMD =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkMemoryMarkerInfoAMD);
/// [`vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** Action
///
/// # Parameters
/// - `commandBuffer`
/// - `micromapCount`
/// - `pMicromaps`: len: micromapCount
/// - `queryType`
/// - `queryPool`
/// - `firstQuery`
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
);
/// [`vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR, OpticalFlowNV
/// - **Render Pass:** Both
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `pipelineStage`
/// - `queryPool`
/// - `query`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCmdWriteTimestamp2`")]
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
);
/// [`vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
/// - **Render Pass:** Both
/// - **Tasks:** Action
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `commandBuffer`
/// - `stage`: optional: true
/// - `queryPool`
/// - `query`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
);
/// [`vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdWriteTimestamp2KHR = PFN_vkCmdWriteTimestamp2;
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
pub type PFN_vkCompileDeferredNV =
    unsafe extern "system" fn(device: VkDevice, pipeline: VkPipeline, shader: u32) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cooperative_vector")]
pub type PFN_vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkConvertCooperativeVectorMatrixInfoNV,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
    device: VkDevice,
    pCopyImageToImageInfo: *const VkCopyImageToImageInfo,
) -> VkResult;
/// [`vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyImageToImageEXT = PFN_vkCopyImageToImage;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
    device: VkDevice,
    pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo,
) -> VkResult;
/// [`vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyImageToMemoryEXT = PFN_vkCopyImageToMemory;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
    device: VkDevice,
    pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo,
) -> VkResult;
/// [`vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyMemoryToImageEXT = PFN_vkCopyMemoryToImage;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMemoryToMicromapInfoEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapInfoEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pInfo: *const VkCopyMicromapToMemoryInfoEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_VALIDATION_FAILED
///   - VK_ERROR_UNKNOWN
#[cfg(feature = "VK_KHR_device_address_commands")]
pub type PFN_vkCreateAccelerationStructure2KHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfo2KHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
#[deprecated(note = "superseded by `vkCreateAccelerationStructure2KHR`")]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult;
/// [`vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_android_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_android_surface")]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pBuffer: *mut VkBuffer,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pCollection: *mut VkBufferCollectionFUCHSIA,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkBufferViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkBufferView,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkCommandPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pCommandPool: *mut VkCommandPool,
) -> VkResult;
/// [`vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Allow No Queues:** True
/// - **Export Scopes:** Vulkan
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
///   - VK_SUCCESS
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_SHADER_NV
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkComputePipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkCuFunctionCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCuFunctionNVX,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkCuModuleCreateInfoNVX,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCuModuleNVX,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkCudaFunctionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pFunction: *mut VkCudaFunctionNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkCudaModuleCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pModule: *mut VkCudaModuleNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkDataGraphPipelineSessionCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkDataGraphPipelineSessionARM,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkDataGraphPipelineCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
/// [`vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_report`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pCallback`
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
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pCallback: *mut VkDebugReportCallbackEXT,
) -> VkResult;
/// [`vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pMessenger`
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
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMessenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: VkDevice,
    pAllocator: *const VkAllocationCallbacks,
    pDeferredOperation: *mut VkDeferredOperationKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_FRAGMENTATION_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorPool: *mut VkDescriptorPool,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSetLayout: *mut VkDescriptorSetLayout,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;
/// [`vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = PFN_vkCreateDescriptorUpdateTemplate;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_EXTENSION_NOT_PRESENT
///   - VK_ERROR_FEATURE_NOT_PRESENT
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pCreateInfo: *const VkDeviceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pDevice: *mut VkDevice,
) -> VkResult;
/// [`vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html)
///
/// Provided by:
/// - `VK_EXT_directfb_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_EXT_directfb_surface")]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pCreateInfo: *const VkDisplayModeCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pMode: *mut VkDisplayModeKHR,
) -> VkResult;
/// [`vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_display`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkEventCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pEvent: *mut VkEvent,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkExecutionGraphPipelineCreateInfoAMDX,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_compute_queue")]
pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkExternalComputeQueueCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pExternalQueue: *mut VkExternalComputeQueueNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkFenceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkFramebufferCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pFramebuffer: *mut VkFramebuffer,
) -> VkResult;
/// [`vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Allow No Queues:** True
/// - **Export Scopes:** Vulkan
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
///   - VK_SUCCESS
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_SHADER_NV
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkGraphicsPipelineCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
/// [`vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html)
///
/// Provided by:
/// - `VK_EXT_headless_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_EXT_headless_surface")]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html)
///
/// Provided by:
/// - `VK_MVK_ios_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_MVK_ios_surface")]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_COMPRESSION_EXHAUSTED_EXT
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pImage: *mut VkImage,
) -> VkResult;
/// [`vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html)
///
/// Provided by:
/// - `VK_FUCHSIA_imagepipe_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkImageViewCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkImageView,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkIndirectExecutionSetCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pIndirectExecutionSet: *mut VkIndirectExecutionSetEXT,
) -> VkResult;
/// [`vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pInstance`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_LAYER_NOT_PRESENT
///   - VK_ERROR_EXTENSION_NOT_PRESENT
///   - VK_ERROR_INCOMPATIBLE_DRIVER
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    pCreateInfo: *const VkInstanceCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pInstance: *mut VkInstance,
) -> VkResult;
/// [`vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html)
///
/// Provided by:
/// - `VK_MVK_macos_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_MVK_macos_surface")]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html)
///
/// Provided by:
/// - `VK_EXT_metal_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_metal_surface")]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkMicromapCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pMicromap: *mut VkMicromapEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSession: *mut VkOpticalFlowSessionNV,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///   - VK_PIPELINE_BINARY_MISSING_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkPipelineBinaryCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pBinaries: *mut VkPipelineBinaryHandlesInfoKHR,
) -> VkResult;
/// [`vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html)
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
/// - `pPipelineCache`
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
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkPipelineCacheCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineCache: *mut VkPipelineCache,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkPipelineLayoutCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPipelineLayout: *mut VkPipelineLayout,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkPrivateDataSlotCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult;
/// [`vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkQueryPoolCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pQueryPool: *mut VkQueryPool,
) -> VkResult;
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
/// - `pipelineCache`: optional: true
/// - `createInfoCount`
/// - `pCreateInfos`: len: createInfoCount
/// - `pAllocator`: optional: true
/// - `pPipelines`: len: createInfoCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_OPERATION_DEFERRED_KHR
///   - VK_OPERATION_NOT_DEFERRED_KHR
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: VkDevice,
    deferredOperation: VkDeferredOperationKHR,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
/// [`vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
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
///   - VK_SUCCESS
///   - VK_PIPELINE_COMPILE_REQUIRED_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_SHADER_NV
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    createInfoCount: u32,
    pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pPipelines: *mut VkPipeline,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCreateRenderPass2`")]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkRenderPassCreateInfo2,
    pAllocator: *const VkAllocationCallbacks,
    pRenderPass: *mut VkRenderPass,
) -> VkResult;
/// [`vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkSamplerCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSampler: *mut VkSampler,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;
/// [`vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html)
///
/// Provided by:
/// - `VK_KHR_sampler_ycbcr_conversion`
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;
/// [`vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html)
///
/// Provided by:
/// - `VK_QNX_screen_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_QNX_screen_surface")]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkScreenSurfaceCreateInfoQNX,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphore: *mut VkSemaphore,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub type PFN_vkCreateSemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkSemaphoreSciSyncPoolCreateInfoNV,
    pAllocator: *const VkAllocationCallbacks,
    pSemaphorePool: *mut VkSemaphoreSciSyncPoolNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkCreateShaderInstrumentationARM = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkShaderInstrumentationCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pInstrumentation: *mut VkShaderInstrumentationARM,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_SHADER_NV
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pAllocator: *const VkAllocationCallbacks,
    pShaderModule: *mut VkShaderModule,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPATIBLE_SHADER_BINARY_EXT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_shader_object")]
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
    device: VkDevice,
    createInfoCount: u32,
    pCreateInfos: *const VkShaderCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pShaders: *mut VkShaderEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INCOMPATIBLE_DISPLAY_KHR
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display_swapchain")]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchainCount: u32,
    pCreateInfos: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchains: *mut VkSwapchainKHR,
) -> VkResult;
/// [`vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html)
///
/// Provided by:
/// - `VK_GGP_stream_descriptor_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html)
///
/// Provided by:
/// - `VK_OHOS_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_OHOS_surface")]
pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkSurfaceCreateInfoOHOS,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_COMPRESSION_EXHAUSTED_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkSwapchainCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSwapchain: *mut VkSwapchainKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkTensorCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pTensor: *mut VkTensorARM,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkTensorViewCreateInfoARM,
    pAllocator: *const VkAllocationCallbacks,
    pView: *mut VkTensorViewARM,
) -> VkResult;
/// [`vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html)
///
/// Provided by:
/// - `VK_SEC_ubm_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_SEC_ubm_surface")]
pub type PFN_vkCreateUbmSurfaceSEC = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkUbmSurfaceCreateInfoSEC,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkValidationCacheCreateInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pValidationCache: *mut VkValidationCacheEXT,
) -> VkResult;
/// [`vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html)
///
/// Provided by:
/// - `VK_NN_vi_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NN_vi_surface")]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkViSurfaceCreateInfoNN,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR
///   - VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSession: *mut VkVideoSessionKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pVideoSessionParameters: *mut VkVideoSessionParametersKHR,
) -> VkResult;
/// [`vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_wayland_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_KHR_wayland_surface")]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_win32_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_KHR_win32_surface")]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_xcb_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_KHR_xcb_surface")]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
/// [`vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_xlib_surface`
///
///
/// # Parameters
/// - `instance`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pSurface`
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
#[cfg(feature = "VK_KHR_xlib_surface")]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: VkDevice,
    pNameInfo: *const VkDebugMarkerObjectNameInfoEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: VkDevice,
    pTagInfo: *const VkDebugMarkerObjectTagInfoEXT,
) -> VkResult;
/// [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_report`
///
///
/// # Parameters
/// - `instance`
/// - `flags`
/// - `objectType`
/// - `object`: object_type: objectType
/// - `location`
/// - `messageCode`
/// - `pLayerPrefix`: len: null-terminated
/// - `pMessage`: len: null-terminated
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: VkInstance,
    flags: VkDebugReportFlagsEXT,
    objectType: VkDebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    messageCode: i32,
    pLayerPrefix: *const core::ffi::c_char,
    pMessage: *const core::ffi::c_char,
);
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
///   - VK_SUCCESS
///   - VK_THREAD_DONE_KHR
///   - VK_THREAD_IDLE_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;
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
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureKHR,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
///
/// # Parameters
/// - `device`
/// - `accelerationStructure`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `buffer`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html)
///
/// Provided by:
/// - `VK_FUCHSIA_buffer_collection`
///
///
/// # Parameters
/// - `device`
/// - `collection`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `bufferView`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: VkDevice,
    bufferView: VkBufferView,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `commandPool`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html)
///
/// Provided by:
/// - `VK_NVX_binary_import`
///
///
/// # Parameters
/// - `device`
/// - `function`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: VkDevice,
    function: VkCuFunctionNVX,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html)
///
/// Provided by:
/// - `VK_NVX_binary_import`
///
///
/// # Parameters
/// - `device`
/// - `module`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: VkDevice,
    module: VkCuModuleNVX,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html)
///
/// Provided by:
/// - `VK_NV_cuda_kernel_launch`
///
///
/// # Parameters
/// - `device`
/// - `function`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "system" fn(
    device: VkDevice,
    function: VkCudaFunctionNV,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html)
///
/// Provided by:
/// - `VK_NV_cuda_kernel_launch`
///
///
/// # Parameters
/// - `device`
/// - `module`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkDestroyCudaModuleNV = unsafe extern "system" fn(
    device: VkDevice,
    module: VkCudaModuleNV,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: VkDevice,
    session: VkDataGraphPipelineSessionARM,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_report`
///
///
/// # Parameters
/// - `instance`
/// - `callback`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: VkInstance,
    callback: VkDebugReportCallbackEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `instance`
/// - `messenger`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: VkDevice,
    operation: VkDeferredOperationKHR,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `descriptorSetLayout`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: VkDevice,
    descriptorSetLayout: VkDescriptorSetLayout,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `descriptorUpdateTemplate`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: VkDevice,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = PFN_vkDestroyDescriptorUpdateTemplate;
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
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
/// [`vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `event`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: VkDevice,
    event: VkEvent,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html)
///
/// Provided by:
/// - `VK_NV_external_compute_queue`
///
///
/// # Parameters
/// - `device`
/// - `externalQueue`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_external_compute_queue")]
pub type PFN_vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
    device: VkDevice,
    externalQueue: VkExternalComputeQueueNV,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `fence`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: VkDevice,
    fence: VkFence,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `framebuffer`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `image`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `imageView`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: VkDevice,
    imageView: VkImageView,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
///
/// # Parameters
/// - `device`
/// - `indirectCommandsLayout`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html)
///
/// Provided by:
/// - `VK_NV_device_generated_commands`
///
///
/// # Parameters
/// - `device`
/// - `indirectCommandsLayout`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: VkDevice,
    indirectCommandsLayout: VkIndirectCommandsLayoutNV,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
///
/// # Parameters
/// - `device`
/// - `indirectExecutionSet`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `instance`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyInstance =
    unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
/// [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)
///
/// Provided by:
/// - `VK_EXT_opacity_micromap`
///
///
/// # Parameters
/// - `device`
/// - `micromap`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: VkDevice,
    micromap: VkMicromapEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html)
///
/// Provided by:
/// - `VK_NV_optical_flow`
///
///
/// # Parameters
/// - `device`
/// - `session`
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
    device: VkDevice,
    session: VkOpticalFlowSessionNV,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html)
///
/// Provided by:
/// - `VK_KHR_pipeline_binary`
///
///
/// # Parameters
/// - `device`
/// - `pipelineBinary`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: VkDevice,
    pipelineBinary: VkPipelineBinaryKHR,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `pipelineLayout`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: VkDevice,
    pipelineLayout: VkPipelineLayout,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `privateDataSlot`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: VkDevice,
    privateDataSlot: VkPrivateDataSlot,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;
/// [`vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `queryPool`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `renderPass`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: VkDevice,
    renderPass: VkRenderPass,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `sampler`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: VkDevice,
    sampler: VkSampler,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `ycbcrConversion`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: VkDevice,
    ycbcrConversion: VkSamplerYcbcrConversion,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html)
///
/// Provided by:
/// - `VK_KHR_sampler_ycbcr_conversion`
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;
/// [`vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `semaphore`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySemaphoreSciSyncPoolNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphoreSciSyncPoolNV.html)
///
/// Provided by:
/// - `VK_NV_external_sci_sync2`
///
///
/// # Parameters
/// - `device`
/// - `semaphorePool`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub type PFN_vkDestroySemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: VkDevice,
    semaphorePool: VkSemaphoreSciSyncPoolNV,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html)
///
/// Provided by:
/// - `VK_EXT_shader_object`
///
///
/// # Parameters
/// - `device`
/// - `shader`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_shader_object")]
pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
    device: VkDevice,
    shader: VkShaderEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html)
///
/// Provided by:
/// - `VK_ARM_shader_instrumentation`
///
///
/// # Parameters
/// - `device`
/// - `instrumentation`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkDestroyShaderInstrumentationARM = unsafe extern "system" fn(
    device: VkDevice,
    instrumentation: VkShaderInstrumentationARM,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `shaderModule`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)
///
/// Provided by:
/// - `VK_KHR_surface`
///
///
/// # Parameters
/// - `instance`
/// - `surface`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html)
///
/// Provided by:
/// - `VK_KHR_swapchain`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
    device: VkDevice,
    tensor: VkTensorARM,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: VkDevice,
    tensorView: VkTensorViewARM,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html)
///
/// Provided by:
/// - `VK_EXT_validation_cache`
///
///
/// # Parameters
/// - `device`
/// - `validationCache`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
///
/// # Parameters
/// - `device`
/// - `videoSession`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pAllocator: *const VkAllocationCallbacks,
);
/// [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
///
/// # Parameters
/// - `device`
/// - `videoSessionParameters`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pAllocator: *const VkAllocationCallbacks,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: VkDevice) -> VkResult;
/// [`vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html)
///
/// Provided by:
/// - `VK_EXT_display_control`
///
///
/// # Parameters
/// - `device`
/// - `display`
/// - `pDisplayPowerInfo`
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
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayPowerInfo: *const VkDisplayPowerInfoEXT,
) -> VkResult;
/// [`vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(commandBuffer: VkCommandBuffer) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_LAYER_NOT_PRESENT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pLayerName: *const core::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult;
/// [`vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html)
///
/// Provided by:
/// - `VKSC_VERSION_1_0`
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `physicalDevice`
/// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
/// - `pProperties`: optional: true, len: pPropertyCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VKSC_VERSION_1_0", feature = "VK_BASE_VERSION_1_0"))]
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult;
/// [`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `pLayerName`: optional: true, len: null-terminated
/// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
/// - `pProperties`: optional: true, len: pPropertyCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_LAYER_NOT_PRESENT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    pLayerName: *const core::ffi::c_char,
    pPropertyCount: *mut u32,
    pProperties: *mut VkExtensionProperties,
) -> VkResult;
/// [`vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
/// - `pProperties`: optional: true, len: pPropertyCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
    pPropertyCount: *mut u32,
    pProperties: *mut VkLayerProperties,
) -> VkResult;
/// [`vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `pApiVersion`
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
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(pApiVersion: *mut u32) -> VkResult;
/// [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `instance`
/// - `pPhysicalDeviceGroupCount`: optional: pointer required, values optional if pointer not null
/// - `pPhysicalDeviceGroupProperties`: optional: true, len: pPhysicalDeviceGroupCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: VkInstance,
    pPhysicalDeviceGroupCount: *mut u32,
    pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
) -> VkResult;
/// [`vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group_creation`
///
#[cfg(feature = "VK_KHR_device_group_creation")]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        pCounterCount: *mut u32,
        pCounters: *mut VkPerformanceCounterARM,
        pCounterDescriptions: *mut VkPerformanceCounterDescriptionARM,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_performance_query")]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        pCounterCount: *mut u32,
        pCounters: *mut VkPerformanceCounterKHR,
        pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pDescriptionCount: *mut u32,
        pDescriptions: *mut VkShaderInstrumentationMetricDescriptionARM,
    ) -> VkResult;
/// [`vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `instance`
/// - `pPhysicalDeviceCount`: optional: pointer required, values optional if pointer not null
/// - `pPhysicalDevices`: optional: true, len: pPhysicalDeviceCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: VkInstance,
    pPhysicalDeviceCount: *mut u32,
    pPhysicalDevices: *mut VkPhysicalDevice,
) -> VkResult;
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
pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
    device: VkDevice,
    pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
/// [`vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `commandPool`
/// - `commandBufferCount`
/// - `pCommandBuffers`: len: commandBufferCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBufferCount: u32,
    pCommandBuffers: *const VkCommandBuffer,
);
/// [`vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `descriptorPool`
/// - `descriptorSetCount`
/// - `pDescriptorSets`: len: descriptorSetCount
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
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
) -> VkResult;
/// [`vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `memory`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    pAllocator: *const VkAllocationCallbacks,
);
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
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
    pMaxPrimitiveCounts: *const u32,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);
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
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR,
    ) -> VkDeviceAddress;
/// [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
///
/// # Parameters
/// - `device`
/// - `accelerationStructure`
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
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: VkDevice,
    accelerationStructure: VkAccelerationStructureNV,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
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
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2KHR,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT,
        pData: *mut core::ffi::c_void,
    ) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: VkDevice,
    buffer: *const AHardwareBuffer,
    pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID,
) -> VkResult;
/// [`vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html)
///
/// Provided by:
/// - `VK_FUCHSIA_buffer_collection`
///
///
/// # Parameters
/// - `device`
/// - `collection`
/// - `pProperties`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pProperties: *mut VkBufferCollectionPropertiesFUCHSIA,
) -> VkResult;
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
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkBufferDeviceAddressInfo,
) -> VkDeviceAddress;
/// [`vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html)
///
/// Provided by:
/// - `VK_EXT_buffer_device_address`
///
#[cfg(feature = "VK_EXT_buffer_device_address")]
pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;
/// [`vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;
/// [`vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `buffer`
/// - `pMemoryRequirements`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    pMemoryRequirements: *mut VkMemoryRequirements,
);
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
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkBufferMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
/// [`vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;
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
pub type PFN_vkGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;
/// [`vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkBufferCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html)
///
/// Provided by:
/// - `VK_EXT_calibrated_timestamps`
///
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: VkDevice,
    timestampCount: u32,
    pTimestampInfos: *const VkCalibratedTimestampInfoKHR,
    pTimestamps: *mut u64,
    pMaxDeviation: *mut u64,
) -> VkResult;
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
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkClusterAccelerationStructureInputInfoNV,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);
/// [`vkGetCommandPoolMemoryConsumption`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCommandPoolMemoryConsumption.html)
///
/// Provided by:
/// - `VKSC_VERSION_1_0`
///
/// - **Export Scopes:** VulkanSC
///
/// # Parameters
/// - `device`
/// - `commandPool`
/// - `commandBuffer`: optional: true
/// - `pConsumption`
#[cfg(feature = "VKSC_VERSION_1_0")]
pub type PFN_vkGetCommandPoolMemoryConsumption = unsafe extern "system" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    commandBuffer: VkCommandBuffer,
    pConsumption: *mut VkCommandPoolMemoryConsumption,
);
/// [`vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html)
///
/// Provided by:
/// - `VK_NV_cuda_kernel_launch`
///
///
/// # Parameters
/// - `device`
/// - `module`
/// - `pCacheSize`: optional: pointer required, values optional if pointer not null
/// - `pCacheData`: optional: true, len: pCacheSize
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "system" fn(
    device: VkDevice,
    module: VkCudaModuleNV,
    pCacheSize: *mut usize,
    pCacheData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
    device: VkDevice,
    pPipelineInfo: *const VkDataGraphPipelineInfoARM,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkDataGraphPipelinePropertyARM,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
    device: VkDevice,
    pPipelineInfo: *const VkDataGraphPipelineInfoARM,
    propertiesCount: u32,
    pProperties: *mut VkDataGraphPipelinePropertyQueryResultARM,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "system" fn(
        device: VkDevice,
        pInfo: *const VkDataGraphPipelineSessionBindPointRequirementsInfoARM,
        pBindPointRequirementCount: *mut u32,
        pBindPointRequirements: *mut VkDataGraphPipelineSessionBindPointRequirementARM,
    ) -> VkResult;
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
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDataGraphPipelineSessionMemoryRequirementsInfoARM,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
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
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> u32;
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
///   - VK_SUCCESS
///   - VK_NOT_READY
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;
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
pub type PFN_vkGetDescriptorEXT = unsafe extern "system" fn(
    device: VkDevice,
    pDescriptorInfo: *const VkDescriptorGetInfoEXT,
    dataSize: usize,
    pDescriptor: *mut core::ffi::c_void,
);
/// [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html)
///
/// Provided by:
/// - `VK_VALVE_descriptor_set_host_mapping`
///
///
/// # Parameters
/// - `device`
/// - `descriptorSet`
/// - `ppData`
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    ppData: *mut *mut core::ffi::c_void,
);
/// [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
///
/// # Parameters
/// - `device`
/// - `layout`
/// - `binding`
/// - `pOffset`
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    binding: u32,
    pOffset: *mut VkDeviceSize,
);
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
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: VkDevice,
    pBindingReference: *const VkDescriptorSetBindingReferenceVALVE,
    pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE,
);
/// [`vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
///
/// # Parameters
/// - `device`
/// - `layout`
/// - `pLayoutSizeInBytes`
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
    device: VkDevice,
    layout: VkDescriptorSetLayout,
    pLayoutSizeInBytes: *mut VkDeviceSize,
);
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
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
    pSupport: *mut VkDescriptorSetLayoutSupport,
);
/// [`vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance3`
///
#[cfg(feature = "VK_KHR_maintenance3")]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;
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
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: VkDevice,
    pVersionInfo: *const VkAccelerationStructureVersionInfoKHR,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
);
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
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceBufferMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
/// [`vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;
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
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
    unsafe extern "system" fn(device: VkDevice, imageViewIndex: u64, samplerIndex: u64) -> u64;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_device_fault")]
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
    device: VkDevice,
    pFaultCounts: *mut VkDeviceFaultCountsEXT,
    pFaultInfo: *mut VkDeviceFaultInfoEXT,
) -> VkResult;
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
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: VkDevice,
    heapIndex: u32,
    localDeviceIndex: u32,
    remoteDeviceIndex: u32,
    pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags,
);
/// [`vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeaturesKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
///
#[cfg(feature = "VK_KHR_device_group")]
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: VkDevice,
    pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: VkDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
/// [`vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
/// - `VK_KHR_swapchain`
///
///
/// # Parameters
/// - `device`
/// - `surface`
/// - `pModes`: optional: pointer required, values optional if pointer not null
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: VkDevice,
    surface: VkSurfaceKHR,
    pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
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
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
/// [`vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;
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
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageMemoryRequirements,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
/// [`vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
    PFN_vkGetDeviceImageSparseMemoryRequirements;
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
pub type PFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceImageSubresourceInfo,
    pLayout: *mut VkSubresourceLayout2,
);
/// [`vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = PFN_vkGetDeviceImageSubresourceLayout;
/// [`vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `memory`
/// - `pCommittedMemoryInBytes`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    pCommittedMemoryInBytes: *mut VkDeviceSize,
);
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
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
/// [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = PFN_vkGetDeviceMemoryOpaqueCaptureAddress;
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
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
    device: VkDevice,
    pVersionInfo: *const VkMicromapVersionInfoEXT,
    pCompatibility: *mut VkAccelerationStructureCompatibilityKHR,
);
/// [`vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `pName`: len: null-terminated
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: VkDevice,
    pName: *const core::ffi::c_char,
) -> PFN_vkVoidFunction;
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
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: VkDevice,
    queueFamilyIndex: u32,
    queueIndex: u32,
    pQueue: *mut VkQueue,
);
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
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: VkDevice,
    pQueueInfo: *const VkDeviceQueueInfo2,
    pQueue: *mut VkQueue,
);
/// [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
///
/// Provided by:
/// - `VK_HUAWEI_subpass_shading`
///
///
/// # Parameters
/// - `device`
/// - `renderpass`
/// - `pMaxWorkgroupSize`: len: 1
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
    unsafe extern "system" fn(
        device: VkDevice,
        renderpass: VkRenderPass,
        pMaxWorkgroupSize: *mut VkExtent2D,
    ) -> VkResult;
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
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDeviceTensorMemoryRequirementsARM,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModeProperties2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    display: VkDisplayKHR,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayModePropertiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR,
    pCapabilities: *mut VkDisplayPlaneCapabilities2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    mode: VkDisplayModeKHR,
    planeIndex: u32,
    pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    planeIndex: u32,
    pDisplayCount: *mut u32,
    pDisplays: *mut VkDisplayKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_acquire_drm_display")]
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    drmFd: i32,
    connectorId: u32,
    display: *mut VkDisplayKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_QCOM_tile_properties")]
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: VkDevice,
    pRenderingInfo: *const VkRenderingInfo,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: VkDevice,
    pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR,
    pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `event`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_EVENT_SET
///   - VK_EVENT_RESET
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkGetEventStatus =
    unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
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
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: VkDevice,
    executionGraph: VkPipeline,
    pNodeInfo: *const VkPipelineShaderStageNodeCreateInfoAMDX,
    pNodeIndex: *mut u32,
) -> VkResult;
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
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: VkDevice,
    executionGraph: VkPipeline,
    pSizeInfo: *mut VkExecutionGraphPipelineScratchSizeAMDX,
) -> VkResult;
/// [`vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html)
///
/// Provided by:
/// - `VK_NV_external_compute_queue`
///
///
/// # Parameters
/// - `externalQueue`
/// - `params`
/// - `pData`
#[cfg(feature = "VK_NV_external_compute_queue")]
pub type PFN_vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
    externalQueue: VkExternalComputeQueueNV,
    params: *mut VkExternalComputeQueueDataParamsNV,
    pData: *mut core::ffi::c_void,
);
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VKSC_VERSION_1_0")]
pub type PFN_vkGetFaultData = unsafe extern "system" fn(
    device: VkDevice,
    faultQueryBehavior: VkFaultQueryBehavior,
    pUnrecordedFaults: *mut VkBool32,
    pFaultCount: *mut u32,
    pFaults: *mut VkFaultData,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetFdInfo: *const VkFenceGetFdInfoKHR,
    pFd: *mut core::ffi::c_int,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
))]
pub type PFN_vkGetFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
))]
pub type PFN_vkGetFenceSciSyncObjNV = unsafe extern "system" fn(
    device: VkDevice,
    pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `fence`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_NOT_READY
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetFenceStatus =
    unsafe extern "system" fn(device: VkDevice, fence: VkFence) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
/// [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_tile_properties`
///
///
/// # Parameters
/// - `device`
/// - `framebuffer`
/// - `pPropertiesCount`: optional: pointer required, values optional if pointer not null
/// - `pProperties`: optional: true, len: pPropertiesCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_QCOM_tile_properties")]
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    pPropertiesCount: *mut u32,
    pProperties: *mut VkTilePropertiesQCOM,
) -> VkResult;
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
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoEXT,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
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
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
/// [`vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_image_drm_format_modifier`
///
///
/// # Parameters
/// - `device`
/// - `image`
/// - `pProperties`
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
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
) -> VkResult;
/// [`vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `image`
/// - `pMemoryRequirements`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pMemoryRequirements: *mut VkMemoryRequirements,
);
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
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageMemoryRequirementsInfo2,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
/// [`vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    imageCount: u32,
    pImages: *const VkImage,
    pDatas: *mut VkHostAddressRangeEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `image`
/// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
/// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
);
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
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageSparseMemoryRequirementsInfo2,
    pSparseMemoryRequirementCount: *mut u32,
    pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);
/// [`vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;
/// [`vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `image`
/// - `pSubresource`
/// - `pLayout`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource,
    pLayout: *mut VkSubresourceLayout,
);
/// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_4`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `image`
/// - `pSubresource`
/// - `pLayout`
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
    device: VkDevice,
    image: VkImage,
    pSubresource: *const VkImageSubresource2,
    pLayout: *mut VkSubresourceLayout2,
);
/// [`vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2EXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
/// - `VK_EXT_image_compression_control`
///
#[cfg(any(
    feature = "VK_EXT_host_image_copy",
    feature = "VK_EXT_image_compression_control"
))]
pub type PFN_vkGetImageSubresourceLayout2EXT = PFN_vkGetImageSubresourceLayout2;
/// [`vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetImageSubresourceLayout2KHR = PFN_vkGetImageSubresourceLayout2;
/// [`vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html)
///
/// Provided by:
/// - `VK_NVX_image_view_handle`
///
///
/// # Parameters
/// - `device`
/// - `imageView`
/// - `pProperties`
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
#[cfg(feature = "VK_NVX_image_view_handle")]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: VkDevice,
    imageView: VkImageView,
    pProperties: *mut VkImageViewAddressPropertiesNVX,
) -> VkResult;
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
pub type PFN_vkGetImageViewHandle64NVX =
    unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> u64;
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
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> u32;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `instance`: optional: true
/// - `pName`: len: null-terminated
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: VkInstance,
    pName: *const core::ffi::c_char,
) -> PFN_vkVoidFunction;
/// [`vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html)
///
/// Provided by:
/// - `VK_NV_low_latency2`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pLatencyMarkerInfo`
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *mut VkGetLatencyMarkerInfoNV,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID,
    pBuffer: *mut *mut AHardwareBuffer,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetFdInfo: *const VkMemoryGetFdInfoKHR,
    pFd: *mut core::ffi::c_int,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    fd: core::ffi::c_int,
    pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_external_memory_host")]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHostPointer: *const core::ffi::c_void,
    pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub type PFN_vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
    device: VkDevice,
    pGetMetalHandleInfo: *const VkMemoryGetMetalHandleInfoEXT,
    pHandle: *mut *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    pHandle: *const core::ffi::c_void,
    pMemoryMetalHandleProperties: *mut VkMemoryMetalHandlePropertiesEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_OHOS_external_memory")]
pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkMemoryGetNativeBufferInfoOHOS,
    pBuffer: *mut *mut OH_NativeBuffer,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: VkDevice,
    pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV,
    pAddress: *mut VkRemoteAddressNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub type PFN_vkGetMemorySciBufNV = unsafe extern "system" fn(
    device: VkDevice,
    pGetSciBufInfo: *const VkMemoryGetSciBufInfoNV,
    pHandle: *mut NvSciBufObj,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
/// [`vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html)
///
/// Provided by:
/// - `VK_NV_external_memory_win32`
///
///
/// # Parameters
/// - `device`
/// - `memory`
/// - `handleType`
/// - `pHandle`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_win32")]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    handleType: VkExternalMemoryHandleTypeFlagsNV,
    pHandle: *mut HANDLE,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    zirconHandle: zx_handle_t,
    pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA,
) -> VkResult;
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
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: VkDevice,
    buildType: VkAccelerationStructureBuildTypeKHR,
    pBuildInfo: *const VkMicromapBuildInfoEXT,
    pSizeInfo: *mut VkMicromapBuildSizesInfoEXT,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_OHOS_external_memory")]
pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
    device: VkDevice,
    buffer: *const OH_NativeBuffer,
    pProperties: *mut VkNativeBufferPropertiesOHOS,
) -> VkResult;
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
pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkPartitionedAccelerationStructureInstancesInputNV,
    pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
    device: VkDevice,
    pPastPresentationTimingInfo: *const VkPastPresentationTimingInfoEXT,
    pPastPresentationTimingProperties: *mut VkPastPresentationTimingPropertiesEXT,
) -> VkResult;
/// [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html)
///
/// Provided by:
/// - `VK_GOOGLE_display_timing`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pPresentationTimingCount`: optional: pointer required, values optional if pointer not null
/// - `pPresentationTimings`: optional: true, len: pPresentationTimingCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentationTimingCount: *mut u32,
    pPresentationTimings: *mut VkPastPresentationTimingGOOGLE,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: VkDevice,
    parameter: VkPerformanceParameterTypeINTEL,
    pValue: *mut VkPerformanceValueINTEL,
) -> VkResult;
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
///
/// Provided by:
/// - `VK_EXT_calibrated_timestamps`
///
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
    PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pTimeDomainCount: *mut u32,
    pTimeDomains: *mut VkTimeDomainKHR,
)
    -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cooperative_matrix2")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkCooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_cooperative_matrix")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkCooperativeMatrixPropertiesKHR,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cooperative_matrix")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkCooperativeMatrixPropertiesNV,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_cooperative_vector")]
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPropertyCount: *mut u32,
        pProperties: *mut VkCooperativeVectorPropertiesNV,
    ) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    descriptorType: VkDescriptorType,
) -> VkDeviceSize;
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
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        dfb: *mut IDirectFB,
    ) -> VkBool32;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlaneProperties2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPlanePropertiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayProperties2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkDisplayPropertiesKHR,
) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
    pExternalBufferProperties: *mut VkExternalBufferProperties,
);
/// [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_memory_capabilities`
///
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalBufferProperties;
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
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
    pExternalFenceProperties: *mut VkExternalFenceProperties,
);
/// [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_fence_capabilities`
///
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalFenceProperties;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        format: VkFormat,
        type_: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
        externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
        pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
    ) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub type PFN_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        handleType: VkExternalMemoryHandleTypeFlagBits,
        handle: NvSciBufObj,
        pMemorySciBufProperties: *mut VkMemorySciBufPropertiesNV,
    ) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
    pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
);
/// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_semaphore_capabilities`
///
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
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
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pExternalTensorInfo: *const VkPhysicalDeviceExternalTensorInfoARM,
    pExternalTensorProperties: *mut VkExternalTensorPropertiesARM,
);
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
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures,
);
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
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pFeatures: *mut VkPhysicalDeviceFeatures2,
);
/// [`vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;
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
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties,
);
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
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    pFormatProperties: *mut VkFormatProperties2,
);
/// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pFragmentShadingRateCount: *mut u32,
    pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkGetPhysicalDeviceImageFormatProperties2`")]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    pImageFormatProperties: *mut VkImageFormatProperties,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
    pImageFormatProperties: *mut VkImageFormatProperties2,
) -> VkResult;
/// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceImageFormatProperties2;
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
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
);
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
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2,
);
/// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;
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
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    samples: VkSampleCountFlagBits,
    pMultisampleProperties: *mut VkMultisamplePropertiesEXT,
);
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_EXTENSION_NOT_PRESENT
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV,
) -> VkResult;
/// [`vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
/// - `VK_KHR_swapchain`
///
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain"))]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pRectCount: *mut u32,
    pRects: *mut VkRect2D,
) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties,
);
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
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pProperties: *mut VkPhysicalDeviceProperties2,
);
/// [`vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyDataGraphProcessingEngineInfo: *const VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    pQueueFamilyDataGraphProcessingEngineProperties: *mut VkQueueFamilyDataGraphProcessingEnginePropertiesARM,
);
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        pQueueFamilyDataGraphPropertyCount: *mut u32,
        pQueueFamilyDataGraphProperties: *mut VkQueueFamilyDataGraphPropertiesARM,
    ) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR,
        pNumPasses: *mut u32,
    );
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties,
);
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pQueueFamilyPropertyCount: *mut u32,
    pQueueFamilyProperties: *mut VkQueueFamilyProperties2,
);
/// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
    PFN_vkGetPhysicalDeviceQueueFamilyProperties2;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_object_refresh")]
pub type PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pRefreshableObjectTypeCount: *mut u32,
    pRefreshableObjectTypes: *mut VkObjectType,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub type PFN_vkGetPhysicalDeviceSciBufAttributesNV = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pAttributes: NvSciBufAttrList,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
))]
pub type PFN_vkGetPhysicalDeviceSciSyncAttributesNV = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSciSyncAttributesInfo: *const VkSciSyncAttributesInfoNV,
    pAttributes: NvSciSyncAttrList,
) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        window: *mut _screen_window,
    ) -> VkBool32;
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
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    format: VkFormat,
    type_: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties,
);
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
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
    pPropertyCount: *mut u32,
    pProperties: *mut VkSparseImageFormatProperties2,
);
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pCombinationCount: *mut u32,
        pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
    ) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormat2KHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pSurfaceFormatCount: *mut u32,
    pSurfaceFormats: *mut VkSurfaceFormatKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    pPresentModeCount: *mut u32,
    pPresentModes: *mut VkPresentModeKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_surface")]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    surface: VkSurfaceKHR,
    pSupported: *mut VkBool32,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pToolCount: *mut u32,
    pToolProperties: *mut VkPhysicalDeviceToolProperties,
) -> VkResult;
/// [`vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_tooling_info`
///
#[cfg(feature = "VK_EXT_tooling_info")]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = PFN_vkGetPhysicalDeviceToolProperties;
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
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    device: *mut ubm_device,
) -> VkBool32;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pVideoProfile: *const VkVideoProfileInfoKHR,
    pCapabilities: *mut VkVideoCapabilitiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR
///   - VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR,
    pVideoFormatPropertyCount: *mut u32,
    pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR,
) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
    unsafe extern "system" fn(
        physicalDevice: VkPhysicalDevice,
        queueFamilyIndex: u32,
        display: *mut wl_display,
    ) -> VkBool32;
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
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32) -> VkBool32;
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
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> VkBool32;
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
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    dpy: *mut Display,
    visualID: VisualID,
) -> VkBool32;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_NOT_ENOUGH_SPACE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkPipelineBinaryDataInfoKHR,
    pPipelineBinaryKey: *mut VkPipelineBinaryKeyKHR,
    pPipelineBinaryDataSize: *mut usize,
    pPipelineBinaryData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: VkDevice,
    pipelineCache: VkPipelineCache,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: VkDevice,
        pExecutableInfo: *const VkPipelineExecutableInfoKHR,
        pInternalRepresentationCount: *mut u32,
        pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
    ) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoKHR,
    pExecutableCount: *mut u32,
    pProperties: *mut VkPipelineExecutablePropertiesKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR,
    pStatisticCount: *mut u32,
    pStatistics: *mut VkPipelineExecutableStatisticKHR,
) -> VkResult;
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
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkPipelineIndirectDeviceAddressInfoNV,
) -> VkDeviceAddress;
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
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkComputePipelineCreateInfo,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
    device: VkDevice,
    pPipelineCreateInfo: *const VkPipelineCreateInfoKHR,
    pPipelineKey: *mut VkPipelineBinaryKeyKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_pipeline_properties")]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    pPipelineInfo: *const VkPipelineInfoEXT,
    pPipelineProperties: *mut VkBaseOutStructure,
) -> VkResult;
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
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    pData: *mut u64,
);
/// [`vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;
/// [`vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `queryPool`
/// - `firstQuery`
/// - `queryCount`
/// - `dataSize`
/// - `pData`: len: dataSize
/// - `stride`
/// - `flags`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_NOT_READY
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult;
/// [`vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html)
///
/// Provided by:
/// - `VK_NV_device_diagnostic_checkpoints`
///
///
/// # Parameters
/// - `queue`
/// - `pCheckpointDataCount`: optional: pointer required, values optional if pointer not null
/// - `pCheckpointData`: optional: true, len: pCheckpointDataCount
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointData2NV,
);
/// [`vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html)
///
/// Provided by:
/// - `VK_NV_device_diagnostic_checkpoints`
///
///
/// # Parameters
/// - `queue`
/// - `pCheckpointDataCount`: optional: pointer required, values optional if pointer not null
/// - `pCheckpointData`: optional: true, len: pCheckpointDataCount
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: VkQueue,
    pCheckpointDataCount: *mut u32,
    pCheckpointData: *mut VkCheckpointDataNV,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    dpy: *mut Display,
    rrOutput: RROutput,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult;
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
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: VkDevice,
        pipeline: VkPipeline,
        firstGroup: u32,
        groupCount: u32,
        dataSize: usize,
        pData: *mut core::ffi::c_void,
    ) -> VkResult;
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
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    firstGroup: u32,
    groupCount: u32,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesNV.html)
///
/// Provided by:
/// - `VK_NV_ray_tracing`
///
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;
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
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    group: u32,
    groupShader: VkShaderGroupShaderKHR,
) -> VkDeviceSize;
/// [`vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html)
///
/// Provided by:
/// - `VK_GOOGLE_display_timing`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pDisplayTimingProperties`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE,
) -> VkResult;
/// [`vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html)
///
/// Provided by:
/// - `VK_GRAPHICS_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `renderPass`
/// - `pGranularity`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: VkDevice,
    renderPass: VkRenderPass,
    pGranularity: *mut VkExtent2D,
);
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
pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
    device: VkDevice,
    pRenderingAreaInfo: *const VkRenderingAreaInfo,
    pGranularity: *mut VkExtent2D,
);
/// [`vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetRenderingAreaGranularityKHR = PFN_vkGetRenderingAreaGranularity;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT,
    pData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: VkDevice,
    buffer: *const _screen_buffer,
    pProperties: *mut VkScreenBufferPropertiesQNX,
) -> VkResult;
/// [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_2`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `semaphore`
/// - `pValue`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    pValue: *mut u64,
) -> VkResult;
/// [`vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetFdInfo: *const VkSemaphoreGetFdInfoKHR,
    pFd: *mut core::ffi::c_int,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_sci_sync")]
pub type PFN_vkGetSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: VkDevice,
    pGetSciSyncInfo: *const VkSemaphoreGetSciSyncInfoNV,
    pHandle: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: VkDevice,
    pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR,
    pHandle: *mut HANDLE,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA,
    pZirconHandle: *mut zx_handle_t,
) -> VkResult;
/// [`vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_shader_object`
///
///
/// # Parameters
/// - `device`
/// - `shader`
/// - `pDataSize`: optional: pointer required, values optional if pointer not null
/// - `pData`: optional: true, len: pDataSize
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_shader_object")]
pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    shader: VkShaderEXT,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
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
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    shaderStage: VkShaderStageFlagBits,
    infoType: VkShaderInfoTypeAMD,
    pInfoSize: *mut usize,
    pInfo: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html)
///
/// Provided by:
/// - `VK_ARM_shader_instrumentation`
///
///
/// # Parameters
/// - `device`
/// - `instrumentation`
/// - `pMetricBlockCount`
/// - `pMetricValues`
/// - `flags`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkGetShaderInstrumentationValuesARM = unsafe extern "system" fn(
    device: VkDevice,
    instrumentation: VkShaderInstrumentationARM,
    pMetricBlockCount: *mut u32,
    pMetricValues: *mut core::ffi::c_void,
    flags: VkShaderInstrumentationValuesFlagsARM,
) -> VkResult;
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
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: VkDevice,
    pCreateInfo: *const VkShaderModuleCreateInfo,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
);
/// [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)
///
/// Provided by:
/// - `VK_EXT_shader_module_identifier`
///
///
/// # Parameters
/// - `device`
/// - `shaderModule`
/// - `pIdentifier`
#[cfg(feature = "VK_EXT_shader_module_identifier")]
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: VkDevice,
    shaderModule: VkShaderModule,
    pIdentifier: *mut VkShaderModuleIdentifierEXT,
);
/// [`vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html)
///
/// Provided by:
/// - `VK_EXT_display_control`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `counter`
/// - `pCounterValue`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    counter: VkSurfaceCounterFlagBitsEXT,
    pCounterValue: *mut u64,
) -> VkResult;
/// [`vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html)
///
/// Provided by:
/// - `VK_KHR_swapchain`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pSwapchainImageCount`: optional: pointer required, values optional if pointer not null
/// - `pSwapchainImages`: optional: true, len: pSwapchainImageCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainImageCount: *mut u32,
    pSwapchainImages: *mut VkImage,
) -> VkResult;
/// [`vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html)
///
/// Provided by:
/// - `VK_KHR_shared_presentable_image`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// [`vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_present_timing`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pSwapchainTimeDomainProperties`
/// - `pTimeDomainsCounter`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainTimeDomainProperties: *mut VkSwapchainTimeDomainPropertiesEXT,
    pTimeDomainsCounter: *mut u64,
) -> VkResult;
/// [`vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_present_timing`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pSwapchainTimingProperties`
/// - `pSwapchainTimingPropertiesCounter`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_NOT_READY
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSwapchainTimingProperties: *mut VkSwapchainTimingPropertiesEXT,
    pSwapchainTimingPropertiesCounter: *mut u64,
) -> VkResult;
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
pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkTensorMemoryRequirementsInfoARM,
    pMemoryRequirements: *mut VkMemoryRequirements2,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
    device: VkDevice,
    tensorCount: u32,
    pTensors: *const VkTensorARM,
    pDatas: *mut VkHostAddressRangeEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkTensorCaptureDescriptorDataInfoARM,
    pData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkTensorViewCaptureDescriptorDataInfoARM,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_validation_cache`
///
///
/// # Parameters
/// - `device`
/// - `validationCache`
/// - `pDataSize`: optional: pointer required, values optional if pointer not null
/// - `pData`: optional: true, len: pDataSize
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: VkDevice,
    validationCache: VkValidationCacheEXT,
    pDataSize: *mut usize,
    pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
///
/// # Parameters
/// - `device`
/// - `videoSession`
/// - `pMemoryRequirementsCount`: optional: pointer required, values optional if pointer not null
/// - `pMemoryRequirements`: optional: true, len: pMemoryRequirementsCount
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_INCOMPLETE
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: VkDevice,
    videoSession: VkVideoSessionKHR,
    pMemoryRequirementsCount: *mut u32,
    pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_acquire_winrt_display")]
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    deviceRelativeId: u32,
    pDisplay: *mut VkDisplayKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: VkDevice,
    pImportFenceFdInfo: *const VkImportFenceFdInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
))]
pub type PFN_vkImportFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(any(
    feature = "VK_NV_external_sci_sync",
    feature = "VK_NV_external_sci_sync2"
))]
pub type PFN_vkImportFenceSciSyncObjNV = unsafe extern "system" fn(
    device: VkDevice,
    pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: VkDevice,
    pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: VkDevice,
    pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_NOT_PERMITTED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_external_sci_sync")]
pub type PFN_vkImportSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: VkDevice,
    pImportSemaphoreSciSyncInfo: *const VkImportSemaphoreSciSyncInfoNV,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: VkDevice,
    pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_INVALID_EXTERNAL_HANDLE
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: VkDevice,
    pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: VkDevice,
    memoryRangeCount: u32,
    pMemoryRanges: *const VkMappedMemoryRange,
) -> VkResult;
/// [`vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html)
///
/// Provided by:
/// - `VK_NV_low_latency2`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pSleepInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepInfo: *const VkLatencySleepInfoNV,
) -> VkResult;
/// [`vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `memory`
/// - `offset`
/// - `size`
/// - `flags`: optional: true
/// - `ppData`: optional: pointer required, values optional if pointer not null
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    ppData: *mut *mut core::ffi::c_void,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
    device: VkDevice,
    pMemoryMapInfo: *const VkMemoryMapInfo,
    ppData: *mut *mut core::ffi::c_void,
) -> VkResult;
/// [`vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_map_memory2`
///
#[cfg(feature = "VK_KHR_map_memory2")]
pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: VkDevice,
    dstCache: VkPipelineCache,
    srcCacheCount: u32,
    pSrcCaches: *const VkPipelineCache,
) -> VkResult;
/// [`vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html)
///
/// Provided by:
/// - `VK_EXT_validation_cache`
///
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: VkDevice,
    dstCache: VkValidationCacheEXT,
    srcCacheCount: u32,
    pSrcCaches: *const VkValidationCacheEXT,
) -> VkResult;
/// [`vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `queue`
/// - `pLabelInfo`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
/// [`vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Queues:** SparseBinding
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `queue`
/// - `bindInfoCount`: optional: true
/// - `pBindInfo`: len: bindInfoCount
/// - `fence`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: VkQueue,
    bindInfoCount: u32,
    pBindInfo: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult;
/// [`vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `queue`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: VkQueue);
/// [`vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `queue`
/// - `pLabelInfo`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);
/// [`vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html)
///
/// Provided by:
/// - `VK_NV_low_latency2`
///
///
/// # Parameters
/// - `queue`
/// - `pQueueTypeInfo`
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: VkQueue, pQueueTypeInfo: *const VkOutOfBandQueueTypeInfoNV);
/// [`vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html)
///
/// Provided by:
/// - `VK_KHR_swapchain`
///
///
/// # Parameters
/// - `queue`
/// - `pPresentInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
///   - VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkQueuePresentKHR =
    unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;
/// [`vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
///
/// # Parameters
/// - `queue`
/// - `configuration`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
    queue: VkQueue,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult;
/// [`vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `queue`
/// - `submitCount`: optional: true
/// - `pSubmits`: len: submitCount
/// - `fence`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkQueueSubmit2`")]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult;
/// [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_3`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `queue`
/// - `submitCount`: optional: true
/// - `pSubmits`: len: submitCount
/// - `fence`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
    queue: VkQueue,
    submitCount: u32,
    pSubmits: *const VkSubmitInfo2,
    fence: VkFence,
) -> VkResult;
/// [`vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkQueueSubmit2KHR = PFN_vkQueueSubmit2;
/// [`vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `queue`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
    device: VkDevice,
    pBorderColor: *const VkSamplerCustomBorderColorCreateInfoEXT,
    requestIndex: VkBool32,
    pIndex: *mut u32,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: VkDevice,
    pDeviceEventInfo: *const VkDeviceEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
/// [`vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html)
///
/// Provided by:
/// - `VK_EXT_display_control`
///
///
/// # Parameters
/// - `device`
/// - `display`
/// - `pDisplayEventInfo`
/// - `pAllocator`: optional: true
/// - `pFence`
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
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: VkDevice,
    display: VkDisplayKHR,
    pDisplayEventInfo: *const VkDisplayEventInfoEXT,
    pAllocator: *const VkAllocationCallbacks,
    pFence: *mut VkFence,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkReleaseCapturedPipelineDataInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub type PFN_vkReleaseDisplayEXT =
    unsafe extern "system" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
/// [`vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_full_screen_exclusive`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// [`vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
///
/// # Parameters
/// - `device`
/// - `configuration`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_TOO_MANY_OBJECTS
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: VkDevice,
    configuration: VkPerformanceConfigurationINTEL,
) -> VkResult;
/// [`vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html)
///
/// Provided by:
/// - `VK_KHR_performance_query`
///
///
/// # Parameters
/// - `device`
#[cfg(feature = "VK_KHR_performance_query")]
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: VkDevice);
/// [`vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesEXT.html)
///
/// Provided by:
/// - `VK_EXT_swapchain_maintenance1`
///
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: VkDevice,
    pReleaseInfo: *const VkReleaseSwapchainImagesInfoKHR,
) -> VkResult;
/// [`vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `commandBuffer`
/// - `flags`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    commandBuffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult;
/// [`vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `commandPool`
/// - `flags`: optional: true
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult;
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
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: VkDevice,
    descriptorPool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult;
/// [`vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `event`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkResetFences = unsafe extern "system" fn(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
) -> VkResult;
/// [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_2`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `queryPool`
/// - `firstQuery`
/// - `queryCount`
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: VkDevice,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
);
/// [`vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPoolEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_query_reset`
///
#[cfg(feature = "VK_EXT_host_query_reset")]
pub type PFN_vkResetQueryPoolEXT = PFN_vkResetQueryPool;
/// [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
///
/// Provided by:
/// - `VK_FUCHSIA_buffer_collection`
///
///
/// # Parameters
/// - `device`
/// - `collection`
/// - `pBufferConstraintsInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA,
) -> VkResult;
/// [`vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
///
/// Provided by:
/// - `VK_FUCHSIA_buffer_collection`
///
///
/// # Parameters
/// - `device`
/// - `collection`
/// - `pImageConstraintsInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_FORMAT_NOT_SUPPORTED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: VkDevice,
    collection: VkBufferCollectionFUCHSIA,
    pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: VkDevice,
    pNameInfo: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: VkDevice,
    pTagInfo: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult;
/// [`vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html)
///
/// Provided by:
/// - `VK_EXT_pageable_device_local_memory`
///
///
/// # Parameters
/// - `device`
/// - `memory`
/// - `priority`
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory, priority: f32);
/// [`vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `event`
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
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
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
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: VkDevice,
    swapchainCount: u32,
    pSwapchains: *const VkSwapchainKHR,
    pMetadata: *const VkHdrMetadataEXT,
);
/// [`vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html)
///
/// Provided by:
/// - `VK_NV_low_latency2`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pLatencyMarkerInfo`
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pLatencyMarkerInfo: *const VkSetLatencyMarkerInfoNV,
);
/// [`vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html)
///
/// Provided by:
/// - `VK_NV_low_latency2`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pSleepModeInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pSleepModeInfo: *const VkLatencySleepModeInfoNV,
) -> VkResult;
/// [`vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html)
///
/// Provided by:
/// - `VK_AMD_display_native_hdr`
///
///
/// # Parameters
/// - `device`
/// - `swapChain`
/// - `localDimmingEnable`
#[cfg(feature = "VK_AMD_display_native_hdr")]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: VkDevice,
    swapChain: VkSwapchainKHR,
    localDimmingEnable: VkBool32,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
    device: VkDevice,
    objectType: VkObjectType,
    objectHandle: u64,
    privateDataSlot: VkPrivateDataSlot,
    data: u64,
) -> VkResult;
/// [`vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;
/// [`vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html)
///
/// Provided by:
/// - `VK_EXT_present_timing`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `size`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_NOT_READY
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
    unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR, size: u32) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
    device: VkDevice,
    pSignalInfo: *const VkSemaphoreSignalInfo,
) -> VkResult;
/// [`vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;
/// [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)
///
/// Provided by:
/// - `VK_EXT_debug_utils`
///
///
/// # Parameters
/// - `instance`
/// - `messageSeverity`
/// - `messageTypes`
/// - `pCallbackData`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: VkInstance,
    messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_INITIALIZATION_FAILED
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
    device: VkDevice,
    transitionCount: u32,
    pTransitions: *const VkHostImageLayoutTransitionInfo,
) -> VkResult;
/// [`vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkTransitionImageLayoutEXT = PFN_vkTransitionImageLayout;
/// [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `commandPool`
/// - `flags`: optional: true
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: VkDevice,
    commandPool: VkCommandPool,
    flags: VkCommandPoolTrimFlags,
);
/// [`vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPoolKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance1`
///
#[cfg(feature = "VK_KHR_maintenance1")]
pub type PFN_vkTrimCommandPoolKHR = PFN_vkTrimCommandPool;
/// [`vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html)
///
/// Provided by:
/// - `VK_INTEL_performance_query`
///
///
/// # Parameters
/// - `device`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: VkDevice);
/// [`vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html)
///
/// Provided by:
/// - `VK_BASE_VERSION_1_0`
///
/// - **Export Scopes:** Vulkan, VulkanSC
///
/// # Parameters
/// - `device`
/// - `memory`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: VkDevice, memory: VkDeviceMemory);
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_MEMORY_MAP_FAILED
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
    device: VkDevice,
    pMemoryUnmapInfo: *const VkMemoryUnmapInfo,
) -> VkResult;
/// [`vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_map_memory2`
///
#[cfg(feature = "VK_KHR_map_memory2")]
pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
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
pub type PFN_vkUnregisterCustomBorderColorEXT =
    unsafe extern "system" fn(device: VkDevice, index: u32);
/// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html)
///
/// Provided by:
/// - `VK_COMPUTE_VERSION_1_1`
///
/// - **Export Scopes:** Vulkan
///
/// # Parameters
/// - `device`
/// - `descriptorSet`
/// - `descriptorUpdateTemplate`
/// - `pData`
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: VkDevice,
    descriptorSet: VkDescriptorSet,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    pData: *const core::ffi::c_void,
);
/// [`vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = PFN_vkUpdateDescriptorSetWithTemplate;
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
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: VkDevice,
    descriptorWriteCount: u32,
    pDescriptorWrites: *const VkWriteDescriptorSet,
    descriptorCopyCount: u32,
    pDescriptorCopies: *const VkCopyDescriptorSet,
);
/// [`vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
///
/// # Parameters
/// - `device`
/// - `indirectExecutionSet`
/// - `executionSetWriteCount`
/// - `pExecutionSetWrites`: len: executionSetWriteCount
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const VkWriteIndirectExecutionSetPipelineEXT,
);
/// [`vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html)
///
/// Provided by:
/// - `VK_EXT_device_generated_commands`
///
///
/// # Parameters
/// - `device`
/// - `indirectExecutionSet`
/// - `executionSetWriteCount`
/// - `pExecutionSetWrites`: len: executionSetWriteCount
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
    device: VkDevice,
    indirectExecutionSet: VkIndirectExecutionSetEXT,
    executionSetWriteCount: u32,
    pExecutionSetWrites: *const VkWriteIndirectExecutionSetShaderEXT,
);
/// [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html)
///
/// Provided by:
/// - `VK_KHR_video_queue`
///
///
/// # Parameters
/// - `device`
/// - `videoSessionParameters`
/// - `pUpdateInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: VkDevice,
    videoSessionParameters: VkVideoSessionParametersKHR,
    pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_TIMEOUT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
    device: VkDevice,
    fenceCount: u32,
    pFences: *const VkFence,
    waitAll: VkBool32,
    timeout: u64,
) -> VkResult;
/// [`vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html)
///
/// Provided by:
/// - `VK_KHR_present_wait2`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `pPresentWait2Info`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_TIMEOUT
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_present_wait2")]
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    pPresentWait2Info: *const VkPresentWait2InfoKHR,
) -> VkResult;
/// [`vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html)
///
/// Provided by:
/// - `VK_KHR_present_wait`
///
///
/// # Parameters
/// - `device`
/// - `swapchain`
/// - `presentId`
/// - `timeout`
///
/// # Returns
///
/// **Success Codes:**
///   - VK_SUCCESS
///   - VK_TIMEOUT
///   - VK_SUBOPTIMAL_KHR
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_OUT_OF_DATE_KHR
///   - VK_ERROR_SURFACE_LOST_KHR
///   - VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_present_wait")]
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    presentId: u64,
    timeout: u64,
) -> VkResult;
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
///   - VK_SUCCESS
///   - VK_TIMEOUT
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_DEVICE_LOST
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: VkDevice,
    pWaitInfo: *const VkSemaphoreWaitInfo,
    timeout: u64,
) -> VkResult;
/// [`vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: VkDevice,
    accelerationStructureCount: u32,
    pAccelerationStructures: *const VkAccelerationStructureKHR,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
    stride: usize,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    device: VkDevice,
    micromapCount: u32,
    pMicromaps: *const VkMicromapEXT,
    queryType: VkQueryType,
    dataSize: usize,
    pData: *mut core::ffi::c_void,
    stride: usize,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
    device: VkDevice,
    resourceCount: u32,
    pResources: *const VkResourceDescriptorInfoEXT,
    pDescriptors: *const VkHostAddressRangeEXT,
) -> VkResult;
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
///   - VK_SUCCESS
///
/// **Error Codes:**
///   - VK_ERROR_OUT_OF_HOST_MEMORY
///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
///   - VK_ERROR_UNKNOWN
///   - VK_ERROR_VALIDATION_FAILED
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
    device: VkDevice,
    samplerCount: u32,
    pSamplers: *const VkSamplerCreateInfo,
    pDescriptors: *const VkHostAddressRangeEXT,
) -> VkResult;
