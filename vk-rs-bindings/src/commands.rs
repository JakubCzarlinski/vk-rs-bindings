//! Vulkan command function pointer types (`PFN_vk*`).
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1ChromaSamplePosition;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1ColorPrimaries;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1FrameRestorationType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1FrameType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1InterpolationFilter;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1Level;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1MatrixCoefficients;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1Profile;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1ReferenceName;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1TransferCharacteristics;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoAV1TxMode;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::enums::StdVideoDecodeH264FieldOrderCount;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264AspectRatioIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264CabacInitIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264ChromaFormatIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264DisableDeblockingFilterIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264LevelIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264MemMgmtControlOp;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264ModificationOfPicNumsIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264NonVclNaluType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264PictureType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264PocType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264ProfileIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264SliceType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH264WeightedBipredIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265AspectRatioIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265ChromaFormatIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265LevelIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265PictureType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265ProfileIdc;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoH265SliceType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9ColorSpace;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9FrameType;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9InterpolationFilter;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9Level;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9Profile;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::StdVideoVP9ReferenceName;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::enums::VkAccelerationStructureBuildTypeKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::enums::VkAccelerationStructureCompatibilityKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_EXT_descriptor_buffer"
))]
use crate::enums::VkAccelerationStructureCreateFlagBitsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkAccelerationStructureMemoryRequirementsTypeNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::enums::VkAccelerationStructureMotionInstanceTypeNV;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::enums::VkAccelerationStructureSerializedBlockTypeKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkAccelerationStructureTypeKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkAccelerationStructureTypeNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkAccessFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_KHR_synchronization2",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_NV_optical_flow",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkAccessFlagBits2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::enums::VkAccessFlagBits2KHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::enums::VkAccessFlagBits3KHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::enums::VkAcquireProfilingLockFlagBitsKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::enums::VkAddressCommandFlagBitsKHR;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::enums::VkAddressCopyFlagBitsKHR;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::enums::VkAntiLagModeAMD;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::enums::VkAntiLagStageAMD;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_0", feature = "VK_KHR_maintenance10"))]
use crate::enums::VkAttachmentDescriptionFlagBits;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_load_store_op_none",
  feature = "VK_KHR_load_store_op_none"
))]
use crate::enums::VkAttachmentLoadOp;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_KHR_dynamic_rendering",
  feature = "VK_QCOM_render_pass_store_ops",
  feature = "VK_EXT_load_store_op_none"
))]
use crate::enums::VkAttachmentStoreOp;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkBlendFactor;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_blend_operation_advanced"
))]
use crate::enums::VkBlendOp;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::enums::VkBlendOverlapEXT;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::enums::VkBlockMatchWindowCompareModeQCOM;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_custom_border_color"
))]
use crate::enums::VkBorderColor;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkBufferUsageFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_KHR_maintenance5",
  feature = "VK_AMDX_dense_geometry_format",
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_EXT_memory_decompression",
  feature = "VK_EXT_device_generated_commands"
))]
use crate::enums::VkBufferUsageFlagBits2;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::enums::VkBufferUsageFlagBits2KHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkBuildAccelerationStructureFlagBitsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkBuildAccelerationStructureFlagBitsNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::enums::VkBuildAccelerationStructureModeKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkBuildMicromapFlagBitsEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkBuildMicromapModeEXT;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkChromaLocation;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::enums::VkChromaLocationKHR;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureAddressResolutionFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureClusterFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureGeometryFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureIndexFormatFlagBitsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureOpModeNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureOpTypeNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::enums::VkClusterAccelerationStructureTypeNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::enums::VkCoarseSampleOrderTypeNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkColorComponentFlagBits;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkColorSpaceKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferLevel;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferResetFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandBufferUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandPoolCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkCommandPoolResetFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkCompareOp;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkComponentSwizzle;
#[cfg(any(
  feature = "VK_NV_cooperative_vector",
  feature = "VK_KHR_cooperative_matrix",
  feature = "VK_NV_cooperative_matrix"
))]
use crate::enums::VkComponentTypeKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::enums::VkComponentTypeNV;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkCompositeAlphaFlagBitsKHR;
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
use crate::enums::VkCompressedTriangleFormatAMDX;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::enums::VkConditionalRenderingFlagBitsEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::enums::VkConservativeRasterizationModeEXT;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::enums::VkCooperativeVectorMatrixLayoutNV;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkCopyAccelerationStructureModeKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkCopyAccelerationStructureModeNV;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkCopyMicromapModeEXT;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
use crate::enums::VkCoverageModulationModeNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::enums::VkCoverageReductionModeNV;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::enums::VkCubicFilterWeightsQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkCullModeFlagBits;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::enums::VkDataGraphModelCacheTypeQCOM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowExecuteFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowGridSizeFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowImageUsageFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphOpticalFlowPerformanceLevelARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineDispatchFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphPipelineNodeConnectionTypeARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::enums::VkDataGraphPipelineNodeTypeARM;
#[cfg(any(
  feature = "VK_ARM_data_graph",
  feature = "VK_ARM_data_graph_neural_accelerator_statistics"
))]
use crate::enums::VkDataGraphPipelinePropertyARM;
#[cfg(any(
  feature = "VK_ARM_data_graph",
  feature = "VK_ARM_data_graph_neural_accelerator_statistics"
))]
use crate::enums::VkDataGraphPipelineSessionBindPointARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineSessionBindPointTypeARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkDataGraphPipelineSessionCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::enums::VkDataGraphTOSALevelARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::enums::VkDataGraphTOSAQualityFlagBitsARM;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::enums::VkDebugReportFlagBitsEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::enums::VkDebugReportObjectTypeEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::enums::VkDebugUtilsMessageSeverityFlagBitsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::enums::VkDebugUtilsMessageTypeFlagBitsEXT;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::enums::VkDefaultVertexAttributeValueKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDependencyFlagBits;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::enums::VkDepthBiasRepresentationEXT;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::enums::VkDepthClampModeEXT;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_2",
  feature = "VK_EXT_descriptor_indexing"
))]
use crate::enums::VkDescriptorBindingFlagBits;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::enums::VkDescriptorBindingFlagBitsEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::enums::VkDescriptorMappingSourceEXT;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_descriptor_indexing",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_EXT_mutable_descriptor_type"
))]
use crate::enums::VkDescriptorPoolCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_push_descriptor",
  feature = "VK_EXT_descriptor_indexing",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_VALVE_mutable_descriptor_type",
  feature = "VK_NV_device_generated_commands_compute",
  feature = "VK_EXT_mutable_descriptor_type",
  feature = "VK_NV_per_stage_descriptor_set"
))]
use crate::enums::VkDescriptorSetLayoutCreateFlagBits;
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
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_descriptor_update_template"
))]
use crate::enums::VkDescriptorUpdateTemplateType;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
use crate::enums::VkDescriptorUpdateTemplateTypeKHR;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::enums::VkDeviceAddressBindingFlagBitsEXT;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::enums::VkDeviceAddressBindingTypeEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDeviceCreateFlagBits;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::enums::VkDeviceDiagnosticsConfigFlagBitsNV;
#[cfg(feature = "VK_EXT_display_control")]
use crate::enums::VkDeviceEventTypeEXT;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::enums::VkDeviceFaultAddressTypeEXT;
#[cfg(any(feature = "VK_KHR_device_fault", feature = "VK_EXT_device_fault"))]
use crate::enums::VkDeviceFaultAddressTypeKHR;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::enums::VkDeviceFaultFlagBitsKHR;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::enums::VkDeviceFaultVendorBinaryHeaderVersionEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::enums::VkDeviceFaultVendorBinaryHeaderVersionKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
use crate::enums::VkDeviceGroupPresentModeFlagBitsKHR;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::enums::VkDeviceMemoryReportEventTypeEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkDeviceQueueCreateFlagBits;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::enums::VkDirectDriverLoadingModeLUNARG;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::enums::VkDiscardRectangleModeEXT;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::enums::VkDisplacementMicromapFormatNV;
#[cfg(feature = "VK_EXT_display_control")]
use crate::enums::VkDisplayEventTypeEXT;
#[cfg(feature = "VK_KHR_display")]
use crate::enums::VkDisplayPlaneAlphaFlagBitsKHR;
#[cfg(feature = "VK_EXT_display_control")]
use crate::enums::VkDisplayPowerStateEXT;
#[cfg(feature = "VK_NV_display_stereo")]
use crate::enums::VkDisplaySurfaceStereoTypeNV;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_driver_properties"))]
use crate::enums::VkDriverId;
#[cfg(feature = "VK_KHR_driver_properties")]
use crate::enums::VkDriverIdKHR;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_NV_clip_space_w_scaling",
  feature = "VK_EXT_discard_rectangles",
  feature = "VK_EXT_sample_locations",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_shading_rate_image",
  feature = "VK_NV_scissor_exclusive",
  feature = "VK_KHR_fragment_shading_rate",
  feature = "VK_EXT_line_rasterization",
  feature = "VK_EXT_extended_dynamic_state",
  feature = "VK_EXT_vertex_input_dynamic_state",
  feature = "VK_EXT_extended_dynamic_state2",
  feature = "VK_EXT_color_write_enable",
  feature = "VK_EXT_extended_dynamic_state3",
  feature = "VK_EXT_attachment_feedback_loop_dynamic_state",
  feature = "VK_KHR_line_rasterization",
  feature = "VK_EXT_depth_clamp_control"
))]
use crate::enums::VkDynamicState;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_synchronization2"
))]
use crate::enums::VkEventCreateFlagBits;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::enums::VkExportMetalObjectTypeFlagBitsEXT;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::enums::VkExternalFenceFeatureFlagBits;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::enums::VkExternalFenceFeatureFlagBitsKHR;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::enums::VkExternalFenceHandleTypeFlagBits;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::enums::VkExternalFenceHandleTypeFlagBitsKHR;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_memory_capabilities"
))]
use crate::enums::VkExternalMemoryFeatureFlagBits;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::enums::VkExternalMemoryFeatureFlagBitsKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::enums::VkExternalMemoryFeatureFlagBitsNV;
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
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::enums::VkExternalMemoryHandleTypeFlagBitsKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::enums::VkExternalMemoryHandleTypeFlagBitsNV;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreFeatureFlagBits;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::enums::VkExternalSemaphoreFeatureFlagBitsKHR;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_external_semaphore_capabilities"
))]
use crate::enums::VkExternalSemaphoreHandleTypeFlagBits;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::enums::VkExternalSemaphoreHandleTypeFlagBitsKHR;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkFaultLevel;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkFaultQueryBehavior;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkFaultType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFenceCreateFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_external_fence"))]
use crate::enums::VkFenceImportFlagBits;
#[cfg(feature = "VK_KHR_external_fence")]
use crate::enums::VkFenceImportFlagBitsKHR;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_IMG_filter_cubic",
  feature = "VK_EXT_filter_cubic"
))]
use crate::enums::VkFilter;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormat;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkFormatFeatureFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_EXT_host_image_copy",
  all(
    feature = "VK_QCOM_image_processing",
    feature = "VK_QCOM_image_processing3"
  ),
  feature = "VK_KHR_format_feature_flags2",
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  feature = "VK_NV_optical_flow",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::enums::VkFormatFeatureFlagBits2;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
use crate::enums::VkFormatFeatureFlagBits2KHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::enums::VkFragmentShadingRateCombinerOpKHR;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::enums::VkFragmentShadingRateNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::enums::VkFragmentShadingRateTypeNV;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::enums::VkFrameBoundaryFlagBitsEXT;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_KHR_imageless_framebuffer"
))]
use crate::enums::VkFramebufferCreateFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkFrontFace;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::enums::VkFullScreenExclusiveEXT;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryFlagBitsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkGeometryFlagBitsNV;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryInstanceFlagBitsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkGeometryInstanceFlagBitsNV;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_ray_tracing"
))]
use crate::enums::VkGeometryTypeKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkGeometryTypeNV;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaDeviceClockModeAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaPerfBlockAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaSampleTypeAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::enums::VkGpaSqShaderStageFlagBitsAMD;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::enums::VkGraphicsPipelineLibraryFlagBitsEXT;
#[cfg(any(feature = "VK_BASE_VERSION_1_4", feature = "VK_EXT_host_image_copy"))]
use crate::enums::VkHostImageCopyFlagBits;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::enums::VkHostImageCopyFlagBitsEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageAspectFlagBits;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::enums::VkImageCompressionFixedRateFlagBitsEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::enums::VkImageCompressionFlagBitsEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::enums::VkImageConstraintsInfoFlagBitsFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageCreateFlagBits;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::enums::VkImageFormatConstraintsFlagBitsFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageLayout;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageTiling;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageUsageFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageViewCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkImageViewType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkIndexType;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::enums::VkIndirectCommandsInputModeFlagBitsEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::enums::VkIndirectCommandsLayoutUsageFlagBitsEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectCommandsLayoutUsageFlagBitsNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::enums::VkIndirectCommandsTokenTypeEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectCommandsTokenTypeNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::enums::VkIndirectExecutionSetInfoTypeEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::enums::VkIndirectStateFlagBitsNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkInstanceCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkInternalAllocationType;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::enums::VkLatencyMarkerNV;
#[cfg(feature = "VK_EXT_layer_settings")]
use crate::enums::VkLayerSettingTypeEXT;
#[cfg(feature = "VK_MSFT_layered_driver")]
use crate::enums::VkLayeredDriverUnderlyingApiMSFT;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_4",
  feature = "VK_EXT_line_rasterization",
  feature = "VK_KHR_line_rasterization"
))]
use crate::enums::VkLineRasterizationMode;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::enums::VkLineRasterizationModeEXT;
#[cfg(feature = "VK_KHR_line_rasterization")]
use crate::enums::VkLineRasterizationModeKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkLogicOp;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_device_group",
  feature = "VK_KHR_buffer_device_address",
  feature = "VK_EXT_zero_initialize_device_memory"
))]
use crate::enums::VkMemoryAllocateFlagBits;
#[cfg(feature = "VK_KHR_device_group")]
use crate::enums::VkMemoryAllocateFlagBitsKHR;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::enums::VkMemoryDecompressionMethodFlagBitsEXT;
#[cfg(feature = "VK_NV_memory_decompression")]
use crate::enums::VkMemoryDecompressionMethodFlagBitsNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryHeapFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryMapFlagBits;
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
use crate::enums::VkMemoryOverallocationBehaviorAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkMemoryPropertyFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_4", feature = "VK_EXT_map_memory_placed"))]
use crate::enums::VkMemoryUnmapFlagBits;
#[cfg(feature = "VK_KHR_map_memory2")]
use crate::enums::VkMemoryUnmapFlagBitsKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkMicromapCreateFlagBitsEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkMicromapTypeEXT;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::enums::VkNeuralAcceleratorStatisticsModeARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkObjectType;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkOpacityMicromapFormatEXT;
#[cfg(any(
  feature = "VK_KHR_opacity_micromap",
  feature = "VK_EXT_opacity_micromap"
))]
use crate::enums::VkOpacityMicromapFormatKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::enums::VkOpacityMicromapSpecialIndexEXT;
#[cfg(any(
  feature = "VK_KHR_opacity_micromap",
  feature = "VK_EXT_opacity_micromap"
))]
use crate::enums::VkOpacityMicromapSpecialIndexKHR;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowExecuteFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowGridSizeFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowPerformanceLevelNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowSessionBindingPointNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowSessionCreateFlagBitsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::enums::VkOpticalFlowUsageFlagBitsNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::enums::VkOutOfBandQueueTypeNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::enums::VkPartitionedAccelerationStructureInstanceFlagBitsNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::enums::VkPartitionedAccelerationStructureOpTypeNV;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::enums::VkPastPresentationTimingFlagBitsEXT;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_device_group"))]
use crate::enums::VkPeerMemoryFeatureFlagBits;
#[cfg(feature = "VK_KHR_device_group")]
use crate::enums::VkPeerMemoryFeatureFlagBitsKHR;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::enums::VkPerfHintTypeQCOM;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceConfigurationTypeINTEL;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::enums::VkPerformanceCounterDescriptionFlagBitsARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::enums::VkPerformanceCounterDescriptionFlagBitsKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::enums::VkPerformanceCounterScopeKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::enums::VkPerformanceCounterStorageKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::enums::VkPerformanceCounterUnitKHR;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceOverrideTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceParameterTypeINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkPerformanceValueTypeINTEL;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkPhysicalDeviceDataGraphOperationTypeARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::enums::VkPhysicalDeviceDataGraphProcessingEngineTypeARM;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::enums::VkPhysicalDeviceLayeredApiKHR;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::enums::VkPhysicalDeviceSchedulingControlsFlagBitsARM;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPhysicalDeviceType;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_AMDX_shader_enqueue",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  feature = "VK_HUAWEI_subpass_shading"
))]
use crate::enums::VkPipelineBindPoint;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_pipeline_creation_cache_control"
))]
use crate::enums::VkPipelineCacheCreateFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkPipelineCacheHeaderVersion;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkPipelineCacheValidationVersion;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_ARM_rasterization_order_attachment_access"
))]
use crate::enums::VkPipelineColorBlendStateCreateFlagBits;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::enums::VkPipelineCompilerControlFlagBitsAMD;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_device_group",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_ray_tracing",
  all(
    feature = "VK_EXT_fragment_density_map",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_fragment_shading_rate"
  ),
  feature = "VK_KHR_pipeline_executable_properties",
  feature = "VK_NV_device_generated_commands",
  feature = "VK_KHR_pipeline_library",
  feature = "VK_EXT_pipeline_creation_cache_control",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_attachment_feedback_loop_layout",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_EXT_pipeline_protected_access",
  feature = "VK_KHR_opacity_micromap"
))]
use crate::enums::VkPipelineCreateFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_4",
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_KHR_maintenance5",
  feature = "VK_KHR_pipeline_binary",
  feature = "VK_EXT_device_generated_commands",
  feature = "VK_VALVE_fragment_density_map_layered",
  feature = "VK_EXT_shader_64bit_indexing"
))]
use crate::enums::VkPipelineCreateFlagBits2;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::enums::VkPipelineCreateFlagBits2KHR;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_3",
  feature = "VK_EXT_pipeline_creation_feedback"
))]
use crate::enums::VkPipelineCreationFeedbackFlagBits;
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
use crate::enums::VkPipelineCreationFeedbackFlagBitsEXT;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_ARM_rasterization_order_attachment_access"
))]
use crate::enums::VkPipelineDepthStencilStateCreateFlagBits;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::enums::VkPipelineExecutableStatisticFormatKHR;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_graphics_pipeline_library",
  all(
    feature = "VK_EXT_mesh_shader",
    feature = "VK_EXT_shader_object",
    feature = "VK_KHR_maintenance11"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_KHR_maintenance11",
    feature = "VK_NV_mesh_shader"
  )
))]
use crate::enums::VkPipelineLayoutCreateFlagBits;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::enums::VkPipelineMatchControl;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_pipeline_robustness"
))]
use crate::enums::VkPipelineRobustnessBufferBehavior;
#[cfg(feature = "VK_EXT_pipeline_robustness")]
use crate::enums::VkPipelineRobustnessBufferBehaviorEXT;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_pipeline_robustness"
))]
use crate::enums::VkPipelineRobustnessImageBehavior;
#[cfg(feature = "VK_EXT_pipeline_robustness")]
use crate::enums::VkPipelineRobustnessImageBehaviorEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkPipelineShaderStageCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkPipelineStageFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_3",
  feature = "VK_KHR_video_decode_queue",
  feature = "VK_KHR_video_encode_queue",
  feature = "VK_KHR_synchronization2",
  feature = "VK_HUAWEI_subpass_shading",
  feature = "VK_HUAWEI_invocation_mask",
  feature = "VK_EXT_opacity_micromap",
  feature = "VK_HUAWEI_cluster_culling_shader",
  feature = "VK_NV_optical_flow",
  feature = "VK_NV_cooperative_vector",
  feature = "VK_KHR_copy_memory_indirect",
  feature = "VK_EXT_memory_decompression"
))]
use crate::enums::VkPipelineStageFlagBits2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::enums::VkPipelineStageFlagBits2KHR;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_maintenance2"))]
use crate::enums::VkPointClippingBehavior;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::enums::VkPointClippingBehaviorKHR;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_0", feature = "VK_NV_fill_rectangle"))]
use crate::enums::VkPolygonMode;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::enums::VkPresentGravityFlagBitsEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::enums::VkPresentGravityFlagBitsKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkPresentModeKHR;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::enums::VkPresentScalingFlagBitsEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::enums::VkPresentScalingFlagBitsKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::enums::VkPresentStageFlagBitsEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::enums::VkPresentTimingInfoFlagBitsEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkPrimitiveTopology;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::enums::VkPrivateDataSlotCreateFlagBits;
#[cfg(feature = "VK_EXT_private_data")]
use crate::enums::VkPrivateDataSlotCreateFlagBitsEXT;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::enums::VkProvokingVertexModeEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryControlFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryPipelineStatisticFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryPoolCreateFlagBits;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::enums::VkQueryPoolSamplingModeINTEL;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryResultFlagBits;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkQueryResultStatusKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueryType;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkQueueFlagBits;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_4",
  feature = "VK_EXT_global_priority",
  feature = "VK_KHR_global_priority"
))]
use crate::enums::VkQueueGlobalPriority;
#[cfg(feature = "VK_EXT_global_priority")]
use crate::enums::VkQueueGlobalPriorityEXT;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::enums::VkQueueGlobalPriorityKHR;
#[cfg(feature = "VK_AMD_rasterization_order")]
use crate::enums::VkRasterizationOrderAMD;
#[cfg(any(
  feature = "VK_EXT_ray_tracing_invocation_reorder",
  feature = "VK_NV_ray_tracing_invocation_reorder"
))]
use crate::enums::VkRayTracingInvocationReorderModeEXT;
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
use crate::enums::VkRayTracingInvocationReorderModeNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::enums::VkRayTracingLssIndexingModeNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::enums::VkRayTracingLssPrimitiveEndCapsModeNV;
#[cfg(any(feature = "VK_KHR_ray_tracing_pipeline", feature = "VK_NV_ray_tracing"))]
use crate::enums::VkRayTracingShaderGroupTypeKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::enums::VkRayTracingShaderGroupTypeNV;
#[cfg(feature = "VK_KHR_object_refresh")]
use crate::enums::VkRefreshObjectFlagBitsKHR;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_QCOM_render_pass_transform",
  feature = "VK_VALVE_fragment_density_map_layered"
))]
use crate::enums::VkRenderPassCreateFlagBits;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::enums::VkRenderingAttachmentFlagBitsKHR;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_3",
  feature = "VK_KHR_dynamic_rendering",
  feature = "VK_EXT_nested_command_buffer",
  feature = "VK_KHR_maintenance7",
  feature = "VK_VALVE_fragment_density_map_layered"
))]
use crate::enums::VkRenderingFlagBits;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::enums::VkRenderingFlagBitsKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::enums::VkResolveImageFlagBitsKHR;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_2",
  feature = "VK_KHR_depth_stencil_resolve",
  all(
    feature = "VK_ANDROID_external_format_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::enums::VkResolveModeFlagBits;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
use crate::enums::VkResolveModeFlagBitsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkResult;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSampleCountFlagBits;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_KHR_sampler_mirror_clamp_to_edge"
))]
use crate::enums::VkSamplerAddressMode;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_0",
  feature = "VK_EXT_fragment_density_map",
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_non_seamless_cube_map",
  feature = "VK_QCOM_image_processing"
))]
use crate::enums::VkSamplerCreateFlagBits;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkSamplerMipmapMode;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_2",
  feature = "VK_EXT_sampler_filter_minmax",
  feature = "VK_QCOM_filter_cubic_clamp"
))]
use crate::enums::VkSamplerReductionMode;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
use crate::enums::VkSamplerReductionModeEXT;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkSamplerYcbcrModelConversion;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::enums::VkSamplerYcbcrModelConversionKHR;
#[cfg(any(
  feature = "VK_COMPUTE_VERSION_1_1",
  feature = "VK_KHR_sampler_ycbcr_conversion"
))]
use crate::enums::VkSamplerYcbcrRange;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::enums::VkSamplerYcbcrRangeKHR;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::enums::VkSciSyncClientTypeNV;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::enums::VkSciSyncPrimitiveTypeNV;
#[cfg(any(
  feature = "VK_KHR_cooperative_matrix",
  feature = "VK_NV_cooperative_matrix"
))]
use crate::enums::VkScopeKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::enums::VkScopeNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSemaphoreCreateFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_1", feature = "VK_KHR_external_semaphore"))]
use crate::enums::VkSemaphoreImportFlagBits;
#[cfg(feature = "VK_KHR_external_semaphore")]
use crate::enums::VkSemaphoreImportFlagBitsKHR;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_timeline_semaphore"))]
use crate::enums::VkSemaphoreType;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::enums::VkSemaphoreTypeKHR;
#[cfg(any(feature = "VK_BASE_VERSION_1_2", feature = "VK_KHR_timeline_semaphore"))]
use crate::enums::VkSemaphoreWaitFlagBits;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::enums::VkSemaphoreWaitFlagBitsKHR;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::enums::VkShaderCodeTypeEXT;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::enums::VkShaderCorePropertiesFlagBitsAMD;
#[cfg(any(
  feature = "VK_EXT_shader_object",
  all(
    feature = "VK_ARM_shader_instrumentation",
    feature = "VK_KHR_maintenance5"
  ),
  feature = "VK_EXT_device_generated_commands",
  feature = "VK_EXT_shader_64bit_indexing"
))]
use crate::enums::VkShaderCreateFlagBitsEXT;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_2",
  feature = "VK_KHR_shader_float_controls"
))]
use crate::enums::VkShaderFloatControlsIndependence;
#[cfg(feature = "VK_KHR_shader_float_controls")]
use crate::enums::VkShaderFloatControlsIndependenceKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::enums::VkShaderGroupShaderKHR;
#[cfg(feature = "VK_AMD_shader_info")]
use crate::enums::VkShaderInfoTypeAMD;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::enums::VkShaderModuleCreateFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkShaderStageFlagBits;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::enums::VkShadingRatePaletteEntryNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSharingMode;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSparseImageFormatFlagBits;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSparseMemoryBindFlagBits;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::enums::VkSpirvResourceTypeFlagBitsEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkStencilFaceFlagBits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkStencilOp;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkStructureType;
#[cfg(any(
  feature = "VK_BASE_VERSION_1_1",
  feature = "VK_KHR_shader_subgroup_rotate",
  feature = "VK_EXT_shader_subgroup_partitioned"
))]
use crate::enums::VkSubgroupFeatureFlagBits;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_KHR_synchronization2"))]
use crate::enums::VkSubmitFlagBits;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::enums::VkSubmitFlagBitsKHR;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_EXT_nested_command_buffer"
))]
use crate::enums::VkSubpassContents;
#[cfg(any(
  feature = "VK_GRAPHICS_VERSION_1_0",
  feature = "VK_NVX_multiview_per_view_attributes",
  feature = "VK_QCOM_render_pass_shader_resolve",
  feature = "VK_QCOM_tile_shading",
  feature = "VK_ARM_rasterization_order_attachment_access",
  feature = "VK_EXT_rasterization_order_attachment_access",
  feature = "VK_EXT_legacy_dithering",
  feature = "VK_EXT_custom_resolve"
))]
use crate::enums::VkSubpassDescriptionFlagBits;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::enums::VkSubpassMergeStatusEXT;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::enums::VkSurfaceCounterFlagBitsEXT;
#[cfg(feature = "VK_KHR_surface")]
use crate::enums::VkSurfaceTransformFlagBitsKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::enums::VkSwapchainCreateFlagBitsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkSystemAllocationScope;
#[cfg(feature = "VK_ARM_tensors")]
use crate::enums::VkTensorCreateFlagBitsARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::enums::VkTensorTilingARM;
#[cfg(any(feature = "VK_ARM_tensors", feature = "VK_ARM_data_graph"))]
use crate::enums::VkTensorUsageFlagBitsARM;
#[cfg(any(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_heap"))]
use crate::enums::VkTensorViewCreateFlagBitsARM;
#[cfg(any(feature = "VK_GRAPHICS_VERSION_1_1", feature = "VK_KHR_maintenance2"))]
use crate::enums::VkTessellationDomainOrigin;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::enums::VkTessellationDomainOriginKHR;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::enums::VkThrottleHintTypeSEC;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::enums::VkTileShadingRenderPassFlagBitsQCOM;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::enums::VkTimeDomainEXT;
#[cfg(any(
  feature = "VK_KHR_calibrated_timestamps",
  feature = "VK_EXT_calibrated_timestamps"
))]
use crate::enums::VkTimeDomainKHR;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_EXT_tooling_info"))]
use crate::enums::VkToolPurposeFlagBits;
#[cfg(feature = "VK_EXT_tooling_info")]
use crate::enums::VkToolPurposeFlagBitsEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::enums::VkValidationCacheHeaderVersionEXT;
#[cfg(feature = "VK_EXT_validation_flags")]
use crate::enums::VkValidationCheckEXT;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::enums::VkValidationFeatureDisableEXT;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::enums::VkValidationFeatureEnableEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::enums::VkVendorId;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::enums::VkVertexInputRate;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoCapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoChromaSubsamplingFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoCodecOperationFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoCodingControlFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoComponentBitDepthFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::enums::VkVideoDecodeCapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::enums::VkVideoDecodeH264PictureLayoutFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::enums::VkVideoDecodeUsageFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1CapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1PredictionModeKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1RateControlFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1RateControlGroupKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1StdFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::enums::VkVideoEncodeAV1SuperblockSizeFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeCapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeContentFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeFeedbackFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::enums::VkVideoEncodeH264CapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::enums::VkVideoEncodeH264RateControlFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::enums::VkVideoEncodeH264StdFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::enums::VkVideoEncodeH265CapabilityFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::enums::VkVideoEncodeH265CtbSizeFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::enums::VkVideoEncodeH265RateControlFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::enums::VkVideoEncodeH265StdFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::enums::VkVideoEncodeH265TransformBlockSizeFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::enums::VkVideoEncodeIntraRefreshModeFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeRateControlModeFlagBitsKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbChromaOffsetFlagBitsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbModelConversionFlagBitsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::enums::VkVideoEncodeRgbRangeCompressionFlagBitsVALVE;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeTuningModeKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::enums::VkVideoEncodeUsageFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoSessionCreateFlagBitsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::enums::VkVideoSessionParametersCreateFlagBitsKHR;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::enums::VkViewportCoordinateSwizzleNV;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::enums::VkWaylandSurfaceCreateFlagBitsKHR;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::_screen_buffer;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::types::_screen_context;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::types::_screen_window;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::AHardwareBuffer;
#[cfg(feature = "VK_KHR_android_surface")]
use crate::types::ANativeWindow;
#[cfg(feature = "VK_EXT_metal_surface")]
use crate::types::CAMetalLayer;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::DWORD;
#[cfg(any(
  feature = "VK_KHR_xlib_surface",
  feature = "VK_EXT_acquire_xlib_display"
))]
use crate::types::Display;
#[cfg(feature = "VK_GGP_frame_token")]
use crate::types::GgpFrameToken;
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
use crate::types::GgpStreamDescriptor;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::HANDLE;
#[cfg(feature = "VK_KHR_win32_surface")]
use crate::types::HINSTANCE;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::types::HMONITOR;
#[cfg(feature = "VK_KHR_win32_surface")]
use crate::types::HWND;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::types::IDirectFB;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::types::IDirectFBSurface;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::IOSurfaceRef;
#[cfg(any(
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::LPCWSTR;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::MTLBuffer_id;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::MTLCommandQueue_id;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::MTLDevice_id;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::MTLSharedEvent_id;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::MTLTexture_id;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::NvSciBufAttrList;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::NvSciBufObj;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::NvSciSyncAttrList;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::NvSciSyncFence;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::NvSciSyncObj;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::OH_NativeBuffer;
#[cfg(feature = "VK_OHOS_surface")]
use crate::types::OHNativeWindow;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkAllocationFunction;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::types::PFN_vkDebugReportCallbackEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::PFN_vkDebugUtilsMessengerCallbackEXT;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::PFN_vkDeviceMemoryReportCallbackEXT;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::PFN_vkFaultCallbackFunction;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkFreeFunction;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::types::PFN_vkGetInstanceProcAddrLUNARG;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkInternalAllocationNotification;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkInternalFreeNotification;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkReallocationFunction;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::PFN_vkVoidFunction;
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
use crate::types::RROutput;
#[cfg(any(
  feature = "VK_NV_external_memory_win32",
  feature = "VK_KHR_external_memory_win32",
  feature = "VK_KHR_external_semaphore_win32",
  feature = "VK_KHR_external_fence_win32"
))]
use crate::types::SECURITY_ATTRIBUTES;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1CDEF;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1ColorConfig;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1ColorConfigFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1FilmGrain;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1FilmGrainFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1GlobalMotion;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1LoopFilter;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1LoopFilterFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1LoopRestoration;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1Quantization;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1QuantizationFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1Segmentation;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1SequenceHeader;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1SequenceHeaderFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1TileInfo;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1TileInfoFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1TimingInfo;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoAV1TimingInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::StdVideoDecodeAV1PictureInfo;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::StdVideoDecodeAV1PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::StdVideoDecodeAV1ReferenceInfo;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::StdVideoDecodeAV1ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::StdVideoDecodeH264PictureInfo;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::StdVideoDecodeH264PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::StdVideoDecodeH264ReferenceInfo;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::StdVideoDecodeH264ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::StdVideoDecodeH265PictureInfo;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::StdVideoDecodeH265PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::StdVideoDecodeH265ReferenceInfo;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::StdVideoDecodeH265ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::StdVideoDecodeVP9PictureInfo;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::StdVideoDecodeVP9PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1DecoderModelInfo;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1ExtensionHeader;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1OperatingPointInfo;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1OperatingPointInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1PictureInfo;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1ReferenceInfo;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::StdVideoEncodeAV1ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264PictureInfo;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264RefListModEntry;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264RefPicMarkingEntry;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264ReferenceInfo;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264ReferenceListsInfo;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264ReferenceListsInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264SliceHeader;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264SliceHeaderFlags;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264WeightTable;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::StdVideoEncodeH264WeightTableFlags;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265LongTermRefPics;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265PictureInfo;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265PictureInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265ReferenceInfo;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265ReferenceInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265ReferenceListsInfo;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265ReferenceListsInfoFlags;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265SliceSegmentHeader;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265SliceSegmentHeaderFlags;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265WeightTable;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::StdVideoEncodeH265WeightTableFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264HrdParameters;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264PictureParameterSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264PpsFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264ScalingLists;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264SequenceParameterSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264SequenceParameterSetVui;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264SpsFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH264SpsVuiFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265DecPicBufMgr;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265HrdFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265HrdParameters;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265LongTermRefPicsSps;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265PictureParameterSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265PpsFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265PredictorPaletteEntries;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265ProfileTierLevel;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265ProfileTierLevelFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265ScalingLists;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265SequenceParameterSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265SequenceParameterSetVui;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265ShortTermRefPicSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265ShortTermRefPicSetFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265SpsFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265SpsVuiFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265SubLayerHrdParameters;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265VideoParameterSet;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoH265VpsFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9ColorConfig;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9ColorConfigFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9LoopFilter;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9LoopFilterFlags;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9Segmentation;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::StdVideoVP9SegmentationFlags;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::types::VisualID;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAabbPositionsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAabbPositionsNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureBuildGeometryInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureBuildRangeInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureBuildSizesInfoKHR;
#[cfg(any(
  all(
    feature = "VK_EXT_descriptor_buffer",
    feature = "VK_KHR_acceleration_structure"
  ),
  all(feature = "VK_EXT_descriptor_buffer", feature = "VK_NV_ray_tracing")
))]
use crate::types::VkAccelerationStructureCaptureDescriptorDataInfoEXT;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureCreateFlagsKHR;
#[cfg(all(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_KHR_device_address_commands"
))]
use crate::types::VkAccelerationStructureCreateInfo2KHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAccelerationStructureCreateInfoNV;
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
use crate::types::VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureDeviceAddressInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryAabbsDataKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryDataKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryInstancesDataKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryKHR;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::types::VkAccelerationStructureGeometryLinearSweptSpheresDataNV;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkAccelerationStructureGeometryMicromapDataKHR;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureGeometryMotionTrianglesDataNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::types::VkAccelerationStructureGeometrySpheresDataNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureGeometryTrianglesDataKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAccelerationStructureInfoNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureInstanceKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAccelerationStructureInstanceNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureKHR;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMatrixMotionInstanceNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAccelerationStructureMemoryRequirementsInfoNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMotionInfoFlagsNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMotionInfoNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMotionInstanceDataNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMotionInstanceFlagsNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureMotionInstanceNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkAccelerationStructureNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkAccelerationStructureSRTMotionInstanceNV;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkAccelerationStructureTrianglesDisplacementMicromapNV;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkAccelerationStructureTrianglesOpacityMicromapEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkAccelerationStructureTrianglesOpacityMicromapKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkAccelerationStructureVersionInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAccessFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkAccessFlags2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkAccessFlags2KHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkAccessFlags3KHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkAcquireNextImageInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkAcquireProfilingLockFlagsKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkAcquireProfilingLockInfoKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkAddressCommandFlagsKHR;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkAddressCopyFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkAllocationCallbacks;
#[cfg(feature = "VK_SEC_amigo_profiling")]
use crate::types::VkAmigoProfilingSubmitInfoSEC;
#[cfg(any(
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_ANDROID_external_memory_android_hardware_buffer",
    feature = "VK_VERSION_1_3"
  )
))]
use crate::types::VkAndroidHardwareBufferFormatProperties2ANDROID;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkAndroidHardwareBufferFormatPropertiesANDROID;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkAndroidHardwareBufferFormatResolvePropertiesANDROID;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkAndroidHardwareBufferPropertiesANDROID;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkAndroidHardwareBufferUsageANDROID;
#[cfg(feature = "VK_KHR_android_surface")]
use crate::types::VkAndroidSurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_android_surface")]
use crate::types::VkAndroidSurfaceCreateInfoKHR;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::types::VkAntiLagDataAMD;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::types::VkAntiLagPresentationInfoAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkApplicationInfo;
#[cfg(feature = "VK_EXT_application_parameters")]
use crate::types::VkApplicationParametersEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkAttachmentDescription;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkAttachmentDescription2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkAttachmentDescription2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkAttachmentDescriptionFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkAttachmentDescriptionStencilLayout;
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
use crate::types::VkAttachmentDescriptionStencilLayoutKHR;
#[cfg(any(
  all(
    feature = "VK_EXT_attachment_feedback_loop_layout",
    feature = "VK_KHR_unified_image_layouts",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_EXT_attachment_feedback_loop_layout",
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_unified_image_layouts"
  )
))]
use crate::types::VkAttachmentFeedbackLoopInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkAttachmentReference;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkAttachmentReference2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkAttachmentReference2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkAttachmentReferenceStencilLayout;
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
use crate::types::VkAttachmentReferenceStencilLayoutKHR;
#[cfg(any(
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_AMD_mixed_attachment_samples",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::types::VkAttachmentSampleCountInfoAMD;
#[cfg(any(
  all(
    feature = "VK_NV_framebuffer_mixed_samples",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
))]
use crate::types::VkAttachmentSampleCountInfoNV;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkAttachmentSampleLocationsEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBaseInStructure;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBaseOutStructure;
#[cfg(any(
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
))]
use crate::types::VkBeginCustomResolveInfoEXT;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkBindAccelerationStructureMemoryInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindBufferMemoryDeviceGroupInfo;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]
use crate::types::VkBindBufferMemoryDeviceGroupInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindBufferMemoryInfo;
#[cfg(feature = "VK_KHR_bind_memory2")]
use crate::types::VkBindBufferMemoryInfoKHR;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkBindDataGraphPipelineSessionMemoryInfoARM;
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
use crate::types::VkBindDescriptorBufferEmbeddedSamplersInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkBindDescriptorSetsInfo;
#[cfg(feature = "VK_KHR_maintenance6")]
use crate::types::VkBindDescriptorSetsInfoKHR;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkBindHeapInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindImageMemoryDeviceGroupInfo;
#[cfg(all(feature = "VK_KHR_bind_memory2", feature = "VK_KHR_device_group"))]
use crate::types::VkBindImageMemoryDeviceGroupInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindImageMemoryInfo;
#[cfg(feature = "VK_KHR_bind_memory2")]
use crate::types::VkBindImageMemoryInfoKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkBindImageMemorySwapchainInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBindImagePlaneMemoryInfo;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkBindImagePlaneMemoryInfoKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkBindIndexBuffer3InfoKHR;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkBindIndexBufferIndirectCommandEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkBindIndexBufferIndirectCommandNV;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBindMemoryStatus;
#[cfg(feature = "VK_KHR_maintenance6")]
use crate::types::VkBindMemoryStatusKHR;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkBindPipelineIndirectCommandNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkBindShaderGroupIndirectCommandNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBindSparseInfo;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkBindTensorMemoryInfoARM;
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  feature = "VK_KHR_device_address_commands"
))]
use crate::types::VkBindTransformFeedbackBuffer2InfoEXT;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkBindVertexBuffer3InfoKHR;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkBindVertexBufferIndirectCommandEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkBindVertexBufferIndirectCommandNV;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkBindVideoSessionMemoryInfoKHR;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkBlitImageCubicWeightsInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkBlitImageInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkBlitImageInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBool32;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBuffer;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkBufferCaptureDescriptorDataInfoEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionBufferCreateInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionCreateInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionImageCreateInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferCollectionPropertiesFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkBufferConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCopy;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkBufferCopy2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkBufferCopy2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferCreateInfo;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkBufferDeviceAddressCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkBufferDeviceAddressInfo;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkBufferDeviceAddressInfoEXT;
#[cfg(feature = "VK_KHR_buffer_device_address")]
use crate::types::VkBufferDeviceAddressInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferImageCopy;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkBufferImageCopy2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkBufferImageCopy2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferMemoryBarrier;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkBufferMemoryBarrier2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkBufferMemoryBarrier2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkBufferMemoryRequirementsInfo2;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
use crate::types::VkBufferMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkBufferOpaqueCaptureAddressCreateInfo;
#[cfg(feature = "VK_KHR_buffer_device_address")]
use crate::types::VkBufferOpaqueCaptureAddressCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkBufferUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBufferUsageFlags2;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkBufferUsageFlags2CreateInfo;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkBufferUsageFlags2CreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkBufferUsageFlags2KHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkBufferView;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkBufferViewCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkBufferViewCreateInfo;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkBuildAccelerationStructureFlagsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkBuildAccelerationStructureFlagsNV;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkBuildMicromapFlagsEXT;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkBuildPartitionedAccelerationStructureIndirectCommandNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkBuildPartitionedAccelerationStructureInfoNV;
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
use crate::types::VkCalibratedTimestampInfoEXT;
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
use crate::types::VkCalibratedTimestampInfoKHR;
#[cfg(any(
  all(
    feature = "VK_NV_device_diagnostic_checkpoints",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_synchronization2",
    feature = "VK_NV_device_diagnostic_checkpoints"
  )
))]
use crate::types::VkCheckpointData2NV;
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
use crate::types::VkCheckpointDataNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkClearAttachment;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkClearColorValue;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkClearDepthStencilValue;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkClearRect;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkClearValue;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureAddressResolutionFlagsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureBuildTriangleClusterInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureClusterFlagsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureClustersBottomLevelInputNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureCommandsInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureGeometryFlagsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureGetTemplateIndicesInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureIndexFormatFlagsNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureInputInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureInstantiateClusterInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureMoveObjectsInfoNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureMoveObjectsInputNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureOpInputNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkClusterAccelerationStructureTriangleClusterInputNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkCoarseSampleLocationNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkCoarseSampleOrderCustomNV;
#[cfg(any(
  feature = "VK_EXT_extended_dynamic_state3",
  feature = "VK_EXT_shader_object"
))]
use crate::types::VkColorBlendAdvancedEXT;
#[cfg(any(
  feature = "VK_EXT_extended_dynamic_state3",
  feature = "VK_EXT_shader_object"
))]
use crate::types::VkColorBlendEquationEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkColorComponentFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBuffer;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferAllocateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferBeginInfo;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkCommandBufferInheritanceConditionalRenderingInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkCommandBufferInheritanceDescriptorHeapInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferInheritanceInfo;
#[cfg(feature = "VK_QCOM_render_pass_transform")]
use crate::types::VkCommandBufferInheritanceRenderPassTransformInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkCommandBufferInheritanceRenderingInfo;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::types::VkCommandBufferInheritanceRenderingInfoKHR;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
use crate::types::VkCommandBufferInheritanceViewportScissorInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferResetFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCommandBufferSubmitInfo;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkCommandBufferSubmitInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandBufferUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPool;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPoolCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPoolCreateInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkCommandPoolMemoryConsumption;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkCommandPoolMemoryReservationCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkCommandPoolResetFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkCommandPoolTrimFlags;
#[cfg(feature = "VK_KHR_maintenance1")]
use crate::types::VkCommandPoolTrimFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkComponentMapping;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkCompositeAlphaFlagsKHR;
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
use crate::types::VkComputeOccupancyPriorityParametersNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkComputePipelineCreateInfo;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkComputePipelineIndirectBufferInfoNV;
#[cfg(all(
  feature = "VK_EXT_conditional_rendering",
  feature = "VK_KHR_device_address_commands"
))]
use crate::types::VkConditionalRenderingBeginInfo2EXT;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkConditionalRenderingBeginInfoEXT;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkConditionalRenderingFlagsEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkConformanceVersion;
#[cfg(feature = "VK_KHR_driver_properties")]
use crate::types::VkConformanceVersionKHR;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkConvertCooperativeVectorMatrixInfoNV;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkCooperativeMatrixFlexibleDimensionsPropertiesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkCooperativeMatrixPropertiesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkCooperativeVectorPropertiesNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkCopyAccelerationStructureInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkCopyAccelerationStructureToMemoryInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCopyBufferInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkCopyBufferInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCopyBufferToImageInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkCopyBufferToImageInfo2KHR;
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
use crate::types::VkCopyCommandTransformInfoQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkCopyDescriptorSet;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkCopyDeviceMemoryImageInfoKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkCopyDeviceMemoryInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCopyImageInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkCopyImageInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkCopyImageToBufferInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkCopyImageToBufferInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkCopyImageToImageInfo;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkCopyImageToImageInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkCopyImageToMemoryInfo;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkCopyImageToMemoryInfoEXT;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkCopyMemoryIndirectCommandKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkCopyMemoryIndirectCommandNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkCopyMemoryIndirectInfoKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkCopyMemoryToAccelerationStructureInfoKHR;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkCopyMemoryToImageIndirectCommandKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkCopyMemoryToImageIndirectCommandNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkCopyMemoryToImageIndirectInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkCopyMemoryToImageInfo;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkCopyMemoryToImageInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkCopyMemoryToMicromapInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkCopyMicromapInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkCopyMicromapToMemoryInfoEXT;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkCopyTensorInfoARM;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuFunctionCreateInfoNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuFunctionNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuLaunchInfoNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuModuleCreateInfoNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuModuleNVX;
#[cfg(feature = "VK_NVX_binary_import")]
use crate::types::VkCuModuleTexturingModeCreateInfoNVX;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkCudaFunctionCreateInfoNV;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkCudaFunctionNV;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkCudaLaunchInfoNV;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkCudaModuleCreateInfoNV;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkCudaModuleNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkCullModeFlags;
#[cfg(any(
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
))]
use crate::types::VkCustomResolveCreateInfoEXT;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkD3D12FenceSubmitInfoKHR;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowCreateFlagsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowExecuteFlagsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowGridSizeFlagsARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageFormatPropertiesARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphOpticalFlowImageUsageFlagsARM;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::types::VkDataGraphPipelineBuiltinModelCreateInfoQCOM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineCompilerControlCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineConstantARM;
#[cfg(all(feature = "VK_ARM_data_graph", feature = "VK_ARM_tensors"))]
use crate::types::VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineDispatchFlagsARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineDispatchInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineIdentifierCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineInfoARM;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::types::VkDataGraphPipelineNeuralStatisticsCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphPipelineOpticalFlowCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphPipelineOpticalFlowDispatchInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelinePropertyQueryResultARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineResourceInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphPipelineResourceInfoImageLayoutARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionBindPointRequirementARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionBindPointRequirementsInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionCreateFlagsARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineSessionMemoryRequirementsInfoARM;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::types::VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphPipelineShaderModuleCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphPipelineSingleNodeConnectionARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkDataGraphPipelineSingleNodeCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkDataGraphProcessingEngineCreateInfoARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::types::VkDataGraphTOSANameQualityARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::types::VkDataGraphTOSAQualityFlagsARM;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::types::VkDebugMarkerMarkerInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::types::VkDebugMarkerObjectNameInfoEXT;
#[cfg(feature = "VK_EXT_debug_marker")]
use crate::types::VkDebugMarkerObjectTagInfoEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::types::VkDebugReportCallbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::types::VkDebugReportCallbackEXT;
#[cfg(feature = "VK_EXT_debug_report")]
use crate::types::VkDebugReportFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsLabelEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessageSeverityFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessageTypeFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerCallbackDataEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerCallbackDataFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerCreateFlagsEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerCreateInfoEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsMessengerEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsObjectNameInfoEXT;
#[cfg(feature = "VK_EXT_debug_utils")]
use crate::types::VkDebugUtilsObjectTagInfoEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkDecompressMemoryInfoEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkDecompressMemoryRegionEXT;
#[cfg(feature = "VK_NV_memory_decompression")]
use crate::types::VkDecompressMemoryRegionNV;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationBufferCreateInfoNV;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationImageCreateInfoNV;
#[cfg(feature = "VK_NV_dedicated_allocation")]
use crate::types::VkDedicatedAllocationMemoryAllocateInfoNV;
#[cfg(feature = "VK_KHR_deferred_host_operations")]
use crate::types::VkDeferredOperationKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDependencyFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDependencyInfo;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkDependencyInfoKHR;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::types::VkDepthBiasInfoEXT;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::types::VkDepthBiasRepresentationInfoEXT;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::types::VkDepthClampRangeEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorAddressInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorBindingFlags;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkDescriptorBindingFlagsEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorBufferBindingInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorBufferBindingPushDescriptorBufferHandleEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorBufferInfo;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorDataEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkDescriptorGetInfoEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkDescriptorGetTensorInfoARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorImageInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceConstantOffsetEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceDataEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceHeapDataEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceIndirectAddressEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceIndirectIndexArrayEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceIndirectIndexEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourcePushIndexEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorMappingSourceShaderRecordIndexEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPool;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkDescriptorPoolInlineUniformBlockCreateInfo;
#[cfg(feature = "VK_EXT_inline_uniform_block")]
use crate::types::VkDescriptorPoolInlineUniformBlockCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolResetFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorPoolSize;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSet;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetAllocateInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDescriptorSetAndBindingMappingEXT;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
use crate::types::VkDescriptorSetBindingReferenceVALVE;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayoutBinding;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetLayoutBindingFlagsCreateInfo;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkDescriptorSetLayoutBindingFlagsCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayoutCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDescriptorSetLayoutCreateInfo;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
use crate::types::VkDescriptorSetLayoutHostMappingInfoVALVE;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorSetLayoutSupport;
#[cfg(feature = "VK_KHR_maintenance3")]
use crate::types::VkDescriptorSetLayoutSupportKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetVariableDescriptorCountAllocateInfo;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkDescriptorSetVariableDescriptorCountAllocateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkDescriptorSetVariableDescriptorCountLayoutSupport;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkDescriptorSetVariableDescriptorCountLayoutSupportEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorUpdateTemplate;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorUpdateTemplateCreateFlags;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
use crate::types::VkDescriptorUpdateTemplateCreateFlagsKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorUpdateTemplateCreateInfo;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
use crate::types::VkDescriptorUpdateTemplateCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkDescriptorUpdateTemplateEntry;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
use crate::types::VkDescriptorUpdateTemplateEntryKHR;
#[cfg(feature = "VK_KHR_descriptor_update_template")]
use crate::types::VkDescriptorUpdateTemplateKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDevice;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceAddress;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::types::VkDeviceAddressBindingCallbackDataEXT;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::types::VkDeviceAddressBindingFlagsEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkDeviceAddressRangeEXT;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDeviceAddressRangeKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDeviceBufferMemoryRequirements;
#[cfg(feature = "VK_KHR_maintenance4")]
use crate::types::VkDeviceBufferMemoryRequirementsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceCreateInfo;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkDeviceDeviceMemoryReportCreateInfoEXT;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkDeviceDiagnosticsConfigCreateInfoNV;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkDeviceDiagnosticsConfigFlagsNV;
#[cfg(feature = "VK_EXT_display_control")]
use crate::types::VkDeviceEventInfoEXT;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkDeviceFaultAddressInfoEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultAddressInfoKHR;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkDeviceFaultCountsEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultDebugInfoKHR;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultFlagsKHR;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkDeviceFaultInfoEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultInfoKHR;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkDeviceFaultShaderAbortMessageInfoKHR;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkDeviceFaultVendorBinaryHeaderVersionOneEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultVendorBinaryHeaderVersionOneKHR;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkDeviceFaultVendorInfoEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkDeviceFaultVendorInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupBindSparseInfo;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkDeviceGroupBindSparseInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupCommandBufferBeginInfo;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkDeviceGroupCommandBufferBeginInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupDeviceCreateInfo;
#[cfg(feature = "VK_KHR_device_group_creation")]
use crate::types::VkDeviceGroupDeviceCreateInfoKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
use crate::types::VkDeviceGroupPresentCapabilitiesKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkDeviceGroupPresentInfoKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
use crate::types::VkDeviceGroupPresentModeFlagsKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkDeviceGroupRenderPassBeginInfo;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkDeviceGroupRenderPassBeginInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceGroupSubmitInfo;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkDeviceGroupSubmitInfoKHR;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkDeviceGroupSwapchainCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDeviceImageMemoryRequirements;
#[cfg(feature = "VK_KHR_maintenance4")]
use crate::types::VkDeviceImageMemoryRequirementsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkDeviceImageSubresourceInfo;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkDeviceImageSubresourceInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceMemory;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDeviceMemoryCopyKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDeviceMemoryImageCopyKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkDeviceMemoryOpaqueCaptureAddressInfo;
#[cfg(feature = "VK_KHR_buffer_device_address")]
use crate::types::VkDeviceMemoryOpaqueCaptureAddressInfoKHR;
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
use crate::types::VkDeviceMemoryOverallocationCreateInfoAMD;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkDeviceMemoryReportCallbackDataEXT;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkDeviceMemoryReportFlagsEXT;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkDeviceObjectReservationCreateInfo;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkDeviceOrHostAddressConstAMDX;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_cooperative_vector"
))]
use crate::types::VkDeviceOrHostAddressConstKHR;
#[cfg(any(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_NV_cooperative_vector"
))]
use crate::types::VkDeviceOrHostAddressKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkDevicePipelineBinaryInternalCacheControlKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkDevicePrivateDataCreateInfo;
#[cfg(feature = "VK_EXT_private_data")]
use crate::types::VkDevicePrivateDataCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceQueueCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceQueueCreateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkDeviceQueueGlobalPriorityCreateInfo;
#[cfg(feature = "VK_EXT_global_priority")]
use crate::types::VkDeviceQueueGlobalPriorityCreateInfoEXT;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::types::VkDeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkDeviceQueueInfo2;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkDeviceQueueShaderCoreControlCreateInfoARM;
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_NV_external_sci_sync2"))]
use crate::types::VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkDeviceSize;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkDeviceTensorMemoryRequirementsARM;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::types::VkDirectDriverLoadingFlagsLUNARG;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::types::VkDirectDriverLoadingInfoLUNARG;
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
use crate::types::VkDirectDriverLoadingListLUNARG;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::types::VkDirectFBSurfaceCreateFlagsEXT;
#[cfg(feature = "VK_EXT_directfb_surface")]
use crate::types::VkDirectFBSurfaceCreateInfoEXT;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkDispatchGraphCountInfoAMDX;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkDispatchGraphInfoAMDX;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDispatchIndirect2InfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkDispatchIndirectCommand;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkDispatchParametersARM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkDispatchTileInfoQCOM;
#[cfg(feature = "VK_EXT_display_control")]
use crate::types::VkDisplayEventInfoEXT;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeCreateFlagsKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeCreateInfoKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModeParametersKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayModeProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayModePropertiesKHR;
#[cfg(feature = "VK_NV_display_stereo")]
use crate::types::VkDisplayModeStereoPropertiesNV;
#[cfg(feature = "VK_AMD_display_native_hdr")]
use crate::types::VkDisplayNativeHdrSurfaceCapabilitiesAMD;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayPlaneAlphaFlagsKHR;
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
#[cfg(feature = "VK_EXT_display_control")]
use crate::types::VkDisplayPowerInfoEXT;
#[cfg(feature = "VK_KHR_display_swapchain")]
use crate::types::VkDisplayPresentInfoKHR;
#[cfg(feature = "VK_KHR_get_display_properties2")]
use crate::types::VkDisplayProperties2KHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplayPropertiesKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplaySurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_display")]
use crate::types::VkDisplaySurfaceCreateInfoKHR;
#[cfg(feature = "VK_NV_display_stereo")]
use crate::types::VkDisplaySurfaceStereoCreateInfoNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkDrawIndexedIndirectCommand;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDrawIndirect2InfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkDrawIndirectCommand;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkDrawIndirectCount2InfoKHR;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkDrawIndirectCountIndirectCommandEXT;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkDrawMeshTasksIndirectCommandEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkDrawMeshTasksIndirectCommandNV;
#[cfg(any(
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_VERSION_1_3"
  )
))]
use crate::types::VkDrmFormatModifierProperties2EXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkDrmFormatModifierPropertiesEXT;
#[cfg(any(
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_KHR_format_feature_flags2"
  ),
  all(
    feature = "VK_EXT_image_drm_format_modifier",
    feature = "VK_VERSION_1_3"
  )
))]
use crate::types::VkDrmFormatModifierPropertiesList2EXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkDrmFormatModifierPropertiesListEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkEvent;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkEventCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkEventCreateInfo;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkExecutionGraphPipelineCreateInfoAMDX;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkExecutionGraphPipelineScratchSizeAMDX;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportFenceCreateInfo;
#[cfg(feature = "VK_KHR_external_fence")]
use crate::types::VkExportFenceCreateInfoKHR;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkExportFenceSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_fence_win32")]
use crate::types::VkExportFenceWin32HandleInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportMemoryAllocateInfo;
#[cfg(feature = "VK_KHR_external_memory")]
use crate::types::VkExportMemoryAllocateInfoKHR;
#[cfg(feature = "VK_NV_external_memory")]
use crate::types::VkExportMemoryAllocateInfoNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkExportMemorySciBufInfoNV;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkExportMemoryWin32HandleInfoKHR;
#[cfg(feature = "VK_NV_external_memory_win32")]
use crate::types::VkExportMemoryWin32HandleInfoNV;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalBufferInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalCommandQueueInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalDeviceInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalIOSurfaceInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalObjectCreateInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalObjectTypeFlagsEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalObjectsInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalSharedEventInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkExportMetalTextureInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExportSemaphoreCreateInfo;
#[cfg(feature = "VK_KHR_external_semaphore")]
use crate::types::VkExportSemaphoreCreateInfoKHR;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkExportSemaphoreSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkExportSemaphoreWin32HandleInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtensionProperties;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent2D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkExtent3D;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalBufferProperties;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalBufferPropertiesKHR;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkExternalComputeQueueCreateInfoNV;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkExternalComputeQueueDataParamsNV;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkExternalComputeQueueDeviceCreateInfoNV;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkExternalComputeQueueNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalFenceFeatureFlags;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkExternalFenceFeatureFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalFenceHandleTypeFlags;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkExternalFenceHandleTypeFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalFenceProperties;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkExternalFencePropertiesKHR;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkExternalFormatANDROID;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkExternalFormatOHOS;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkExternalFormatQNX;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalImageFormatProperties;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalImageFormatPropertiesKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::types::VkExternalImageFormatPropertiesNV;
#[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
use crate::types::VkExternalMemoryAcquireUnmodifiedEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryBufferCreateInfo;
#[cfg(feature = "VK_KHR_external_memory")]
use crate::types::VkExternalMemoryBufferCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryFeatureFlags;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalMemoryFeatureFlagsKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::types::VkExternalMemoryFeatureFlagsNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryHandleTypeFlags;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalMemoryHandleTypeFlagsKHR;
#[cfg(feature = "VK_NV_external_memory_capabilities")]
use crate::types::VkExternalMemoryHandleTypeFlagsNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryImageCreateInfo;
#[cfg(feature = "VK_KHR_external_memory")]
use crate::types::VkExternalMemoryImageCreateInfoKHR;
#[cfg(feature = "VK_NV_external_memory")]
use crate::types::VkExternalMemoryImageCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalMemoryProperties;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkExternalMemoryPropertiesKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkExternalMemoryTensorCreateInfoARM;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalSemaphoreFeatureFlags;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkExternalSemaphoreFeatureFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalSemaphoreHandleTypeFlags;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkExternalSemaphoreHandleTypeFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkExternalSemaphoreProperties;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkExternalSemaphorePropertiesKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkExternalTensorPropertiesARM;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkFaultCallbackInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkFaultData;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFence;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFenceCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFenceCreateInfo;
#[cfg(feature = "VK_KHR_external_fence_fd")]
use crate::types::VkFenceGetFdInfoKHR;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkFenceGetSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_fence_win32")]
use crate::types::VkFenceGetWin32HandleInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkFenceImportFlags;
#[cfg(feature = "VK_KHR_external_fence")]
use crate::types::VkFenceImportFlagsKHR;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::types::VkFilterCubicImageViewImageFormatPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFlags;
#[cfg(any(feature = "VK_BASE_VERSION_1_3", feature = "VK_KHR_synchronization2"))]
use crate::types::VkFlags64;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkFormatFeatureFlags2;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
use crate::types::VkFormatFeatureFlags2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkFormatProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkFormatProperties3;
#[cfg(feature = "VK_KHR_format_feature_flags2")]
use crate::types::VkFormatProperties3KHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkFragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkFrameBoundaryEXT;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkFrameBoundaryFlagsEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_frame_boundary"))]
use crate::types::VkFrameBoundaryTensorsARM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkFramebuffer;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkFramebufferAttachmentImageInfo;
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
use crate::types::VkFramebufferAttachmentImageInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkFramebufferAttachmentsCreateInfo;
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
use crate::types::VkFramebufferAttachmentsCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkFramebufferCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkFramebufferCreateInfo;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkFramebufferMixedSamplesCombinationNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkGeneratedCommandsInfoEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkGeneratedCommandsInfoNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkGeneratedCommandsMemoryRequirementsInfoEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkGeneratedCommandsMemoryRequirementsInfoNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkGeneratedCommandsPipelineInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkGeneratedCommandsShaderInfoEXT;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryAABBNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryDataNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkGeometryFlagsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryFlagsNV;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkGeometryInstanceFlagsKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryInstanceFlagsNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryNV;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkGeometryTrianglesNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkGetLatencyMarkerInfoNV;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaDeviceClockModeInfoAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaDeviceGetClockInfoAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaPerfBlockPropertiesAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaPerfBlockPropertiesFlagsAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaPerfCounterAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaSampleBeginInfoAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaSessionAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaSessionCreateInfoAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkGpaSqShaderStageFlagsAMD;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkGraphicsPipelineCreateInfo;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkGraphicsPipelineLibraryCreateInfoEXT;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkGraphicsPipelineLibraryFlagsEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkGraphicsPipelineShaderGroupsCreateInfoNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkGraphicsShaderGroupCreateInfoNV;
#[cfg(feature = "VK_EXT_hdr_metadata")]
use crate::types::VkHdrMetadataEXT;
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
use crate::types::VkHdrVividDynamicMetadataHUAWEI;
#[cfg(feature = "VK_EXT_headless_surface")]
use crate::types::VkHeadlessSurfaceCreateFlagsEXT;
#[cfg(feature = "VK_EXT_headless_surface")]
use crate::types::VkHeadlessSurfaceCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkHostAddressRangeConstEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkHostAddressRangeEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkHostImageCopyDevicePerformanceQuery;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkHostImageCopyDevicePerformanceQueryEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkHostImageCopyFlags;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkHostImageCopyFlagsEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkHostImageLayoutTransitionInfo;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkHostImageLayoutTransitionInfoEXT;
#[cfg(feature = "VK_MVK_ios_surface")]
use crate::types::VkIOSSurfaceCreateFlagsMVK;
#[cfg(feature = "VK_MVK_ios_surface")]
use crate::types::VkIOSSurfaceCreateInfoMVK;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImage;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkImageAlignmentControlCreateInfoMESA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageAspectFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkImageBlit;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkImageBlit2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkImageBlit2KHR;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkImageCaptureDescriptorDataInfoEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionControlEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionFixedRateFlagsEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionFlagsEXT;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkImageCompressionPropertiesEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImageConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImageConstraintsInfoFlagsFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCopy;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkImageCopy2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkImageCopy2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageCreateInfo;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkImageDescriptorInfoEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkImageDrmFormatModifierExplicitCreateInfoEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkImageDrmFormatModifierListCreateInfoEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkImageDrmFormatModifierPropertiesEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImageFormatConstraintsFlagsFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImageFormatConstraintsInfoFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkImageFormatListCreateInfo;
#[cfg(feature = "VK_KHR_image_format_list")]
use crate::types::VkImageFormatListCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkImageFormatProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageMemoryBarrier;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkImageMemoryBarrier2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkImageMemoryBarrier2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageMemoryRequirementsInfo2;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
use crate::types::VkImageMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
use crate::types::VkImagePipeSurfaceCreateFlagsFUCHSIA;
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
use crate::types::VkImagePipeSurfaceCreateInfoFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImagePlaneMemoryRequirementsInfo;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkImagePlaneMemoryRequirementsInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkImageResolve;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkImageResolve2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkImageResolve2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageSparseMemoryRequirementsInfo2;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
use crate::types::VkImageSparseMemoryRequirementsInfo2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkImageStencilUsageCreateInfo;
#[cfg(feature = "VK_EXT_separate_stencil_usage")]
use crate::types::VkImageStencilUsageCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresource;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkImageSubresource2;
#[cfg(any(
  feature = "VK_EXT_host_image_copy",
  feature = "VK_EXT_image_compression_control"
))]
use crate::types::VkImageSubresource2EXT;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkImageSubresource2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceLayers;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageSubresourceRange;
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
use crate::types::VkImageSwapchainCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkImageToMemoryCopy;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkImageToMemoryCopyEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageUsageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageView;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
use crate::types::VkImageViewASTCDecodeModeEXT;
#[cfg(feature = "VK_NVX_image_view_handle")]
use crate::types::VkImageViewAddressPropertiesNVX;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkImageViewCaptureDescriptorDataInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageViewCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkImageViewCreateInfo;
#[cfg(feature = "VK_NVX_image_view_handle")]
use crate::types::VkImageViewHandleInfoNVX;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
use crate::types::VkImageViewMinLodCreateInfoEXT;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkImageViewSampleWeightCreateInfoQCOM;
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
use crate::types::VkImageViewSlicedCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkImageViewUsageCreateInfo;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::types::VkImageViewUsageCreateInfoKHR;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkImportAndroidHardwareBufferInfoANDROID;
#[cfg(feature = "VK_KHR_external_fence_fd")]
use crate::types::VkImportFenceFdInfoKHR;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkImportFenceSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_fence_win32")]
use crate::types::VkImportFenceWin32HandleInfoKHR;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkImportMemoryBufferCollectionFUCHSIA;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::types::VkImportMemoryFdInfoKHR;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::types::VkImportMemoryHostPointerInfoEXT;
#[cfg(feature = "VK_EXT_external_memory_metal")]
use crate::types::VkImportMemoryMetalHandleInfoEXT;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkImportMemorySciBufInfoNV;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkImportMemoryWin32HandleInfoKHR;
#[cfg(feature = "VK_NV_external_memory_win32")]
use crate::types::VkImportMemoryWin32HandleInfoNV;
#[cfg(feature = "VK_FUCHSIA_external_memory")]
use crate::types::VkImportMemoryZirconHandleInfoFUCHSIA;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalBufferInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalIOSurfaceInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalSharedEventInfoEXT;
#[cfg(feature = "VK_EXT_metal_objects")]
use crate::types::VkImportMetalTextureInfoEXT;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkImportNativeBufferInfoOHOS;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkImportScreenBufferInfoQNX;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
use crate::types::VkImportSemaphoreFdInfoKHR;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkImportSemaphoreSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkImportSemaphoreWin32HandleInfoKHR;
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
use crate::types::VkImportSemaphoreZirconHandleInfoFUCHSIA;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsExecutionSetTokenEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsIndexBufferTokenEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsInputModeFlagsEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutCreateInfoEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutCreateInfoNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutNV;
#[cfg(all(
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_NV_device_generated_commands"
))]
use crate::types::VkIndirectCommandsLayoutPushDataTokenNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutTokenEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutTokenNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutUsageFlagsEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectCommandsLayoutUsageFlagsNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsPushConstantTokenEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectCommandsStreamNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsTokenDataEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectCommandsVertexBufferTokenEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetCreateInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetPipelineInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetShaderInfoEXT;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkIndirectExecutionSetShaderLayoutInfoEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkIndirectStateFlagsNV;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkInitializePerformanceApiInfoINTEL;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkInputAttachmentAspectReference;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::types::VkInputAttachmentAspectReferenceKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkInstance;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkInstanceCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkInstanceCreateInfo;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySleepInfoNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySleepModeInfoNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySubmissionPresentIdNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencySurfaceCapabilitiesNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkLatencyTimingsFrameReportNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkLayerProperties;
#[cfg(feature = "VK_EXT_layer_settings")]
use crate::types::VkLayerSettingEXT;
#[cfg(feature = "VK_EXT_layer_settings")]
use crate::types::VkLayerSettingsCreateInfoEXT;
#[cfg(feature = "VK_MVK_macos_surface")]
use crate::types::VkMacOSSurfaceCreateFlagsMVK;
#[cfg(feature = "VK_MVK_macos_surface")]
use crate::types::VkMacOSSurfaceCreateInfoMVK;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMappedMemoryRange;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryAllocateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryAllocateFlagsInfo;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkMemoryAllocateFlagsInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkMemoryAllocateFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryAllocateInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryBarrier;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkMemoryBarrier2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkMemoryBarrier2KHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkMemoryBarrierAccessFlags3KHR;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkMemoryDecompressionMethodFlagsEXT;
#[cfg(feature = "VK_NV_memory_decompression")]
use crate::types::VkMemoryDecompressionMethodFlagsNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryDedicatedAllocateInfo;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
use crate::types::VkMemoryDedicatedAllocateInfoKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkMemoryDedicatedAllocateInfoTensorARM;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryDedicatedRequirements;
#[cfg(feature = "VK_KHR_dedicated_allocation")]
use crate::types::VkMemoryDedicatedRequirementsKHR;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::types::VkMemoryFdPropertiesKHR;
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
use crate::types::VkMemoryGetAndroidHardwareBufferInfoANDROID;
#[cfg(feature = "VK_KHR_external_memory_fd")]
use crate::types::VkMemoryGetFdInfoKHR;
#[cfg(feature = "VK_EXT_external_memory_metal")]
use crate::types::VkMemoryGetMetalHandleInfoEXT;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkMemoryGetNativeBufferInfoOHOS;
#[cfg(feature = "VK_NV_external_memory_rdma")]
use crate::types::VkMemoryGetRemoteAddressInfoNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkMemoryGetSciBufInfoNV;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkMemoryGetWin32HandleInfoKHR;
#[cfg(feature = "VK_FUCHSIA_external_memory")]
use crate::types::VkMemoryGetZirconHandleInfoFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryHeap;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryHeapFlags;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::types::VkMemoryHostPointerPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryMapFlags;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkMemoryMapInfo;
#[cfg(feature = "VK_KHR_map_memory2")]
use crate::types::VkMemoryMapInfoKHR;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkMemoryMapPlacedInfoEXT;
#[cfg(all(
  feature = "VK_AMD_buffer_marker",
  feature = "VK_KHR_device_address_commands"
))]
use crate::types::VkMemoryMarkerInfoAMD;
#[cfg(feature = "VK_EXT_external_memory_metal")]
use crate::types::VkMemoryMetalHandlePropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkMemoryOpaqueCaptureAddressAllocateInfo;
#[cfg(feature = "VK_KHR_buffer_device_address")]
use crate::types::VkMemoryOpaqueCaptureAddressAllocateInfoKHR;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::types::VkMemoryPriorityAllocateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryPropertyFlags;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkMemoryRangeBarrierKHR;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkMemoryRangeBarriersInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryRequirements;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkMemoryRequirements2;
#[cfg(any(
  feature = "VK_KHR_get_memory_requirements2",
  all(feature = "VK_NV_ray_tracing", feature = "VK_VERSION_1_1")
))]
use crate::types::VkMemoryRequirements2KHR;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkMemorySciBufPropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkMemoryToImageCopy;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkMemoryToImageCopyEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkMemoryType;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkMemoryUnmapFlags;
#[cfg(feature = "VK_KHR_map_memory2")]
use crate::types::VkMemoryUnmapFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkMemoryUnmapInfo;
#[cfg(feature = "VK_KHR_map_memory2")]
use crate::types::VkMemoryUnmapInfoKHR;
#[cfg(feature = "VK_KHR_external_memory_win32")]
use crate::types::VkMemoryWin32HandlePropertiesKHR;
#[cfg(feature = "VK_FUCHSIA_external_memory")]
use crate::types::VkMemoryZirconHandlePropertiesFUCHSIA;
#[cfg(feature = "VK_EXT_metal_surface")]
use crate::types::VkMetalSurfaceCreateFlagsEXT;
#[cfg(feature = "VK_EXT_metal_surface")]
use crate::types::VkMetalSurfaceCreateInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapBuildInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapBuildSizesInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapCreateFlagsEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapCreateInfoEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapTriangleEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkMicromapTriangleKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapUsageEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkMicromapUsageKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkMicromapVersionInfoEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkMultiDrawIndexedInfoEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkMultiDrawInfoEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkMultisamplePropertiesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkMultisampledRenderToSingleSampledInfoEXT;
#[cfg(any(
  all(
    feature = "VK_NVX_multiview_per_view_attributes",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_NVX_multiview_per_view_attributes"
  )
))]
use crate::types::VkMultiviewPerViewAttributesInfoNVX;
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkMutableDescriptorTypeCreateInfoEXT;
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
use crate::types::VkMutableDescriptorTypeCreateInfoVALVE;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkMutableDescriptorTypeListEXT;
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
use crate::types::VkMutableDescriptorTypeListVALVE;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkNativeBufferFormatPropertiesOHOS;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkNativeBufferPropertiesOHOS;
#[cfg(feature = "VK_OHOS_external_memory")]
use crate::types::VkNativeBufferUsageOHOS;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset2D;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkOffset3D;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkOpaqueCaptureDataCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkOpaqueCaptureDescriptorDataCreateInfoEXT;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowExecuteFlagsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowExecuteInfoNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowGridSizeFlagsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatInfoNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowImageFormatPropertiesNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowSessionCreateFlagsNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowSessionCreateInfoNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowSessionCreatePrivateDataInfoNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowSessionNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkOpticalFlowUsageFlagsNV;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkOutOfBandQueueTypeInfoNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureFlagsNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureInstanceFlagsNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureInstancesInputNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureUpdateInstanceDataNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureWriteInstanceDataNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPartitionedAccelerationStructureWritePartitionTranslationDataNV;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPastPresentationTimingEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPastPresentationTimingFlagsEXT;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::types::VkPastPresentationTimingGOOGLE;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPastPresentationTimingInfoEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPastPresentationTimingPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPeerMemoryFeatureFlags;
#[cfg(feature = "VK_KHR_device_group")]
use crate::types::VkPeerMemoryFeatureFlagsKHR;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPerTileBeginInfoQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPerTileEndInfoQCOM;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPerfHintInfoQCOM;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceConfigurationAcquireInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceConfigurationINTEL;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPerformanceCounterARM;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPerformanceCounterDescriptionARM;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPerformanceCounterDescriptionFlagsARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterDescriptionFlagsKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterDescriptionKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceCounterResultKHR;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceMarkerInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceOverrideInfoINTEL;
#[cfg(all(feature = "VKSC_VERSION_1_0", feature = "VK_KHR_performance_query"))]
use crate::types::VkPerformanceQueryReservationInfoKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPerformanceQuerySubmitInfoKHR;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceStreamMarkerInfoINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceValueDataINTEL;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkPerformanceValueINTEL;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDevice;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDevice8BitStorageFeatures;
#[cfg(feature = "VK_KHR_8bit_storage")]
use crate::types::VkPhysicalDevice8BitStorageFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDevice16BitStorageFeatures;
#[cfg(feature = "VK_KHR_16bit_storage")]
use crate::types::VkPhysicalDevice16BitStorageFeaturesKHR;
#[cfg(feature = "VK_EXT_4444_formats")]
use crate::types::VkPhysicalDevice4444FormatsFeaturesEXT;
#[cfg(feature = "VK_EXT_astc_decode_mode")]
use crate::types::VkPhysicalDeviceASTCDecodeFeaturesEXT;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkPhysicalDeviceAccelerationStructureFeaturesKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkPhysicalDeviceAccelerationStructurePropertiesKHR;
#[cfg(feature = "VK_EXT_device_address_binding_report")]
use crate::types::VkPhysicalDeviceAddressBindingReportFeaturesEXT;
#[cfg(feature = "VK_SEC_amigo_profiling")]
use crate::types::VkPhysicalDeviceAmigoProfilingFeaturesSEC;
#[cfg(feature = "VK_AMD_anti_lag")]
use crate::types::VkPhysicalDeviceAntiLagFeaturesAMD;
#[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
use crate::types::VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT;
#[cfg(feature = "VK_EXT_attachment_feedback_loop_layout")]
use crate::types::VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::types::VkPhysicalDeviceBorderColorSwizzleFeaturesEXT;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkPhysicalDeviceBufferAddressFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeatures;
#[cfg(feature = "VK_EXT_buffer_device_address")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[cfg(feature = "VK_KHR_buffer_device_address")]
use crate::types::VkPhysicalDeviceBufferDeviceAddressFeaturesKHR;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkPhysicalDeviceClusterAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkPhysicalDeviceClusterAccelerationStructurePropertiesNV;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI;
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
use crate::types::VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI;
#[cfg(feature = "VK_AMD_device_coherent_memory")]
use crate::types::VkPhysicalDeviceCoherentMemoryFeaturesAMD;
#[cfg(feature = "VK_EXT_color_write_enable")]
use crate::types::VkPhysicalDeviceColorWriteEnableFeaturesEXT;
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
use crate::types::VkPhysicalDeviceCommandBufferInheritanceFeaturesNV;
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
use crate::types::VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV;
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
use crate::types::VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR;
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
use crate::types::VkPhysicalDeviceComputeShaderDerivativesFeaturesNV;
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
use crate::types::VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR;
#[cfg(feature = "VK_EXT_conditional_rendering")]
use crate::types::VkPhysicalDeviceConditionalRenderingFeaturesEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::types::VkPhysicalDeviceConservativeRasterizationPropertiesEXT;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkPhysicalDeviceCooperativeMatrix2FeaturesNV;
#[cfg(feature = "VK_NV_cooperative_matrix2")]
use crate::types::VkPhysicalDeviceCooperativeMatrix2PropertiesNV;
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
use crate::types::VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM;
#[cfg(feature = "VK_NV_cooperative_matrix_decode_vector")]
use crate::types::VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixFeaturesNV;
#[cfg(feature = "VK_KHR_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixPropertiesKHR;
#[cfg(feature = "VK_NV_cooperative_matrix")]
use crate::types::VkPhysicalDeviceCooperativeMatrixPropertiesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkPhysicalDeviceCooperativeVectorFeaturesNV;
#[cfg(feature = "VK_NV_cooperative_vector")]
use crate::types::VkPhysicalDeviceCooperativeVectorPropertiesNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectFeaturesNV;
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR;
#[cfg(feature = "VK_NV_copy_memory_indirect")]
use crate::types::VkPhysicalDeviceCopyMemoryIndirectPropertiesNV;
#[cfg(feature = "VK_NV_corner_sampled_image")]
use crate::types::VkPhysicalDeviceCornerSampledImageFeaturesNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkPhysicalDeviceCoverageReductionModeFeaturesNV;
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
use crate::types::VkPhysicalDeviceCubicClampFeaturesQCOM;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkPhysicalDeviceCubicWeightsFeaturesQCOM;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkPhysicalDeviceCudaKernelLaunchFeaturesNV;
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
use crate::types::VkPhysicalDeviceCudaKernelLaunchPropertiesNV;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkPhysicalDeviceCustomBorderColorFeaturesEXT;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkPhysicalDeviceCustomBorderColorPropertiesEXT;
#[cfg(feature = "VK_EXT_custom_resolve")]
use crate::types::VkPhysicalDeviceCustomResolveFeaturesEXT;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceDataGraphFeaturesARM;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::types::VkPhysicalDeviceDataGraphModelFeaturesQCOM;
#[cfg(feature = "VK_ARM_data_graph_neural_accelerator_statistics")]
use crate::types::VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceDataGraphOperationSupportARM;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceDataGraphProcessingEngineARM;
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
use crate::types::VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
use crate::types::VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX;
#[cfg(feature = "VK_EXT_depth_bias_control")]
use crate::types::VkPhysicalDeviceDepthBiasControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::types::VkPhysicalDeviceDepthClampControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clamp_zero_one")]
use crate::types::VkPhysicalDeviceDepthClampZeroOneFeaturesEXT;
#[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
use crate::types::VkPhysicalDeviceDepthClampZeroOneFeaturesKHR;
#[cfg(feature = "VK_EXT_depth_clip_control")]
use crate::types::VkPhysicalDeviceDepthClipControlFeaturesEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::types::VkPhysicalDeviceDepthClipEnableFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDepthStencilResolveProperties;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
use crate::types::VkPhysicalDeviceDepthStencilResolvePropertiesKHR;
#[cfg(all(
  feature = "VK_EXT_descriptor_buffer",
  feature = "VK_EXT_fragment_density_map"
))]
use crate::types::VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkPhysicalDeviceDescriptorBufferFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkPhysicalDeviceDescriptorBufferPropertiesEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkPhysicalDeviceDescriptorBufferTensorFeaturesARM;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkPhysicalDeviceDescriptorBufferTensorPropertiesARM;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPhysicalDeviceDescriptorHeapFeaturesEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPhysicalDeviceDescriptorHeapPropertiesEXT;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_heap"))]
use crate::types::VkPhysicalDeviceDescriptorHeapTensorPropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDescriptorIndexingFeatures;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkPhysicalDeviceDescriptorIndexingFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDescriptorIndexingProperties;
#[cfg(feature = "VK_EXT_descriptor_indexing")]
use crate::types::VkPhysicalDeviceDescriptorIndexingPropertiesEXT;
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
use crate::types::VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV;
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
use crate::types::VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
#[cfg(feature = "VK_KHR_device_address_commands")]
use crate::types::VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
#[cfg(feature = "VK_EXT_device_memory_report")]
use crate::types::VkPhysicalDeviceDeviceMemoryReportFeaturesEXT;
#[cfg(feature = "VK_NV_device_diagnostics_config")]
use crate::types::VkPhysicalDeviceDiagnosticsConfigFeaturesNV;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::types::VkPhysicalDeviceDiscardRectanglePropertiesEXT;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkPhysicalDeviceDisplacementMicromapFeaturesNV;
#[cfg(feature = "VK_NV_displacement_micromap")]
use crate::types::VkPhysicalDeviceDisplacementMicromapPropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceDriverProperties;
#[cfg(feature = "VK_KHR_driver_properties")]
use crate::types::VkPhysicalDeviceDriverPropertiesKHR;
#[cfg(feature = "VK_EXT_physical_device_drm")]
use crate::types::VkPhysicalDeviceDrmPropertiesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkPhysicalDeviceDynamicRenderingFeatures;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::types::VkPhysicalDeviceDynamicRenderingFeaturesKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceDynamicRenderingLocalReadFeatures;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
use crate::types::VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR;
#[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
use crate::types::VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT;
#[cfg(feature = "VK_QCOM_elapsed_timer_query")]
use crate::types::VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM;
#[cfg(feature = "VK_NV_scissor_exclusive")]
use crate::types::VkPhysicalDeviceExclusiveScissorFeaturesNV;
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
use crate::types::VkPhysicalDeviceExtendedDynamicState2FeaturesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
use crate::types::VkPhysicalDeviceExtendedDynamicState3FeaturesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
use crate::types::VkPhysicalDeviceExtendedDynamicState3PropertiesEXT;
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
use crate::types::VkPhysicalDeviceExtendedDynamicStateFeaturesEXT;
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
use crate::types::VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV;
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
use crate::types::VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalBufferInfo;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkPhysicalDeviceExternalBufferInfoKHR;
#[cfg(feature = "VK_NV_external_compute_queue")]
use crate::types::VkPhysicalDeviceExternalComputeQueuePropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalFenceInfo;
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
use crate::types::VkPhysicalDeviceExternalFenceInfoKHR;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkPhysicalDeviceExternalFormatResolveFeaturesANDROID;
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
use crate::types::VkPhysicalDeviceExternalFormatResolvePropertiesANDROID;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalImageFormatInfo;
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
use crate::types::VkPhysicalDeviceExternalImageFormatInfoKHR;
#[cfg(feature = "VK_EXT_external_memory_host")]
use crate::types::VkPhysicalDeviceExternalMemoryHostPropertiesEXT;
#[cfg(feature = "VK_NV_external_memory_rdma")]
use crate::types::VkPhysicalDeviceExternalMemoryRDMAFeaturesNV;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkPhysicalDeviceExternalMemorySciBufFeaturesNV;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX;
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
use crate::types::VkPhysicalDeviceExternalSciBufFeaturesNV;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkPhysicalDeviceExternalSciSync2FeaturesNV;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkPhysicalDeviceExternalSciSyncFeaturesNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceExternalSemaphoreInfo;
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
use crate::types::VkPhysicalDeviceExternalSemaphoreInfoKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceExternalTensorInfoARM;
#[cfg(feature = "VK_EXT_device_fault")]
use crate::types::VkPhysicalDeviceFaultFeaturesEXT;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkPhysicalDeviceFaultFeaturesKHR;
#[cfg(feature = "VK_KHR_device_fault")]
use crate::types::VkPhysicalDeviceFaultPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceFeatures2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceFeatures2KHR;
#[cfg(feature = "VK_KHR_shader_float16_int8")]
use crate::types::VkPhysicalDeviceFloat16Int8FeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceFloatControlsProperties;
#[cfg(feature = "VK_KHR_shader_float_controls")]
use crate::types::VkPhysicalDeviceFloatControlsPropertiesKHR;
#[cfg(feature = "VK_ARM_format_pack")]
use crate::types::VkPhysicalDeviceFormatPackFeaturesARM;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::types::VkPhysicalDeviceFragmentDensityMap2FeaturesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map2")]
use crate::types::VkPhysicalDeviceFragmentDensityMap2PropertiesEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT;
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT;
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
use crate::types::VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkPhysicalDeviceFragmentDensityMapPropertiesEXT;
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR;
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV;
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
use crate::types::VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR;
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
use crate::types::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRateFeaturesKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRateKHR;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPhysicalDeviceFragmentShadingRatePropertiesKHR;
#[cfg(feature = "VK_EXT_frame_boundary")]
use crate::types::VkPhysicalDeviceFrameBoundaryFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceGlobalPriorityQueryFeatures;
#[cfg(feature = "VK_EXT_global_priority_query")]
use crate::types::VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::types::VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaFeaturesAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaProperties2AMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaPropertiesAMD;
#[cfg(feature = "VK_AMD_gpa_interface")]
use crate::types::VkPhysicalDeviceGpaPropertiesFlagsAMD;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT;
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
use crate::types::VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceGroupProperties;
#[cfg(feature = "VK_KHR_device_group_creation")]
use crate::types::VkPhysicalDeviceGroupPropertiesKHR;
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
use crate::types::VkPhysicalDeviceHdrVividFeaturesHUAWEI;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceHostImageCopyFeatures;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkPhysicalDeviceHostImageCopyFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceHostImageCopyProperties;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkPhysicalDeviceHostImageCopyPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceHostQueryResetFeatures;
#[cfg(feature = "VK_EXT_host_query_reset")]
use crate::types::VkPhysicalDeviceHostQueryResetFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceIDProperties;
#[cfg(any(
  feature = "VK_KHR_external_memory_capabilities",
  feature = "VK_KHR_external_semaphore_capabilities",
  feature = "VK_KHR_external_fence_capabilities"
))]
use crate::types::VkPhysicalDeviceIDPropertiesKHR;
#[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
use crate::types::VkPhysicalDeviceImage2DViewOf3DFeaturesEXT;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkPhysicalDeviceImageAlignmentControlFeaturesMESA;
#[cfg(feature = "VK_MESA_image_alignment_control")]
use crate::types::VkPhysicalDeviceImageAlignmentControlPropertiesMESA;
#[cfg(feature = "VK_EXT_image_compression_control")]
use crate::types::VkPhysicalDeviceImageCompressionControlFeaturesEXT;
#[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
use crate::types::VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT;
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
use crate::types::VkPhysicalDeviceImageDrmFormatModifierInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceImageFormatInfo2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceImageFormatInfo2KHR;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkPhysicalDeviceImageProcessing2FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkPhysicalDeviceImageProcessing2PropertiesQCOM;
#[cfg(feature = "VK_QCOM_image_processing3")]
use crate::types::VkPhysicalDeviceImageProcessing3FeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkPhysicalDeviceImageProcessingFeaturesQCOM;
#[cfg(feature = "VK_QCOM_image_processing")]
use crate::types::VkPhysicalDeviceImageProcessingPropertiesQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceImageRobustnessFeatures;
#[cfg(feature = "VK_EXT_image_robustness")]
use crate::types::VkPhysicalDeviceImageRobustnessFeaturesEXT;
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
use crate::types::VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT;
#[cfg(feature = "VK_EXT_filter_cubic")]
use crate::types::VkPhysicalDeviceImageViewImageFormatInfoEXT;
#[cfg(feature = "VK_EXT_image_view_min_lod")]
use crate::types::VkPhysicalDeviceImageViewMinLodFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceImagelessFramebufferFeatures;
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
use crate::types::VkPhysicalDeviceImagelessFramebufferFeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceIndexTypeUint8Features;
#[cfg(feature = "VK_EXT_index_type_uint8")]
use crate::types::VkPhysicalDeviceIndexTypeUint8FeaturesEXT;
#[cfg(feature = "VK_KHR_index_type_uint8")]
use crate::types::VkPhysicalDeviceIndexTypeUint8FeaturesKHR;
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
use crate::types::VkPhysicalDeviceInheritedViewportScissorFeaturesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceInlineUniformBlockFeatures;
#[cfg(feature = "VK_EXT_inline_uniform_block")]
use crate::types::VkPhysicalDeviceInlineUniformBlockFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceInlineUniformBlockProperties;
#[cfg(feature = "VK_EXT_inline_uniform_block")]
use crate::types::VkPhysicalDeviceInlineUniformBlockPropertiesEXT;
#[cfg(feature = "VK_KHR_internally_synchronized_queues")]
use crate::types::VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR;
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
use crate::types::VkPhysicalDeviceInvocationMaskFeaturesHUAWEI;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceLayeredApiPropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceLayeredApiPropertiesListKHR;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceLayeredApiVulkanPropertiesKHR;
#[cfg(feature = "VK_MSFT_layered_driver")]
use crate::types::VkPhysicalDeviceLayeredDriverPropertiesMSFT;
#[cfg(feature = "VK_EXT_legacy_dithering")]
use crate::types::VkPhysicalDeviceLegacyDitheringFeaturesEXT;
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
use crate::types::VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT;
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
use crate::types::VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceLimits;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceLineRasterizationFeatures;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::types::VkPhysicalDeviceLineRasterizationFeaturesEXT;
#[cfg(feature = "VK_KHR_line_rasterization")]
use crate::types::VkPhysicalDeviceLineRasterizationFeaturesKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceLineRasterizationProperties;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::types::VkPhysicalDeviceLineRasterizationPropertiesEXT;
#[cfg(feature = "VK_KHR_line_rasterization")]
use crate::types::VkPhysicalDeviceLineRasterizationPropertiesKHR;
#[cfg(feature = "VK_NV_linear_color_attachment")]
use crate::types::VkPhysicalDeviceLinearColorAttachmentFeaturesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMaintenance3Properties;
#[cfg(feature = "VK_KHR_maintenance3")]
use crate::types::VkPhysicalDeviceMaintenance3PropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceMaintenance4Features;
#[cfg(feature = "VK_KHR_maintenance4")]
use crate::types::VkPhysicalDeviceMaintenance4FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceMaintenance4Properties;
#[cfg(feature = "VK_KHR_maintenance4")]
use crate::types::VkPhysicalDeviceMaintenance4PropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance5Features;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkPhysicalDeviceMaintenance5FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance5Properties;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkPhysicalDeviceMaintenance5PropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance6Features;
#[cfg(feature = "VK_KHR_maintenance6")]
use crate::types::VkPhysicalDeviceMaintenance6FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceMaintenance6Properties;
#[cfg(feature = "VK_KHR_maintenance6")]
use crate::types::VkPhysicalDeviceMaintenance6PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceMaintenance7FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance7")]
use crate::types::VkPhysicalDeviceMaintenance7PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance8")]
use crate::types::VkPhysicalDeviceMaintenance8FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkPhysicalDeviceMaintenance9FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkPhysicalDeviceMaintenance9PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkPhysicalDeviceMaintenance10FeaturesKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkPhysicalDeviceMaintenance10PropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance11")]
use crate::types::VkPhysicalDeviceMaintenance11FeaturesKHR;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkPhysicalDeviceMapMemoryPlacedFeaturesEXT;
#[cfg(feature = "VK_EXT_map_memory_placed")]
use crate::types::VkPhysicalDeviceMapMemoryPlacedPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_budget")]
use crate::types::VkPhysicalDeviceMemoryBudgetPropertiesEXT;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionFeaturesEXT;
#[cfg(feature = "VK_NV_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionFeaturesNV;
#[cfg(feature = "VK_EXT_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionPropertiesEXT;
#[cfg(feature = "VK_NV_memory_decompression")]
use crate::types::VkPhysicalDeviceMemoryDecompressionPropertiesNV;
#[cfg(feature = "VK_EXT_memory_priority")]
use crate::types::VkPhysicalDeviceMemoryPriorityFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceMemoryProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMemoryProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceMemoryProperties2KHR;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderFeaturesNV;
#[cfg(feature = "VK_EXT_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderPropertiesEXT;
#[cfg(feature = "VK_NV_mesh_shader")]
use crate::types::VkPhysicalDeviceMeshShaderPropertiesNV;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkPhysicalDeviceMultiDrawFeaturesEXT;
#[cfg(feature = "VK_EXT_multi_draw")]
use crate::types::VkPhysicalDeviceMultiDrawPropertiesEXT;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMultiviewFeatures;
#[cfg(feature = "VK_KHR_multiview")]
use crate::types::VkPhysicalDeviceMultiviewFeaturesKHR;
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
use crate::types::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
use crate::types::VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM;
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
use crate::types::VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceMultiviewProperties;
#[cfg(feature = "VK_KHR_multiview")]
use crate::types::VkPhysicalDeviceMultiviewPropertiesKHR;
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
use crate::types::VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT;
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
use crate::types::VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
#[cfg(feature = "VK_EXT_nested_command_buffer")]
use crate::types::VkPhysicalDeviceNestedCommandBufferFeaturesEXT;
#[cfg(feature = "VK_EXT_nested_command_buffer")]
use crate::types::VkPhysicalDeviceNestedCommandBufferPropertiesEXT;
#[cfg(feature = "VK_EXT_non_seamless_cube_map")]
use crate::types::VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapFeaturesKHR;
#[cfg(feature = "VK_EXT_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapPropertiesEXT;
#[cfg(feature = "VK_KHR_opacity_micromap")]
use crate::types::VkPhysicalDeviceOpacityMicromapPropertiesKHR;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkPhysicalDeviceOpticalFlowFeaturesNV;
#[cfg(feature = "VK_NV_optical_flow")]
use crate::types::VkPhysicalDeviceOpticalFlowPropertiesNV;
#[cfg(feature = "VK_EXT_pci_bus_info")]
use crate::types::VkPhysicalDevicePCIBusInfoPropertiesEXT;
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
use crate::types::VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV;
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
use crate::types::VkPhysicalDevicePerStageDescriptorSetFeaturesNV;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPhysicalDevicePerformanceCountersByRegionFeaturesARM;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkPhysicalDevicePerformanceCountersByRegionPropertiesARM;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPhysicalDevicePerformanceQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkPhysicalDevicePerformanceQueryPropertiesKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPhysicalDevicePipelineBinaryFeaturesKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPhysicalDevicePipelineBinaryPropertiesKHR;
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
use crate::types::VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDevicePipelineCreationCacheControlFeatures;
#[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
use crate::types::VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
#[cfg(feature = "VK_EXT_pipeline_library_group_handles")]
use crate::types::VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT;
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
use crate::types::VkPhysicalDevicePipelineOpacityMicromapFeaturesARM;
#[cfg(feature = "VK_EXT_pipeline_properties")]
use crate::types::VkPhysicalDevicePipelinePropertiesFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineProtectedAccessFeatures;
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
use crate::types::VkPhysicalDevicePipelineProtectedAccessFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineRobustnessFeatures;
#[cfg(feature = "VK_EXT_pipeline_robustness")]
use crate::types::VkPhysicalDevicePipelineRobustnessFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePipelineRobustnessProperties;
#[cfg(feature = "VK_EXT_pipeline_robustness")]
use crate::types::VkPhysicalDevicePipelineRobustnessPropertiesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDevicePointClippingProperties;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::types::VkPhysicalDevicePointClippingPropertiesKHR;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::types::VkPhysicalDevicePortabilitySubsetFeaturesKHR;
#[cfg(feature = "VK_KHR_portability_subset")]
use crate::types::VkPhysicalDevicePortabilitySubsetPropertiesKHR;
#[cfg(feature = "VK_NV_present_barrier")]
use crate::types::VkPhysicalDevicePresentBarrierFeaturesNV;
#[cfg(feature = "VK_KHR_present_id2")]
use crate::types::VkPhysicalDevicePresentId2FeaturesKHR;
#[cfg(feature = "VK_KHR_present_id")]
use crate::types::VkPhysicalDevicePresentIdFeaturesKHR;
#[cfg(feature = "VK_NV_present_metering")]
use crate::types::VkPhysicalDevicePresentMeteringFeaturesNV;
#[cfg(feature = "VK_EXT_present_mode_fifo_latest_ready")]
use crate::types::VkPhysicalDevicePresentModeFifoLatestReadyFeaturesEXT;
#[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
use crate::types::VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPhysicalDevicePresentTimingFeaturesEXT;
#[cfg(feature = "VK_KHR_present_wait2")]
use crate::types::VkPhysicalDevicePresentWait2FeaturesKHR;
#[cfg(feature = "VK_KHR_present_wait")]
use crate::types::VkPhysicalDevicePresentWaitFeaturesKHR;
#[cfg(feature = "VK_EXT_primitive_restart_index")]
use crate::types::VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT;
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
use crate::types::VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
#[cfg(feature = "VK_EXT_primitives_generated_query")]
use crate::types::VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDevicePrivateDataFeatures;
#[cfg(feature = "VK_EXT_private_data")]
use crate::types::VkPhysicalDevicePrivateDataFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProtectedMemoryFeatures;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceProtectedMemoryProperties;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPhysicalDeviceProvokingVertexFeaturesEXT;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPhysicalDeviceProvokingVertexPropertiesEXT;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPhysicalDevicePushConstantBankFeaturesNV;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPhysicalDevicePushConstantBankPropertiesNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDevicePushDescriptorProperties;
#[cfg(feature = "VK_KHR_push_descriptor")]
use crate::types::VkPhysicalDevicePushDescriptorPropertiesKHR;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPhysicalDeviceQueuePerfHintFeaturesQCOM;
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
use crate::types::VkPhysicalDeviceQueuePerfHintPropertiesQCOM;
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
use crate::types::VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT;
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
use crate::types::VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
#[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
use crate::types::VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT;
#[cfg(feature = "VK_NV_raw_access_chains")]
use crate::types::VkPhysicalDeviceRawAccessChainsFeaturesNV;
#[cfg(feature = "VK_KHR_ray_query")]
use crate::types::VkPhysicalDeviceRayQueryFeaturesKHR;
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT;
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV;
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT;
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
use crate::types::VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
use crate::types::VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
use crate::types::VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkPhysicalDeviceRayTracingMotionBlurFeaturesNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkPhysicalDeviceRayTracingPipelineFeaturesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkPhysicalDeviceRayTracingPipelinePropertiesKHR;
#[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
use crate::types::VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkPhysicalDeviceRayTracingPropertiesNV;
#[cfg(feature = "VK_NV_ray_tracing_validation")]
use crate::types::VkPhysicalDeviceRayTracingValidationFeaturesNV;
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
use crate::types::VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkPhysicalDeviceRenderPassStripedFeaturesARM;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkPhysicalDeviceRenderPassStripedPropertiesARM;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::types::VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV;
#[cfg(feature = "VK_EXT_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2FeaturesEXT;
#[cfg(feature = "VK_KHR_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2FeaturesKHR;
#[cfg(feature = "VK_EXT_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2PropertiesEXT;
#[cfg(feature = "VK_KHR_robustness2")]
use crate::types::VkPhysicalDeviceRobustness2PropertiesKHR;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkPhysicalDeviceSampleLocationsPropertiesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceSamplerFilterMinmaxProperties;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
use crate::types::VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSamplerYcbcrConversionFeatures;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceScalarBlockLayoutFeatures;
#[cfg(feature = "VK_EXT_scalar_block_layout")]
use crate::types::VkPhysicalDeviceScalarBlockLayoutFeaturesEXT;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsFeaturesARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsFlagsARM;
#[cfg(feature = "VK_ARM_scheduling_controls")]
use crate::types::VkPhysicalDeviceSchedulingControlsPropertiesARM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
use crate::types::VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_64bit_indexing")]
use crate::types::VkPhysicalDeviceShader64BitIndexingFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkPhysicalDeviceShaderAbortFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_abort")]
use crate::types::VkPhysicalDeviceShaderAbortPropertiesKHR;
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
use crate::types::VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT;
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
use crate::types::VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV;
#[cfg(feature = "VK_EXT_shader_atomic_float")]
use crate::types::VkPhysicalDeviceShaderAtomicFloatFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderAtomicInt64Features;
#[cfg(feature = "VK_KHR_shader_atomic_int64")]
use crate::types::VkPhysicalDeviceShaderAtomicInt64FeaturesKHR;
#[cfg(feature = "VK_KHR_shader_bfloat16")]
use crate::types::VkPhysicalDeviceShaderBfloat16FeaturesKHR;
#[cfg(feature = "VK_KHR_shader_clock")]
use crate::types::VkPhysicalDeviceShaderClockFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_constant_data")]
use crate::types::VkPhysicalDeviceShaderConstantDataFeaturesKHR;
#[cfg(feature = "VK_ARM_shader_core_builtins")]
use crate::types::VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM;
#[cfg(feature = "VK_ARM_shader_core_builtins")]
use crate::types::VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::types::VkPhysicalDeviceShaderCoreProperties2AMD;
#[cfg(feature = "VK_AMD_shader_core_properties")]
use crate::types::VkPhysicalDeviceShaderCorePropertiesAMD;
#[cfg(feature = "VK_ARM_shader_core_properties")]
use crate::types::VkPhysicalDeviceShaderCorePropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
#[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
use crate::types::VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceShaderDrawParameterFeatures;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPhysicalDeviceShaderDrawParametersFeatures;
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
use crate::types::VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPhysicalDeviceShaderEnqueueFeaturesAMDX;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPhysicalDeviceShaderEnqueuePropertiesAMDX;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderExpectAssumeFeatures;
#[cfg(feature = "VK_KHR_shader_expect_assume")]
use crate::types::VkPhysicalDeviceShaderExpectAssumeFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_float8")]
use crate::types::VkPhysicalDeviceShaderFloat8FeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderFloat16Int8Features;
#[cfg(feature = "VK_KHR_shader_float16_int8")]
use crate::types::VkPhysicalDeviceShaderFloat16Int8FeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderFloatControls2Features;
#[cfg(feature = "VK_KHR_shader_float_controls2")]
use crate::types::VkPhysicalDeviceShaderFloatControls2FeaturesKHR;
#[cfg(feature = "VK_KHR_shader_fma")]
use crate::types::VkPhysicalDeviceShaderFmaFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
use crate::types::VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
#[cfg(feature = "VK_NV_shader_image_footprint")]
use crate::types::VkPhysicalDeviceShaderImageFootprintFeaturesNV;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkPhysicalDeviceShaderInstrumentationFeaturesARM;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkPhysicalDeviceShaderInstrumentationPropertiesARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductFeatures;
#[cfg(feature = "VK_KHR_shader_integer_dot_product")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductProperties;
#[cfg(feature = "VK_KHR_shader_integer_dot_product")]
use crate::types::VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR;
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
use crate::types::VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
#[cfg(feature = "VK_EXT_shader_long_vector")]
use crate::types::VkPhysicalDeviceShaderLongVectorFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_long_vector")]
use crate::types::VkPhysicalDeviceShaderLongVectorPropertiesEXT;
#[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
use crate::types::VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR;
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
use crate::types::VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT;
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
use crate::types::VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM;
#[cfg(feature = "VK_QCOM_shader_multiple_wait_queues")]
use crate::types::VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkPhysicalDeviceShaderObjectFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkPhysicalDeviceShaderObjectPropertiesEXT;
#[cfg(feature = "VK_KHR_shader_quad_control")]
use crate::types::VkPhysicalDeviceShaderQuadControlFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
use crate::types::VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_replicated_composites")]
use crate::types::VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::types::VkPhysicalDeviceShaderSMBuiltinsFeaturesNV;
#[cfg(feature = "VK_NV_shader_sm_builtins")]
use crate::types::VkPhysicalDeviceShaderSMBuiltinsPropertiesNV;
#[cfg(feature = "VK_EXT_shader_split_barrier")]
use crate::types::VkPhysicalDeviceShaderSplitBarrierFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_split_barrier")]
use crate::types::VkPhysicalDeviceShaderSplitBarrierPropertiesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
#[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
use crate::types::VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
use crate::types::VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceShaderSubgroupRotateFeatures;
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
use crate::types::VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR;
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
use crate::types::VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceShaderTerminateInvocationFeatures;
#[cfg(feature = "VK_KHR_shader_terminate_invocation")]
use crate::types::VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR;
#[cfg(feature = "VK_EXT_shader_tile_image")]
use crate::types::VkPhysicalDeviceShaderTileImageFeaturesEXT;
#[cfg(feature = "VK_EXT_shader_tile_image")]
use crate::types::VkPhysicalDeviceShaderTileImagePropertiesEXT;
#[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
use crate::types::VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT;
#[cfg(feature = "VK_KHR_shader_untyped_pointers")]
use crate::types::VkPhysicalDeviceShaderUntypedPointersFeaturesKHR;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPhysicalDeviceShadingRateImageFeaturesNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPhysicalDeviceShadingRateImagePropertiesNV;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSparseImageFormatInfo2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkPhysicalDeviceSparseImageFormatInfo2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPhysicalDeviceSparseProperties;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceSubgroupProperties;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlFeatures;
#[cfg(feature = "VK_EXT_subgroup_size_control")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlProperties;
#[cfg(feature = "VK_EXT_subgroup_size_control")]
use crate::types::VkPhysicalDeviceSubgroupSizeControlPropertiesEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkPhysicalDeviceSubpassShadingFeaturesHUAWEI;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkPhysicalDeviceSubpassShadingPropertiesHUAWEI;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkPhysicalDeviceSurfaceInfo2KHR;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceSynchronization2Features;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkPhysicalDeviceSynchronization2FeaturesKHR;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceTensorFeaturesARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkPhysicalDeviceTensorPropertiesARM;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentProperties;
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
use crate::types::VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT;
#[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
use crate::types::VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
#[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
use crate::types::VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::types::VkPhysicalDeviceThrottleHintFeaturesSEC;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkPhysicalDeviceTileMemoryHeapFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkPhysicalDeviceTileMemoryHeapPropertiesQCOM;
#[cfg(feature = "VK_QCOM_tile_properties")]
use crate::types::VkPhysicalDeviceTilePropertiesFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPhysicalDeviceTileShadingFeaturesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkPhysicalDeviceTileShadingPropertiesQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreFeatures;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreFeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceTimelineSemaphoreProperties;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkPhysicalDeviceTimelineSemaphorePropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceToolProperties;
#[cfg(feature = "VK_EXT_tooling_info")]
use crate::types::VkPhysicalDeviceToolPropertiesEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPhysicalDeviceTransformFeedbackFeaturesEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPhysicalDeviceTransformFeedbackPropertiesEXT;
#[cfg(feature = "VK_KHR_unified_image_layouts")]
use crate::types::VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
#[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
use crate::types::VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceVariablePointerFeatures;
#[cfg(feature = "VK_KHR_variable_pointers")]
use crate::types::VkPhysicalDeviceVariablePointerFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkPhysicalDeviceVariablePointersFeatures;
#[cfg(feature = "VK_KHR_variable_pointers")]
use crate::types::VkPhysicalDeviceVariablePointersFeaturesKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorFeatures;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorProperties;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
use crate::types::VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR;
#[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
use crate::types::VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT;
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
use crate::types::VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkPhysicalDeviceVideoDecodeVP9FeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkPhysicalDeviceVideoEncodeAV1FeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkPhysicalDeviceVideoFormatInfoKHR;
#[cfg(feature = "VK_KHR_video_maintenance1")]
use crate::types::VkPhysicalDeviceVideoMaintenance1FeaturesKHR;
#[cfg(feature = "VK_KHR_video_maintenance2")]
use crate::types::VkPhysicalDeviceVideoMaintenance2FeaturesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan11Features;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan11Properties;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan12Features;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkan12Properties;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceVulkan13Features;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceVulkan13Properties;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVulkan14Features;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkPhysicalDeviceVulkan14Properties;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkPhysicalDeviceVulkanMemoryModelFeatures;
#[cfg(feature = "VK_KHR_vulkan_memory_model")]
use crate::types::VkPhysicalDeviceVulkanMemoryModelFeaturesKHR;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPhysicalDeviceVulkanSC10Features;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPhysicalDeviceVulkanSC10Properties;
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
use crate::types::VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
use crate::types::VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT;
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
use crate::types::VkPhysicalDeviceYcbcrDegammaFeaturesQCOM;
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
use crate::types::VkPhysicalDeviceYcbcrImageArraysFeaturesEXT;
#[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
use crate::types::VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
#[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
use crate::types::VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipeline;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryCreateInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryDataInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryDataKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryHandlesInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryKeyKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineBinaryKeysAndDataKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCache;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCacheCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCacheCreateInfo;
#[cfg(feature = "VK_QCOM_data_graph_model")]
use crate::types::VkPipelineCacheHeaderVersionDataGraphQCOM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCacheHeaderVersionOne;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineCacheHeaderVersionSafetyCriticalOne;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineCacheSafetyCriticalIndexEntry;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineCacheStageValidationIndexEntry;
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
use crate::types::VkPipelineColorBlendAdvancedStateCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineColorBlendAttachmentState;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineColorBlendStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineColorBlendStateCreateInfo;
#[cfg(feature = "VK_EXT_color_write_enable")]
use crate::types::VkPipelineColorWriteCreateInfoEXT;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::types::VkPipelineCompilerControlCreateInfoAMD;
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
use crate::types::VkPipelineCompilerControlFlagsAMD;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
use crate::types::VkPipelineCoverageModulationStateCreateFlagsNV;
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
use crate::types::VkPipelineCoverageModulationStateCreateInfoNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkPipelineCoverageReductionStateCreateFlagsNV;
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
use crate::types::VkPipelineCoverageReductionStateCreateInfoNV;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
use crate::types::VkPipelineCoverageToColorStateCreateFlagsNV;
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
use crate::types::VkPipelineCoverageToColorStateCreateInfoNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineCreateFlags2CreateInfo;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkPipelineCreateFlags2CreateInfoKHR;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkPipelineCreateFlags2KHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkPipelineCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedback;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackCreateInfo;
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
use crate::types::VkPipelineCreationFeedbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
use crate::types::VkPipelineCreationFeedbackEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineCreationFeedbackFlags;
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
use crate::types::VkPipelineCreationFeedbackFlagsEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineDepthStencilStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineDepthStencilStateCreateInfo;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::types::VkPipelineDiscardRectangleStateCreateFlagsEXT;
#[cfg(feature = "VK_EXT_discard_rectangles")]
use crate::types::VkPipelineDiscardRectangleStateCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineDynamicStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineDynamicStateCreateInfo;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineExecutableInfoKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineExecutableInternalRepresentationKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineExecutablePropertiesKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineExecutableStatisticKHR;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineExecutableStatisticValueKHR;
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
use crate::types::VkPipelineFragmentDensityMapLayeredCreateInfoVALVE;
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
use crate::types::VkPipelineFragmentShadingRateEnumStateCreateInfoNV;
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
use crate::types::VkPipelineFragmentShadingRateStateCreateInfoKHR;
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
use crate::types::VkPipelineIndirectDeviceAddressInfoNV;
#[cfg(feature = "VK_EXT_pipeline_properties")]
use crate::types::VkPipelineInfoEXT;
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
use crate::types::VkPipelineInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineInputAssemblyStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineInputAssemblyStateCreateInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayout;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayoutCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineLayoutCreateInfo;
#[cfg(feature = "VK_KHR_pipeline_library")]
use crate::types::VkPipelineLibraryCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineMultisampleStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineMultisampleStateCreateInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelineOfflineCreateInfo;
#[cfg(feature = "VKSC_VERSION_1_0")]
use crate::types::VkPipelinePoolSize;
#[cfg(feature = "VK_EXT_pipeline_properties")]
use crate::types::VkPipelinePropertiesIdentifierEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::types::VkPipelineRasterizationConservativeStateCreateFlagsEXT;
#[cfg(feature = "VK_EXT_conservative_rasterization")]
use crate::types::VkPipelineRasterizationConservativeStateCreateInfoEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::types::VkPipelineRasterizationDepthClipStateCreateFlagsEXT;
#[cfg(feature = "VK_EXT_depth_clip_enable")]
use crate::types::VkPipelineRasterizationDepthClipStateCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPipelineRasterizationLineStateCreateInfo;
#[cfg(feature = "VK_EXT_line_rasterization")]
use crate::types::VkPipelineRasterizationLineStateCreateInfoEXT;
#[cfg(feature = "VK_KHR_line_rasterization")]
use crate::types::VkPipelineRasterizationLineStateCreateInfoKHR;
#[cfg(feature = "VK_EXT_provoking_vertex")]
use crate::types::VkPipelineRasterizationProvokingVertexStateCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineRasterizationStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineRasterizationStateCreateInfo;
#[cfg(feature = "VK_AMD_rasterization_order")]
use crate::types::VkPipelineRasterizationStateRasterizationOrderAMD;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPipelineRasterizationStateStreamCreateFlagsEXT;
#[cfg(feature = "VK_EXT_transform_feedback")]
use crate::types::VkPipelineRasterizationStateStreamCreateInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkPipelineRenderingCreateInfo;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::types::VkPipelineRenderingCreateInfoKHR;
#[cfg(feature = "VK_NV_representative_fragment_test")]
use crate::types::VkPipelineRepresentativeFragmentTestStateCreateInfoNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPipelineRobustnessCreateInfo;
#[cfg(feature = "VK_EXT_pipeline_robustness")]
use crate::types::VkPipelineRobustnessCreateInfoEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkPipelineSampleLocationsStateCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPipelineShaderStageCreateInfo;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkPipelineShaderStageModuleIdentifierCreateInfoEXT;
#[cfg(feature = "VK_AMDX_shader_enqueue")]
use crate::types::VkPipelineShaderStageNodeCreateInfoAMDX;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
#[cfg(feature = "VK_EXT_subgroup_size_control")]
use crate::types::VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkPipelineStageFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPipelineStageFlags2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkPipelineStageFlags2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkPipelineTessellationDomainOriginStateCreateInfo;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::types::VkPipelineTessellationDomainOriginStateCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineTessellationStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineTessellationStateCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkPipelineVertexInputDivisorStateCreateInfo;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::types::VkPipelineVertexInputDivisorStateCreateInfoEXT;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
use crate::types::VkPipelineVertexInputDivisorStateCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineVertexInputStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineVertexInputStateCreateInfo;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPipelineViewportCoarseSampleOrderStateCreateInfoNV;
#[cfg(feature = "VK_EXT_depth_clamp_control")]
use crate::types::VkPipelineViewportDepthClampControlCreateInfoEXT;
#[cfg(feature = "VK_EXT_depth_clip_control")]
use crate::types::VkPipelineViewportDepthClipControlCreateInfoEXT;
#[cfg(feature = "VK_NV_scissor_exclusive")]
use crate::types::VkPipelineViewportExclusiveScissorStateCreateInfoNV;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkPipelineViewportShadingRateImageStateCreateInfoNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineViewportStateCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkPipelineViewportStateCreateInfo;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::types::VkPipelineViewportSwizzleStateCreateFlagsNV;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::types::VkPipelineViewportSwizzleStateCreateInfoNV;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
use crate::types::VkPipelineViewportWScalingStateCreateInfoNV;
#[cfg(feature = "VK_GGP_frame_token")]
use crate::types::VkPresentFrameTokenGGP;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::types::VkPresentGravityFlagsEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::types::VkPresentGravityFlagsKHR;
#[cfg(feature = "VK_KHR_present_id2")]
use crate::types::VkPresentId2KHR;
#[cfg(feature = "VK_KHR_present_id")]
use crate::types::VkPresentIdKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkPresentInfoKHR;
#[cfg(feature = "VK_KHR_incremental_present")]
use crate::types::VkPresentRegionKHR;
#[cfg(feature = "VK_KHR_incremental_present")]
use crate::types::VkPresentRegionsKHR;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::types::VkPresentScalingFlagsEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::types::VkPresentScalingFlagsKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentStageFlagsEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentStageTimeEXT;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::types::VkPresentTimeGOOGLE;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::types::VkPresentTimesInfoGOOGLE;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentTimingInfoEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentTimingInfoFlagsEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentTimingSurfaceCapabilitiesEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkPresentTimingsInfoEXT;
#[cfg(feature = "VK_KHR_present_wait2")]
use crate::types::VkPresentWait2InfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPrivateDataSlot;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPrivateDataSlotCreateFlags;
#[cfg(feature = "VK_EXT_private_data")]
use crate::types::VkPrivateDataSlotCreateFlagsEXT;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkPrivateDataSlotCreateInfo;
#[cfg(feature = "VK_EXT_private_data")]
use crate::types::VkPrivateDataSlotCreateInfoEXT;
#[cfg(feature = "VK_EXT_private_data")]
use crate::types::VkPrivateDataSlotEXT;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkProtectedSubmitInfo;
#[cfg(feature = "VK_NV_push_constant_bank")]
use crate::types::VkPushConstantBankInfoNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkPushConstantRange;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushConstantsInfo;
#[cfg(feature = "VK_KHR_maintenance6")]
use crate::types::VkPushConstantsInfoKHR;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkPushDataInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushDescriptorSetInfo;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]
use crate::types::VkPushDescriptorSetInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
use crate::types::VkPushDescriptorSetWithTemplateInfo;
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]
use crate::types::VkPushDescriptorSetWithTemplateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryControlFlags;
#[cfg(feature = "VK_NV_low_latency")]
use crate::types::VkQueryLowLatencySupportNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryPipelineStatisticFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryPool;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryPoolCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryPoolCreateInfo;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkQueryPoolCreateInfoINTEL;
#[cfg(feature = "VK_KHR_performance_query")]
use crate::types::VkQueryPoolPerformanceCreateInfoKHR;
#[cfg(feature = "VK_INTEL_performance_query")]
use crate::types::VkQueryPoolPerformanceQueryCreateInfoINTEL;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkQueryPoolVideoEncodeFeedbackCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueryResultFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueue;
#[cfg(any(
  all(
    feature = "VK_NV_device_diagnostic_checkpoints",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_synchronization2",
    feature = "VK_NV_device_diagnostic_checkpoints"
  )
))]
use crate::types::VkQueueFamilyCheckpointProperties2NV;
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
use crate::types::VkQueueFamilyCheckpointPropertiesNV;
#[cfg(feature = "VK_ARM_data_graph_optical_flow")]
use crate::types::VkQueueFamilyDataGraphOpticalFlowPropertiesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkQueueFamilyDataGraphProcessingEnginePropertiesARM;
#[cfg(feature = "VK_ARM_data_graph")]
use crate::types::VkQueueFamilyDataGraphPropertiesARM;
#[cfg(feature = "VK_ARM_data_graph_instruction_set_tosa")]
use crate::types::VkQueueFamilyDataGraphTOSAPropertiesARM;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkQueueFamilyGlobalPriorityProperties;
#[cfg(feature = "VK_EXT_global_priority_query")]
use crate::types::VkQueueFamilyGlobalPriorityPropertiesEXT;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::types::VkQueueFamilyGlobalPriorityPropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance11")]
use crate::types::VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR;
#[cfg(feature = "VK_KHR_maintenance9")]
use crate::types::VkQueueFamilyOwnershipTransferPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFamilyProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkQueueFamilyProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkQueueFamilyProperties2KHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkQueueFamilyQueryResultStatusPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkQueueFamilyVideoPropertiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkQueueFlags;
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  feature = "VK_NV_cluster_acceleration_structure"
))]
use crate::types::VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkRayTracingPipelineCreateInfoNV;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingPipelineInterfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkRayTracingShaderGroupCreateInfoKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkRayTracingShaderGroupCreateInfoNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkRect2D;
#[cfg(feature = "VK_KHR_incremental_present")]
use crate::types::VkRectLayerKHR;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::types::VkRefreshCycleDurationGOOGLE;
#[cfg(feature = "VK_KHR_object_refresh")]
use crate::types::VkRefreshObjectFlagsKHR;
#[cfg(feature = "VK_KHR_object_refresh")]
use crate::types::VkRefreshObjectKHR;
#[cfg(feature = "VK_KHR_object_refresh")]
use crate::types::VkRefreshObjectListKHR;
#[cfg(feature = "VK_KHR_pipeline_binary")]
use crate::types::VkReleaseCapturedPipelineDataInfoKHR;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkReleaseSwapchainImagesInfoEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkReleaseSwapchainImagesInfoKHR;
#[cfg(feature = "VK_NV_external_memory_rdma")]
use crate::types::VkRemoteAddressNV;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPass;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkRenderPassAttachmentBeginInfo;
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
use crate::types::VkRenderPassAttachmentBeginInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassBeginInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassCreateFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkRenderPassCreateInfo;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkRenderPassCreateInfo2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkRenderPassCreateInfo2KHR;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassCreationControlEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassCreationFeedbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassCreationFeedbackInfoEXT;
#[cfg(feature = "VK_EXT_fragment_density_map")]
use crate::types::VkRenderPassFragmentDensityMapCreateInfoEXT;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkRenderPassFragmentDensityMapOffsetEndInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkRenderPassInputAttachmentAspectCreateInfo;
#[cfg(feature = "VK_KHR_maintenance2")]
use crate::types::VkRenderPassInputAttachmentAspectCreateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_1")]
use crate::types::VkRenderPassMultiviewCreateInfo;
#[cfg(feature = "VK_KHR_multiview")]
use crate::types::VkRenderPassMultiviewCreateInfoKHR;
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
use crate::types::VkRenderPassPerformanceCountersByRegionBeginInfoARM;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkRenderPassSampleLocationsBeginInfoEXT;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeBeginInfoARM;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeInfoARM;
#[cfg(feature = "VK_ARM_render_pass_striped")]
use crate::types::VkRenderPassStripeSubmitInfoARM;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassSubpassFeedbackCreateInfoEXT;
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
use crate::types::VkRenderPassSubpassFeedbackInfoEXT;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkRenderPassTileShadingCreateInfoQCOM;
#[cfg(feature = "VK_QCOM_render_pass_transform")]
use crate::types::VkRenderPassTransformBeginInfoQCOM;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingAreaInfo;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkRenderingAreaInfoKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkRenderingAttachmentFlagsInfoKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkRenderingAttachmentFlagsKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingAttachmentInfo;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::types::VkRenderingAttachmentInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingAttachmentLocationInfo;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
use crate::types::VkRenderingAttachmentLocationInfoKHR;
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
use crate::types::VkRenderingEndInfoEXT;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkRenderingEndInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingFlags;
#[cfg(feature = "VK_KHR_dynamic_rendering")]
use crate::types::VkRenderingFlagsKHR;
#[cfg(any(
  all(feature = "VK_EXT_fragment_density_map", feature = "VK_VERSION_1_3"),
  all(
    feature = "VK_EXT_fragment_density_map",
    feature = "VK_KHR_dynamic_rendering"
  )
))]
use crate::types::VkRenderingFragmentDensityMapAttachmentInfoEXT;
#[cfg(any(
  all(feature = "VK_KHR_fragment_shading_rate", feature = "VK_VERSION_1_3"),
  all(
    feature = "VK_KHR_dynamic_rendering",
    feature = "VK_KHR_fragment_shading_rate"
  )
))]
use crate::types::VkRenderingFragmentShadingRateAttachmentInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkRenderingInfo;
#[cfg(any(
  feature = "VK_KHR_dynamic_rendering",
  all(feature = "VK_QCOM_tile_properties", feature = "VK_VERSION_1_3")
))]
use crate::types::VkRenderingInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkRenderingInputAttachmentIndexInfo;
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
use crate::types::VkRenderingInputAttachmentIndexInfoKHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkResolveImageFlagsKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
use crate::types::VkResolveImageInfo2;
#[cfg(feature = "VK_KHR_copy_commands2")]
use crate::types::VkResolveImageInfo2KHR;
#[cfg(feature = "VK_KHR_maintenance10")]
use crate::types::VkResolveImageModeInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkResolveModeFlags;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
use crate::types::VkResolveModeFlagsKHR;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkResourceDescriptorDataEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkResourceDescriptorInfoEXT;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::types::VkSRTDataNV;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSampleCountFlags;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkSampleLocationEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkSampleLocationsInfoEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkSampleMask;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSampler;
#[cfg(feature = "VK_QCOM_image_processing2")]
use crate::types::VkSamplerBlockMatchWindowCreateInfoQCOM;
#[cfg(feature = "VK_EXT_border_color_swizzle")]
use crate::types::VkSamplerBorderColorComponentMappingCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_buffer")]
use crate::types::VkSamplerCaptureDescriptorDataInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSamplerCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSamplerCreateInfo;
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
use crate::types::VkSamplerCubicWeightsCreateInfoQCOM;
#[cfg(feature = "VK_EXT_custom_border_color")]
use crate::types::VkSamplerCustomBorderColorCreateInfoEXT;
#[cfg(all(
  feature = "VK_EXT_custom_border_color",
  feature = "VK_EXT_descriptor_heap"
))]
use crate::types::VkSamplerCustomBorderColorIndexCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_2")]
use crate::types::VkSamplerReductionModeCreateInfo;
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
use crate::types::VkSamplerReductionModeCreateInfoEXT;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversion;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionCreateInfo;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkSamplerYcbcrConversionCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionImageFormatProperties;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkSamplerYcbcrConversionImageFormatPropertiesKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
use crate::types::VkSamplerYcbcrConversionInfo;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkSamplerYcbcrConversionInfoKHR;
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
use crate::types::VkSamplerYcbcrConversionKHR;
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
use crate::types::VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM;
#[cfg(any(
  feature = "VK_NV_external_sci_sync",
  feature = "VK_NV_external_sci_sync2"
))]
use crate::types::VkSciSyncAttributesInfoNV;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkScreenBufferFormatPropertiesQNX;
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
use crate::types::VkScreenBufferPropertiesQNX;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::types::VkScreenSurfaceCreateFlagsQNX;
#[cfg(feature = "VK_QNX_screen_surface")]
use crate::types::VkScreenSurfaceCreateInfoQNX;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphore;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphoreCreateFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSemaphoreCreateInfo;
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
use crate::types::VkSemaphoreGetFdInfoKHR;
#[cfg(feature = "VK_NV_external_sci_sync")]
use crate::types::VkSemaphoreGetSciSyncInfoNV;
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
use crate::types::VkSemaphoreGetWin32HandleInfoKHR;
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
use crate::types::VkSemaphoreGetZirconHandleInfoFUCHSIA;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSemaphoreImportFlags;
#[cfg(feature = "VK_KHR_external_semaphore")]
use crate::types::VkSemaphoreImportFlagsKHR;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkSemaphoreSciSyncCreateInfoNV;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkSemaphoreSciSyncPoolCreateInfoNV;
#[cfg(feature = "VK_NV_external_sci_sync2")]
use crate::types::VkSemaphoreSciSyncPoolNV;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreSignalInfo;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkSemaphoreSignalInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSemaphoreSubmitInfo;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkSemaphoreSubmitInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreTypeCreateInfo;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkSemaphoreTypeCreateInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreWaitFlags;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkSemaphoreWaitFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkSemaphoreWaitInfo;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkSemaphoreWaitInfoKHR;
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
use crate::types::VkSetDescriptorBufferOffsetsInfoEXT;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkSetLatencyMarkerInfoNV;
#[cfg(feature = "VK_NV_present_metering")]
use crate::types::VkSetPresentConfigNV;
#[cfg(feature = "VK_NV_device_generated_commands")]
use crate::types::VkSetStateFlagsIndirectCommandNV;
#[cfg(feature = "VK_AMD_shader_core_properties2")]
use crate::types::VkShaderCorePropertiesFlagsAMD;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkShaderCreateFlagsEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkShaderCreateInfoEXT;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkShaderDescriptorSetAndBindingMappingInfoEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkShaderEXT;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationARM;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationCreateInfoARM;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationMetricDataHeaderARM;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationMetricDescriptionARM;
#[cfg(feature = "VK_ARM_shader_instrumentation")]
use crate::types::VkShaderInstrumentationValuesFlagsARM;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModule;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModuleCreateFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkShaderModuleCreateInfo;
#[cfg(feature = "VK_EXT_shader_module_identifier")]
use crate::types::VkShaderModuleIdentifierEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::types::VkShaderModuleValidationCacheCreateInfoEXT;
#[cfg(feature = "VK_EXT_shader_object")]
use crate::types::VkShaderRequiredSubgroupSizeCreateInfoEXT;
#[cfg(feature = "VK_AMD_shader_info")]
use crate::types::VkShaderResourceUsageAMD;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkShaderStageFlags;
#[cfg(feature = "VK_AMD_shader_info")]
use crate::types::VkShaderStatisticsInfoAMD;
#[cfg(feature = "VK_NV_shading_rate_image")]
use crate::types::VkShadingRatePaletteNV;
#[cfg(feature = "VK_KHR_shared_presentable_image")]
use crate::types::VkSharedPresentSurfaceCapabilitiesKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseBufferMemoryBindInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageFormatFlags;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageFormatProperties;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSparseImageFormatProperties2;
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
use crate::types::VkSparseImageFormatProperties2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageMemoryBind;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageMemoryBindInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageMemoryRequirements;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSparseImageMemoryRequirements2;
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
use crate::types::VkSparseImageMemoryRequirements2KHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseImageOpaqueMemoryBindInfo;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseMemoryBind;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSparseMemoryBindFlags;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSpecializationInfo;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkSpecializationMapEntry;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkSpirvResourceTypeFlagsEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkStencilFaceFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkStencilOpState;
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
use crate::types::VkStreamDescriptorSurfaceCreateFlagsGGP;
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
use crate::types::VkStreamDescriptorSurfaceCreateInfoGGP;
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
use crate::types::VkStridedDeviceAddressNV;
#[cfg(any(
  feature = "VK_KHR_device_address_commands",
  feature = "VK_KHR_copy_memory_indirect"
))]
use crate::types::VkStridedDeviceAddressRangeKHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkStridedDeviceAddressRegionKHR;
#[cfg(feature = "VK_BASE_VERSION_1_1")]
use crate::types::VkSubgroupFeatureFlags;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSubmitFlags;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkSubmitFlagsKHR;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubmitInfo;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkSubmitInfo2;
#[cfg(feature = "VK_KHR_synchronization2")]
use crate::types::VkSubmitInfo2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassBeginInfo;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkSubpassBeginInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkSubpassDependency;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassDependency2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkSubpassDependency2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkSubpassDescription;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassDescription2;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkSubpassDescription2KHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassDescriptionDepthStencilResolve;
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
use crate::types::VkSubpassDescriptionDepthStencilResolveKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkSubpassDescriptionFlags;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
use crate::types::VkSubpassEndInfo;
#[cfg(feature = "VK_KHR_create_renderpass2")]
use crate::types::VkSubpassEndInfoKHR;
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
use crate::types::VkSubpassFragmentDensityMapOffsetEndInfoQCOM;
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
use crate::types::VkSubpassResolvePerformanceQueryEXT;
#[cfg(feature = "VK_EXT_sample_locations")]
use crate::types::VkSubpassSampleLocationsEXT;
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
use crate::types::VkSubpassShadingPipelineCreateInfoHUAWEI;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkSubresourceHostMemcpySize;
#[cfg(feature = "VK_EXT_host_image_copy")]
use crate::types::VkSubresourceHostMemcpySizeEXT;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
use crate::types::VkSubresourceLayout;
#[cfg(feature = "VK_BASE_VERSION_1_4")]
use crate::types::VkSubresourceLayout2;
#[cfg(any(
  feature = "VK_EXT_host_image_copy",
  feature = "VK_EXT_image_compression_control"
))]
use crate::types::VkSubresourceLayout2EXT;
#[cfg(feature = "VK_KHR_maintenance5")]
use crate::types::VkSubresourceLayout2KHR;
#[cfg(all(
  feature = "VK_EXT_descriptor_heap",
  feature = "VK_EXT_fragment_density_map"
))]
use crate::types::VkSubsampledImageFormatPropertiesEXT;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::types::VkSurfaceCapabilities2EXT;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceCapabilities2KHR;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::types::VkSurfaceCapabilitiesFullScreenExclusiveEXT;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceCapabilitiesKHR;
#[cfg(feature = "VK_NV_present_barrier")]
use crate::types::VkSurfaceCapabilitiesPresentBarrierNV;
#[cfg(feature = "VK_KHR_present_id2")]
use crate::types::VkSurfaceCapabilitiesPresentId2KHR;
#[cfg(feature = "VK_KHR_present_wait2")]
use crate::types::VkSurfaceCapabilitiesPresentWait2KHR;
#[cfg(feature = "VK_EXT_display_surface_counter")]
use crate::types::VkSurfaceCounterFlagsEXT;
#[cfg(feature = "VK_OHOS_surface")]
use crate::types::VkSurfaceCreateFlagsOHOS;
#[cfg(feature = "VK_OHOS_surface")]
use crate::types::VkSurfaceCreateInfoOHOS;
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
use crate::types::VkSurfaceFormat2KHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceFormatKHR;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::types::VkSurfaceFullScreenExclusiveInfoEXT;
#[cfg(all(
  feature = "VK_EXT_full_screen_exclusive",
  feature = "VK_KHR_win32_surface"
))]
use crate::types::VkSurfaceFullScreenExclusiveWin32InfoEXT;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceKHR;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::types::VkSurfacePresentModeCompatibilityEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::types::VkSurfacePresentModeCompatibilityKHR;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::types::VkSurfacePresentModeEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::types::VkSurfacePresentModeKHR;
#[cfg(feature = "VK_EXT_surface_maintenance1")]
use crate::types::VkSurfacePresentScalingCapabilitiesEXT;
#[cfg(feature = "VK_KHR_surface_maintenance1")]
use crate::types::VkSurfacePresentScalingCapabilitiesKHR;
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
use crate::types::VkSurfaceProtectedCapabilitiesKHR;
#[cfg(feature = "VK_KHR_surface")]
use crate::types::VkSurfaceTransformFlagsKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkSwapchainCalibratedTimestampInfoEXT;
#[cfg(feature = "VK_EXT_display_control")]
use crate::types::VkSwapchainCounterCreateInfoEXT;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainCreateFlagsKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainCreateInfoKHR;
#[cfg(feature = "VK_AMD_display_native_hdr")]
use crate::types::VkSwapchainDisplayNativeHdrCreateInfoAMD;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::types::VkSwapchainKHR;
#[cfg(feature = "VK_NV_low_latency2")]
use crate::types::VkSwapchainLatencyCreateInfoNV;
#[cfg(feature = "VK_NV_present_barrier")]
use crate::types::VkSwapchainPresentBarrierCreateInfoNV;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentFenceInfoEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentFenceInfoKHR;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentModeInfoEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentModeInfoKHR;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentModesCreateInfoEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentModesCreateInfoKHR;
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentScalingCreateInfoEXT;
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
use crate::types::VkSwapchainPresentScalingCreateInfoKHR;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkSwapchainTimeDomainPropertiesEXT;
#[cfg(feature = "VK_EXT_present_timing")]
use crate::types::VkSwapchainTimingPropertiesEXT;
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
use crate::types::VkSysmemColorSpaceFUCHSIA;
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
use crate::types::VkTensorARM;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkTensorCaptureDescriptorDataInfoARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorCopyARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorCreateFlagsARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorCreateInfoARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorDependencyInfoARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorDescriptionARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorFormatPropertiesARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorMemoryBarrierARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorMemoryRequirementsInfoARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorUsageFlagsARM;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkTensorViewARM;
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
use crate::types::VkTensorViewCaptureDescriptorDataInfoARM;
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
use crate::types::VkTensorViewCreateFlagsARM;
#[cfg(any(feature = "VK_EXT_descriptor_heap", feature = "VK_ARM_tensors"))]
use crate::types::VkTensorViewCreateInfoARM;
#[cfg(feature = "VK_EXT_descriptor_heap")]
use crate::types::VkTexelBufferDescriptorInfoEXT;
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
use crate::types::VkTextureLODGatherFormatPropertiesAMD;
#[cfg(feature = "VK_SEC_throttle_hint")]
use crate::types::VkThrottleHintSubmitInfoSEC;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkTileMemoryBindInfoQCOM;
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
use crate::types::VkTileMemoryRequirementsQCOM;
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  feature = "VK_QCOM_tile_properties"
))]
use crate::types::VkTileMemorySizeInfoQCOM;
#[cfg(feature = "VK_QCOM_tile_properties")]
use crate::types::VkTilePropertiesQCOM;
#[cfg(feature = "VK_QCOM_tile_shading")]
use crate::types::VkTileShadingRenderPassFlagsQCOM;
#[cfg(feature = "VK_BASE_VERSION_1_2")]
use crate::types::VkTimelineSemaphoreSubmitInfo;
#[cfg(feature = "VK_KHR_timeline_semaphore")]
use crate::types::VkTimelineSemaphoreSubmitInfoKHR;
#[cfg(feature = "VK_BASE_VERSION_1_3")]
use crate::types::VkToolPurposeFlags;
#[cfg(feature = "VK_EXT_tooling_info")]
use crate::types::VkToolPurposeFlagsEXT;
#[cfg(all(
  feature = "VK_KHR_ray_tracing_maintenance1",
  feature = "VK_KHR_ray_tracing_pipeline"
))]
use crate::types::VkTraceRaysIndirectCommand2KHR;
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
use crate::types::VkTraceRaysIndirectCommandKHR;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkTransformMatrixKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkTransformMatrixNV;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::types::VkUbmSurfaceCreateFlagsSEC;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::types::VkUbmSurfaceCreateInfoSEC;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::types::VkValidationCacheCreateFlagsEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::types::VkValidationCacheCreateInfoEXT;
#[cfg(feature = "VK_EXT_validation_cache")]
use crate::types::VkValidationCacheEXT;
#[cfg(feature = "VK_EXT_validation_features")]
use crate::types::VkValidationFeaturesEXT;
#[cfg(feature = "VK_EXT_validation_flags")]
use crate::types::VkValidationFlagsEXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkVertexInputAttributeDescription;
#[cfg(any(
  feature = "VK_EXT_vertex_input_dynamic_state",
  feature = "VK_EXT_shader_object"
))]
use crate::types::VkVertexInputAttributeDescription2EXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkVertexInputBindingDescription;
#[cfg(any(
  feature = "VK_EXT_vertex_input_dynamic_state",
  feature = "VK_EXT_shader_object"
))]
use crate::types::VkVertexInputBindingDescription2EXT;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
use crate::types::VkVertexInputBindingDivisorDescription;
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
use crate::types::VkVertexInputBindingDivisorDescriptionEXT;
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
use crate::types::VkVertexInputBindingDivisorDescriptionKHR;
#[cfg(feature = "VK_NN_vi_surface")]
use crate::types::VkViSurfaceCreateFlagsNN;
#[cfg(feature = "VK_NN_vi_surface")]
use crate::types::VkViSurfaceCreateInfoNN;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoBeginCodingFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoBeginCodingInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoChromaSubsamplingFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCodecOperationFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCodingControlFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoCodingControlInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoComponentBitDepthFlagsKHR;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1DpbSlotInfoKHR;
#[cfg(all(
  feature = "VK_KHR_video_decode_av1",
  feature = "VK_KHR_video_maintenance2"
))]
use crate::types::VkVideoDecodeAV1InlineSessionParametersInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_av1")]
use crate::types::VkVideoDecodeAV1SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeCapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeFlagsKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264DpbSlotInfoKHR;
#[cfg(all(
  feature = "VK_KHR_video_decode_h264",
  feature = "VK_KHR_video_maintenance2"
))]
use crate::types::VkVideoDecodeH264InlineSessionParametersInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264PictureLayoutFlagsKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264SessionParametersAddInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h264")]
use crate::types::VkVideoDecodeH264SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265DpbSlotInfoKHR;
#[cfg(all(
  feature = "VK_KHR_video_decode_h265",
  feature = "VK_KHR_video_maintenance2"
))]
use crate::types::VkVideoDecodeH265InlineSessionParametersInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265SessionParametersAddInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_h265")]
use crate::types::VkVideoDecodeH265SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeUsageFlagsKHR;
#[cfg(feature = "VK_KHR_video_decode_queue")]
use crate::types::VkVideoDecodeUsageInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkVideoDecodeVP9CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkVideoDecodeVP9PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_decode_vp9")]
use crate::types::VkVideoDecodeVP9ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1CapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1DpbSlotInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1FrameSizeKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1GopRemainingFrameInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1QIndexKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1QualityLevelPropertiesKHR;
#[cfg(all(
  feature = "VK_KHR_video_encode_av1",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::types::VkVideoEncodeAV1QuantizationMapCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1RateControlFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1RateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1RateControlLayerInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1SessionCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1StdFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_av1")]
use crate::types::VkVideoEncodeAV1SuperblockSizeFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeCapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeContentFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeFeedbackFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264CapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264DpbSlotInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264FrameSizeKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264GopRemainingFrameInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264NaluSliceInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264QpKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264QualityLevelPropertiesKHR;
#[cfg(all(
  feature = "VK_KHR_video_encode_h264",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::types::VkVideoEncodeH264QuantizationMapCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264RateControlFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264RateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264RateControlLayerInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264SessionCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264SessionParametersAddInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264SessionParametersFeedbackInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264SessionParametersGetInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h264")]
use crate::types::VkVideoEncodeH264StdFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265CapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265CapabilityFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265CtbSizeFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265DpbSlotInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265FrameSizeKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265GopRemainingFrameInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265NaluSliceSegmentInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265PictureInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265ProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265QpKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265QualityLevelPropertiesKHR;
#[cfg(all(
  feature = "VK_KHR_video_encode_h265",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::types::VkVideoEncodeH265QuantizationMapCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265RateControlFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265RateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265RateControlLayerInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265SessionCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265SessionParametersAddInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265SessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265SessionParametersFeedbackInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265SessionParametersGetInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265StdFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_h265")]
use crate::types::VkVideoEncodeH265TransformBlockSizeFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkVideoEncodeIntraRefreshCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkVideoEncodeIntraRefreshInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkVideoEncodeIntraRefreshModeFlagsKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeProfileRgbConversionInfoVALVE;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeQualityLevelInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeQualityLevelPropertiesKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkVideoEncodeQuantizationMapCapabilitiesKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkVideoEncodeQuantizationMapInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeRateControlFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeRateControlInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeRateControlLayerInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeRateControlModeFlagsKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeRgbChromaOffsetFlagsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeRgbConversionCapabilitiesVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeRgbModelConversionFlagsVALVE;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeRgbRangeCompressionFlagsVALVE;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkVideoEncodeSessionIntraRefreshCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeSessionParametersFeedbackInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeSessionParametersGetInfoKHR;
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
use crate::types::VkVideoEncodeSessionRgbConversionCreateInfoVALVE;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeUsageFlagsKHR;
#[cfg(feature = "VK_KHR_video_encode_queue")]
use crate::types::VkVideoEncodeUsageInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoEndCodingFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoEndCodingInfoKHR;
#[cfg(all(
  feature = "VK_KHR_video_encode_av1",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::types::VkVideoFormatAV1QuantizationMapPropertiesKHR;
#[cfg(all(
  feature = "VK_KHR_video_encode_h265",
  feature = "VK_KHR_video_encode_quantization_map"
))]
use crate::types::VkVideoFormatH265QuantizationMapPropertiesKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoFormatPropertiesKHR;
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
use crate::types::VkVideoFormatQuantizationMapPropertiesKHR;
#[cfg(feature = "VK_KHR_video_maintenance1")]
use crate::types::VkVideoInlineQueryInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoPictureResourceInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoProfileListInfoKHR;
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
use crate::types::VkVideoReferenceIntraRefreshInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoReferenceSlotInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionCreateFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionMemoryRequirementsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionParametersCreateFlagsKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionParametersCreateInfoKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionParametersKHR;
#[cfg(feature = "VK_KHR_video_queue")]
use crate::types::VkVideoSessionParametersUpdateInfoKHR;
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
use crate::types::VkViewport;
#[cfg(feature = "VK_NV_viewport_swizzle")]
use crate::types::VkViewportSwizzleNV;
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
use crate::types::VkViewportWScalingNV;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::types::VkWaylandSurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::types::VkWaylandSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoKHR;
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
use crate::types::VkWin32KeyedMutexAcquireReleaseInfoNV;
#[cfg(feature = "VK_KHR_win32_surface")]
use crate::types::VkWin32SurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_win32_surface")]
use crate::types::VkWin32SurfaceCreateInfoKHR;
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
use crate::types::VkWriteDescriptorSet;
#[cfg(feature = "VK_KHR_acceleration_structure")]
use crate::types::VkWriteDescriptorSetAccelerationStructureKHR;
#[cfg(feature = "VK_NV_ray_tracing")]
use crate::types::VkWriteDescriptorSetAccelerationStructureNV;
#[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
use crate::types::VkWriteDescriptorSetInlineUniformBlock;
#[cfg(feature = "VK_EXT_inline_uniform_block")]
use crate::types::VkWriteDescriptorSetInlineUniformBlockEXT;
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
use crate::types::VkWriteDescriptorSetPartitionedAccelerationStructureNV;
#[cfg(feature = "VK_ARM_tensors")]
use crate::types::VkWriteDescriptorSetTensorARM;
#[cfg(feature = "VK_EXT_device_generated_commands")]
use crate::types::VkWriteIndirectExecutionSetPipelineEXT;
#[cfg(all(
  feature = "VK_EXT_device_generated_commands",
  feature = "VK_EXT_shader_object"
))]
use crate::types::VkWriteIndirectExecutionSetShaderEXT;
#[cfg(feature = "VK_EXT_hdr_metadata")]
use crate::types::VkXYColorEXT;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::VkXcbSurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::VkXcbSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::types::VkXlibSurfaceCreateFlagsKHR;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::types::VkXlibSurfaceCreateInfoKHR;
#[cfg(feature = "VK_KHR_xlib_surface")]
use crate::types::Window;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::types::ubm_device;
#[cfg(feature = "VK_SEC_ubm_surface")]
use crate::types::ubm_surface;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::types::wl_display;
#[cfg(feature = "VK_KHR_wayland_surface")]
use crate::types::wl_surface;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::xcb_connection_t;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::xcb_visualid_t;
#[cfg(feature = "VK_KHR_xcb_surface")]
use crate::types::xcb_window_t;
#[cfg(any(
  feature = "VK_FUCHSIA_imagepipe_surface",
  feature = "VK_FUCHSIA_external_memory",
  feature = "VK_FUCHSIA_external_semaphore"
))]
use crate::types::zx_handle_t;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
  unsafe extern "system" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
/// [`vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
/// - `VK_KHR_swapchain`
///
/// - **Availability:** depends on `VK_VERSION_1_1`
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
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_swapchain")
))]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pAcquireInfo: *const VkAcquireNextImageInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
  device: VkDevice,
  pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_TIMEOUT`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_performance_query")]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkAcquireProfilingLockInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
  device: VkDevice,
  pAllocateInfo: *const VkCommandBufferAllocateInfo<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_FRAGMENTED_POOL`
///   - `VK_ERROR_OUT_OF_POOL_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
  device: VkDevice,
  pAllocateInfo: *const VkDescriptorSetAllocateInfo<'_>,
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
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
  device: VkDevice,
  pAllocateInfo: *const VkMemoryAllocateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  unsafe extern "system" fn(device: VkDevice, pData: *const VkAntiLagDataAMD<'_>);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBeginInfo: *const VkCommandBufferBeginInfo<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindBufferMemoryInfo<'_>,
) -> VkResult;
/// [`vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_bind_memory2`
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindBufferMemory2KHR = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindBufferMemoryInfoKHR<'_>,
) -> VkResult;
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
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindDataGraphPipelineSessionMemoryInfoARM<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindImageMemoryInfo<'_>,
) -> VkResult;
/// [`vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_bind_memory2`
///
#[cfg(feature = "VK_KHR_bind_memory2")]
pub type PFN_vkBindImageMemory2KHR = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindImageMemoryInfoKHR<'_>,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
  device: VkDevice,
  bindInfoCount: u32,
  pBindInfos: *const VkBindTensorMemoryInfoARM<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
  device: VkDevice,
  videoSession: VkVideoSessionKHR,
  bindSessionMemoryInfoCount: u32,
  pBindSessionMemoryInfos: *const VkBindVideoSessionMemoryInfoKHR<'_>,
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
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  infoCount: u32,
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR<'_>,
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
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  infoCount: u32,
  pInfos: *const VkMicromapBuildInfoEXT<'_>,
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
/// - **Availability:** depends on `VK_EXT_conditional_rendering`
///
/// # Parameters
/// - `commandBuffer`
/// - `pConditionalRenderingBegin`
#[cfg(all(
  feature = "VK_EXT_conditional_rendering",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdBeginConditionalRendering2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfo2EXT<'_>,
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
  pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT<'_>,
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
/// - **Availability:** depends on `VK_KHR_dynamic_rendering + VK_VERSION_1_3`
///
/// # Parameters
/// - `commandBuffer`
/// - `pBeginCustomResolveInfo`: optional: true
#[cfg(any(
  all(
    feature = "VK_EXT_custom_resolve",
    feature = "VK_KHR_dynamic_rendering"
  ),
  all(feature = "VK_EXT_custom_resolve", feature = "VK_VERSION_1_3")
))]
pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBeginCustomResolveInfo: *const VkBeginCustomResolveInfoEXT<'_>,
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
  pLabelInfo: *const VkDebugUtilsLabelEXT<'_>,
);
/// [`vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `gpaSession`
/// - `pGpaSampleBeginInfo`
/// - `pSampleID`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCmdBeginGpaSampleAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  gpaSession: VkGpaSessionAMD,
  pGpaSampleBeginInfo: *const VkGpaSampleBeginInfoAMD<'_>,
  pSampleID: *mut u32,
) -> VkResult;
/// [`vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `gpaSession`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCmdBeginGpaSessionAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  gpaSession: VkGpaSessionAMD,
) -> VkResult;
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
  pPerTileBeginInfo: *const VkPerTileBeginInfoQCOM<'_>,
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
  pRenderPassBegin: *const VkRenderPassBeginInfo<'_>,
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
  pRenderPassBegin: *const VkRenderPassBeginInfo<'_>,
  pSubpassBeginInfo: *const VkSubpassBeginInfo<'_>,
);
/// [`vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdBeginRenderPass2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRenderPassBegin: *const VkRenderPassBeginInfo<'_>,
  pSubpassBeginInfo: *const VkSubpassBeginInfoKHR<'_>,
);
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
  pRenderingInfo: *const VkRenderingInfo<'_>,
);
/// [`vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderingKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering`
///
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub type PFN_vkCmdBeginRenderingKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRenderingInfo: *const VkRenderingInfoKHR<'_>,
);
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
/// - **Availability:** depends on `VK_EXT_transform_feedback`
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterRange`
/// - `counterRangeCount`: optional: true
/// - `pCounterInfos`: optional: true, len: counterRangeCount
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdBeginTransformFeedback2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstCounterRange: u32,
  counterRangeCount: u32,
  pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT<'_>,
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
  pBeginInfo: *const VkVideoBeginCodingInfoKHR<'_>,
);
/// [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_descriptor_buffer`
///
/// # Parameters
/// - `commandBuffer`
/// - `pBindDescriptorBufferEmbeddedSamplersInfo`
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBindDescriptorBufferEmbeddedSamplersInfo: *const VkBindDescriptorBufferEmbeddedSamplersInfoEXT<
    '_,
  >,
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
  pBindingInfos: *const VkDescriptorBufferBindingInfoEXT<'_>,
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
  pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfo<'_>,
);
/// [`vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdBindDescriptorSets2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBindDescriptorSetsInfo: *const VkBindDescriptorSetsInfoKHR<'_>,
);
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
pub type PFN_vkCmdBindIndexBuffer2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  size: VkDeviceSize,
  indexType: VkIndexType,
);
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
  pInfo: *const VkBindIndexBuffer3InfoKHR<'_>,
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
pub type PFN_vkCmdBindResourceHeapEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBindInfo: *const VkBindHeapInfoEXT<'_>,
);
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
pub type PFN_vkCmdBindSamplerHeapEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBindInfo: *const VkBindHeapInfoEXT<'_>,
);
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
  pTileMemoryBindInfo: *const VkTileMemoryBindInfoQCOM<'_>,
);
/// [`vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_transform_feedback`
///
/// # Parameters
/// - `commandBuffer`
/// - `firstBinding`
/// - `bindingCount`
/// - `pBindingInfos`: optional: true, len: bindingCount
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstBinding: u32,
  bindingCount: u32,
  pBindingInfos: *const VkBindTransformFeedbackBuffer2InfoEXT<'_>,
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
pub type PFN_vkCmdBindVertexBuffers2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstBinding: u32,
  bindingCount: u32,
  pBuffers: *const VkBuffer,
  pOffsets: *const VkDeviceSize,
  pSizes: *const VkDeviceSize,
  pStrides: *const VkDeviceSize,
);
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
  pBindingInfos: *const VkBindVertexBuffer3InfoKHR<'_>,
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
  pBlitImageInfo: *const VkBlitImageInfo2<'_>,
);
/// [`vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdBlitImage2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pBlitImageInfo: *const VkBlitImageInfo2KHR<'_>,
);
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
  pInfo: *const VkAccelerationStructureInfoNV<'_>,
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
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR<'_>,
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
  pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR<'_>,
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
  pCommandInfos: *const VkClusterAccelerationStructureCommandsInfoNV<'_>,
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
  pInfos: *const VkMicromapBuildInfoEXT<'_>,
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
  pBuildInfo: *const VkBuildPartitionedAccelerationStructureInfoNV<'_>,
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
  pCodingControlInfo: *const VkVideoCodingControlInfoKHR<'_>,
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
  pInfos: *const VkConvertCooperativeVectorMatrixInfoNV<'_>,
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
  pInfo: *const VkCopyAccelerationStructureInfoKHR<'_>,
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
  pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR<'_>,
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
  pCopyBufferInfo: *const VkCopyBufferInfo2<'_>,
);
/// [`vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyBuffer2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pCopyBufferInfo: *const VkCopyBufferInfo2KHR<'_>,
);
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
  pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2<'_>,
);
/// [`vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyBufferToImage2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2KHR<'_>,
);
/// [`vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
/// - **Queues:** Graphics, Compute, Transfer
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `gpaSession`
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCmdCopyGpaSessionResultsAMD =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, gpaSession: VkGpaSessionAMD);
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
  pCopyImageInfo: *const VkCopyImageInfo2<'_>,
);
/// [`vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyImage2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pCopyImageInfo: *const VkCopyImageInfo2KHR<'_>,
);
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
  pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2<'_>,
);
/// [`vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdCopyImageToBuffer2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2KHR<'_>,
);
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
  pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR<'_>,
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
  pCopyMemoryIndirectInfo: *const VkCopyMemoryIndirectInfoKHR<'_>,
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
  pCopyMemoryInfo: *const VkCopyDeviceMemoryInfoKHR<'_>,
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
  pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR<'_>,
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
  pCopyMemoryToImageIndirectInfo: *const VkCopyMemoryToImageIndirectInfoKHR<'_>,
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
  pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR<'_>,
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
  pInfo: *const VkCopyMemoryToMicromapInfoEXT<'_>,
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
pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkCopyMicromapInfoEXT<'_>,
);
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
  pInfo: *const VkCopyMicromapToMemoryInfoEXT<'_>,
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
  pCopyTensorInfo: *const VkCopyTensorInfoARM<'_>,
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
  pLaunchInfo: *const VkCuLaunchInfoNVX<'_>,
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
  pLaunchInfo: *const VkCudaLaunchInfoNV<'_>,
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
  pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT<'_>,
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
  pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT<'_>,
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
  pDecodeInfo: *const VkVideoDecodeInfoKHR<'_>,
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
  pDecompressMemoryInfoEXT: *const VkDecompressMemoryInfoEXT<'_>,
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
pub type PFN_vkCmdDispatchBaseKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  baseGroupX: u32,
  baseGroupY: u32,
  baseGroupZ: u32,
  groupCountX: u32,
  groupCountY: u32,
  groupCountZ: u32,
);
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
  pInfo: *const VkDataGraphPipelineDispatchInfoARM<'_>,
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
  pCountInfo: *const VkDispatchGraphCountInfoAMDX<'_>,
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
  pCountInfo: *const VkDispatchGraphCountInfoAMDX<'_>,
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
pub type PFN_vkCmdDispatchIndirect =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
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
  pInfo: *const VkDispatchIndirect2InfoKHR<'_>,
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
  pDispatchTileInfo: *const VkDispatchTileInfoQCOM<'_>,
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
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
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
pub type PFN_vkCmdDrawIndexedIndirect2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirect2InfoKHR<'_>,
);
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
/// - **Availability:** depends on `VK_KHR_draw_indirect_count + VK_VERSION_1_2`
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(any(
  all(
    feature = "VK_KHR_device_address_commands",
    feature = "VK_KHR_draw_indirect_count"
  ),
  all(feature = "VK_KHR_device_address_commands", feature = "VK_VERSION_1_2")
))]
pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirectCount2InfoKHR<'_>,
);
/// [`vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountAMD.html)
///
/// Provided by:
/// - `VK_AMD_draw_indirect_count`
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
);
/// [`vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCountKHR.html)
///
/// Provided by:
/// - `VK_KHR_draw_indirect_count`
///
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
);
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
pub type PFN_vkCmdDrawIndirect2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirect2InfoKHR<'_>,
);
/// [`vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Availability:** depends on `VK_EXT_transform_feedback`
///
/// # Parameters
/// - `commandBuffer`
/// - `instanceCount`
/// - `firstInstance`
/// - `pCounterInfo`
/// - `counterOffset`
/// - `vertexStride`
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdDrawIndirectByteCount2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  instanceCount: u32,
  firstInstance: u32,
  pCounterInfo: *const VkBindTransformFeedbackBuffer2InfoEXT<'_>,
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
/// - **Availability:** depends on `VK_KHR_draw_indirect_count + VK_VERSION_1_2`
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(any(
  all(
    feature = "VK_KHR_device_address_commands",
    feature = "VK_KHR_draw_indirect_count"
  ),
  all(feature = "VK_KHR_device_address_commands", feature = "VK_VERSION_1_2")
))]
pub type PFN_vkCmdDrawIndirectCount2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirectCount2InfoKHR<'_>,
);
/// [`vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountAMD.html)
///
/// Provided by:
/// - `VK_AMD_draw_indirect_count`
///
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub type PFN_vkCmdDrawIndirectCountAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
);
/// [`vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCountKHR.html)
///
/// Provided by:
/// - `VK_KHR_draw_indirect_count`
///
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub type PFN_vkCmdDrawIndirectCountKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  buffer: VkBuffer,
  offset: VkDeviceSize,
  countBuffer: VkBuffer,
  countBufferOffset: VkDeviceSize,
  maxDrawCount: u32,
  stride: u32,
);
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
/// - **Availability:** depends on `VK_EXT_mesh_shader`
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(all(
  feature = "VK_EXT_mesh_shader",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdDrawMeshTasksIndirect2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirect2InfoKHR<'_>,
);
/// [`vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Inside
/// - **Conditional Rendering:** Affected
/// - **Tasks:** Action
/// - **Availability:** depends on `VK_KHR_draw_indirect_count + VK_VERSION_1_2 + VK_EXT_mesh_shader`
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(any(
  all(
    feature = "VK_EXT_mesh_shader",
    feature = "VK_KHR_device_address_commands",
    feature = "VK_KHR_draw_indirect_count"
  ),
  all(
    feature = "VK_EXT_mesh_shader",
    feature = "VK_KHR_device_address_commands",
    feature = "VK_VERSION_1_2"
  )
))]
pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkDrawIndirectCount2InfoKHR<'_>,
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
/// - **Availability:** depends on `VK_VERSION_1_2 + VK_KHR_draw_indirect_count + VK_AMD_draw_indirect_count`
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(any(
  all(feature = "VK_EXT_mesh_shader", feature = "VK_VERSION_1_2"),
  all(feature = "VK_EXT_mesh_shader", feature = "VK_KHR_draw_indirect_count"),
  all(feature = "VK_AMD_draw_indirect_count", feature = "VK_EXT_mesh_shader")
))]
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
/// - **Availability:** depends on `VK_VERSION_1_2 + VK_KHR_draw_indirect_count + VK_AMD_draw_indirect_count`
///
/// # Parameters
/// - `commandBuffer`
/// - `buffer`
/// - `offset`
/// - `countBuffer`
/// - `countBufferOffset`
/// - `maxDrawCount`
/// - `stride`
#[cfg(any(
  all(feature = "VK_NV_mesh_shader", feature = "VK_VERSION_1_2"),
  all(feature = "VK_KHR_draw_indirect_count", feature = "VK_NV_mesh_shader"),
  all(feature = "VK_AMD_draw_indirect_count", feature = "VK_NV_mesh_shader")
))]
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
  pEncodeInfo: *const VkVideoEncodeInfoKHR<'_>,
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
/// [`vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `gpaSession`
/// - `sampleID`
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCmdEndGpaSampleAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  gpaSession: VkGpaSessionAMD,
  sampleID: u32,
);
/// [`vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
/// - **Queues:** Graphics, Compute
/// - **Render Pass:** Both
/// - **Tasks:** Action, State
///
/// # Parameters
/// - `commandBuffer`
/// - `gpaSession`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCmdEndGpaSessionAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  gpaSession: VkGpaSessionAMD,
) -> VkResult;
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
  pPerTileEndInfo: *const VkPerTileEndInfoQCOM<'_>,
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
  pSubpassEndInfo: *const VkSubpassEndInfo<'_>,
);
/// [`vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdEndRenderPass2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pSubpassEndInfo: *const VkSubpassEndInfoKHR<'_>,
);
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
pub type PFN_vkCmdEndRendering2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pRenderingEndInfo: *const VkRenderingEndInfoEXT<'_>,
);
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
  pRenderingEndInfo: *const VkRenderingEndInfoKHR<'_>,
);
/// [`vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderingKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering`
///
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub type PFN_vkCmdEndRenderingKHR = unsafe extern "system" fn(commandBuffer: VkCommandBuffer);
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
/// - **Availability:** depends on `VK_EXT_transform_feedback`
///
/// # Parameters
/// - `commandBuffer`
/// - `firstCounterRange`
/// - `counterRangeCount`: optional: true
/// - `pCounterInfos`: optional: true, len: counterRangeCount
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdEndTransformFeedback2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  firstCounterRange: u32,
  counterRangeCount: u32,
  pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT<'_>,
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
  pEndCodingInfo: *const VkVideoEndCodingInfoKHR<'_>,
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
  pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT<'_>,
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
  pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV<'_>,
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
  pLabelInfo: *const VkDebugUtilsLabelEXT<'_>,
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
  pSubpassBeginInfo: *const VkSubpassBeginInfo<'_>,
  pSubpassEndInfo: *const VkSubpassEndInfo<'_>,
);
/// [`vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCmdNextSubpass2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pSubpassBeginInfo: *const VkSubpassBeginInfoKHR<'_>,
  pSubpassEndInfo: *const VkSubpassEndInfoKHR<'_>,
);
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
  pExecuteInfo: *const VkOpticalFlowExecuteInfoNV<'_>,
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
  pMemoryBarriers: *const VkMemoryBarrier<'_>,
  bufferMemoryBarrierCount: u32,
  pBufferMemoryBarriers: *const VkBufferMemoryBarrier<'_>,
  imageMemoryBarrierCount: u32,
  pImageMemoryBarriers: *const VkImageMemoryBarrier<'_>,
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
  pDependencyInfo: *const VkDependencyInfo<'_>,
);
/// [`vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdPipelineBarrier2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pDependencyInfo: *const VkDependencyInfoKHR<'_>,
);
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
  pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoEXT<'_>,
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
  pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV<'_>,
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
  pPushConstantsInfo: *const VkPushConstantsInfo<'_>,
);
/// [`vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
#[cfg(feature = "VK_KHR_maintenance6")]
pub type PFN_vkCmdPushConstants2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pPushConstantsInfo: *const VkPushConstantsInfoKHR<'_>,
);
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
  pPushDataInfo: *const VkPushDataInfoEXT<'_>,
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
  pDescriptorWrites: *const VkWriteDescriptorSet<'_>,
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
  pPushDescriptorSetInfo: *const VkPushDescriptorSetInfo<'_>,
);
/// [`vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Availability:** depends on `VK_KHR_push_descriptor`
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]
pub type PFN_vkCmdPushDescriptorSet2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pPushDescriptorSetInfo: *const VkPushDescriptorSetInfoKHR<'_>,
);
/// [`vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetKHR.html)
///
/// Provided by:
/// - `VK_KHR_push_descriptor`
///
#[cfg(feature = "VK_KHR_push_descriptor")]
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pipelineBindPoint: VkPipelineBindPoint,
  layout: VkPipelineLayout,
  set: u32,
  descriptorWriteCount: u32,
  pDescriptorWrites: *const VkWriteDescriptorSet<'_>,
);
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
  pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfo<'_>,
);
/// [`vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Availability:** depends on `VK_KHR_push_descriptor`
#[cfg(all(feature = "VK_KHR_maintenance6", feature = "VK_KHR_push_descriptor"))]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pPushDescriptorSetWithTemplateInfo: *const VkPushDescriptorSetWithTemplateInfoKHR<'_>,
);
/// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
/// - `VK_KHR_push_descriptor`
///
/// - **Availability:** depends on `VK_VERSION_1_1 + VK_KHR_descriptor_update_template`
#[cfg(any(
  all(feature = "VK_KHR_push_descriptor", feature = "VK_VERSION_1_1"),
  all(
    feature = "VK_KHR_descriptor_update_template",
    feature = "VK_KHR_push_descriptor"
  )
))]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
  layout: VkPipelineLayout,
  set: u32,
  pData: *const core::ffi::c_void,
);
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
  pRefreshObjects: *const VkRefreshObjectListKHR<'_>,
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
pub type PFN_vkCmdResetEvent2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  event: VkEvent,
  stageMask: VkPipelineStageFlags2KHR,
);
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
  pResolveImageInfo: *const VkResolveImageInfo2<'_>,
);
/// [`vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2KHR.html)
///
/// Provided by:
/// - `VK_KHR_copy_commands2`
///
#[cfg(feature = "VK_KHR_copy_commands2")]
pub type PFN_vkCmdResolveImage2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pResolveImageInfo: *const VkResolveImageInfo2KHR<'_>,
);
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
  pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV<'_>,
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
/// - **Availability:** depends on `VK_EXT_blend_operation_advanced`
///
/// # Parameters
/// - `commandBuffer`
/// - `firstAttachment`
/// - `attachmentCount`
/// - `pColorBlendAdvanced`: len: attachmentCount
#[cfg(any(
  all(
    feature = "VK_EXT_blend_operation_advanced",
    feature = "VK_EXT_extended_dynamic_state3"
  ),
  all(
    feature = "VK_EXT_blend_operation_advanced",
    feature = "VK_EXT_shader_object"
  )
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
  pParameters: *const VkComputeOccupancyPriorityParametersNV<'_>,
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
/// - **Availability:** depends on `VK_EXT_conservative_rasterization`
///
/// # Parameters
/// - `commandBuffer`
/// - `conservativeRasterizationMode`
#[cfg(any(
  all(
    feature = "VK_EXT_conservative_rasterization",
    feature = "VK_EXT_extended_dynamic_state3"
  ),
  all(
    feature = "VK_EXT_conservative_rasterization",
    feature = "VK_EXT_shader_object"
  )
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
/// - **Availability:** depends on `VK_NV_framebuffer_mixed_samples`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationMode`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_framebuffer_mixed_samples"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
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
/// - **Availability:** depends on `VK_NV_framebuffer_mixed_samples`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationTableEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_framebuffer_mixed_samples"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
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
/// - **Availability:** depends on `VK_NV_framebuffer_mixed_samples`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageModulationTableCount`
/// - `pCoverageModulationTable`: len: coverageModulationTableCount
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_framebuffer_mixed_samples"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_framebuffer_mixed_samples"
  )
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
/// - **Availability:** depends on `VK_NV_coverage_reduction_mode`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageReductionMode`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_coverage_reduction_mode"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_coverage_reduction_mode"
  )
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
/// - **Availability:** depends on `VK_NV_fragment_coverage_to_color`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageToColorEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_fragment_coverage_to_color"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_fragment_coverage_to_color"
  )
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
/// - **Availability:** depends on `VK_NV_fragment_coverage_to_color`
///
/// # Parameters
/// - `commandBuffer`
/// - `coverageToColorLocation`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_fragment_coverage_to_color"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_fragment_coverage_to_color"
  )
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
pub type PFN_vkCmdSetCullModeEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);
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
  pDepthBiasInfo: *const VkDepthBiasInfoEXT<'_>,
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
pub type PFN_vkCmdSetDepthBiasEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);
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
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);
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
#[cfg(feature = "VK_EXT_depth_clamp_control")]
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
/// - **Availability:** depends on `VK_EXT_depth_clip_enable`
///
/// # Parameters
/// - `commandBuffer`
/// - `depthClipEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_depth_clip_enable",
    feature = "VK_EXT_extended_dynamic_state3"
  ),
  all(feature = "VK_EXT_depth_clip_enable", feature = "VK_EXT_shader_object")
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
/// - **Availability:** depends on `VK_EXT_depth_clip_control`
///
/// # Parameters
/// - `commandBuffer`
/// - `negativeOneToOne`
#[cfg(any(
  all(
    feature = "VK_EXT_depth_clip_control",
    feature = "VK_EXT_extended_dynamic_state3"
  ),
  all(
    feature = "VK_EXT_depth_clip_control",
    feature = "VK_EXT_shader_object"
  )
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
pub type PFN_vkCmdSetDepthCompareOpEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);
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
pub type PFN_vkCmdSetDepthTestEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);
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
pub type PFN_vkCmdSetDepthWriteEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);
/// [`vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html)
///
/// Provided by:
/// - `VK_KHR_maintenance6`
///
/// - **Queues:** Graphics, Compute, DataGraphArm
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_descriptor_buffer`
///
/// # Parameters
/// - `commandBuffer`
/// - `pSetDescriptorBufferOffsetsInfo`
#[cfg(all(feature = "VK_EXT_descriptor_buffer", feature = "VK_KHR_maintenance6"))]
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pSetDescriptorBufferOffsetsInfo: *const VkSetDescriptorBufferOffsetsInfoEXT<'_>,
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
pub type PFN_vkCmdSetDeviceMaskKHR =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);
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
/// [`vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html)
///
/// Provided by:
/// - `VK_ARM_scheduling_controls`
///
/// - **Queues:** Compute
/// - **Render Pass:** Outside
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `pDispatchParameters`
#[cfg(feature = "VK_ARM_scheduling_controls")]
pub type PFN_vkCmdSetDispatchParametersARM = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pDispatchParameters: *const VkDispatchParametersARM<'_>,
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
  pDependencyInfo: *const VkDependencyInfo<'_>,
);
/// [`vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdSetEvent2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  event: VkEvent,
  pDependencyInfo: *const VkDependencyInfoKHR<'_>,
);
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
/// - **Availability:** depends on `VK_EXT_conservative_rasterization`
///
/// # Parameters
/// - `commandBuffer`
/// - `extraPrimitiveOverestimationSize`
#[cfg(any(
  all(
    feature = "VK_EXT_conservative_rasterization",
    feature = "VK_EXT_extended_dynamic_state3"
  ),
  all(
    feature = "VK_EXT_conservative_rasterization",
    feature = "VK_EXT_shader_object"
  )
))]
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, extraPrimitiveOverestimationSize: f32);
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
pub type PFN_vkCmdSetFrontFaceEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);
/// [`vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_line_rasterization`
///
/// # Parameters
/// - `commandBuffer`
/// - `lineRasterizationMode`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_line_rasterization"
  ),
  all(
    feature = "VK_EXT_line_rasterization",
    feature = "VK_EXT_shader_object"
  )
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
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  lineStippleFactor: u32,
  lineStipplePattern: u16,
);
/// [`vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_line_rasterization`
///
/// # Parameters
/// - `commandBuffer`
/// - `stippledLineEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_line_rasterization"
  ),
  all(
    feature = "VK_EXT_line_rasterization",
    feature = "VK_EXT_shader_object"
  )
))]
pub type PFN_vkCmdSetLineStippleEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stippledLineEnable: VkBool32);
/// [`vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleKHR.html)
///
/// Provided by:
/// - `VK_KHR_line_rasterization`
///
#[cfg(feature = "VK_KHR_line_rasterization")]
pub type PFN_vkCmdSetLineStippleKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  lineStippleFactor: u32,
  lineStipplePattern: u16,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pMarkerInfo: *const VkPerformanceMarkerInfoINTEL<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pOverrideInfo: *const VkPerformanceOverrideInfoINTEL<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL<'_>,
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
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
/// [`vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html)
///
/// Provided by:
/// - `VK_EXT_primitive_restart_index`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
///
/// # Parameters
/// - `commandBuffer`
/// - `primitiveRestartIndex`: optional: true
#[cfg(feature = "VK_EXT_primitive_restart_index")]
pub type PFN_vkCmdSetPrimitiveRestartIndexEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveRestartIndex: u32);
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
pub type PFN_vkCmdSetPrimitiveTopology =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
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
pub type PFN_vkCmdSetPrimitiveTopologyEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);
/// [`vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_EXT_provoking_vertex`
///
/// # Parameters
/// - `commandBuffer`
/// - `provokingVertexMode`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_provoking_vertex"
  ),
  all(feature = "VK_EXT_provoking_vertex", feature = "VK_EXT_shader_object")
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
/// - **Availability:** depends on `VK_EXT_transform_feedback`
///
/// # Parameters
/// - `commandBuffer`
/// - `rasterizationStream`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_transform_feedback"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_EXT_transform_feedback"
  )
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
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);
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
  pLocationInfo: *const VkRenderingAttachmentLocationInfo<'_>,
);
/// [`vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocationsKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering_local_read`
///
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pLocationInfo: *const VkRenderingAttachmentLocationInfoKHR<'_>,
);
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
  pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfo<'_>,
);
/// [`vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndicesKHR.html)
///
/// Provided by:
/// - `VK_KHR_dynamic_rendering_local_read`
///
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInputAttachmentIndexInfo: *const VkRenderingInputAttachmentIndexInfoKHR<'_>,
);
/// [`vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_NV_representative_fragment_test`
///
/// # Parameters
/// - `commandBuffer`
/// - `representativeFragmentTestEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_representative_fragment_test"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_representative_fragment_test"
  )
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
  pSampleLocationsInfo: *const VkSampleLocationsInfoEXT<'_>,
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
/// - **Availability:** depends on `VK_EXT_sample_locations`
///
/// # Parameters
/// - `commandBuffer`
/// - `sampleLocationsEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_sample_locations"
  ),
  all(feature = "VK_EXT_sample_locations", feature = "VK_EXT_shader_object")
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
pub type PFN_vkCmdSetScissorWithCountEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  scissorCount: u32,
  pScissors: *const VkRect2D,
);
/// [`vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html)
///
/// Provided by:
/// - `VK_EXT_extended_dynamic_state3`
/// - `VK_EXT_shader_object`
///
/// - **Queues:** Graphics
/// - **Render Pass:** Both
/// - **Tasks:** State
/// - **Availability:** depends on `VK_NV_shading_rate_image`
///
/// # Parameters
/// - `commandBuffer`
/// - `shadingRateImageEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_shading_rate_image"
  ),
  all(feature = "VK_EXT_shader_object", feature = "VK_NV_shading_rate_image")
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
pub type PFN_vkCmdSetStencilOpEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  faceMask: VkStencilFaceFlags,
  failOp: VkStencilOp,
  passOp: VkStencilOp,
  depthFailOp: VkStencilOp,
  compareOp: VkCompareOp,
);
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
pub type PFN_vkCmdSetStencilTestEnableEXT =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);
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
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_KHR_maintenance2"
  ),
  all(feature = "VK_EXT_extended_dynamic_state3", feature = "VK_VERSION_1_1"),
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
  feature = "VK_EXT_vertex_input_dynamic_state",
  feature = "VK_EXT_shader_object"
))]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  vertexBindingDescriptionCount: u32,
  pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT<'_>,
  vertexAttributeDescriptionCount: u32,
  pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT<'_>,
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
  pShadingRatePalettes: *const VkShadingRatePaletteNV<'_>,
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
/// - **Availability:** depends on `VK_NV_viewport_swizzle`
///
/// # Parameters
/// - `commandBuffer`
/// - `firstViewport`
/// - `viewportCount`
/// - `pViewportSwizzles`: len: viewportCount
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_viewport_swizzle"
  ),
  all(feature = "VK_EXT_shader_object", feature = "VK_NV_viewport_swizzle")
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
/// - **Availability:** depends on `VK_NV_clip_space_w_scaling`
///
/// # Parameters
/// - `commandBuffer`
/// - `viewportWScalingEnable`
#[cfg(any(
  all(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_NV_clip_space_w_scaling"
  ),
  all(
    feature = "VK_EXT_shader_object",
    feature = "VK_NV_clip_space_w_scaling"
  )
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
pub type PFN_vkCmdSetViewportWithCountEXT = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  viewportCount: u32,
  pViewports: *const VkViewport,
);
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
/// - **Availability:** depends on `VK_KHR_ray_tracing_pipeline`
///
/// # Parameters
/// - `commandBuffer`
/// - `indirectDeviceAddress`
#[cfg(all(
  feature = "VK_KHR_ray_tracing_maintenance1",
  feature = "VK_KHR_ray_tracing_pipeline"
))]
pub type PFN_vkCmdTraceRaysIndirect2KHR =
  unsafe extern "system" fn(commandBuffer: VkCommandBuffer, indirectDeviceAddress: VkDeviceAddress);
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
  pMemoryBarriers: *const VkMemoryBarrier<'_>,
  bufferMemoryBarrierCount: u32,
  pBufferMemoryBarriers: *const VkBufferMemoryBarrier<'_>,
  imageMemoryBarrierCount: u32,
  pImageMemoryBarriers: *const VkImageMemoryBarrier<'_>,
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
  pDependencyInfos: *const VkDependencyInfo<'_>,
);
/// [`vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkCmdWaitEvents2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  eventCount: u32,
  pEvents: *const VkEvent,
  pDependencyInfos: *const VkDependencyInfoKHR<'_>,
);
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
#[cfg(any(
  all(feature = "VK_AMD_buffer_marker", feature = "VK_VERSION_1_3"),
  all(feature = "VK_AMD_buffer_marker", feature = "VK_KHR_synchronization2")
))]
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
/// - **Availability:** depends on `VK_AMD_buffer_marker`
///
/// # Parameters
/// - `commandBuffer`
/// - `pInfo`
#[cfg(all(
  feature = "VK_AMD_buffer_marker",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCmdWriteMarkerToMemoryAMD = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  pInfo: *const VkMemoryMarkerInfoAMD<'_>,
);
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
pub type PFN_vkCmdWriteTimestamp2KHR = unsafe extern "system" fn(
  commandBuffer: VkCommandBuffer,
  stage: VkPipelineStageFlags2KHR,
  queryPool: VkQueryPool,
  query: u32,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cooperative_vector")]
pub type PFN_vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkConvertCooperativeVectorMatrixInfoNV<'_>,
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
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyAccelerationStructureInfoKHR<'_>,
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
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR<'_>,
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
pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
  device: VkDevice,
  pCopyImageToImageInfo: *const VkCopyImageToImageInfo<'_>,
) -> VkResult;
/// [`vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImageEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyImageToImageEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCopyImageToImageInfo: *const VkCopyImageToImageInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
  device: VkDevice,
  pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfo<'_>,
) -> VkResult;
/// [`vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemoryEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyImageToMemoryEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCopyImageToMemoryInfo: *const VkCopyImageToMemoryInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR<'_>,
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
pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
  device: VkDevice,
  pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfo<'_>,
) -> VkResult;
/// [`vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImageEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkCopyMemoryToImageEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCopyMemoryToImageInfo: *const VkCopyMemoryToImageInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyMemoryToMicromapInfoEXT<'_>,
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
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyMicromapInfoEXT<'_>,
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
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pInfo: *const VkCopyMicromapToMemoryInfoEXT<'_>,
) -> VkResult;
/// [`vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html)
///
/// Provided by:
/// - `VK_KHR_device_address_commands`
///
/// - **Availability:** depends on `VK_KHR_acceleration_structure`
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
#[cfg(all(
  feature = "VK_KHR_acceleration_structure",
  feature = "VK_KHR_device_address_commands"
))]
pub type PFN_vkCreateAccelerationStructure2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkAccelerationStructureCreateInfo2KHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_acceleration_structure")]
#[deprecated(note = "superseded by `vkCreateAccelerationStructure2KHR`")]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkAccelerationStructureCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkAccelerationStructureCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_android_surface")]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkBufferCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkBufferCollectionCreateInfoFUCHSIA<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkBufferViewCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkCommandPoolCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_SHADER_NV`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkComputePipelineCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkCuFunctionCreateInfoNVX<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NVX_binary_import")]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkCuModuleCreateInfoNVX<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkCudaFunctionCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkCudaModuleCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDataGraphPipelineSessionCreateInfoARM<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkDataGraphPipelineCreateInfoARM<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_report")]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
  device: VkDevice,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_FRAGMENTATION_EXT`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorPoolCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorSetLayoutCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;
/// [`vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplateKHR,
) -> VkResult;
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
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pCreateInfo: *const VkDeviceCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_directfb_surface")]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pCreateInfo: *const VkDisplayModeCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkEventCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkExecutionGraphPipelineCreateInfoAMDX<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_compute_queue")]
pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkExternalComputeQueueCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateFence = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkFenceCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkFramebufferCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pFramebuffer: *mut VkFramebuffer,
) -> VkResult;
/// [`vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
///
/// # Parameters
/// - `device`
/// - `pCreateInfo`
/// - `pAllocator`: optional: true
/// - `pGpaSession`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkCreateGpaSessionAMD = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkGpaSessionCreateInfoAMD<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pGpaSession: *mut VkGpaSessionAMD,
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
///   - `VK_SUCCESS`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_SHADER_NV`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkGraphicsPipelineCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_headless_surface")]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_MVK_ios_surface")]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkIOSSurfaceCreateInfoMVK<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateImage = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkImageCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkImageViewCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_device_generated_commands")]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkIndirectExecutionSetCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_LAYER_NOT_PRESENT`
///   - `VK_ERROR_EXTENSION_NOT_PRESENT`
///   - `VK_ERROR_INCOMPATIBLE_DRIVER`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
  pCreateInfo: *const VkInstanceCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_MVK_macos_surface")]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_metal_surface")]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkMetalSurfaceCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkMicromapCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_optical_flow")]
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkOpticalFlowSessionCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPipelineBinaryCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pBinaries: *mut VkPipelineBinaryHandlesInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPipelineCacheCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPipelineLayoutCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPrivateDataSlotCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pPrivateDataSlot: *mut VkPrivateDataSlot,
) -> VkResult;
/// [`vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlotEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkCreatePrivateDataSlotEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkPrivateDataSlotCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pPrivateDataSlot: *mut VkPrivateDataSlotEXT,
) -> VkResult;
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
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkQueryPoolCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_OPERATION_DEFERRED_KHR`
///   - `VK_OPERATION_NOT_DEFERRED_KHR`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
  device: VkDevice,
  deferredOperation: VkDeferredOperationKHR,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_PIPELINE_COMPILE_REQUIRED_EXT`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_SHADER_NV`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_ray_tracing")]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
  device: VkDevice,
  pipelineCache: VkPipelineCache,
  createInfoCount: u32,
  pCreateInfos: *const VkRayTracingPipelineCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
#[deprecated(note = "superseded by `vkCreateRenderPass2`")]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkRenderPassCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkRenderPassCreateInfo2<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pRenderPass: *mut VkRenderPass,
) -> VkResult;
/// [`vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2KHR.html)
///
/// Provided by:
/// - `VK_KHR_create_renderpass2`
///
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub type PFN_vkCreateRenderPass2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkRenderPassCreateInfo2KHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pRenderPass: *mut VkRenderPass,
) -> VkResult;
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
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSamplerCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;
/// [`vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversionKHR.html)
///
/// Provided by:
/// - `VK_KHR_sampler_ycbcr_conversion`
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkCreateSamplerYcbcrConversionKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSamplerYcbcrConversionCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
  pYcbcrConversion: *mut VkSamplerYcbcrConversionKHR,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_QNX_screen_surface")]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkScreenSurfaceCreateInfoQNX<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSemaphoreCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub type PFN_vkCreateSemaphoreSciSyncPoolNV = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSemaphoreSciSyncPoolCreateInfoNV<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub type PFN_vkCreateShaderInstrumentationARM = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkShaderInstrumentationCreateInfoARM<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_SHADER_NV`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkShaderModuleCreateInfo<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
  device: VkDevice,
  createInfoCount: u32,
  pCreateInfos: *const VkShaderCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
  device: VkDevice,
  swapchainCount: u32,
  pCreateInfos: *const VkSwapchainCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_OHOS_surface")]
pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkSurfaceCreateInfoOHOS<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkSwapchainCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkTensorCreateInfoARM<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_tensors")]
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkTensorViewCreateInfoARM<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_SEC_ubm_surface")]
pub type PFN_vkCreateUbmSurfaceSEC = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkUbmSurfaceCreateInfoSEC<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_validation_cache")]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkValidationCacheCreateInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NN_vi_surface")]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkViSurfaceCreateInfoNN<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkVideoSessionCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkVideoSessionParametersCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_wayland_surface")]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_win32_surface")]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkWin32SurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_xcb_surface")]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkXcbSurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_xlib_surface")]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pCreateInfo: *const VkXlibSurfaceCreateInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
  device: VkDevice,
  pNameInfo: *const VkDebugMarkerObjectNameInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_marker")]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
  device: VkDevice,
  pTagInfo: *const VkDebugMarkerObjectTagInfoEXT<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
);
/// [`vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplateKHR.html)
///
/// Provided by:
/// - `VK_KHR_descriptor_update_template`
///
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "system" fn(
  device: VkDevice,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
  pAllocator: *const VkAllocationCallbacks<'_>,
);
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
  unsafe extern "system" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks<'_>);
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
);
/// [`vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
///
/// # Parameters
/// - `device`
/// - `gpaSession`: optional: true
/// - `pAllocator`: optional: true
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkDestroyGpaSessionAMD = unsafe extern "system" fn(
  device: VkDevice,
  gpaSession: VkGpaSessionAMD,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  unsafe extern "system" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks<'_>);
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
);
/// [`vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlotEXT.html)
///
/// Provided by:
/// - `VK_EXT_private_data`
///
#[cfg(feature = "VK_EXT_private_data")]
pub type PFN_vkDestroyPrivateDataSlotEXT = unsafe extern "system" fn(
  device: VkDevice,
  privateDataSlot: VkPrivateDataSlotEXT,
  pAllocator: *const VkAllocationCallbacks<'_>,
);
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
);
/// [`vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversionKHR.html)
///
/// Provided by:
/// - `VK_KHR_sampler_ycbcr_conversion`
///
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = unsafe extern "system" fn(
  device: VkDevice,
  ycbcrConversion: VkSamplerYcbcrConversionKHR,
  pAllocator: *const VkAllocationCallbacks<'_>,
);
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
  device: VkDevice,
  display: VkDisplayKHR,
  pDisplayPowerInfo: *const VkDisplayPowerInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
  instance: VkInstance,
  pPhysicalDeviceGroupCount: *mut u32,
  pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties<'_>,
) -> VkResult;
/// [`vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroupsKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group_creation`
///
#[cfg(feature = "VK_KHR_device_group_creation")]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = unsafe extern "system" fn(
  instance: VkInstance,
  pPhysicalDeviceGroupCount: *mut u32,
  pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupPropertiesKHR<'_>,
) -> VkResult;
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
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut VkPerformanceCounterARM<'_>,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionARM<'_>,
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
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pCounterCount: *mut u32,
    pCounters: *mut VkPerformanceCounterKHR<'_>,
    pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR<'_>,
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
pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pDescriptionCount: *mut u32,
    pDescriptions: *mut VkShaderInstrumentationMetricDescriptionARM<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pMetalObjectsInfo: *mut VkExportMetalObjectsInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
  device: VkDevice,
  memoryRangeCount: u32,
  pMemoryRanges: *const VkMappedMemoryRange<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pAllocator: *const VkAllocationCallbacks<'_>,
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
  pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR<'_>,
  pMaxPrimitiveCounts: *const u32,
  pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR<'_>,
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
    pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_buffer`
///
/// - **Availability:** depends on `VK_KHR_acceleration_structure + VK_NV_ray_tracing`
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
#[cfg(any(
  all(
    feature = "VK_EXT_descriptor_buffer",
    feature = "VK_KHR_acceleration_structure"
  ),
  all(feature = "VK_EXT_descriptor_buffer", feature = "VK_NV_ray_tracing")
))]
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
  unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkAccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
  device: VkDevice,
  buffer: *const AHardwareBuffer,
  pProperties: *mut VkAndroidHardwareBufferPropertiesANDROID<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  collection: VkBufferCollectionFUCHSIA,
  pProperties: *mut VkBufferCollectionPropertiesFUCHSIA<'_>,
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
  pInfo: *const VkBufferDeviceAddressInfo<'_>,
) -> VkDeviceAddress;
/// [`vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressEXT.html)
///
/// Provided by:
/// - `VK_EXT_buffer_device_address`
///
#[cfg(feature = "VK_EXT_buffer_device_address")]
pub type PFN_vkGetBufferDeviceAddressEXT = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkBufferDeviceAddressInfoEXT<'_>,
) -> VkDeviceAddress;
/// [`vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetBufferDeviceAddressKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkBufferDeviceAddressInfoKHR<'_>,
) -> VkDeviceAddress;
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
  pInfo: *const VkBufferMemoryRequirementsInfo2<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkBufferMemoryRequirementsInfo2KHR<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2KHR<'_>,
);
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
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo<'_>) -> u64;
/// [`vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkBufferDeviceAddressInfoKHR<'_>,
) -> u64;
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
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkBufferCaptureDescriptorDataInfoEXT<'_>,
  pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsEXT.html)
///
/// Provided by:
/// - `VK_EXT_calibrated_timestamps`
///
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub type PFN_vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(
  device: VkDevice,
  timestampCount: u32,
  pTimestampInfos: *const VkCalibratedTimestampInfoEXT<'_>,
  pTimestamps: *mut u64,
  pMaxDeviation: *mut u64,
) -> VkResult;
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
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
  device: VkDevice,
  timestampCount: u32,
  pTimestampInfos: *const VkCalibratedTimestampInfoKHR<'_>,
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
  pInfo: *const VkClusterAccelerationStructureInputInfoNV<'_>,
  pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR<'_>,
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
  pConsumption: *mut VkCommandPoolMemoryConsumption<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineInfo: *const VkDataGraphPipelineInfoARM<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineInfo: *const VkDataGraphPipelineInfoARM<'_>,
  propertiesCount: u32,
  pProperties: *mut VkDataGraphPipelinePropertyQueryResultARM<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
  unsafe extern "system" fn(
    device: VkDevice,
    pInfo: *const VkDataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
    pBindPointRequirementCount: *mut u32,
    pBindPointRequirements: *mut VkDataGraphPipelineSessionBindPointRequirementARM<'_>,
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
  pInfo: *const VkDataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_NOT_READY`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pDescriptorInfo: *const VkDescriptorGetInfoEXT<'_>,
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
  pBindingReference: *const VkDescriptorSetBindingReferenceVALVE<'_>,
  pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE<'_>,
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
  pCreateInfo: *const VkDescriptorSetLayoutCreateInfo<'_>,
  pSupport: *mut VkDescriptorSetLayoutSupport<'_>,
);
/// [`vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupportKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance3`
///
#[cfg(feature = "VK_KHR_maintenance3")]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = unsafe extern "system" fn(
  device: VkDevice,
  pCreateInfo: *const VkDescriptorSetLayoutCreateInfo<'_>,
  pSupport: *mut VkDescriptorSetLayoutSupportKHR<'_>,
);
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
  pVersionInfo: *const VkAccelerationStructureVersionInfoKHR<'_>,
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
  pInfo: *const VkDeviceBufferMemoryRequirements<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkDeviceBufferMemoryRequirementsKHR<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
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
pub type PFN_vkGetDeviceFaultDebugInfoKHR = unsafe extern "system" fn(
  device: VkDevice,
  pDebugInfo: *mut VkDeviceFaultDebugInfoKHR<'_>,
) -> VkResult;
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
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
  device: VkDevice,
  pFaultCounts: *mut VkDeviceFaultCountsEXT<'_>,
  pFaultInfo: *mut VkDeviceFaultInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkGetDeviceFaultReportsKHR = unsafe extern "system" fn(
  device: VkDevice,
  timeout: u64,
  pFaultCounts: *mut u32,
  pFaultInfo: *mut VkDeviceFaultInfoKHR<'_>,
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
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = unsafe extern "system" fn(
  device: VkDevice,
  heapIndex: u32,
  localDeviceIndex: u32,
  remoteDeviceIndex: u32,
  pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlagsKHR,
);
/// [`vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
/// - `VK_KHR_swapchain`
///
/// - **Availability:** depends on `VK_VERSION_1_1`
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
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
  device: VkDevice,
  pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR<'_>,
) -> VkResult;
/// [`vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html)
///
/// Provided by:
/// - `VK_EXT_full_screen_exclusive`
///
/// - **Availability:** depends on `VK_KHR_device_group + VK_VERSION_1_1`
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
#[cfg(any(
  all(
    feature = "VK_EXT_full_screen_exclusive",
    feature = "VK_KHR_device_group"
  ),
  all(feature = "VK_EXT_full_screen_exclusive", feature = "VK_VERSION_1_1")
))]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
  device: VkDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR<'_>,
  pModes: *mut VkDeviceGroupPresentModeFlagsKHR,
) -> VkResult;
/// [`vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html)
///
/// Provided by:
/// - `VK_KHR_device_group`
/// - `VK_KHR_swapchain`
///
/// - **Availability:** depends on `VK_VERSION_1_1`
///
/// # Parameters
/// - `device`
/// - `surface`
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
#[cfg(any(
  all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1"),
  all(feature = "VK_KHR_device_group", feature = "VK_KHR_surface")
))]
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
  pInfo: *const VkDeviceImageMemoryRequirements<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkDeviceImageMemoryRequirementsKHR<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
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
  pInfo: *const VkDeviceImageMemoryRequirements<'_>,
  pSparseMemoryRequirementCount: *mut u32,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2<'_>,
);
/// [`vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirementsKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance4`
///
#[cfg(feature = "VK_KHR_maintenance4")]
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkDeviceImageMemoryRequirementsKHR<'_>,
  pSparseMemoryRequirementCount: *mut u32,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2<'_>,
);
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
  pInfo: *const VkDeviceImageSubresourceInfo<'_>,
  pLayout: *mut VkSubresourceLayout2<'_>,
);
/// [`vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayoutKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkDeviceImageSubresourceInfoKHR<'_>,
  pLayout: *mut VkSubresourceLayout2KHR<'_>,
);
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
  pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo<'_>,
) -> u64;
/// [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html)
///
/// Provided by:
/// - `VK_KHR_buffer_device_address`
///
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfoKHR<'_>,
) -> u64;
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
  pVersionInfo: *const VkMicromapVersionInfoEXT<'_>,
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
  pQueueInfo: *const VkDeviceQueueInfo2<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pInfo: *const VkDeviceTensorMemoryRequirementsARM<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  display: VkDisplayKHR,
  pPropertyCount: *mut u32,
  pProperties: *mut VkDisplayModeProperties2KHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR<'_>,
  pCapabilities: *mut VkDisplayPlaneCapabilities2KHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_QCOM_tile_properties")]
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
  device: VkDevice,
  pRenderingInfo: *const VkRenderingInfo<'_>,
  pProperties: *mut VkTilePropertiesQCOM<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
  device: VkDevice,
  pVideoSessionParametersInfo: *const VkVideoEncodeSessionParametersGetInfoKHR<'_>,
  pFeedbackInfo: *mut VkVideoEncodeSessionParametersFeedbackInfoKHR<'_>,
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
///   - `VK_EVENT_SET`
///   - `VK_EVENT_RESET`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
  device: VkDevice,
  executionGraph: VkPipeline,
  pNodeInfo: *const VkPipelineShaderStageNodeCreateInfoAMDX<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
  device: VkDevice,
  executionGraph: VkPipeline,
  pSizeInfo: *mut VkExecutionGraphPipelineScratchSizeAMDX<'_>,
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
  params: *mut VkExternalComputeQueueDataParamsNV<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VKSC_VERSION_1_0")]
pub type PFN_vkGetFaultData = unsafe extern "system" fn(
  device: VkDevice,
  faultQueryBehavior: VkFaultQueryBehavior,
  pUnrecordedFaults: *mut VkBool32,
  pFaultCount: *mut u32,
  pFaults: *mut VkFaultData<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetFdInfo: *const VkFenceGetFdInfoKHR<'_>,
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
pub type PFN_vkGetFenceSciSyncFenceNV = unsafe extern "system" fn(
  device: VkDevice,
  pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV<'_>,
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
pub type PFN_vkGetFenceSciSyncObjNV = unsafe extern "system" fn(
  device: VkDevice,
  pGetSciSyncHandleInfo: *const VkFenceGetSciSyncInfoNV<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_NOT_READY`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetWin32HandleInfo: *const VkFenceGetWin32HandleInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_QCOM_tile_properties")]
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
  device: VkDevice,
  framebuffer: VkFramebuffer,
  pPropertiesCount: *mut u32,
  pProperties: *mut VkTilePropertiesQCOM<'_>,
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
  pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoEXT<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
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
  pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
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
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkGetGpaDeviceClockInfoAMD = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *mut VkGpaDeviceGetClockInfoAMD<'_>,
) -> VkResult;
/// [`vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
///
/// # Parameters
/// - `device`
/// - `gpaSession`
/// - `sampleID`
/// - `pSizeInBytes`
/// - `pData`: optional: true, len: pSizeInBytes
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkGetGpaSessionResultsAMD = unsafe extern "system" fn(
  device: VkDevice,
  gpaSession: VkGpaSessionAMD,
  sampleID: u32,
  pSizeInBytes: *mut usize,
  pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
///
/// # Parameters
/// - `device`
/// - `gpaSession`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkGetGpaSessionStatusAMD =
  unsafe extern "system" fn(device: VkDevice, gpaSession: VkGpaSessionAMD) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  pProperties: *mut VkImageDrmFormatModifierPropertiesEXT<'_>,
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
  pInfo: *const VkImageMemoryRequirementsInfo2<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkImageMemoryRequirementsInfo2KHR<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2KHR<'_>,
);
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
pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  imageCount: u32,
  pImages: *const VkImage,
  pDatas: *mut VkHostAddressRangeEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkImageCaptureDescriptorDataInfoEXT<'_>,
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
  pInfo: *const VkImageSparseMemoryRequirementsInfo2<'_>,
  pSparseMemoryRequirementCount: *mut u32,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2<'_>,
);
/// [`vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_memory_requirements2`
///
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkImageSparseMemoryRequirementsInfo2KHR<'_>,
  pSparseMemoryRequirementCount: *mut u32,
  pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2KHR<'_>,
);
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
  pSubresource: *const VkImageSubresource2<'_>,
  pLayout: *mut VkSubresourceLayout2<'_>,
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
pub type PFN_vkGetImageSubresourceLayout2EXT = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  pSubresource: *const VkImageSubresource2EXT<'_>,
  pLayout: *mut VkSubresourceLayout2EXT<'_>,
);
/// [`vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2KHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetImageSubresourceLayout2KHR = unsafe extern "system" fn(
  device: VkDevice,
  image: VkImage,
  pSubresource: *const VkImageSubresource2KHR<'_>,
  pLayout: *mut VkSubresourceLayout2KHR<'_>,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NVX_image_view_handle")]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
  device: VkDevice,
  imageView: VkImageView,
  pProperties: *mut VkImageViewAddressPropertiesNVX<'_>,
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
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX<'_>) -> u64;
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
  unsafe extern "system" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX<'_>) -> u32;
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
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkImageViewCaptureDescriptorDataInfoEXT<'_>,
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
  pLatencyMarkerInfo: *mut VkGetLatencyMarkerInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkMemoryGetAndroidHardwareBufferInfoANDROID<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetFdInfo: *const VkMemoryGetFdInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  fd: core::ffi::c_int,
  pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_external_memory_host")]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  pHostPointer: *const core::ffi::c_void,
  pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub type PFN_vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
  device: VkDevice,
  pGetMetalHandleInfo: *const VkMemoryGetMetalHandleInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  pHandle: *const core::ffi::c_void,
  pMemoryMetalHandleProperties: *mut VkMemoryMetalHandlePropertiesEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_OHOS_external_memory")]
pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkMemoryGetNativeBufferInfoOHOS<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
  device: VkDevice,
  pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub type PFN_vkGetMemorySciBufNV = unsafe extern "system" fn(
  device: VkDevice,
  pGetSciBufInfo: *const VkMemoryGetSciBufInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetWin32HandleInfo: *const VkMemoryGetWin32HandleInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  handle: HANDLE,
  pMemoryWin32HandleProperties: *mut VkMemoryWin32HandlePropertiesKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  pGetZirconHandleInfo: *const VkMemoryGetZirconHandleInfoFUCHSIA<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  handleType: VkExternalMemoryHandleTypeFlagBits,
  zirconHandle: zx_handle_t,
  pMemoryZirconHandleProperties: *mut VkMemoryZirconHandlePropertiesFUCHSIA<'_>,
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
  pBuildInfo: *const VkMicromapBuildInfoEXT<'_>,
  pSizeInfo: *mut VkMicromapBuildSizesInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_OHOS_external_memory")]
pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
  device: VkDevice,
  buffer: *const OH_NativeBuffer,
  pProperties: *mut VkNativeBufferPropertiesOHOS<'_>,
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
  pInfo: *const VkPartitionedAccelerationStructureInstancesInputNV<'_>,
  pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR<'_>,
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
pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
  device: VkDevice,
  pPastPresentationTimingInfo: *const VkPastPresentationTimingInfoEXT<'_>,
  pPastPresentationTimingProperties: *mut VkPastPresentationTimingPropertiesEXT<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
  device: VkDevice,
  parameter: VkPerformanceParameterTypeINTEL,
  pValue: *mut VkPerformanceValueINTEL<'_>,
) -> VkResult;
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html)
///
/// Provided by:
/// - `VK_EXT_calibrated_timestamps`
///
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pTimeDomainCount: *mut u32,
  pTimeDomains: *mut VkTimeDomainEXT,
)
  -> VkResult;
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cooperative_matrix2")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_cooperative_matrix")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cooperative_matrix")]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeMatrixPropertiesNV<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_cooperative_vector")]
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pPropertyCount: *mut u32,
    pProperties: *mut VkCooperativeVectorPropertiesNV<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut u32,
  pProperties: *mut VkDisplayPlaneProperties2KHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut u32,
  pProperties: *mut VkDisplayProperties2KHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_display")]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pPropertyCount: *mut u32,
  pProperties: *mut VkDisplayPropertiesKHR<'_>,
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
  pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo<'_>,
  pExternalBufferProperties: *mut VkExternalBufferProperties<'_>,
);
/// [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_memory_capabilities`
///
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfoKHR<'_>,
  pExternalBufferProperties: *mut VkExternalBufferPropertiesKHR<'_>,
);
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
  pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo<'_>,
  pExternalFenceProperties: *mut VkExternalFenceProperties<'_>,
);
/// [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFencePropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_fence_capabilities`
///
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfoKHR<'_>,
  pExternalFenceProperties: *mut VkExternalFencePropertiesKHR<'_>,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub type PFN_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    handleType: VkExternalMemoryHandleTypeFlagBits,
    handle: NvSciBufObj,
    pMemorySciBufProperties: *mut VkMemorySciBufPropertiesNV<'_>,
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
  pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo<'_>,
  pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties<'_>,
);
/// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html)
///
/// Provided by:
/// - `VK_KHR_external_semaphore_capabilities`
///
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfoKHR<'_>,
  pExternalSemaphoreProperties: *mut VkExternalSemaphorePropertiesKHR<'_>,
);
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
  pExternalTensorInfo: *const VkPhysicalDeviceExternalTensorInfoARM<'_>,
  pExternalTensorProperties: *mut VkExternalTensorPropertiesARM<'_>,
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
  pFeatures: *mut VkPhysicalDeviceFeatures2<'_>,
);
/// [`vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pFeatures: *mut VkPhysicalDeviceFeatures2KHR<'_>,
);
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
  pFormatProperties: *mut VkFormatProperties2<'_>,
);
/// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  format: VkFormat,
  pFormatProperties: *mut VkFormatProperties2KHR<'_>,
);
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
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pFragmentShadingRateCount: *mut u32,
  pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR<'_>,
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
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2<'_>,
  pImageFormatProperties: *mut VkImageFormatProperties2<'_>,
) -> VkResult;
/// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2KHR<'_>,
  pImageFormatProperties: *mut VkImageFormatProperties2KHR<'_>,
) -> VkResult;
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
  pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2<'_>,
);
/// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2KHR<'_>,
);
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
  pMultisampleProperties: *mut VkMultisamplePropertiesEXT<'_>,
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
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pOpticalFlowImageFormatInfo: *const VkOpticalFlowImageFormatInfoNV<'_>,
  pFormatCount: *mut u32,
  pImageFormatProperties: *mut VkOpticalFlowImageFormatPropertiesNV<'_>,
) -> VkResult;
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
  pProperties: *mut VkPhysicalDeviceProperties2<'_>,
);
/// [`vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pProperties: *mut VkPhysicalDeviceProperties2KHR<'_>,
);
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const VkQueueFamilyDataGraphPropertiesARM<'_>,
    pProperties: *mut VkBaseOutStructure<'_>,
  ) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphProperties: *const VkQueueFamilyDataGraphPropertiesARM<'_>,
    pOpticalFlowImageFormatInfo: *const VkDataGraphOpticalFlowImageFormatInfoARM<'_>,
    pFormatCount: *mut u32,
    pImageFormatProperties: *mut VkDataGraphOpticalFlowImageFormatPropertiesARM<'_>,
  ) -> VkResult;
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
    pQueueFamilyDataGraphProcessingEngineInfo: *const VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<
        '_,
    >,
    pQueueFamilyDataGraphProcessingEngineProperties: *mut VkQueueFamilyDataGraphProcessingEnginePropertiesARM<
        '_,
    >,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_ARM_data_graph")]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    queueFamilyIndex: u32,
    pQueueFamilyDataGraphPropertyCount: *mut u32,
    pQueueFamilyDataGraphProperties: *mut VkQueueFamilyDataGraphPropertiesARM<'_>,
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
    pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR<'_>,
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
  pQueueFamilyProperties: *mut VkQueueFamilyProperties2<'_>,
);
/// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pQueueFamilyPropertyCount: *mut u32,
  pQueueFamilyProperties: *mut VkQueueFamilyProperties2KHR<'_>,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
pub type PFN_vkGetPhysicalDeviceSciSyncAttributesNV = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSciSyncAttributesInfo: *const VkSciSyncAttributesInfoNV<'_>,
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
  pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2<'_>,
  pPropertyCount: *mut u32,
  pProperties: *mut VkSparseImageFormatProperties2<'_>,
);
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html)
///
/// Provided by:
/// - `VK_KHR_get_physical_device_properties2`
///
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2KHR<'_>,
  pPropertyCount: *mut u32,
  pProperties: *mut VkSparseImageFormatProperties2KHR<'_>,
);
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
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pCombinationCount: *mut u32,
    pCombinations: *mut VkFramebufferMixedSamplesCombinationNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  surface: VkSurfaceKHR,
  pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR<'_>,
  pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR<'_>,
  pSurfaceFormatCount: *mut u32,
  pSurfaceFormats: *mut VkSurfaceFormat2KHR<'_>,
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
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pToolCount: *mut u32,
  pToolProperties: *mut VkPhysicalDeviceToolProperties<'_>,
) -> VkResult;
/// [`vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolPropertiesEXT.html)
///
/// Provided by:
/// - `VK_EXT_tooling_info`
///
#[cfg(feature = "VK_EXT_tooling_info")]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pToolCount: *mut u32,
  pToolProperties: *mut VkPhysicalDeviceToolPropertiesEXT<'_>,
) -> VkResult;
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
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pVideoProfile: *const VkVideoProfileInfoKHR<'_>,
  pCapabilities: *mut VkVideoCapabilitiesKHR<'_>,
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
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
  unsafe extern "system" fn(
    physicalDevice: VkPhysicalDevice,
    pQualityLevelInfo: *const VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
    pQualityLevelProperties: *mut VkVideoEncodeQualityLevelPropertiesKHR<'_>,
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
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
  physicalDevice: VkPhysicalDevice,
  pVideoFormatInfo: *const VkPhysicalDeviceVideoFormatInfoKHR<'_>,
  pVideoFormatPropertyCount: *mut u32,
  pVideoFormatProperties: *mut VkVideoFormatPropertiesKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_NOT_ENOUGH_SPACE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkPipelineBinaryDataInfoKHR<'_>,
  pPipelineBinaryKey: *mut VkPipelineBinaryKeyKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
  unsafe extern "system" fn(
    device: VkDevice,
    pExecutableInfo: *const VkPipelineExecutableInfoKHR<'_>,
    pInternalRepresentationCount: *mut u32,
    pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineInfo: *const VkPipelineInfoKHR<'_>,
  pExecutableCount: *mut u32,
  pProperties: *mut VkPipelineExecutablePropertiesKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
  device: VkDevice,
  pExecutableInfo: *const VkPipelineExecutableInfoKHR<'_>,
  pStatisticCount: *mut u32,
  pStatistics: *mut VkPipelineExecutableStatisticKHR<'_>,
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
  pInfo: *const VkPipelineIndirectDeviceAddressInfoNV<'_>,
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
  pCreateInfo: *const VkComputePipelineCreateInfo<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineCreateInfo: *const VkPipelineCreateInfoKHR<'_>,
  pPipelineKey: *mut VkPipelineBinaryKeyKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_pipeline_properties")]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  pPipelineInfo: *const VkPipelineInfoKHR<'_>,
  pPipelineProperties: *mut VkBaseOutStructure<'_>,
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
pub type PFN_vkGetPrivateDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  objectType: VkObjectType,
  objectHandle: u64,
  privateDataSlot: VkPrivateDataSlotEXT,
  pData: *mut u64,
);
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
///   - `VK_SUCCESS`
///   - `VK_NOT_READY`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
/// - **Availability:** depends on `VK_VERSION_1_3 + VK_KHR_synchronization2`
///
/// # Parameters
/// - `queue`
/// - `pCheckpointDataCount`: optional: pointer required, values optional if pointer not null
/// - `pCheckpointData`: optional: true, len: pCheckpointDataCount
#[cfg(any(
  all(
    feature = "VK_NV_device_diagnostic_checkpoints",
    feature = "VK_VERSION_1_3"
  ),
  all(
    feature = "VK_KHR_synchronization2",
    feature = "VK_NV_device_diagnostic_checkpoints"
  )
))]
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
  queue: VkQueue,
  pCheckpointDataCount: *mut u32,
  pCheckpointData: *mut VkCheckpointData2NV<'_>,
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
  pCheckpointData: *mut VkCheckpointDataNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = unsafe extern "system" fn(
  device: VkDevice,
  pipeline: VkPipeline,
  firstGroup: u32,
  groupCount: u32,
  dataSize: usize,
  pData: *mut core::ffi::c_void,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pRenderingAreaInfo: *const VkRenderingAreaInfo<'_>,
  pGranularity: *mut VkExtent2D,
);
/// [`vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularityKHR.html)
///
/// Provided by:
/// - `VK_KHR_maintenance5`
///
#[cfg(feature = "VK_KHR_maintenance5")]
pub type PFN_vkGetRenderingAreaGranularityKHR = unsafe extern "system" fn(
  device: VkDevice,
  pRenderingAreaInfo: *const VkRenderingAreaInfoKHR<'_>,
  pGranularity: *mut VkExtent2D,
);
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
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkSamplerCaptureDescriptorDataInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
  device: VkDevice,
  buffer: *const _screen_buffer,
  pProperties: *mut VkScreenBufferPropertiesQNX<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkGetSemaphoreCounterValue =
  unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
/// [`vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValueKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkGetSemaphoreCounterValueKHR =
  unsafe extern "system" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;
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
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetFdInfo: *const VkSemaphoreGetFdInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_NOT_PERMITTED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_sci_sync")]
pub type PFN_vkGetSemaphoreSciSyncObjNV = unsafe extern "system" fn(
  device: VkDevice,
  pGetSciSyncInfo: *const VkSemaphoreGetSciSyncInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
  device: VkDevice,
  pGetWin32HandleInfo: *const VkSemaphoreGetWin32HandleInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  pGetZirconHandleInfo: *const VkSemaphoreGetZirconHandleInfoFUCHSIA<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_FEATURE_NOT_PRESENT`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  pCreateInfo: *const VkShaderModuleCreateInfo<'_>,
  pIdentifier: *mut VkShaderModuleIdentifierEXT<'_>,
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
  pIdentifier: *mut VkShaderModuleIdentifierEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pSwapchainTimeDomainProperties: *mut VkSwapchainTimeDomainPropertiesEXT<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_NOT_READY`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_present_timing")]
pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pSwapchainTimingProperties: *mut VkSwapchainTimingPropertiesEXT<'_>,
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
  pInfo: *const VkTensorMemoryRequirementsInfoARM<'_>,
  pMemoryRequirements: *mut VkMemoryRequirements2<'_>,
);
/// [`vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Availability:** depends on `VK_ARM_tensors`
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
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_heap"))]
pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
  device: VkDevice,
  tensorCount: u32,
  pTensors: *const VkTensorARM,
  pDatas: *mut VkHostAddressRangeEXT<'_>,
) -> VkResult;
/// [`vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html)
///
/// Provided by:
/// - `VK_ARM_tensors`
///
/// - **Availability:** depends on `VK_EXT_descriptor_buffer`
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
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkTensorCaptureDescriptorDataInfoARM<'_>,
  pData: *mut core::ffi::c_void,
) -> VkResult;
/// [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html)
///
/// Provided by:
/// - `VK_ARM_tensors`
///
/// - **Availability:** depends on `VK_EXT_descriptor_buffer`
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
#[cfg(all(feature = "VK_ARM_tensors", feature = "VK_EXT_descriptor_buffer"))]
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkTensorViewCaptureDescriptorDataInfoARM<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///   - `VK_INCOMPLETE`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
  device: VkDevice,
  videoSession: VkVideoSessionKHR,
  pMemoryRequirementsCount: *mut u32,
  pMemoryRequirements: *mut VkVideoSessionMemoryRequirementsKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
  device: VkDevice,
  pImportFenceFdInfo: *const VkImportFenceFdInfoKHR<'_>,
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
pub type PFN_vkImportFenceSciSyncFenceNV = unsafe extern "system" fn(
  device: VkDevice,
  pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV<'_>,
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
pub type PFN_vkImportFenceSciSyncObjNV = unsafe extern "system" fn(
  device: VkDevice,
  pImportFenceSciSyncInfo: *const VkImportFenceSciSyncInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
  device: VkDevice,
  pImportFenceWin32HandleInfo: *const VkImportFenceWin32HandleInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
  device: VkDevice,
  pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_NOT_PERMITTED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_external_sci_sync")]
pub type PFN_vkImportSemaphoreSciSyncObjNV = unsafe extern "system" fn(
  device: VkDevice,
  pImportSemaphoreSciSyncInfo: *const VkImportSemaphoreSciSyncInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
  device: VkDevice,
  pImportSemaphoreWin32HandleInfo: *const VkImportSemaphoreWin32HandleInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  pImportSemaphoreZirconHandleInfo: *const VkImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_INTEL_performance_query")]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
  device: VkDevice,
  pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
  device: VkDevice,
  memoryRangeCount: u32,
  pMemoryRanges: *const VkMappedMemoryRange<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pSleepInfo: *const VkLatencySleepInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_MEMORY_MAP_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_MEMORY_MAP_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
  device: VkDevice,
  pMemoryMapInfo: *const VkMemoryMapInfo<'_>,
  ppData: *mut *mut core::ffi::c_void,
) -> VkResult;
/// [`vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_map_memory2`
///
#[cfg(feature = "VK_KHR_map_memory2")]
pub type PFN_vkMapMemory2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pMemoryMapInfo: *const VkMemoryMapInfoKHR<'_>,
  ppData: *mut *mut core::ffi::c_void,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
  unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT<'_>);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
  queue: VkQueue,
  bindInfoCount: u32,
  pBindInfo: *const VkBindSparseInfo<'_>,
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
  unsafe extern "system" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT<'_>);
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
  unsafe extern "system" fn(queue: VkQueue, pQueueTypeInfo: *const VkOutOfBandQueueTypeInfoNV<'_>);
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
///   - `VK_SUCCESS`
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
///   - `VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT`
#[cfg(feature = "VK_KHR_swapchain")]
pub type PFN_vkQueuePresentKHR =
  unsafe extern "system" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR<'_>) -> VkResult;
/// [`vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html)
///
/// Provided by:
/// - `VK_QCOM_queue_perf_hint`
///
///
/// # Parameters
/// - `queue`
/// - `pPerfHintInfo`
///
/// # Returns
///
/// **Success Codes:**
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_QCOM_queue_perf_hint")]
pub type PFN_vkQueueSetPerfHintQCOM = unsafe extern "system" fn(
  queue: VkQueue,
  pPerfHintInfo: *const VkPerfHintInfoQCOM<'_>,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[deprecated(note = "superseded by `vkQueueSubmit2`")]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
  queue: VkQueue,
  submitCount: u32,
  pSubmits: *const VkSubmitInfo<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
  queue: VkQueue,
  submitCount: u32,
  pSubmits: *const VkSubmitInfo2<'_>,
  fence: VkFence,
) -> VkResult;
/// [`vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2KHR.html)
///
/// Provided by:
/// - `VK_KHR_synchronization2`
///
#[cfg(feature = "VK_KHR_synchronization2")]
pub type PFN_vkQueueSubmit2KHR = unsafe extern "system" fn(
  queue: VkQueue,
  submitCount: u32,
  pSubmits: *const VkSubmitInfo2KHR<'_>,
  fence: VkFence,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: VkQueue) -> VkResult;
/// [`vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Availability:** depends on `VK_EXT_custom_border_color`
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
#[cfg(all(
  feature = "VK_EXT_custom_border_color",
  feature = "VK_EXT_descriptor_heap"
))]
pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
  device: VkDevice,
  pBorderColor: *const VkSamplerCustomBorderColorCreateInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
  device: VkDevice,
  pDeviceEventInfo: *const VkDeviceEventInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_display_control")]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
  device: VkDevice,
  display: VkDisplayKHR,
  pDisplayEventInfo: *const VkDisplayEventInfoEXT<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *const VkReleaseCapturedPipelineDataInfoKHR<'_>,
  pAllocator: *const VkAllocationCallbacks<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_SURFACE_LOST_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_TOO_MANY_OBJECTS`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
pub type PFN_vkReleaseSwapchainImagesEXT = unsafe extern "system" fn(
  device: VkDevice,
  pReleaseInfo: *const VkReleaseSwapchainImagesInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
  device: VkDevice,
  pReleaseInfo: *const VkReleaseSwapchainImagesInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub type PFN_vkResetFences =
  unsafe extern "system" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
/// [`vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
///
///
/// # Parameters
/// - `device`
/// - `gpaSession`
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
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkResetGpaSessionAMD =
  unsafe extern "system" fn(device: VkDevice, gpaSession: VkGpaSessionAMD) -> VkResult;
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
pub type PFN_vkResetQueryPoolEXT = unsafe extern "system" fn(
  device: VkDevice,
  queryPool: VkQueryPool,
  firstQuery: u32,
  queryCount: u32,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  collection: VkBufferCollectionFUCHSIA,
  pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
  device: VkDevice,
  collection: VkBufferCollectionFUCHSIA,
  pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
  device: VkDevice,
  pNameInfo: *const VkDebugUtilsObjectNameInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_debug_utils")]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
  device: VkDevice,
  pTagInfo: *const VkDebugUtilsObjectTagInfoEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: VkDevice, event: VkEvent) -> VkResult;
/// [`vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html)
///
/// Provided by:
/// - `VK_AMD_gpa_interface`
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
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_AMD_gpa_interface")]
pub type PFN_vkSetGpaDeviceClockModeAMD = unsafe extern "system" fn(
  device: VkDevice,
  pInfo: *mut VkGpaDeviceClockModeInfoAMD<'_>,
) -> VkResult;
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
  pMetadata: *const VkHdrMetadataEXT<'_>,
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
  pLatencyMarkerInfo: *const VkSetLatencyMarkerInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_INITIALIZATION_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_NV_low_latency2")]
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pSleepModeInfo: *const VkLatencySleepModeInfoNV<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
pub type PFN_vkSetPrivateDataEXT = unsafe extern "system" fn(
  device: VkDevice,
  objectType: VkObjectType,
  objectHandle: u64,
  privateDataSlot: VkPrivateDataSlotEXT,
  data: u64,
) -> VkResult;
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
///   - `VK_SUCCESS`
///   - `VK_NOT_READY`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
  device: VkDevice,
  pSignalInfo: *const VkSemaphoreSignalInfo<'_>,
) -> VkResult;
/// [`vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphoreKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkSignalSemaphoreKHR = unsafe extern "system" fn(
  device: VkDevice,
  pSignalInfo: *const VkSemaphoreSignalInfoKHR<'_>,
) -> VkResult;
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
  pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT<'_>,
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
pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
  device: VkDevice,
  transitionCount: u32,
  pTransitions: *const VkHostImageLayoutTransitionInfo<'_>,
) -> VkResult;
/// [`vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayoutEXT.html)
///
/// Provided by:
/// - `VK_EXT_host_image_copy`
///
#[cfg(feature = "VK_EXT_host_image_copy")]
pub type PFN_vkTransitionImageLayoutEXT = unsafe extern "system" fn(
  device: VkDevice,
  transitionCount: u32,
  pTransitions: *const VkHostImageLayoutTransitionInfoEXT<'_>,
) -> VkResult;
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
pub type PFN_vkTrimCommandPoolKHR = unsafe extern "system" fn(
  device: VkDevice,
  commandPool: VkCommandPool,
  flags: VkCommandPoolTrimFlagsKHR,
);
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_MEMORY_MAP_FAILED`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
  device: VkDevice,
  pMemoryUnmapInfo: *const VkMemoryUnmapInfo<'_>,
) -> VkResult;
/// [`vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html)
///
/// Provided by:
/// - `VK_KHR_map_memory2`
///
#[cfg(feature = "VK_KHR_map_memory2")]
pub type PFN_vkUnmapMemory2KHR = unsafe extern "system" fn(
  device: VkDevice,
  pMemoryUnmapInfo: *const VkMemoryUnmapInfoKHR<'_>,
) -> VkResult;
/// [`vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html)
///
/// Provided by:
/// - `VK_EXT_descriptor_heap`
///
/// - **Availability:** depends on `VK_EXT_custom_border_color`
///
/// # Parameters
/// - `device`
/// - `index`
#[cfg(all(
  feature = "VK_EXT_custom_border_color",
  feature = "VK_EXT_descriptor_heap"
))]
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
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
  device: VkDevice,
  descriptorSet: VkDescriptorSet,
  descriptorUpdateTemplate: VkDescriptorUpdateTemplateKHR,
  pData: *const core::ffi::c_void,
);
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
  pDescriptorWrites: *const VkWriteDescriptorSet<'_>,
  descriptorCopyCount: u32,
  pDescriptorCopies: *const VkCopyDescriptorSet<'_>,
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
  pExecutionSetWrites: *const VkWriteIndirectExecutionSetPipelineEXT<'_>,
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
  pExecutionSetWrites: *const VkWriteIndirectExecutionSetShaderEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_KHR_video_queue")]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
  device: VkDevice,
  videoSessionParameters: VkVideoSessionParametersKHR,
  pUpdateInfo: *const VkVideoSessionParametersUpdateInfoKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_TIMEOUT`
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
#[cfg(feature = "VK_KHR_present_wait2")]
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
  device: VkDevice,
  swapchain: VkSwapchainKHR,
  pPresentWait2Info: *const VkPresentWait2InfoKHR<'_>,
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
///   - `VK_SUCCESS`
///   - `VK_TIMEOUT`
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
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
  device: VkDevice,
  pWaitInfo: *const VkSemaphoreWaitInfo<'_>,
  timeout: u64,
) -> VkResult;
/// [`vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphoresKHR.html)
///
/// Provided by:
/// - `VK_KHR_timeline_semaphore`
///
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub type PFN_vkWaitSemaphoresKHR = unsafe extern "system" fn(
  device: VkDevice,
  pWaitInfo: *const VkSemaphoreWaitInfoKHR<'_>,
  timeout: u64,
) -> VkResult;
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
  device: VkDevice,
  resourceCount: u32,
  pResources: *const VkResourceDescriptorInfoEXT<'_>,
  pDescriptors: *const VkHostAddressRangeEXT<'_>,
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
///   - `VK_SUCCESS`
///
/// **Error Codes:**
///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
///   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///   - `VK_ERROR_UNKNOWN`
///   - `VK_ERROR_VALIDATION_FAILED`
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub type PFN_vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
  device: VkDevice,
  samplerCount: u32,
  pSamplers: *const VkSamplerCreateInfo<'_>,
  pDescriptors: *const VkHostAddressRangeEXT<'_>,
) -> VkResult;
