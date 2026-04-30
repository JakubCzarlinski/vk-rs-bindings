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
pub struct CommandBufferDispatchTable {
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  pub vkCmdDispatchGraphAMDX: Option<PFN_vkCmdDispatchGraphAMDX>,
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  pub vkCmdDispatchGraphIndirectAMDX: Option<PFN_vkCmdDispatchGraphIndirectAMDX>,
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  pub vkCmdDispatchGraphIndirectCountAMDX: Option<PFN_vkCmdDispatchGraphIndirectCountAMDX>,
  #[cfg(feature = "VK_AMDX_shader_enqueue")]
  pub vkCmdInitializeGraphScratchMemoryAMDX: Option<PFN_vkCmdInitializeGraphScratchMemoryAMDX>,
  #[cfg(feature = "VK_AMD_buffer_marker")]
  pub vkCmdWriteBufferMarker2AMD: Option<PFN_vkCmdWriteBufferMarker2AMD>,
  #[cfg(feature = "VK_AMD_buffer_marker")]
  pub vkCmdWriteBufferMarkerAMD: Option<PFN_vkCmdWriteBufferMarkerAMD>,
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  pub vkCmdDrawIndexedIndirectCountAMD: Option<PFN_vkCmdDrawIndexedIndirectCountAMD>,
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  pub vkCmdDrawIndirectCountAMD: Option<PFN_vkCmdDrawIndirectCountAMD>,
  #[cfg(feature = "VK_ARM_data_graph")]
  pub vkCmdDispatchDataGraphARM: Option<PFN_vkCmdDispatchDataGraphARM>,
  #[cfg(feature = "VK_ARM_scheduling_controls")]
  pub vkCmdSetDispatchParametersARM: Option<PFN_vkCmdSetDispatchParametersARM>,
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  pub vkCmdBeginShaderInstrumentationARM: Option<PFN_vkCmdBeginShaderInstrumentationARM>,
  #[cfg(feature = "VK_ARM_shader_instrumentation")]
  pub vkCmdEndShaderInstrumentationARM: Option<PFN_vkCmdEndShaderInstrumentationARM>,
  #[cfg(feature = "VK_ARM_tensors")]
  pub vkCmdCopyTensorARM: Option<PFN_vkCmdCopyTensorARM>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkBeginCommandBuffer: Option<PFN_vkBeginCommandBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdBeginQuery: Option<PFN_vkCmdBeginQuery>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdCopyBuffer: Option<PFN_vkCmdCopyBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdCopyBufferToImage: Option<PFN_vkCmdCopyBufferToImage>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdCopyImage: Option<PFN_vkCmdCopyImage>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdCopyImageToBuffer: Option<PFN_vkCmdCopyImageToBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdCopyQueryPoolResults: Option<PFN_vkCmdCopyQueryPoolResults>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdEndQuery: Option<PFN_vkCmdEndQuery>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdExecuteCommands: Option<PFN_vkCmdExecuteCommands>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdFillBuffer: Option<PFN_vkCmdFillBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdPipelineBarrier: Option<PFN_vkCmdPipelineBarrier>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdResetQueryPool: Option<PFN_vkCmdResetQueryPool>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdUpdateBuffer: Option<PFN_vkCmdUpdateBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkCmdWriteTimestamp: Option<PFN_vkCmdWriteTimestamp>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkEndCommandBuffer: Option<PFN_vkEndCommandBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_0")]
  pub vkResetCommandBuffer: Option<PFN_vkResetCommandBuffer>,
  #[cfg(feature = "VK_BASE_VERSION_1_1")]
  pub vkCmdSetDeviceMask: Option<PFN_vkCmdSetDeviceMask>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdCopyBuffer2: Option<PFN_vkCmdCopyBuffer2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdCopyBufferToImage2: Option<PFN_vkCmdCopyBufferToImage2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdCopyImage2: Option<PFN_vkCmdCopyImage2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdCopyImageToBuffer2: Option<PFN_vkCmdCopyImageToBuffer2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdPipelineBarrier2: Option<PFN_vkCmdPipelineBarrier2>,
  #[cfg(feature = "VK_BASE_VERSION_1_3")]
  pub vkCmdWriteTimestamp2: Option<PFN_vkCmdWriteTimestamp2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdBindDescriptorSets: Option<PFN_vkCmdBindDescriptorSets>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdBindPipeline: Option<PFN_vkCmdBindPipeline>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdClearColorImage: Option<PFN_vkCmdClearColorImage>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdDispatch: Option<PFN_vkCmdDispatch>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdDispatchIndirect: Option<PFN_vkCmdDispatchIndirect>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdPushConstants: Option<PFN_vkCmdPushConstants>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdResetEvent: Option<PFN_vkCmdResetEvent>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdSetEvent: Option<PFN_vkCmdSetEvent>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
  pub vkCmdWaitEvents: Option<PFN_vkCmdWaitEvents>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
  pub vkCmdDispatchBase: Option<PFN_vkCmdDispatchBase>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  pub vkCmdResetEvent2: Option<PFN_vkCmdResetEvent2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  pub vkCmdSetEvent2: Option<PFN_vkCmdSetEvent2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
  pub vkCmdWaitEvents2: Option<PFN_vkCmdWaitEvents2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdBindDescriptorSets2: Option<PFN_vkCmdBindDescriptorSets2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdPushConstants2: Option<PFN_vkCmdPushConstants2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdPushDescriptorSet: Option<PFN_vkCmdPushDescriptorSet>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdPushDescriptorSet2: Option<PFN_vkCmdPushDescriptorSet2>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdPushDescriptorSetWithTemplate: Option<PFN_vkCmdPushDescriptorSetWithTemplate>,
  #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
  pub vkCmdPushDescriptorSetWithTemplate2: Option<PFN_vkCmdPushDescriptorSetWithTemplate2>,
  #[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
  pub vkCmdSetAttachmentFeedbackLoopEnableEXT: Option<PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT>,
  #[cfg(feature = "VK_EXT_color_write_enable")]
  pub vkCmdSetColorWriteEnableEXT: Option<PFN_vkCmdSetColorWriteEnableEXT>,
  #[cfg(feature = "VK_EXT_conditional_rendering")]
  pub vkCmdBeginConditionalRenderingEXT: Option<PFN_vkCmdBeginConditionalRenderingEXT>,
  #[cfg(feature = "VK_EXT_conditional_rendering")]
  pub vkCmdEndConditionalRenderingEXT: Option<PFN_vkCmdEndConditionalRenderingEXT>,
  #[cfg(feature = "VK_EXT_custom_resolve")]
  pub vkCmdBeginCustomResolveEXT: Option<PFN_vkCmdBeginCustomResolveEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerBeginEXT: Option<PFN_vkCmdDebugMarkerBeginEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerEndEXT: Option<PFN_vkCmdDebugMarkerEndEXT>,
  #[cfg(feature = "VK_EXT_debug_marker")]
  pub vkCmdDebugMarkerInsertEXT: Option<PFN_vkCmdDebugMarkerInsertEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkCmdBeginDebugUtilsLabelEXT: Option<PFN_vkCmdBeginDebugUtilsLabelEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkCmdEndDebugUtilsLabelEXT: Option<PFN_vkCmdEndDebugUtilsLabelEXT>,
  #[cfg(feature = "VK_EXT_debug_utils")]
  pub vkCmdInsertDebugUtilsLabelEXT: Option<PFN_vkCmdInsertDebugUtilsLabelEXT>,
  #[cfg(feature = "VK_EXT_depth_bias_control")]
  pub vkCmdSetDepthBias2EXT: Option<PFN_vkCmdSetDepthBias2EXT>,
  #[cfg(any(
    feature = "VK_EXT_depth_clamp_control",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthClampRangeEXT: Option<PFN_vkCmdSetDepthClampRangeEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkCmdBindDescriptorBufferEmbeddedSamplersEXT:
    Option<PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkCmdBindDescriptorBuffersEXT: Option<PFN_vkCmdBindDescriptorBuffersEXT>,
  #[cfg(feature = "VK_EXT_descriptor_buffer")]
  pub vkCmdSetDescriptorBufferOffsetsEXT: Option<PFN_vkCmdSetDescriptorBufferOffsetsEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkCmdBindResourceHeapEXT: Option<PFN_vkCmdBindResourceHeapEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkCmdBindSamplerHeapEXT: Option<PFN_vkCmdBindSamplerHeapEXT>,
  #[cfg(feature = "VK_EXT_descriptor_heap")]
  pub vkCmdPushDataEXT: Option<PFN_vkCmdPushDataEXT>,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkCmdExecuteGeneratedCommandsEXT: Option<PFN_vkCmdExecuteGeneratedCommandsEXT>,
  #[cfg(feature = "VK_EXT_device_generated_commands")]
  pub vkCmdPreprocessGeneratedCommandsEXT: Option<PFN_vkCmdPreprocessGeneratedCommandsEXT>,
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  pub vkCmdSetDiscardRectangleEXT: Option<PFN_vkCmdSetDiscardRectangleEXT>,
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  pub vkCmdSetDiscardRectangleEnableEXT: Option<PFN_vkCmdSetDiscardRectangleEnableEXT>,
  #[cfg(feature = "VK_EXT_discard_rectangles")]
  pub vkCmdSetDiscardRectangleModeEXT: Option<PFN_vkCmdSetDiscardRectangleModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdBindVertexBuffers2EXT: Option<PFN_vkCmdBindVertexBuffers2EXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCullModeEXT: Option<PFN_vkCmdSetCullModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthBoundsTestEnableEXT: Option<PFN_vkCmdSetDepthBoundsTestEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthCompareOpEXT: Option<PFN_vkCmdSetDepthCompareOpEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthTestEnableEXT: Option<PFN_vkCmdSetDepthTestEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthWriteEnableEXT: Option<PFN_vkCmdSetDepthWriteEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetFrontFaceEXT: Option<PFN_vkCmdSetFrontFaceEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetPrimitiveTopologyEXT: Option<PFN_vkCmdSetPrimitiveTopologyEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetScissorWithCountEXT: Option<PFN_vkCmdSetScissorWithCountEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetStencilOpEXT: Option<PFN_vkCmdSetStencilOpEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetStencilTestEnableEXT: Option<PFN_vkCmdSetStencilTestEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetViewportWithCountEXT: Option<PFN_vkCmdSetViewportWithCountEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthBiasEnableEXT: Option<PFN_vkCmdSetDepthBiasEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetLogicOpEXT: Option<PFN_vkCmdSetLogicOpEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetPatchControlPointsEXT: Option<PFN_vkCmdSetPatchControlPointsEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetPrimitiveRestartEnableEXT: Option<PFN_vkCmdSetPrimitiveRestartEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetRasterizerDiscardEnableEXT: Option<PFN_vkCmdSetRasterizerDiscardEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetAlphaToCoverageEnableEXT: Option<PFN_vkCmdSetAlphaToCoverageEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetAlphaToOneEnableEXT: Option<PFN_vkCmdSetAlphaToOneEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetColorBlendAdvancedEXT: Option<PFN_vkCmdSetColorBlendAdvancedEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetColorBlendEnableEXT: Option<PFN_vkCmdSetColorBlendEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetColorBlendEquationEXT: Option<PFN_vkCmdSetColorBlendEquationEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetColorWriteMaskEXT: Option<PFN_vkCmdSetColorWriteMaskEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetConservativeRasterizationModeEXT:
    Option<PFN_vkCmdSetConservativeRasterizationModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageModulationModeNV: Option<PFN_vkCmdSetCoverageModulationModeNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageModulationTableEnableNV: Option<PFN_vkCmdSetCoverageModulationTableEnableNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageModulationTableNV: Option<PFN_vkCmdSetCoverageModulationTableNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageReductionModeNV: Option<PFN_vkCmdSetCoverageReductionModeNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageToColorEnableNV: Option<PFN_vkCmdSetCoverageToColorEnableNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetCoverageToColorLocationNV: Option<PFN_vkCmdSetCoverageToColorLocationNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthClampEnableEXT: Option<PFN_vkCmdSetDepthClampEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthClipEnableEXT: Option<PFN_vkCmdSetDepthClipEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetDepthClipNegativeOneToOneEXT: Option<PFN_vkCmdSetDepthClipNegativeOneToOneEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetExtraPrimitiveOverestimationSizeEXT:
    Option<PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetLineRasterizationModeEXT: Option<PFN_vkCmdSetLineRasterizationModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetLineStippleEnableEXT: Option<PFN_vkCmdSetLineStippleEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetLogicOpEnableEXT: Option<PFN_vkCmdSetLogicOpEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetPolygonModeEXT: Option<PFN_vkCmdSetPolygonModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetProvokingVertexModeEXT: Option<PFN_vkCmdSetProvokingVertexModeEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetRasterizationSamplesEXT: Option<PFN_vkCmdSetRasterizationSamplesEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetRasterizationStreamEXT: Option<PFN_vkCmdSetRasterizationStreamEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetRepresentativeFragmentTestEnableNV:
    Option<PFN_vkCmdSetRepresentativeFragmentTestEnableNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetSampleLocationsEnableEXT: Option<PFN_vkCmdSetSampleLocationsEnableEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetSampleMaskEXT: Option<PFN_vkCmdSetSampleMaskEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetShadingRateImageEnableNV: Option<PFN_vkCmdSetShadingRateImageEnableNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetTessellationDomainOriginEXT: Option<PFN_vkCmdSetTessellationDomainOriginEXT>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetViewportSwizzleNV: Option<PFN_vkCmdSetViewportSwizzleNV>,
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state3",
    feature = "VK_EXT_shader_object"
  ))]
  pub vkCmdSetViewportWScalingEnableNV: Option<PFN_vkCmdSetViewportWScalingEnableNV>,
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  pub vkCmdEndRendering2EXT: Option<PFN_vkCmdEndRendering2EXT>,
  #[cfg(feature = "VK_EXT_line_rasterization")]
  pub vkCmdSetLineStippleEXT: Option<PFN_vkCmdSetLineStippleEXT>,
  #[cfg(feature = "VK_EXT_memory_decompression")]
  pub vkCmdDecompressMemoryEXT: Option<PFN_vkCmdDecompressMemoryEXT>,
  #[cfg(feature = "VK_EXT_memory_decompression")]
  pub vkCmdDecompressMemoryIndirectCountEXT: Option<PFN_vkCmdDecompressMemoryIndirectCountEXT>,
  #[cfg(feature = "VK_EXT_mesh_shader")]
  pub vkCmdDrawMeshTasksEXT: Option<PFN_vkCmdDrawMeshTasksEXT>,
  #[cfg(feature = "VK_EXT_mesh_shader")]
  pub vkCmdDrawMeshTasksIndirectCountEXT: Option<PFN_vkCmdDrawMeshTasksIndirectCountEXT>,
  #[cfg(feature = "VK_EXT_mesh_shader")]
  pub vkCmdDrawMeshTasksIndirectEXT: Option<PFN_vkCmdDrawMeshTasksIndirectEXT>,
  #[cfg(feature = "VK_EXT_multi_draw")]
  pub vkCmdDrawMultiEXT: Option<PFN_vkCmdDrawMultiEXT>,
  #[cfg(feature = "VK_EXT_multi_draw")]
  pub vkCmdDrawMultiIndexedEXT: Option<PFN_vkCmdDrawMultiIndexedEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCmdBuildMicromapsEXT: Option<PFN_vkCmdBuildMicromapsEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCmdCopyMemoryToMicromapEXT: Option<PFN_vkCmdCopyMemoryToMicromapEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCmdCopyMicromapEXT: Option<PFN_vkCmdCopyMicromapEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCmdCopyMicromapToMemoryEXT: Option<PFN_vkCmdCopyMicromapToMemoryEXT>,
  #[cfg(feature = "VK_EXT_opacity_micromap")]
  pub vkCmdWriteMicromapsPropertiesEXT: Option<PFN_vkCmdWriteMicromapsPropertiesEXT>,
  #[cfg(feature = "VK_EXT_primitive_restart_index")]
  pub vkCmdSetPrimitiveRestartIndexEXT: Option<PFN_vkCmdSetPrimitiveRestartIndexEXT>,
  #[cfg(feature = "VK_EXT_sample_locations")]
  pub vkCmdSetSampleLocationsEXT: Option<PFN_vkCmdSetSampleLocationsEXT>,
  #[cfg(feature = "VK_EXT_shader_object")]
  pub vkCmdBindShadersEXT: Option<PFN_vkCmdBindShadersEXT>,
  #[cfg(any(
    feature = "VK_EXT_shader_object",
    feature = "VK_EXT_vertex_input_dynamic_state"
  ))]
  pub vkCmdSetVertexInputEXT: Option<PFN_vkCmdSetVertexInputEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdBeginQueryIndexedEXT: Option<PFN_vkCmdBeginQueryIndexedEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdBeginTransformFeedbackEXT: Option<PFN_vkCmdBeginTransformFeedbackEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdBindTransformFeedbackBuffersEXT: Option<PFN_vkCmdBindTransformFeedbackBuffersEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdDrawIndirectByteCountEXT: Option<PFN_vkCmdDrawIndirectByteCountEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdEndQueryIndexedEXT: Option<PFN_vkCmdEndQueryIndexedEXT>,
  #[cfg(feature = "VK_EXT_transform_feedback")]
  pub vkCmdEndTransformFeedbackEXT: Option<PFN_vkCmdEndTransformFeedbackEXT>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdBeginRenderPass: Option<PFN_vkCmdBeginRenderPass>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdBindIndexBuffer: Option<PFN_vkCmdBindIndexBuffer>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdBindVertexBuffers: Option<PFN_vkCmdBindVertexBuffers>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdBlitImage: Option<PFN_vkCmdBlitImage>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdClearAttachments: Option<PFN_vkCmdClearAttachments>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdClearDepthStencilImage: Option<PFN_vkCmdClearDepthStencilImage>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdDraw: Option<PFN_vkCmdDraw>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdDrawIndexed: Option<PFN_vkCmdDrawIndexed>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdDrawIndexedIndirect: Option<PFN_vkCmdDrawIndexedIndirect>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdDrawIndirect: Option<PFN_vkCmdDrawIndirect>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdEndRenderPass: Option<PFN_vkCmdEndRenderPass>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdNextSubpass: Option<PFN_vkCmdNextSubpass>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdResolveImage: Option<PFN_vkCmdResolveImage>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetBlendConstants: Option<PFN_vkCmdSetBlendConstants>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetDepthBias: Option<PFN_vkCmdSetDepthBias>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetDepthBounds: Option<PFN_vkCmdSetDepthBounds>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetLineWidth: Option<PFN_vkCmdSetLineWidth>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetScissor: Option<PFN_vkCmdSetScissor>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetStencilCompareMask: Option<PFN_vkCmdSetStencilCompareMask>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetStencilReference: Option<PFN_vkCmdSetStencilReference>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetStencilWriteMask: Option<PFN_vkCmdSetStencilWriteMask>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
  pub vkCmdSetViewport: Option<PFN_vkCmdSetViewport>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCmdBeginRenderPass2: Option<PFN_vkCmdBeginRenderPass2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCmdDrawIndexedIndirectCount: Option<PFN_vkCmdDrawIndexedIndirectCount>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCmdDrawIndirectCount: Option<PFN_vkCmdDrawIndirectCount>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCmdEndRenderPass2: Option<PFN_vkCmdEndRenderPass2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
  pub vkCmdNextSubpass2: Option<PFN_vkCmdNextSubpass2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdBeginRendering: Option<PFN_vkCmdBeginRendering>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdBindVertexBuffers2: Option<PFN_vkCmdBindVertexBuffers2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdBlitImage2: Option<PFN_vkCmdBlitImage2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdEndRendering: Option<PFN_vkCmdEndRendering>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdResolveImage2: Option<PFN_vkCmdResolveImage2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetCullMode: Option<PFN_vkCmdSetCullMode>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetDepthBiasEnable: Option<PFN_vkCmdSetDepthBiasEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetDepthBoundsTestEnable: Option<PFN_vkCmdSetDepthBoundsTestEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetDepthCompareOp: Option<PFN_vkCmdSetDepthCompareOp>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetDepthTestEnable: Option<PFN_vkCmdSetDepthTestEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetDepthWriteEnable: Option<PFN_vkCmdSetDepthWriteEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetFrontFace: Option<PFN_vkCmdSetFrontFace>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetPrimitiveRestartEnable: Option<PFN_vkCmdSetPrimitiveRestartEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetPrimitiveTopology: Option<PFN_vkCmdSetPrimitiveTopology>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetRasterizerDiscardEnable: Option<PFN_vkCmdSetRasterizerDiscardEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetScissorWithCount: Option<PFN_vkCmdSetScissorWithCount>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetStencilOp: Option<PFN_vkCmdSetStencilOp>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetStencilTestEnable: Option<PFN_vkCmdSetStencilTestEnable>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
  pub vkCmdSetViewportWithCount: Option<PFN_vkCmdSetViewportWithCount>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  pub vkCmdBindIndexBuffer2: Option<PFN_vkCmdBindIndexBuffer2>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  pub vkCmdSetLineStipple: Option<PFN_vkCmdSetLineStipple>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  pub vkCmdSetRenderingAttachmentLocations: Option<PFN_vkCmdSetRenderingAttachmentLocations>,
  #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
  pub vkCmdSetRenderingInputAttachmentIndices: Option<PFN_vkCmdSetRenderingInputAttachmentIndices>,
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  pub vkCmdDrawClusterHUAWEI: Option<PFN_vkCmdDrawClusterHUAWEI>,
  #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
  pub vkCmdDrawClusterIndirectHUAWEI: Option<PFN_vkCmdDrawClusterIndirectHUAWEI>,
  #[cfg(feature = "VK_HUAWEI_invocation_mask")]
  pub vkCmdBindInvocationMaskHUAWEI: Option<PFN_vkCmdBindInvocationMaskHUAWEI>,
  #[cfg(feature = "VK_HUAWEI_subpass_shading")]
  pub vkCmdSubpassShadingHUAWEI: Option<PFN_vkCmdSubpassShadingHUAWEI>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkCmdSetPerformanceMarkerINTEL: Option<PFN_vkCmdSetPerformanceMarkerINTEL>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkCmdSetPerformanceOverrideINTEL: Option<PFN_vkCmdSetPerformanceOverrideINTEL>,
  #[cfg(feature = "VK_INTEL_performance_query")]
  pub vkCmdSetPerformanceStreamMarkerINTEL: Option<PFN_vkCmdSetPerformanceStreamMarkerINTEL>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdBuildAccelerationStructuresIndirectKHR:
    Option<PFN_vkCmdBuildAccelerationStructuresIndirectKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdBuildAccelerationStructuresKHR: Option<PFN_vkCmdBuildAccelerationStructuresKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdCopyAccelerationStructureKHR: Option<PFN_vkCmdCopyAccelerationStructureKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdCopyAccelerationStructureToMemoryKHR:
    Option<PFN_vkCmdCopyAccelerationStructureToMemoryKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdCopyMemoryToAccelerationStructureKHR:
    Option<PFN_vkCmdCopyMemoryToAccelerationStructureKHR>,
  #[cfg(feature = "VK_KHR_acceleration_structure")]
  pub vkCmdWriteAccelerationStructuresPropertiesKHR:
    Option<PFN_vkCmdWriteAccelerationStructuresPropertiesKHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdBlitImage2KHR: Option<PFN_vkCmdBlitImage2KHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdCopyBuffer2KHR: Option<PFN_vkCmdCopyBuffer2KHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdCopyBufferToImage2KHR: Option<PFN_vkCmdCopyBufferToImage2KHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdCopyImage2KHR: Option<PFN_vkCmdCopyImage2KHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdCopyImageToBuffer2KHR: Option<PFN_vkCmdCopyImageToBuffer2KHR>,
  #[cfg(feature = "VK_KHR_copy_commands2")]
  pub vkCmdResolveImage2KHR: Option<PFN_vkCmdResolveImage2KHR>,
  #[cfg(feature = "VK_KHR_copy_memory_indirect")]
  pub vkCmdCopyMemoryIndirectKHR: Option<PFN_vkCmdCopyMemoryIndirectKHR>,
  #[cfg(feature = "VK_KHR_copy_memory_indirect")]
  pub vkCmdCopyMemoryToImageIndirectKHR: Option<PFN_vkCmdCopyMemoryToImageIndirectKHR>,
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  pub vkCmdBeginRenderPass2KHR: Option<PFN_vkCmdBeginRenderPass2KHR>,
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  pub vkCmdEndRenderPass2KHR: Option<PFN_vkCmdEndRenderPass2KHR>,
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  pub vkCmdNextSubpass2KHR: Option<PFN_vkCmdNextSubpass2KHR>,
  #[cfg(any(
    feature = "VK_KHR_descriptor_update_template",
    feature = "VK_KHR_push_descriptor"
  ))]
  pub vkCmdPushDescriptorSetWithTemplateKHR: Option<PFN_vkCmdPushDescriptorSetWithTemplateKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdBeginConditionalRendering2EXT: Option<PFN_vkCmdBeginConditionalRendering2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdBeginTransformFeedback2EXT: Option<PFN_vkCmdBeginTransformFeedback2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdBindIndexBuffer3KHR: Option<PFN_vkCmdBindIndexBuffer3KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdBindTransformFeedbackBuffers2EXT: Option<PFN_vkCmdBindTransformFeedbackBuffers2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdBindVertexBuffers3KHR: Option<PFN_vkCmdBindVertexBuffers3KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdCopyImageToMemoryKHR: Option<PFN_vkCmdCopyImageToMemoryKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdCopyMemoryKHR: Option<PFN_vkCmdCopyMemoryKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdCopyMemoryToImageKHR: Option<PFN_vkCmdCopyMemoryToImageKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdCopyQueryPoolResultsToMemoryKHR: Option<PFN_vkCmdCopyQueryPoolResultsToMemoryKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDispatchIndirect2KHR: Option<PFN_vkCmdDispatchIndirect2KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawIndexedIndirect2KHR: Option<PFN_vkCmdDrawIndexedIndirect2KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawIndexedIndirectCount2KHR: Option<PFN_vkCmdDrawIndexedIndirectCount2KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawIndirect2KHR: Option<PFN_vkCmdDrawIndirect2KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawIndirectByteCount2EXT: Option<PFN_vkCmdDrawIndirectByteCount2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawIndirectCount2KHR: Option<PFN_vkCmdDrawIndirectCount2KHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawMeshTasksIndirect2EXT: Option<PFN_vkCmdDrawMeshTasksIndirect2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdDrawMeshTasksIndirectCount2EXT: Option<PFN_vkCmdDrawMeshTasksIndirectCount2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdEndTransformFeedback2EXT: Option<PFN_vkCmdEndTransformFeedback2EXT>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdFillMemoryKHR: Option<PFN_vkCmdFillMemoryKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdUpdateMemoryKHR: Option<PFN_vkCmdUpdateMemoryKHR>,
  #[cfg(feature = "VK_KHR_device_address_commands")]
  pub vkCmdWriteMarkerToMemoryAMD: Option<PFN_vkCmdWriteMarkerToMemoryAMD>,
  #[cfg(feature = "VK_KHR_device_group")]
  pub vkCmdDispatchBaseKHR: Option<PFN_vkCmdDispatchBaseKHR>,
  #[cfg(feature = "VK_KHR_device_group")]
  pub vkCmdSetDeviceMaskKHR: Option<PFN_vkCmdSetDeviceMaskKHR>,
  #[cfg(feature = "VK_KHR_draw_indirect_count")]
  pub vkCmdDrawIndexedIndirectCountKHR: Option<PFN_vkCmdDrawIndexedIndirectCountKHR>,
  #[cfg(feature = "VK_KHR_draw_indirect_count")]
  pub vkCmdDrawIndirectCountKHR: Option<PFN_vkCmdDrawIndirectCountKHR>,
  #[cfg(feature = "VK_KHR_dynamic_rendering")]
  pub vkCmdBeginRenderingKHR: Option<PFN_vkCmdBeginRenderingKHR>,
  #[cfg(feature = "VK_KHR_dynamic_rendering")]
  pub vkCmdEndRenderingKHR: Option<PFN_vkCmdEndRenderingKHR>,
  #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
  pub vkCmdSetRenderingAttachmentLocationsKHR: Option<PFN_vkCmdSetRenderingAttachmentLocationsKHR>,
  #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
  pub vkCmdSetRenderingInputAttachmentIndicesKHR:
    Option<PFN_vkCmdSetRenderingInputAttachmentIndicesKHR>,
  #[cfg(feature = "VK_KHR_fragment_shading_rate")]
  pub vkCmdSetFragmentShadingRateKHR: Option<PFN_vkCmdSetFragmentShadingRateKHR>,
  #[cfg(feature = "VK_KHR_line_rasterization")]
  pub vkCmdSetLineStippleKHR: Option<PFN_vkCmdSetLineStippleKHR>,
  #[cfg(feature = "VK_KHR_maintenance10")]
  pub vkCmdEndRendering2KHR: Option<PFN_vkCmdEndRendering2KHR>,
  #[cfg(feature = "VK_KHR_maintenance5")]
  pub vkCmdBindIndexBuffer2KHR: Option<PFN_vkCmdBindIndexBuffer2KHR>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdBindDescriptorBufferEmbeddedSamplers2EXT:
    Option<PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdBindDescriptorSets2KHR: Option<PFN_vkCmdBindDescriptorSets2KHR>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdPushConstants2KHR: Option<PFN_vkCmdPushConstants2KHR>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdPushDescriptorSet2KHR: Option<PFN_vkCmdPushDescriptorSet2KHR>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdPushDescriptorSetWithTemplate2KHR: Option<PFN_vkCmdPushDescriptorSetWithTemplate2KHR>,
  #[cfg(feature = "VK_KHR_maintenance6")]
  pub vkCmdSetDescriptorBufferOffsets2EXT: Option<PFN_vkCmdSetDescriptorBufferOffsets2EXT>,
  #[cfg(feature = "VK_KHR_object_refresh")]
  pub vkCmdRefreshObjectsKHR: Option<PFN_vkCmdRefreshObjectsKHR>,
  #[cfg(feature = "VK_KHR_push_descriptor")]
  pub vkCmdPushDescriptorSetKHR: Option<PFN_vkCmdPushDescriptorSetKHR>,
  #[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
  pub vkCmdTraceRaysIndirect2KHR: Option<PFN_vkCmdTraceRaysIndirect2KHR>,
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  pub vkCmdSetRayTracingPipelineStackSizeKHR: Option<PFN_vkCmdSetRayTracingPipelineStackSizeKHR>,
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  pub vkCmdTraceRaysIndirectKHR: Option<PFN_vkCmdTraceRaysIndirectKHR>,
  #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
  pub vkCmdTraceRaysKHR: Option<PFN_vkCmdTraceRaysKHR>,
  #[cfg(feature = "VK_KHR_synchronization2")]
  pub vkCmdPipelineBarrier2KHR: Option<PFN_vkCmdPipelineBarrier2KHR>,
  #[cfg(feature = "VK_KHR_synchronization2")]
  pub vkCmdResetEvent2KHR: Option<PFN_vkCmdResetEvent2KHR>,
  #[cfg(feature = "VK_KHR_synchronization2")]
  pub vkCmdSetEvent2KHR: Option<PFN_vkCmdSetEvent2KHR>,
  #[cfg(feature = "VK_KHR_synchronization2")]
  pub vkCmdWaitEvents2KHR: Option<PFN_vkCmdWaitEvents2KHR>,
  #[cfg(feature = "VK_KHR_synchronization2")]
  pub vkCmdWriteTimestamp2KHR: Option<PFN_vkCmdWriteTimestamp2KHR>,
  #[cfg(feature = "VK_KHR_video_decode_queue")]
  pub vkCmdDecodeVideoKHR: Option<PFN_vkCmdDecodeVideoKHR>,
  #[cfg(feature = "VK_KHR_video_encode_queue")]
  pub vkCmdEncodeVideoKHR: Option<PFN_vkCmdEncodeVideoKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkCmdBeginVideoCodingKHR: Option<PFN_vkCmdBeginVideoCodingKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkCmdControlVideoCodingKHR: Option<PFN_vkCmdControlVideoCodingKHR>,
  #[cfg(feature = "VK_KHR_video_queue")]
  pub vkCmdEndVideoCodingKHR: Option<PFN_vkCmdEndVideoCodingKHR>,
  #[cfg(feature = "VK_NVX_binary_import")]
  pub vkCmdCuLaunchKernelNVX: Option<PFN_vkCmdCuLaunchKernelNVX>,
  #[cfg(feature = "VK_NV_clip_space_w_scaling")]
  pub vkCmdSetViewportWScalingNV: Option<PFN_vkCmdSetViewportWScalingNV>,
  #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
  pub vkCmdBuildClusterAccelerationStructureIndirectNV:
    Option<PFN_vkCmdBuildClusterAccelerationStructureIndirectNV>,
  #[cfg(feature = "VK_NV_compute_occupancy_priority")]
  pub vkCmdSetComputeOccupancyPriorityNV: Option<PFN_vkCmdSetComputeOccupancyPriorityNV>,
  #[cfg(feature = "VK_NV_cooperative_vector")]
  pub vkCmdConvertCooperativeVectorMatrixNV: Option<PFN_vkCmdConvertCooperativeVectorMatrixNV>,
  #[cfg(feature = "VK_NV_copy_memory_indirect")]
  pub vkCmdCopyMemoryIndirectNV: Option<PFN_vkCmdCopyMemoryIndirectNV>,
  #[cfg(feature = "VK_NV_copy_memory_indirect")]
  pub vkCmdCopyMemoryToImageIndirectNV: Option<PFN_vkCmdCopyMemoryToImageIndirectNV>,
  #[cfg(feature = "VK_NV_cuda_kernel_launch")]
  pub vkCmdCudaLaunchKernelNV: Option<PFN_vkCmdCudaLaunchKernelNV>,
  #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
  pub vkCmdSetCheckpointNV: Option<PFN_vkCmdSetCheckpointNV>,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkCmdBindPipelineShaderGroupNV: Option<PFN_vkCmdBindPipelineShaderGroupNV>,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkCmdExecuteGeneratedCommandsNV: Option<PFN_vkCmdExecuteGeneratedCommandsNV>,
  #[cfg(feature = "VK_NV_device_generated_commands")]
  pub vkCmdPreprocessGeneratedCommandsNV: Option<PFN_vkCmdPreprocessGeneratedCommandsNV>,
  #[cfg(feature = "VK_NV_device_generated_commands_compute")]
  pub vkCmdUpdatePipelineIndirectBufferNV: Option<PFN_vkCmdUpdatePipelineIndirectBufferNV>,
  #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
  pub vkCmdSetFragmentShadingRateEnumNV: Option<PFN_vkCmdSetFragmentShadingRateEnumNV>,
  #[cfg(feature = "VK_NV_memory_decompression")]
  pub vkCmdDecompressMemoryIndirectCountNV: Option<PFN_vkCmdDecompressMemoryIndirectCountNV>,
  #[cfg(feature = "VK_NV_memory_decompression")]
  pub vkCmdDecompressMemoryNV: Option<PFN_vkCmdDecompressMemoryNV>,
  #[cfg(feature = "VK_NV_mesh_shader")]
  pub vkCmdDrawMeshTasksIndirectCountNV: Option<PFN_vkCmdDrawMeshTasksIndirectCountNV>,
  #[cfg(feature = "VK_NV_mesh_shader")]
  pub vkCmdDrawMeshTasksIndirectNV: Option<PFN_vkCmdDrawMeshTasksIndirectNV>,
  #[cfg(feature = "VK_NV_mesh_shader")]
  pub vkCmdDrawMeshTasksNV: Option<PFN_vkCmdDrawMeshTasksNV>,
  #[cfg(feature = "VK_NV_optical_flow")]
  pub vkCmdOpticalFlowExecuteNV: Option<PFN_vkCmdOpticalFlowExecuteNV>,
  #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
  pub vkCmdBuildPartitionedAccelerationStructuresNV:
    Option<PFN_vkCmdBuildPartitionedAccelerationStructuresNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkCmdBuildAccelerationStructureNV: Option<PFN_vkCmdBuildAccelerationStructureNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkCmdCopyAccelerationStructureNV: Option<PFN_vkCmdCopyAccelerationStructureNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkCmdTraceRaysNV: Option<PFN_vkCmdTraceRaysNV>,
  #[cfg(feature = "VK_NV_ray_tracing")]
  pub vkCmdWriteAccelerationStructuresPropertiesNV:
    Option<PFN_vkCmdWriteAccelerationStructuresPropertiesNV>,
  #[cfg(feature = "VK_NV_scissor_exclusive")]
  pub vkCmdSetExclusiveScissorEnableNV: Option<PFN_vkCmdSetExclusiveScissorEnableNV>,
  #[cfg(feature = "VK_NV_scissor_exclusive")]
  pub vkCmdSetExclusiveScissorNV: Option<PFN_vkCmdSetExclusiveScissorNV>,
  #[cfg(feature = "VK_NV_shading_rate_image")]
  pub vkCmdBindShadingRateImageNV: Option<PFN_vkCmdBindShadingRateImageNV>,
  #[cfg(feature = "VK_NV_shading_rate_image")]
  pub vkCmdSetCoarseSampleOrderNV: Option<PFN_vkCmdSetCoarseSampleOrderNV>,
  #[cfg(feature = "VK_NV_shading_rate_image")]
  pub vkCmdSetViewportShadingRatePaletteNV: Option<PFN_vkCmdSetViewportShadingRatePaletteNV>,
  #[cfg(feature = "VK_QCOM_tile_memory_heap")]
  pub vkCmdBindTileMemoryQCOM: Option<PFN_vkCmdBindTileMemoryQCOM>,
  #[cfg(feature = "VK_QCOM_tile_shading")]
  pub vkCmdBeginPerTileExecutionQCOM: Option<PFN_vkCmdBeginPerTileExecutionQCOM>,
  #[cfg(feature = "VK_QCOM_tile_shading")]
  pub vkCmdDispatchTileQCOM: Option<PFN_vkCmdDispatchTileQCOM>,
  #[cfg(feature = "VK_QCOM_tile_shading")]
  pub vkCmdEndPerTileExecutionQCOM: Option<PFN_vkCmdEndPerTileExecutionQCOM>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl CommandBufferDispatchTable {
  pub const EMPTY: Self = Self {
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    vkCmdDispatchGraphAMDX: None,
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    vkCmdDispatchGraphIndirectAMDX: None,
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    vkCmdDispatchGraphIndirectCountAMDX: None,
    #[cfg(feature = "VK_AMDX_shader_enqueue")]
    vkCmdInitializeGraphScratchMemoryAMDX: None,
    #[cfg(feature = "VK_AMD_buffer_marker")]
    vkCmdWriteBufferMarker2AMD: None,
    #[cfg(feature = "VK_AMD_buffer_marker")]
    vkCmdWriteBufferMarkerAMD: None,
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    vkCmdDrawIndexedIndirectCountAMD: None,
    #[cfg(feature = "VK_AMD_draw_indirect_count")]
    vkCmdDrawIndirectCountAMD: None,
    #[cfg(feature = "VK_ARM_data_graph")]
    vkCmdDispatchDataGraphARM: None,
    #[cfg(feature = "VK_ARM_scheduling_controls")]
    vkCmdSetDispatchParametersARM: None,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    vkCmdBeginShaderInstrumentationARM: None,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    vkCmdEndShaderInstrumentationARM: None,
    #[cfg(feature = "VK_ARM_tensors")]
    vkCmdCopyTensorARM: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkBeginCommandBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdBeginQuery: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdCopyBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdCopyBufferToImage: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdCopyImage: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdCopyImageToBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdCopyQueryPoolResults: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdEndQuery: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdExecuteCommands: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdFillBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdPipelineBarrier: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdResetQueryPool: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdUpdateBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkCmdWriteTimestamp: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkEndCommandBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    vkResetCommandBuffer: None,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    vkCmdSetDeviceMask: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdCopyBuffer2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdCopyBufferToImage2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdCopyImage2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdCopyImageToBuffer2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdPipelineBarrier2: None,
    #[cfg(feature = "VK_BASE_VERSION_1_3")]
    vkCmdWriteTimestamp2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdBindDescriptorSets: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdBindPipeline: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdClearColorImage: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdDispatch: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdDispatchIndirect: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdPushConstants: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdResetEvent: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdSetEvent: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    vkCmdWaitEvents: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
    vkCmdDispatchBase: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
    vkCmdResetEvent2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
    vkCmdSetEvent2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
    vkCmdWaitEvents2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdBindDescriptorSets2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdPushConstants2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdPushDescriptorSet: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdPushDescriptorSet2: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdPushDescriptorSetWithTemplate: None,
    #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
    vkCmdPushDescriptorSetWithTemplate2: None,
    #[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
    vkCmdSetAttachmentFeedbackLoopEnableEXT: None,
    #[cfg(feature = "VK_EXT_color_write_enable")]
    vkCmdSetColorWriteEnableEXT: None,
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    vkCmdBeginConditionalRenderingEXT: None,
    #[cfg(feature = "VK_EXT_conditional_rendering")]
    vkCmdEndConditionalRenderingEXT: None,
    #[cfg(feature = "VK_EXT_custom_resolve")]
    vkCmdBeginCustomResolveEXT: None,
    #[cfg(feature = "VK_EXT_debug_marker")]
    vkCmdDebugMarkerBeginEXT: None,
    #[cfg(feature = "VK_EXT_debug_marker")]
    vkCmdDebugMarkerEndEXT: None,
    #[cfg(feature = "VK_EXT_debug_marker")]
    vkCmdDebugMarkerInsertEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkCmdBeginDebugUtilsLabelEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkCmdEndDebugUtilsLabelEXT: None,
    #[cfg(feature = "VK_EXT_debug_utils")]
    vkCmdInsertDebugUtilsLabelEXT: None,
    #[cfg(feature = "VK_EXT_depth_bias_control")]
    vkCmdSetDepthBias2EXT: None,
    #[cfg(any(
      feature = "VK_EXT_depth_clamp_control",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthClampRangeEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkCmdBindDescriptorBufferEmbeddedSamplersEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkCmdBindDescriptorBuffersEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_buffer")]
    vkCmdSetDescriptorBufferOffsetsEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkCmdBindResourceHeapEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkCmdBindSamplerHeapEXT: None,
    #[cfg(feature = "VK_EXT_descriptor_heap")]
    vkCmdPushDataEXT: None,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkCmdExecuteGeneratedCommandsEXT: None,
    #[cfg(feature = "VK_EXT_device_generated_commands")]
    vkCmdPreprocessGeneratedCommandsEXT: None,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    vkCmdSetDiscardRectangleEXT: None,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    vkCmdSetDiscardRectangleEnableEXT: None,
    #[cfg(feature = "VK_EXT_discard_rectangles")]
    vkCmdSetDiscardRectangleModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdBindVertexBuffers2EXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCullModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthBoundsTestEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthCompareOpEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthTestEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthWriteEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetFrontFaceEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetPrimitiveTopologyEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetScissorWithCountEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetStencilOpEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetStencilTestEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetViewportWithCountEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state2",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthBiasEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state2",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetLogicOpEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state2",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetPatchControlPointsEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state2",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetPrimitiveRestartEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state2",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetRasterizerDiscardEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetAlphaToCoverageEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetAlphaToOneEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetColorBlendAdvancedEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetColorBlendEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetColorBlendEquationEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetColorWriteMaskEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetConservativeRasterizationModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageModulationModeNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageModulationTableEnableNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageModulationTableNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageReductionModeNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageToColorEnableNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetCoverageToColorLocationNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthClampEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthClipEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetDepthClipNegativeOneToOneEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetExtraPrimitiveOverestimationSizeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetLineRasterizationModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetLineStippleEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetLogicOpEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetPolygonModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetProvokingVertexModeEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetRasterizationSamplesEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetRasterizationStreamEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetRepresentativeFragmentTestEnableNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetSampleLocationsEnableEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetSampleMaskEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetShadingRateImageEnableNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetTessellationDomainOriginEXT: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetViewportSwizzleNV: None,
    #[cfg(any(
      feature = "VK_EXT_extended_dynamic_state3",
      feature = "VK_EXT_shader_object"
    ))]
    vkCmdSetViewportWScalingEnableNV: None,
    #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
    vkCmdEndRendering2EXT: None,
    #[cfg(feature = "VK_EXT_line_rasterization")]
    vkCmdSetLineStippleEXT: None,
    #[cfg(feature = "VK_EXT_memory_decompression")]
    vkCmdDecompressMemoryEXT: None,
    #[cfg(feature = "VK_EXT_memory_decompression")]
    vkCmdDecompressMemoryIndirectCountEXT: None,
    #[cfg(feature = "VK_EXT_mesh_shader")]
    vkCmdDrawMeshTasksEXT: None,
    #[cfg(feature = "VK_EXT_mesh_shader")]
    vkCmdDrawMeshTasksIndirectCountEXT: None,
    #[cfg(feature = "VK_EXT_mesh_shader")]
    vkCmdDrawMeshTasksIndirectEXT: None,
    #[cfg(feature = "VK_EXT_multi_draw")]
    vkCmdDrawMultiEXT: None,
    #[cfg(feature = "VK_EXT_multi_draw")]
    vkCmdDrawMultiIndexedEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCmdBuildMicromapsEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCmdCopyMemoryToMicromapEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCmdCopyMicromapEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCmdCopyMicromapToMemoryEXT: None,
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    vkCmdWriteMicromapsPropertiesEXT: None,
    #[cfg(feature = "VK_EXT_primitive_restart_index")]
    vkCmdSetPrimitiveRestartIndexEXT: None,
    #[cfg(feature = "VK_EXT_sample_locations")]
    vkCmdSetSampleLocationsEXT: None,
    #[cfg(feature = "VK_EXT_shader_object")]
    vkCmdBindShadersEXT: None,
    #[cfg(any(
      feature = "VK_EXT_shader_object",
      feature = "VK_EXT_vertex_input_dynamic_state"
    ))]
    vkCmdSetVertexInputEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdBeginQueryIndexedEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdBeginTransformFeedbackEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdBindTransformFeedbackBuffersEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdDrawIndirectByteCountEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdEndQueryIndexedEXT: None,
    #[cfg(feature = "VK_EXT_transform_feedback")]
    vkCmdEndTransformFeedbackEXT: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdBeginRenderPass: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdBindIndexBuffer: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdBindVertexBuffers: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdBlitImage: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdClearAttachments: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdClearDepthStencilImage: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdDraw: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdDrawIndexed: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdDrawIndexedIndirect: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdDrawIndirect: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdEndRenderPass: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdNextSubpass: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdResolveImage: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetBlendConstants: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetDepthBias: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetDepthBounds: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetLineWidth: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetScissor: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetStencilCompareMask: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetStencilReference: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetStencilWriteMask: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    vkCmdSetViewport: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCmdBeginRenderPass2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCmdDrawIndexedIndirectCount: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCmdDrawIndirectCount: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCmdEndRenderPass2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
    vkCmdNextSubpass2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdBeginRendering: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdBindVertexBuffers2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdBlitImage2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdEndRendering: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdResolveImage2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetCullMode: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetDepthBiasEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetDepthBoundsTestEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetDepthCompareOp: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetDepthTestEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetDepthWriteEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetFrontFace: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetPrimitiveRestartEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetPrimitiveTopology: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetRasterizerDiscardEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetScissorWithCount: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetStencilOp: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetStencilTestEnable: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
    vkCmdSetViewportWithCount: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
    vkCmdBindIndexBuffer2: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
    vkCmdSetLineStipple: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
    vkCmdSetRenderingAttachmentLocations: None,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
    vkCmdSetRenderingInputAttachmentIndices: None,
    #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
    vkCmdDrawClusterHUAWEI: None,
    #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
    vkCmdDrawClusterIndirectHUAWEI: None,
    #[cfg(feature = "VK_HUAWEI_invocation_mask")]
    vkCmdBindInvocationMaskHUAWEI: None,
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    vkCmdSubpassShadingHUAWEI: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkCmdSetPerformanceMarkerINTEL: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkCmdSetPerformanceOverrideINTEL: None,
    #[cfg(feature = "VK_INTEL_performance_query")]
    vkCmdSetPerformanceStreamMarkerINTEL: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdBuildAccelerationStructuresIndirectKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdBuildAccelerationStructuresKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdCopyAccelerationStructureKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdCopyAccelerationStructureToMemoryKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdCopyMemoryToAccelerationStructureKHR: None,
    #[cfg(feature = "VK_KHR_acceleration_structure")]
    vkCmdWriteAccelerationStructuresPropertiesKHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdBlitImage2KHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdCopyBuffer2KHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdCopyBufferToImage2KHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdCopyImage2KHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdCopyImageToBuffer2KHR: None,
    #[cfg(feature = "VK_KHR_copy_commands2")]
    vkCmdResolveImage2KHR: None,
    #[cfg(feature = "VK_KHR_copy_memory_indirect")]
    vkCmdCopyMemoryIndirectKHR: None,
    #[cfg(feature = "VK_KHR_copy_memory_indirect")]
    vkCmdCopyMemoryToImageIndirectKHR: None,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    vkCmdBeginRenderPass2KHR: None,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    vkCmdEndRenderPass2KHR: None,
    #[cfg(feature = "VK_KHR_create_renderpass2")]
    vkCmdNextSubpass2KHR: None,
    #[cfg(any(
      feature = "VK_KHR_descriptor_update_template",
      feature = "VK_KHR_push_descriptor"
    ))]
    vkCmdPushDescriptorSetWithTemplateKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdBeginConditionalRendering2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdBeginTransformFeedback2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdBindIndexBuffer3KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdBindTransformFeedbackBuffers2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdBindVertexBuffers3KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdCopyImageToMemoryKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdCopyMemoryKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdCopyMemoryToImageKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdCopyQueryPoolResultsToMemoryKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDispatchIndirect2KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawIndexedIndirect2KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawIndexedIndirectCount2KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawIndirect2KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawIndirectByteCount2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawIndirectCount2KHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawMeshTasksIndirect2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdDrawMeshTasksIndirectCount2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdEndTransformFeedback2EXT: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdFillMemoryKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdUpdateMemoryKHR: None,
    #[cfg(feature = "VK_KHR_device_address_commands")]
    vkCmdWriteMarkerToMemoryAMD: None,
    #[cfg(feature = "VK_KHR_device_group")]
    vkCmdDispatchBaseKHR: None,
    #[cfg(feature = "VK_KHR_device_group")]
    vkCmdSetDeviceMaskKHR: None,
    #[cfg(feature = "VK_KHR_draw_indirect_count")]
    vkCmdDrawIndexedIndirectCountKHR: None,
    #[cfg(feature = "VK_KHR_draw_indirect_count")]
    vkCmdDrawIndirectCountKHR: None,
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    vkCmdBeginRenderingKHR: None,
    #[cfg(feature = "VK_KHR_dynamic_rendering")]
    vkCmdEndRenderingKHR: None,
    #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
    vkCmdSetRenderingAttachmentLocationsKHR: None,
    #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
    vkCmdSetRenderingInputAttachmentIndicesKHR: None,
    #[cfg(feature = "VK_KHR_fragment_shading_rate")]
    vkCmdSetFragmentShadingRateKHR: None,
    #[cfg(feature = "VK_KHR_line_rasterization")]
    vkCmdSetLineStippleKHR: None,
    #[cfg(feature = "VK_KHR_maintenance10")]
    vkCmdEndRendering2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance5")]
    vkCmdBindIndexBuffer2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdBindDescriptorBufferEmbeddedSamplers2EXT: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdBindDescriptorSets2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdPushConstants2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdPushDescriptorSet2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdPushDescriptorSetWithTemplate2KHR: None,
    #[cfg(feature = "VK_KHR_maintenance6")]
    vkCmdSetDescriptorBufferOffsets2EXT: None,
    #[cfg(feature = "VK_KHR_object_refresh")]
    vkCmdRefreshObjectsKHR: None,
    #[cfg(feature = "VK_KHR_push_descriptor")]
    vkCmdPushDescriptorSetKHR: None,
    #[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
    vkCmdTraceRaysIndirect2KHR: None,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    vkCmdSetRayTracingPipelineStackSizeKHR: None,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    vkCmdTraceRaysIndirectKHR: None,
    #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
    vkCmdTraceRaysKHR: None,
    #[cfg(feature = "VK_KHR_synchronization2")]
    vkCmdPipelineBarrier2KHR: None,
    #[cfg(feature = "VK_KHR_synchronization2")]
    vkCmdResetEvent2KHR: None,
    #[cfg(feature = "VK_KHR_synchronization2")]
    vkCmdSetEvent2KHR: None,
    #[cfg(feature = "VK_KHR_synchronization2")]
    vkCmdWaitEvents2KHR: None,
    #[cfg(feature = "VK_KHR_synchronization2")]
    vkCmdWriteTimestamp2KHR: None,
    #[cfg(feature = "VK_KHR_video_decode_queue")]
    vkCmdDecodeVideoKHR: None,
    #[cfg(feature = "VK_KHR_video_encode_queue")]
    vkCmdEncodeVideoKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkCmdBeginVideoCodingKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkCmdControlVideoCodingKHR: None,
    #[cfg(feature = "VK_KHR_video_queue")]
    vkCmdEndVideoCodingKHR: None,
    #[cfg(feature = "VK_NVX_binary_import")]
    vkCmdCuLaunchKernelNVX: None,
    #[cfg(feature = "VK_NV_clip_space_w_scaling")]
    vkCmdSetViewportWScalingNV: None,
    #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
    vkCmdBuildClusterAccelerationStructureIndirectNV: None,
    #[cfg(feature = "VK_NV_compute_occupancy_priority")]
    vkCmdSetComputeOccupancyPriorityNV: None,
    #[cfg(feature = "VK_NV_cooperative_vector")]
    vkCmdConvertCooperativeVectorMatrixNV: None,
    #[cfg(feature = "VK_NV_copy_memory_indirect")]
    vkCmdCopyMemoryIndirectNV: None,
    #[cfg(feature = "VK_NV_copy_memory_indirect")]
    vkCmdCopyMemoryToImageIndirectNV: None,
    #[cfg(feature = "VK_NV_cuda_kernel_launch")]
    vkCmdCudaLaunchKernelNV: None,
    #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
    vkCmdSetCheckpointNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkCmdBindPipelineShaderGroupNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkCmdExecuteGeneratedCommandsNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands")]
    vkCmdPreprocessGeneratedCommandsNV: None,
    #[cfg(feature = "VK_NV_device_generated_commands_compute")]
    vkCmdUpdatePipelineIndirectBufferNV: None,
    #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
    vkCmdSetFragmentShadingRateEnumNV: None,
    #[cfg(feature = "VK_NV_memory_decompression")]
    vkCmdDecompressMemoryIndirectCountNV: None,
    #[cfg(feature = "VK_NV_memory_decompression")]
    vkCmdDecompressMemoryNV: None,
    #[cfg(feature = "VK_NV_mesh_shader")]
    vkCmdDrawMeshTasksIndirectCountNV: None,
    #[cfg(feature = "VK_NV_mesh_shader")]
    vkCmdDrawMeshTasksIndirectNV: None,
    #[cfg(feature = "VK_NV_mesh_shader")]
    vkCmdDrawMeshTasksNV: None,
    #[cfg(feature = "VK_NV_optical_flow")]
    vkCmdOpticalFlowExecuteNV: None,
    #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
    vkCmdBuildPartitionedAccelerationStructuresNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkCmdBuildAccelerationStructureNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkCmdCopyAccelerationStructureNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkCmdTraceRaysNV: None,
    #[cfg(feature = "VK_NV_ray_tracing")]
    vkCmdWriteAccelerationStructuresPropertiesNV: None,
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    vkCmdSetExclusiveScissorEnableNV: None,
    #[cfg(feature = "VK_NV_scissor_exclusive")]
    vkCmdSetExclusiveScissorNV: None,
    #[cfg(feature = "VK_NV_shading_rate_image")]
    vkCmdBindShadingRateImageNV: None,
    #[cfg(feature = "VK_NV_shading_rate_image")]
    vkCmdSetCoarseSampleOrderNV: None,
    #[cfg(feature = "VK_NV_shading_rate_image")]
    vkCmdSetViewportShadingRatePaletteNV: None,
    #[cfg(feature = "VK_QCOM_tile_memory_heap")]
    vkCmdBindTileMemoryQCOM: None,
    #[cfg(feature = "VK_QCOM_tile_shading")]
    vkCmdBeginPerTileExecutionQCOM: None,
    #[cfg(feature = "VK_QCOM_tile_shading")]
    vkCmdDispatchTileQCOM: None,
    #[cfg(feature = "VK_QCOM_tile_shading")]
    vkCmdEndPerTileExecutionQCOM: None,
  };
  pub fn load<F>(loader: F) -> Self
  where
    F: Fn(*const c_char) -> Option<unsafe extern "system" fn()>,
  {
    Self {
      #[cfg(feature = "VK_AMDX_shader_enqueue")]
      vkCmdDispatchGraphAMDX: loader(c"vkCmdDispatchGraphAMDX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMDX_shader_enqueue")]
      vkCmdDispatchGraphIndirectAMDX: loader(c"vkCmdDispatchGraphIndirectAMDX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMDX_shader_enqueue")]
      vkCmdDispatchGraphIndirectCountAMDX: loader(c"vkCmdDispatchGraphIndirectCountAMDX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMDX_shader_enqueue")]
      vkCmdInitializeGraphScratchMemoryAMDX: loader(
        c"vkCmdInitializeGraphScratchMemoryAMDX".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_buffer_marker")]
      vkCmdWriteBufferMarker2AMD: loader(c"vkCmdWriteBufferMarker2AMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_buffer_marker")]
      vkCmdWriteBufferMarkerAMD: loader(c"vkCmdWriteBufferMarkerAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_draw_indirect_count")]
      vkCmdDrawIndexedIndirectCountAMD: loader(c"vkCmdDrawIndexedIndirectCountAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_AMD_draw_indirect_count")]
      vkCmdDrawIndirectCountAMD: loader(c"vkCmdDrawIndirectCountAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_data_graph")]
      vkCmdDispatchDataGraphARM: loader(c"vkCmdDispatchDataGraphARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_scheduling_controls")]
      vkCmdSetDispatchParametersARM: loader(c"vkCmdSetDispatchParametersARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      vkCmdBeginShaderInstrumentationARM: loader(c"vkCmdBeginShaderInstrumentationARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_shader_instrumentation")]
      vkCmdEndShaderInstrumentationARM: loader(c"vkCmdEndShaderInstrumentationARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_ARM_tensors")]
      vkCmdCopyTensorARM: loader(c"vkCmdCopyTensorARM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkBeginCommandBuffer: loader(c"vkBeginCommandBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdBeginQuery: loader(c"vkCmdBeginQuery".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdCopyBuffer: loader(c"vkCmdCopyBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdCopyBufferToImage: loader(c"vkCmdCopyBufferToImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdCopyImage: loader(c"vkCmdCopyImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdCopyImageToBuffer: loader(c"vkCmdCopyImageToBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdCopyQueryPoolResults: loader(c"vkCmdCopyQueryPoolResults".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdEndQuery: loader(c"vkCmdEndQuery".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdExecuteCommands: loader(c"vkCmdExecuteCommands".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdFillBuffer: loader(c"vkCmdFillBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdPipelineBarrier: loader(c"vkCmdPipelineBarrier".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdResetQueryPool: loader(c"vkCmdResetQueryPool".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdUpdateBuffer: loader(c"vkCmdUpdateBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkCmdWriteTimestamp: loader(c"vkCmdWriteTimestamp".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkEndCommandBuffer: loader(c"vkEndCommandBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_0")]
      vkResetCommandBuffer: loader(c"vkResetCommandBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_1")]
      vkCmdSetDeviceMask: loader(c"vkCmdSetDeviceMask".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdCopyBuffer2: loader(c"vkCmdCopyBuffer2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdCopyBufferToImage2: loader(c"vkCmdCopyBufferToImage2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdCopyImage2: loader(c"vkCmdCopyImage2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdCopyImageToBuffer2: loader(c"vkCmdCopyImageToBuffer2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdPipelineBarrier2: loader(c"vkCmdPipelineBarrier2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_BASE_VERSION_1_3")]
      vkCmdWriteTimestamp2: loader(c"vkCmdWriteTimestamp2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdBindDescriptorSets: loader(c"vkCmdBindDescriptorSets".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdBindPipeline: loader(c"vkCmdBindPipeline".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdClearColorImage: loader(c"vkCmdClearColorImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdDispatch: loader(c"vkCmdDispatch".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdDispatchIndirect: loader(c"vkCmdDispatchIndirect".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdPushConstants: loader(c"vkCmdPushConstants".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdResetEvent: loader(c"vkCmdResetEvent".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdSetEvent: loader(c"vkCmdSetEvent".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
      vkCmdWaitEvents: loader(c"vkCmdWaitEvents".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_1")]
      vkCmdDispatchBase: loader(c"vkCmdDispatchBase".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
      vkCmdResetEvent2: loader(c"vkCmdResetEvent2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
      vkCmdSetEvent2: loader(c"vkCmdSetEvent2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_3")]
      vkCmdWaitEvents2: loader(c"vkCmdWaitEvents2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdBindDescriptorSets2: loader(c"vkCmdBindDescriptorSets2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdPushConstants2: loader(c"vkCmdPushConstants2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdPushDescriptorSet: loader(c"vkCmdPushDescriptorSet".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdPushDescriptorSet2: loader(c"vkCmdPushDescriptorSet2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdPushDescriptorSetWithTemplate: loader(c"vkCmdPushDescriptorSetWithTemplate".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_COMPUTE_VERSION_1_4")]
      vkCmdPushDescriptorSetWithTemplate2: loader(c"vkCmdPushDescriptorSetWithTemplate2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
      vkCmdSetAttachmentFeedbackLoopEnableEXT: loader(
        c"vkCmdSetAttachmentFeedbackLoopEnableEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_color_write_enable")]
      vkCmdSetColorWriteEnableEXT: loader(c"vkCmdSetColorWriteEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_conditional_rendering")]
      vkCmdBeginConditionalRenderingEXT: loader(c"vkCmdBeginConditionalRenderingEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_conditional_rendering")]
      vkCmdEndConditionalRenderingEXT: loader(c"vkCmdEndConditionalRenderingEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_custom_resolve")]
      vkCmdBeginCustomResolveEXT: loader(c"vkCmdBeginCustomResolveEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_marker")]
      vkCmdDebugMarkerBeginEXT: loader(c"vkCmdDebugMarkerBeginEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_marker")]
      vkCmdDebugMarkerEndEXT: loader(c"vkCmdDebugMarkerEndEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_marker")]
      vkCmdDebugMarkerInsertEXT: loader(c"vkCmdDebugMarkerInsertEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkCmdBeginDebugUtilsLabelEXT: loader(c"vkCmdBeginDebugUtilsLabelEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkCmdEndDebugUtilsLabelEXT: loader(c"vkCmdEndDebugUtilsLabelEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_debug_utils")]
      vkCmdInsertDebugUtilsLabelEXT: loader(c"vkCmdInsertDebugUtilsLabelEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_depth_bias_control")]
      vkCmdSetDepthBias2EXT: loader(c"vkCmdSetDepthBias2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_depth_clamp_control",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthClampRangeEXT: loader(c"vkCmdSetDepthClampRangeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkCmdBindDescriptorBufferEmbeddedSamplersEXT: loader(
        c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkCmdBindDescriptorBuffersEXT: loader(c"vkCmdBindDescriptorBuffersEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_buffer")]
      vkCmdSetDescriptorBufferOffsetsEXT: loader(c"vkCmdSetDescriptorBufferOffsetsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkCmdBindResourceHeapEXT: loader(c"vkCmdBindResourceHeapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkCmdBindSamplerHeapEXT: loader(c"vkCmdBindSamplerHeapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_descriptor_heap")]
      vkCmdPushDataEXT: loader(c"vkCmdPushDataEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkCmdExecuteGeneratedCommandsEXT: loader(c"vkCmdExecuteGeneratedCommandsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_device_generated_commands")]
      vkCmdPreprocessGeneratedCommandsEXT: loader(c"vkCmdPreprocessGeneratedCommandsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_discard_rectangles")]
      vkCmdSetDiscardRectangleEXT: loader(c"vkCmdSetDiscardRectangleEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_discard_rectangles")]
      vkCmdSetDiscardRectangleEnableEXT: loader(c"vkCmdSetDiscardRectangleEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_discard_rectangles")]
      vkCmdSetDiscardRectangleModeEXT: loader(c"vkCmdSetDiscardRectangleModeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdBindVertexBuffers2EXT: loader(c"vkCmdBindVertexBuffers2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCullModeEXT: loader(c"vkCmdSetCullModeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthBoundsTestEnableEXT: loader(c"vkCmdSetDepthBoundsTestEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthCompareOpEXT: loader(c"vkCmdSetDepthCompareOpEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthTestEnableEXT: loader(c"vkCmdSetDepthTestEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthWriteEnableEXT: loader(c"vkCmdSetDepthWriteEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetFrontFaceEXT: loader(c"vkCmdSetFrontFaceEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetPrimitiveTopologyEXT: loader(c"vkCmdSetPrimitiveTopologyEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetScissorWithCountEXT: loader(c"vkCmdSetScissorWithCountEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetStencilOpEXT: loader(c"vkCmdSetStencilOpEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetStencilTestEnableEXT: loader(c"vkCmdSetStencilTestEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetViewportWithCountEXT: loader(c"vkCmdSetViewportWithCountEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state2",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthBiasEnableEXT: loader(c"vkCmdSetDepthBiasEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state2",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetLogicOpEXT: loader(c"vkCmdSetLogicOpEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state2",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetPatchControlPointsEXT: loader(c"vkCmdSetPatchControlPointsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state2",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetPrimitiveRestartEnableEXT: loader(c"vkCmdSetPrimitiveRestartEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state2",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetRasterizerDiscardEnableEXT: loader(c"vkCmdSetRasterizerDiscardEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetAlphaToCoverageEnableEXT: loader(c"vkCmdSetAlphaToCoverageEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetAlphaToOneEnableEXT: loader(c"vkCmdSetAlphaToOneEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetColorBlendAdvancedEXT: loader(c"vkCmdSetColorBlendAdvancedEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetColorBlendEnableEXT: loader(c"vkCmdSetColorBlendEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetColorBlendEquationEXT: loader(c"vkCmdSetColorBlendEquationEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetColorWriteMaskEXT: loader(c"vkCmdSetColorWriteMaskEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetConservativeRasterizationModeEXT: loader(
        c"vkCmdSetConservativeRasterizationModeEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageModulationModeNV: loader(c"vkCmdSetCoverageModulationModeNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageModulationTableEnableNV: loader(
        c"vkCmdSetCoverageModulationTableEnableNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageModulationTableNV: loader(c"vkCmdSetCoverageModulationTableNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageReductionModeNV: loader(c"vkCmdSetCoverageReductionModeNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageToColorEnableNV: loader(c"vkCmdSetCoverageToColorEnableNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetCoverageToColorLocationNV: loader(c"vkCmdSetCoverageToColorLocationNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthClampEnableEXT: loader(c"vkCmdSetDepthClampEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthClipEnableEXT: loader(c"vkCmdSetDepthClipEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetDepthClipNegativeOneToOneEXT: loader(
        c"vkCmdSetDepthClipNegativeOneToOneEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetExtraPrimitiveOverestimationSizeEXT: loader(
        c"vkCmdSetExtraPrimitiveOverestimationSizeEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetLineRasterizationModeEXT: loader(c"vkCmdSetLineRasterizationModeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetLineStippleEnableEXT: loader(c"vkCmdSetLineStippleEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetLogicOpEnableEXT: loader(c"vkCmdSetLogicOpEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetPolygonModeEXT: loader(c"vkCmdSetPolygonModeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetProvokingVertexModeEXT: loader(c"vkCmdSetProvokingVertexModeEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetRasterizationSamplesEXT: loader(c"vkCmdSetRasterizationSamplesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetRasterizationStreamEXT: loader(c"vkCmdSetRasterizationStreamEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetRepresentativeFragmentTestEnableNV: loader(
        c"vkCmdSetRepresentativeFragmentTestEnableNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetSampleLocationsEnableEXT: loader(c"vkCmdSetSampleLocationsEnableEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetSampleMaskEXT: loader(c"vkCmdSetSampleMaskEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetShadingRateImageEnableNV: loader(c"vkCmdSetShadingRateImageEnableNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetTessellationDomainOriginEXT: loader(c"vkCmdSetTessellationDomainOriginEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetViewportSwizzleNV: loader(c"vkCmdSetViewportSwizzleNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_extended_dynamic_state3",
        feature = "VK_EXT_shader_object"
      ))]
      vkCmdSetViewportWScalingEnableNV: loader(c"vkCmdSetViewportWScalingEnableNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
      vkCmdEndRendering2EXT: loader(c"vkCmdEndRendering2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_line_rasterization")]
      vkCmdSetLineStippleEXT: loader(c"vkCmdSetLineStippleEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_memory_decompression")]
      vkCmdDecompressMemoryEXT: loader(c"vkCmdDecompressMemoryEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_memory_decompression")]
      vkCmdDecompressMemoryIndirectCountEXT: loader(
        c"vkCmdDecompressMemoryIndirectCountEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_mesh_shader")]
      vkCmdDrawMeshTasksEXT: loader(c"vkCmdDrawMeshTasksEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_mesh_shader")]
      vkCmdDrawMeshTasksIndirectCountEXT: loader(c"vkCmdDrawMeshTasksIndirectCountEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_mesh_shader")]
      vkCmdDrawMeshTasksIndirectEXT: loader(c"vkCmdDrawMeshTasksIndirectEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_multi_draw")]
      vkCmdDrawMultiEXT: loader(c"vkCmdDrawMultiEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_multi_draw")]
      vkCmdDrawMultiIndexedEXT: loader(c"vkCmdDrawMultiIndexedEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCmdBuildMicromapsEXT: loader(c"vkCmdBuildMicromapsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCmdCopyMemoryToMicromapEXT: loader(c"vkCmdCopyMemoryToMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCmdCopyMicromapEXT: loader(c"vkCmdCopyMicromapEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCmdCopyMicromapToMemoryEXT: loader(c"vkCmdCopyMicromapToMemoryEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_opacity_micromap")]
      vkCmdWriteMicromapsPropertiesEXT: loader(c"vkCmdWriteMicromapsPropertiesEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_primitive_restart_index")]
      vkCmdSetPrimitiveRestartIndexEXT: loader(c"vkCmdSetPrimitiveRestartIndexEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_sample_locations")]
      vkCmdSetSampleLocationsEXT: loader(c"vkCmdSetSampleLocationsEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_shader_object")]
      vkCmdBindShadersEXT: loader(c"vkCmdBindShadersEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_EXT_shader_object",
        feature = "VK_EXT_vertex_input_dynamic_state"
      ))]
      vkCmdSetVertexInputEXT: loader(c"vkCmdSetVertexInputEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdBeginQueryIndexedEXT: loader(c"vkCmdBeginQueryIndexedEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdBeginTransformFeedbackEXT: loader(c"vkCmdBeginTransformFeedbackEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdBindTransformFeedbackBuffersEXT: loader(
        c"vkCmdBindTransformFeedbackBuffersEXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdDrawIndirectByteCountEXT: loader(c"vkCmdDrawIndirectByteCountEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdEndQueryIndexedEXT: loader(c"vkCmdEndQueryIndexedEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_EXT_transform_feedback")]
      vkCmdEndTransformFeedbackEXT: loader(c"vkCmdEndTransformFeedbackEXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdBeginRenderPass: loader(c"vkCmdBeginRenderPass".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdBindIndexBuffer: loader(c"vkCmdBindIndexBuffer".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdBindVertexBuffers: loader(c"vkCmdBindVertexBuffers".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdBlitImage: loader(c"vkCmdBlitImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdClearAttachments: loader(c"vkCmdClearAttachments".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdClearDepthStencilImage: loader(c"vkCmdClearDepthStencilImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdDraw: loader(c"vkCmdDraw".as_ptr()).map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdDrawIndexed: loader(c"vkCmdDrawIndexed".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdDrawIndexedIndirect: loader(c"vkCmdDrawIndexedIndirect".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdDrawIndirect: loader(c"vkCmdDrawIndirect".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdEndRenderPass: loader(c"vkCmdEndRenderPass".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdNextSubpass: loader(c"vkCmdNextSubpass".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdResolveImage: loader(c"vkCmdResolveImage".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetBlendConstants: loader(c"vkCmdSetBlendConstants".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetDepthBias: loader(c"vkCmdSetDepthBias".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetDepthBounds: loader(c"vkCmdSetDepthBounds".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetLineWidth: loader(c"vkCmdSetLineWidth".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetScissor: loader(c"vkCmdSetScissor".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetStencilCompareMask: loader(c"vkCmdSetStencilCompareMask".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetStencilReference: loader(c"vkCmdSetStencilReference".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetStencilWriteMask: loader(c"vkCmdSetStencilWriteMask".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
      vkCmdSetViewport: loader(c"vkCmdSetViewport".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCmdBeginRenderPass2: loader(c"vkCmdBeginRenderPass2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCmdDrawIndexedIndirectCount: loader(c"vkCmdDrawIndexedIndirectCount".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCmdDrawIndirectCount: loader(c"vkCmdDrawIndirectCount".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCmdEndRenderPass2: loader(c"vkCmdEndRenderPass2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_2")]
      vkCmdNextSubpass2: loader(c"vkCmdNextSubpass2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdBeginRendering: loader(c"vkCmdBeginRendering".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdBindVertexBuffers2: loader(c"vkCmdBindVertexBuffers2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdBlitImage2: loader(c"vkCmdBlitImage2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdEndRendering: loader(c"vkCmdEndRendering".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdResolveImage2: loader(c"vkCmdResolveImage2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetCullMode: loader(c"vkCmdSetCullMode".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetDepthBiasEnable: loader(c"vkCmdSetDepthBiasEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetDepthBoundsTestEnable: loader(c"vkCmdSetDepthBoundsTestEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetDepthCompareOp: loader(c"vkCmdSetDepthCompareOp".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetDepthTestEnable: loader(c"vkCmdSetDepthTestEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetDepthWriteEnable: loader(c"vkCmdSetDepthWriteEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetFrontFace: loader(c"vkCmdSetFrontFace".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetPrimitiveRestartEnable: loader(c"vkCmdSetPrimitiveRestartEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetPrimitiveTopology: loader(c"vkCmdSetPrimitiveTopology".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetRasterizerDiscardEnable: loader(c"vkCmdSetRasterizerDiscardEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetScissorWithCount: loader(c"vkCmdSetScissorWithCount".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetStencilOp: loader(c"vkCmdSetStencilOp".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetStencilTestEnable: loader(c"vkCmdSetStencilTestEnable".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_3")]
      vkCmdSetViewportWithCount: loader(c"vkCmdSetViewportWithCount".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
      vkCmdBindIndexBuffer2: loader(c"vkCmdBindIndexBuffer2".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
      vkCmdSetLineStipple: loader(c"vkCmdSetLineStipple".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
      vkCmdSetRenderingAttachmentLocations: loader(
        c"vkCmdSetRenderingAttachmentLocations".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_GRAPHICS_VERSION_1_4")]
      vkCmdSetRenderingInputAttachmentIndices: loader(
        c"vkCmdSetRenderingInputAttachmentIndices".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
      vkCmdDrawClusterHUAWEI: loader(c"vkCmdDrawClusterHUAWEI".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
      vkCmdDrawClusterIndirectHUAWEI: loader(c"vkCmdDrawClusterIndirectHUAWEI".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_HUAWEI_invocation_mask")]
      vkCmdBindInvocationMaskHUAWEI: loader(c"vkCmdBindInvocationMaskHUAWEI".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_HUAWEI_subpass_shading")]
      vkCmdSubpassShadingHUAWEI: loader(c"vkCmdSubpassShadingHUAWEI".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkCmdSetPerformanceMarkerINTEL: loader(c"vkCmdSetPerformanceMarkerINTEL".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkCmdSetPerformanceOverrideINTEL: loader(c"vkCmdSetPerformanceOverrideINTEL".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_INTEL_performance_query")]
      vkCmdSetPerformanceStreamMarkerINTEL: loader(
        c"vkCmdSetPerformanceStreamMarkerINTEL".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdBuildAccelerationStructuresIndirectKHR: loader(
        c"vkCmdBuildAccelerationStructuresIndirectKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdBuildAccelerationStructuresKHR: loader(c"vkCmdBuildAccelerationStructuresKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdCopyAccelerationStructureKHR: loader(c"vkCmdCopyAccelerationStructureKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdCopyAccelerationStructureToMemoryKHR: loader(
        c"vkCmdCopyAccelerationStructureToMemoryKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdCopyMemoryToAccelerationStructureKHR: loader(
        c"vkCmdCopyMemoryToAccelerationStructureKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_acceleration_structure")]
      vkCmdWriteAccelerationStructuresPropertiesKHR: loader(
        c"vkCmdWriteAccelerationStructuresPropertiesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdBlitImage2KHR: loader(c"vkCmdBlitImage2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdCopyBuffer2KHR: loader(c"vkCmdCopyBuffer2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdCopyBufferToImage2KHR: loader(c"vkCmdCopyBufferToImage2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdCopyImage2KHR: loader(c"vkCmdCopyImage2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdCopyImageToBuffer2KHR: loader(c"vkCmdCopyImageToBuffer2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_commands2")]
      vkCmdResolveImage2KHR: loader(c"vkCmdResolveImage2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_memory_indirect")]
      vkCmdCopyMemoryIndirectKHR: loader(c"vkCmdCopyMemoryIndirectKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_copy_memory_indirect")]
      vkCmdCopyMemoryToImageIndirectKHR: loader(c"vkCmdCopyMemoryToImageIndirectKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_create_renderpass2")]
      vkCmdBeginRenderPass2KHR: loader(c"vkCmdBeginRenderPass2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_create_renderpass2")]
      vkCmdEndRenderPass2KHR: loader(c"vkCmdEndRenderPass2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_create_renderpass2")]
      vkCmdNextSubpass2KHR: loader(c"vkCmdNextSubpass2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(any(
        feature = "VK_KHR_descriptor_update_template",
        feature = "VK_KHR_push_descriptor"
      ))]
      vkCmdPushDescriptorSetWithTemplateKHR: loader(
        c"vkCmdPushDescriptorSetWithTemplateKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdBeginConditionalRendering2EXT: loader(c"vkCmdBeginConditionalRendering2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdBeginTransformFeedback2EXT: loader(c"vkCmdBeginTransformFeedback2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdBindIndexBuffer3KHR: loader(c"vkCmdBindIndexBuffer3KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdBindTransformFeedbackBuffers2EXT: loader(
        c"vkCmdBindTransformFeedbackBuffers2EXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdBindVertexBuffers3KHR: loader(c"vkCmdBindVertexBuffers3KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdCopyImageToMemoryKHR: loader(c"vkCmdCopyImageToMemoryKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdCopyMemoryKHR: loader(c"vkCmdCopyMemoryKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdCopyMemoryToImageKHR: loader(c"vkCmdCopyMemoryToImageKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdCopyQueryPoolResultsToMemoryKHR: loader(
        c"vkCmdCopyQueryPoolResultsToMemoryKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDispatchIndirect2KHR: loader(c"vkCmdDispatchIndirect2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawIndexedIndirect2KHR: loader(c"vkCmdDrawIndexedIndirect2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawIndexedIndirectCount2KHR: loader(c"vkCmdDrawIndexedIndirectCount2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawIndirect2KHR: loader(c"vkCmdDrawIndirect2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawIndirectByteCount2EXT: loader(c"vkCmdDrawIndirectByteCount2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawIndirectCount2KHR: loader(c"vkCmdDrawIndirectCount2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawMeshTasksIndirect2EXT: loader(c"vkCmdDrawMeshTasksIndirect2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdDrawMeshTasksIndirectCount2EXT: loader(c"vkCmdDrawMeshTasksIndirectCount2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdEndTransformFeedback2EXT: loader(c"vkCmdEndTransformFeedback2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdFillMemoryKHR: loader(c"vkCmdFillMemoryKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdUpdateMemoryKHR: loader(c"vkCmdUpdateMemoryKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_address_commands")]
      vkCmdWriteMarkerToMemoryAMD: loader(c"vkCmdWriteMarkerToMemoryAMD".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_group")]
      vkCmdDispatchBaseKHR: loader(c"vkCmdDispatchBaseKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_device_group")]
      vkCmdSetDeviceMaskKHR: loader(c"vkCmdSetDeviceMaskKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_draw_indirect_count")]
      vkCmdDrawIndexedIndirectCountKHR: loader(c"vkCmdDrawIndexedIndirectCountKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_draw_indirect_count")]
      vkCmdDrawIndirectCountKHR: loader(c"vkCmdDrawIndirectCountKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_dynamic_rendering")]
      vkCmdBeginRenderingKHR: loader(c"vkCmdBeginRenderingKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_dynamic_rendering")]
      vkCmdEndRenderingKHR: loader(c"vkCmdEndRenderingKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
      vkCmdSetRenderingAttachmentLocationsKHR: loader(
        c"vkCmdSetRenderingAttachmentLocationsKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
      vkCmdSetRenderingInputAttachmentIndicesKHR: loader(
        c"vkCmdSetRenderingInputAttachmentIndicesKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_fragment_shading_rate")]
      vkCmdSetFragmentShadingRateKHR: loader(c"vkCmdSetFragmentShadingRateKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_line_rasterization")]
      vkCmdSetLineStippleKHR: loader(c"vkCmdSetLineStippleKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance10")]
      vkCmdEndRendering2KHR: loader(c"vkCmdEndRendering2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance5")]
      vkCmdBindIndexBuffer2KHR: loader(c"vkCmdBindIndexBuffer2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdBindDescriptorBufferEmbeddedSamplers2EXT: loader(
        c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdBindDescriptorSets2KHR: loader(c"vkCmdBindDescriptorSets2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdPushConstants2KHR: loader(c"vkCmdPushConstants2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdPushDescriptorSet2KHR: loader(c"vkCmdPushDescriptorSet2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdPushDescriptorSetWithTemplate2KHR: loader(
        c"vkCmdPushDescriptorSetWithTemplate2KHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_maintenance6")]
      vkCmdSetDescriptorBufferOffsets2EXT: loader(c"vkCmdSetDescriptorBufferOffsets2EXT".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_object_refresh")]
      vkCmdRefreshObjectsKHR: loader(c"vkCmdRefreshObjectsKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_push_descriptor")]
      vkCmdPushDescriptorSetKHR: loader(c"vkCmdPushDescriptorSetKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
      vkCmdTraceRaysIndirect2KHR: loader(c"vkCmdTraceRaysIndirect2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
      vkCmdSetRayTracingPipelineStackSizeKHR: loader(
        c"vkCmdSetRayTracingPipelineStackSizeKHR".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
      vkCmdTraceRaysIndirectKHR: loader(c"vkCmdTraceRaysIndirectKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
      vkCmdTraceRaysKHR: loader(c"vkCmdTraceRaysKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_synchronization2")]
      vkCmdPipelineBarrier2KHR: loader(c"vkCmdPipelineBarrier2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_synchronization2")]
      vkCmdResetEvent2KHR: loader(c"vkCmdResetEvent2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_synchronization2")]
      vkCmdSetEvent2KHR: loader(c"vkCmdSetEvent2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_synchronization2")]
      vkCmdWaitEvents2KHR: loader(c"vkCmdWaitEvents2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_synchronization2")]
      vkCmdWriteTimestamp2KHR: loader(c"vkCmdWriteTimestamp2KHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_decode_queue")]
      vkCmdDecodeVideoKHR: loader(c"vkCmdDecodeVideoKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_encode_queue")]
      vkCmdEncodeVideoKHR: loader(c"vkCmdEncodeVideoKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkCmdBeginVideoCodingKHR: loader(c"vkCmdBeginVideoCodingKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkCmdControlVideoCodingKHR: loader(c"vkCmdControlVideoCodingKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_KHR_video_queue")]
      vkCmdEndVideoCodingKHR: loader(c"vkCmdEndVideoCodingKHR".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NVX_binary_import")]
      vkCmdCuLaunchKernelNVX: loader(c"vkCmdCuLaunchKernelNVX".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_clip_space_w_scaling")]
      vkCmdSetViewportWScalingNV: loader(c"vkCmdSetViewportWScalingNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cluster_acceleration_structure")]
      vkCmdBuildClusterAccelerationStructureIndirectNV: loader(
        c"vkCmdBuildClusterAccelerationStructureIndirectNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_compute_occupancy_priority")]
      vkCmdSetComputeOccupancyPriorityNV: loader(c"vkCmdSetComputeOccupancyPriorityNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cooperative_vector")]
      vkCmdConvertCooperativeVectorMatrixNV: loader(
        c"vkCmdConvertCooperativeVectorMatrixNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_copy_memory_indirect")]
      vkCmdCopyMemoryIndirectNV: loader(c"vkCmdCopyMemoryIndirectNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_copy_memory_indirect")]
      vkCmdCopyMemoryToImageIndirectNV: loader(c"vkCmdCopyMemoryToImageIndirectNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_cuda_kernel_launch")]
      vkCmdCudaLaunchKernelNV: loader(c"vkCmdCudaLaunchKernelNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
      vkCmdSetCheckpointNV: loader(c"vkCmdSetCheckpointNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkCmdBindPipelineShaderGroupNV: loader(c"vkCmdBindPipelineShaderGroupNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkCmdExecuteGeneratedCommandsNV: loader(c"vkCmdExecuteGeneratedCommandsNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands")]
      vkCmdPreprocessGeneratedCommandsNV: loader(c"vkCmdPreprocessGeneratedCommandsNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_device_generated_commands_compute")]
      vkCmdUpdatePipelineIndirectBufferNV: loader(c"vkCmdUpdatePipelineIndirectBufferNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
      vkCmdSetFragmentShadingRateEnumNV: loader(c"vkCmdSetFragmentShadingRateEnumNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_memory_decompression")]
      vkCmdDecompressMemoryIndirectCountNV: loader(
        c"vkCmdDecompressMemoryIndirectCountNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_memory_decompression")]
      vkCmdDecompressMemoryNV: loader(c"vkCmdDecompressMemoryNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_mesh_shader")]
      vkCmdDrawMeshTasksIndirectCountNV: loader(c"vkCmdDrawMeshTasksIndirectCountNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_mesh_shader")]
      vkCmdDrawMeshTasksIndirectNV: loader(c"vkCmdDrawMeshTasksIndirectNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_mesh_shader")]
      vkCmdDrawMeshTasksNV: loader(c"vkCmdDrawMeshTasksNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_optical_flow")]
      vkCmdOpticalFlowExecuteNV: loader(c"vkCmdOpticalFlowExecuteNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
      vkCmdBuildPartitionedAccelerationStructuresNV: loader(
        c"vkCmdBuildPartitionedAccelerationStructuresNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkCmdBuildAccelerationStructureNV: loader(c"vkCmdBuildAccelerationStructureNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkCmdCopyAccelerationStructureNV: loader(c"vkCmdCopyAccelerationStructureNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkCmdTraceRaysNV: loader(c"vkCmdTraceRaysNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_ray_tracing")]
      vkCmdWriteAccelerationStructuresPropertiesNV: loader(
        c"vkCmdWriteAccelerationStructuresPropertiesNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_scissor_exclusive")]
      vkCmdSetExclusiveScissorEnableNV: loader(c"vkCmdSetExclusiveScissorEnableNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_scissor_exclusive")]
      vkCmdSetExclusiveScissorNV: loader(c"vkCmdSetExclusiveScissorNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_shading_rate_image")]
      vkCmdBindShadingRateImageNV: loader(c"vkCmdBindShadingRateImageNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_shading_rate_image")]
      vkCmdSetCoarseSampleOrderNV: loader(c"vkCmdSetCoarseSampleOrderNV".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_NV_shading_rate_image")]
      vkCmdSetViewportShadingRatePaletteNV: loader(
        c"vkCmdSetViewportShadingRatePaletteNV".as_ptr(),
      )
      .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_memory_heap")]
      vkCmdBindTileMemoryQCOM: loader(c"vkCmdBindTileMemoryQCOM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_shading")]
      vkCmdBeginPerTileExecutionQCOM: loader(c"vkCmdBeginPerTileExecutionQCOM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_shading")]
      vkCmdDispatchTileQCOM: loader(c"vkCmdDispatchTileQCOM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
      #[cfg(feature = "VK_QCOM_tile_shading")]
      vkCmdEndPerTileExecutionQCOM: loader(c"vkCmdEndPerTileExecutionQCOM".as_ptr())
        .map(|f| unsafe { core::mem::transmute(f) }),
    }
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct CommandBuffer<'dev> {
  pub(crate) raw: VkCommandBuffer,
  pub(crate) parent: &'dev crate::command_pool::CommandPool<'dev>,
  pub(crate) table: &'dev CommandBufferDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Send for CommandBuffer<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'dev> Sync for CommandBuffer<'dev> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for CommandBuffer<'dev> {
  fn drop(&mut self) {
    if self.raw.0.is_null() {
      return;
    }
    unsafe {
      (self.parent.table.vkFreeCommandBuffers).unwrap_unchecked()(
        self.device().raw,
        self.parent.raw,
        1,
        &self.raw,
      )
    };
  }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> CommandBuffer<'dev> {
  #[inline(always)]
  pub const fn raw(&self) -> VkCommandBuffer {
    self.raw
  }
  #[inline(always)]
  pub const fn parent(&self) -> &'dev crate::command_pool::CommandPool<'dev> {
    self.parent
  }
  #[inline(always)]
  pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
    self.parent.device()
  }
  #[inline(always)]
  pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
    self.parent.instance()
  }
  #[inline(always)]
  pub const fn table(&self) -> &CommandBufferDispatchTable {
    self.table
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchGraphAMDX(
    &self,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    pCountInfo: &VkDispatchGraphCountInfoAMDX,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchGraphAMDX.unwrap_unchecked()(
        self.raw,
        scratch,
        scratchSize,
        pCountInfo,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchGraphIndirectAMDX(
    &self,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    pCountInfo: &VkDispatchGraphCountInfoAMDX,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDispatchGraphIndirectAMDX
        .unwrap_unchecked()(self.raw, scratch, scratchSize, pCountInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchGraphIndirectCountAMDX(
    &self,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
    countInfo: VkDeviceAddress,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDispatchGraphIndirectCountAMDX
        .unwrap_unchecked()(self.raw, scratch, scratchSize, countInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdInitializeGraphScratchMemoryAMDX(
    &self,
    executionGraph: VkPipeline,
    scratch: VkDeviceAddress,
    scratchSize: VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdInitializeGraphScratchMemoryAMDX
        .unwrap_unchecked()(self.raw, executionGraph, scratch, scratchSize)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteBufferMarker2AMD(
    &self,
    stage: VkPipelineStageFlags2,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteBufferMarker2AMD.unwrap_unchecked()(
        self.raw, stage, dstBuffer, dstOffset, marker,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteBufferMarkerAMD(
    &self,
    pipelineStage: VkPipelineStageFlagBits,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    marker: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteBufferMarkerAMD.unwrap_unchecked()(
        self.raw,
        pipelineStage,
        dstBuffer,
        dstOffset,
        marker,
      )
    }
  }
  /// [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)
  ///
  /// Provided by:
  /// - `VK_AMD_draw_indirect_count`
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
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  #[deprecated(note = "superseded by `vkCmdDrawIndexedIndirectCount2KHR`")]
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirectCountAMD(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndexedIndirectCountAMD
        .unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
  /// [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)
  ///
  /// Provided by:
  /// - `VK_AMD_draw_indirect_count`
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
  #[cfg(feature = "VK_AMD_draw_indirect_count")]
  #[deprecated(note = "superseded by `vkCmdDrawIndirectCount2KHR`")]
  #[inline(always)]
  pub fn vkCmdDrawIndirectCountAMD(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirectCountAMD.unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchDataGraphARM(
    &self,
    session: VkDataGraphPipelineSessionARM,
    pInfo: *const VkDataGraphPipelineDispatchInfoARM,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchDataGraphARM.unwrap_unchecked()(self.raw, session, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDispatchParametersARM(&self, pDispatchParameters: &VkDispatchParametersARM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDispatchParametersARM
        .unwrap_unchecked()(self.raw, pDispatchParameters)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginShaderInstrumentationARM(&self, instrumentation: VkShaderInstrumentationARM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginShaderInstrumentationARM
        .unwrap_unchecked()(self.raw, instrumentation)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndShaderInstrumentationARM(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdEndShaderInstrumentationARM
        .unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyTensorARM(&self, pCopyTensorInfo: &VkCopyTensorInfoARM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyTensorARM.unwrap_unchecked()(self.raw, pCopyTensorInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkBeginCommandBuffer(
    &self,
    pBeginInfo: &VkCommandBufferBeginInfo,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (self.table).vkBeginCommandBuffer.unwrap_unchecked()(self.raw, pBeginInfo) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginQuery(&self, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginQuery.unwrap_unchecked()(self.raw, queryPool, query, flags)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyBuffer(
    &self,
    srcBuffer: VkBuffer,
    dstBuffer: VkBuffer,
    pRegions: &[VkBufferCopy],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBuffer.unwrap_unchecked()(
        self.raw,
        srcBuffer,
        dstBuffer,
        pRegions.len() as u32,
        pRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyBufferToImage(
    &self,
    srcBuffer: VkBuffer,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pRegions: &[VkBufferImageCopy],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBufferToImage.unwrap_unchecked()(
        self.raw,
        srcBuffer,
        dstImage,
        dstImageLayout,
        pRegions.len() as u32,
        pRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyImage(
    &self,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pRegions: &[VkImageCopy],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImage.unwrap_unchecked()(
        self.raw,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        pRegions.len() as u32,
        pRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyImageToBuffer(
    &self,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstBuffer: VkBuffer,
    pRegions: &[VkBufferImageCopy],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImageToBuffer.unwrap_unchecked()(
        self.raw,
        srcImage,
        srcImageLayout,
        dstBuffer,
        pRegions.len() as u32,
        pRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyQueryPoolResults(
    &self,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyQueryPoolResults.unwrap_unchecked()(
        self.raw, queryPool, firstQuery, queryCount, dstBuffer, dstOffset, stride, flags,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndQuery(&self, queryPool: VkQueryPool, query: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndQuery.unwrap_unchecked()(self.raw, queryPool, query)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdExecuteCommands(&self, pCommandBuffers: &[VkCommandBuffer]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdExecuteCommands.unwrap_unchecked()(
        self.raw,
        pCommandBuffers.len() as u32,
        pCommandBuffers.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdFillBuffer(
    &self,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdFillBuffer.unwrap_unchecked()(self.raw, dstBuffer, dstOffset, size, data)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPipelineBarrier(
    &self,
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    dependencyFlags: VkDependencyFlags,
    pMemoryBarriers: &[VkMemoryBarrier],
    pBufferMemoryBarriers: &[VkBufferMemoryBarrier],
    pImageMemoryBarriers: &[VkImageMemoryBarrier],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPipelineBarrier.unwrap_unchecked()(
        self.raw,
        srcStageMask,
        dstStageMask,
        dependencyFlags,
        pMemoryBarriers.len() as u32,
        pMemoryBarriers.as_ptr(),
        pBufferMemoryBarriers.len() as u32,
        pBufferMemoryBarriers.as_ptr(),
        pImageMemoryBarriers.len() as u32,
        pImageMemoryBarriers.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdResetQueryPool(&self, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResetQueryPool.unwrap_unchecked()(
        self.raw, queryPool, firstQuery, queryCount,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdUpdateBuffer(
    &self,
    dstBuffer: VkBuffer,
    dstOffset: VkDeviceSize,
    dataSize: VkDeviceSize,
    pData: *const core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdUpdateBuffer.unwrap_unchecked()(
        self.raw, dstBuffer, dstOffset, dataSize, pData,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteTimestamp(
    &self,
    pipelineStage: VkPipelineStageFlagBits,
    queryPool: VkQueryPool,
    query: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteTimestamp.unwrap_unchecked()(self.raw, pipelineStage, queryPool, query)
    }
  }
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
  #[inline(always)]
  pub fn vkEndCommandBuffer(&self) -> Result<VkResult, VkResult> {
    let r = unsafe { (self.table).vkEndCommandBuffer.unwrap_unchecked()(self.raw) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkResetCommandBuffer(
    &self,
    flags: VkCommandBufferResetFlags,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe { (self.table).vkResetCommandBuffer.unwrap_unchecked()(self.raw, flags) };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDeviceMask(&self, deviceMask: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDeviceMask.unwrap_unchecked()(self.raw, deviceMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyBuffer2(&self, pCopyBufferInfo: &VkCopyBufferInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBuffer2.unwrap_unchecked()(self.raw, pCopyBufferInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyBufferToImage2(&self, pCopyBufferToImageInfo: &VkCopyBufferToImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBufferToImage2.unwrap_unchecked()(self.raw, pCopyBufferToImageInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyImage2(&self, pCopyImageInfo: &VkCopyImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImage2.unwrap_unchecked()(self.raw, pCopyImageInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyImageToBuffer2(&self, pCopyImageToBufferInfo: &VkCopyImageToBufferInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImageToBuffer2.unwrap_unchecked()(self.raw, pCopyImageToBufferInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPipelineBarrier2(&self, pDependencyInfo: &VkDependencyInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPipelineBarrier2.unwrap_unchecked()(self.raw, pDependencyInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteTimestamp2(
    &self,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteTimestamp2.unwrap_unchecked()(self.raw, stage, queryPool, query)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindDescriptorSets(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    descriptorSetCount: u32,
    pDescriptorSets: *const VkDescriptorSet,
    pDynamicOffsets: &[u32],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindDescriptorSets.unwrap_unchecked()(
        self.raw,
        pipelineBindPoint,
        layout,
        firstSet,
        descriptorSetCount,
        pDescriptorSets,
        pDynamicOffsets.len() as u32,
        pDynamicOffsets.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindPipeline(&self, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindPipeline.unwrap_unchecked()(self.raw, pipelineBindPoint, pipeline)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdClearColorImage(
    &self,
    image: VkImage,
    imageLayout: VkImageLayout,
    pColor: &VkClearColorValue,
    pRanges: &[VkImageSubresourceRange],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdClearColorImage.unwrap_unchecked()(
        self.raw,
        image,
        imageLayout,
        pColor,
        pRanges.len() as u32,
        pRanges.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatch(&self, groupCountX: u32, groupCountY: u32, groupCountZ: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatch.unwrap_unchecked()(self.raw, groupCountX, groupCountY, groupCountZ)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchIndirect(&self, buffer: VkBuffer, offset: VkDeviceSize) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchIndirect.unwrap_unchecked()(self.raw, buffer, offset)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushConstants(
    &self,
    layout: VkPipelineLayout,
    stageFlags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    pValues: *const core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushConstants.unwrap_unchecked()(
        self.raw, layout, stageFlags, offset, size, pValues,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdResetEvent(&self, event: VkEvent, stageMask: VkPipelineStageFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResetEvent.unwrap_unchecked()(self.raw, event, stageMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetEvent(&self, event: VkEvent, stageMask: VkPipelineStageFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetEvent.unwrap_unchecked()(self.raw, event, stageMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWaitEvents(
    &self,
    pEvents: &[VkEvent],
    srcStageMask: VkPipelineStageFlags,
    dstStageMask: VkPipelineStageFlags,
    pMemoryBarriers: &[VkMemoryBarrier],
    pBufferMemoryBarriers: &[VkBufferMemoryBarrier],
    pImageMemoryBarriers: &[VkImageMemoryBarrier],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWaitEvents.unwrap_unchecked()(
        self.raw,
        pEvents.len() as u32,
        pEvents.as_ptr(),
        srcStageMask,
        dstStageMask,
        pMemoryBarriers.len() as u32,
        pMemoryBarriers.as_ptr(),
        pBufferMemoryBarriers.len() as u32,
        pBufferMemoryBarriers.as_ptr(),
        pImageMemoryBarriers.len() as u32,
        pImageMemoryBarriers.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchBase(
    &self,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchBase.unwrap_unchecked()(
        self.raw,
        baseGroupX,
        baseGroupY,
        baseGroupZ,
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdResetEvent2(&self, event: VkEvent, stageMask: VkPipelineStageFlags2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResetEvent2.unwrap_unchecked()(self.raw, event, stageMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetEvent2(&self, event: VkEvent, pDependencyInfo: &VkDependencyInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetEvent2.unwrap_unchecked()(self.raw, event, pDependencyInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWaitEvents2(&self, pEvents: &[VkEvent], pDependencyInfos: &[VkDependencyInfo]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWaitEvents2.unwrap_unchecked()(
        self.raw,
        pDependencyInfos.len() as u32,
        pEvents.as_ptr(),
        pDependencyInfos.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindDescriptorSets2(&self, pBindDescriptorSetsInfo: &VkBindDescriptorSetsInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindDescriptorSets2.unwrap_unchecked()(self.raw, pBindDescriptorSetsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushConstants2(&self, pPushConstantsInfo: &VkPushConstantsInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushConstants2.unwrap_unchecked()(self.raw, pPushConstantsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushDescriptorSet(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    pDescriptorWrites: &[VkWriteDescriptorSet],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushDescriptorSet.unwrap_unchecked()(
        self.raw,
        pipelineBindPoint,
        layout,
        set,
        pDescriptorWrites.len() as u32,
        pDescriptorWrites.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushDescriptorSet2(&self, pPushDescriptorSetInfo: &VkPushDescriptorSetInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushDescriptorSet2.unwrap_unchecked()(self.raw, pPushDescriptorSetInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushDescriptorSetWithTemplate(
    &self,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPushDescriptorSetWithTemplate
        .unwrap_unchecked()(self.raw, descriptorUpdateTemplate, layout, set, pData)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushDescriptorSetWithTemplate2(
    &self,
    pPushDescriptorSetWithTemplateInfo: &VkPushDescriptorSetWithTemplateInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPushDescriptorSetWithTemplate2
        .unwrap_unchecked()(self.raw, pPushDescriptorSetWithTemplateInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetAttachmentFeedbackLoopEnableEXT(&self, aspectMask: VkImageAspectFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetAttachmentFeedbackLoopEnableEXT
        .unwrap_unchecked()(self.raw, aspectMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetColorWriteEnableEXT(&self, pColorWriteEnables: &[VkBool32]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetColorWriteEnableEXT.unwrap_unchecked()(
        self.raw,
        pColorWriteEnables.len() as u32,
        pColorWriteEnables.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginConditionalRenderingEXT(
    &self,
    pConditionalRenderingBegin: &VkConditionalRenderingBeginInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginConditionalRenderingEXT
        .unwrap_unchecked()(self.raw, pConditionalRenderingBegin)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndConditionalRenderingEXT(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdEndConditionalRenderingEXT
        .unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginCustomResolveEXT(
    &self,
    pBeginCustomResolveInfo: *const VkBeginCustomResolveInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginCustomResolveEXT.unwrap_unchecked()(self.raw, pBeginCustomResolveInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDebugMarkerBeginEXT(&self, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDebugMarkerBeginEXT.unwrap_unchecked()(self.raw, pMarkerInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDebugMarkerEndEXT(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDebugMarkerEndEXT.unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDebugMarkerInsertEXT(&self, pMarkerInfo: &VkDebugMarkerMarkerInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDebugMarkerInsertEXT.unwrap_unchecked()(self.raw, pMarkerInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginDebugUtilsLabelEXT(&self, pLabelInfo: &VkDebugUtilsLabelEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginDebugUtilsLabelEXT.unwrap_unchecked()(self.raw, pLabelInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndDebugUtilsLabelEXT(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndDebugUtilsLabelEXT.unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdInsertDebugUtilsLabelEXT(&self, pLabelInfo: &VkDebugUtilsLabelEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdInsertDebugUtilsLabelEXT
        .unwrap_unchecked()(self.raw, pLabelInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthBias2EXT(&self, pDepthBiasInfo: &VkDepthBiasInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthBias2EXT.unwrap_unchecked()(self.raw, pDepthBiasInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthClampRangeEXT(
    &self,
    depthClampMode: VkDepthClampModeEXT,
    pDepthClampRange: *const VkDepthClampRangeEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthClampRangeEXT.unwrap_unchecked()(
        self.raw,
        depthClampMode,
        pDepthClampRange,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindDescriptorBufferEmbeddedSamplersEXT(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindDescriptorBufferEmbeddedSamplersEXT
        .unwrap_unchecked()(self.raw, pipelineBindPoint, layout, set)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindDescriptorBuffersEXT(&self, pBindingInfos: &[VkDescriptorBufferBindingInfoEXT]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindDescriptorBuffersEXT
        .unwrap_unchecked()(self.raw, pBindingInfos.len() as u32, pBindingInfos.as_ptr())
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDescriptorBufferOffsetsEXT(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    firstSet: u32,
    pBufferIndices: &[u32],
    pOffsets: &[VkDeviceSize],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDescriptorBufferOffsetsEXT
        .unwrap_unchecked()(
        self.raw,
        pipelineBindPoint,
        layout,
        firstSet,
        pOffsets.len() as u32,
        pBufferIndices.as_ptr(),
        pOffsets.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindResourceHeapEXT(&self, pBindInfo: &VkBindHeapInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindResourceHeapEXT.unwrap_unchecked()(self.raw, pBindInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindSamplerHeapEXT(&self, pBindInfo: &VkBindHeapInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindSamplerHeapEXT.unwrap_unchecked()(self.raw, pBindInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPushDataEXT(&self, pPushDataInfo: &VkPushDataInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushDataEXT.unwrap_unchecked()(self.raw, pPushDataInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdExecuteGeneratedCommandsEXT(
    &self,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: &VkGeneratedCommandsInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdExecuteGeneratedCommandsEXT
        .unwrap_unchecked()(self.raw, isPreprocessed, pGeneratedCommandsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPreprocessGeneratedCommandsEXT(
    &self,
    pGeneratedCommandsInfo: &VkGeneratedCommandsInfoEXT,
    stateCommandBuffer: VkCommandBuffer,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPreprocessGeneratedCommandsEXT
        .unwrap_unchecked()(self.raw, pGeneratedCommandsInfo, stateCommandBuffer)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDiscardRectangleEXT(
    &self,
    firstDiscardRectangle: u32,
    pDiscardRectangles: &[VkRect2D],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDiscardRectangleEXT.unwrap_unchecked()(
        self.raw,
        firstDiscardRectangle,
        pDiscardRectangles.len() as u32,
        pDiscardRectangles.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDiscardRectangleEnableEXT(&self, discardRectangleEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDiscardRectangleEnableEXT
        .unwrap_unchecked()(self.raw, discardRectangleEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDiscardRectangleModeEXT(&self, discardRectangleMode: VkDiscardRectangleModeEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDiscardRectangleModeEXT
        .unwrap_unchecked()(self.raw, discardRectangleMode)
    }
  }
  /// [`vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
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
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdBindVertexBuffers2EXT(
    &self,
    firstBinding: u32,
    pBuffers: *const VkBuffer,
    pOffsets: &[VkDeviceSize],
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindVertexBuffers2EXT.unwrap_unchecked()(
        self.raw,
        firstBinding,
        pOffsets.len() as u32,
        pBuffers,
        pOffsets.as_ptr(),
        pSizes,
        pStrides,
      )
    }
  }
  /// [`vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `cullMode`: optional: true
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetCullModeEXT(&self, cullMode: VkCullModeFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetCullModeEXT.unwrap_unchecked()(self.raw, cullMode)
    }
  }
  /// [`vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `depthBoundsTestEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetDepthBoundsTestEnableEXT(&self, depthBoundsTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDepthBoundsTestEnableEXT
        .unwrap_unchecked()(self.raw, depthBoundsTestEnable)
    }
  }
  /// [`vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `depthCompareOp`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetDepthCompareOpEXT(&self, depthCompareOp: VkCompareOp) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthCompareOpEXT.unwrap_unchecked()(self.raw, depthCompareOp)
    }
  }
  /// [`vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `depthTestEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetDepthTestEnableEXT(&self, depthTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthTestEnableEXT.unwrap_unchecked()(self.raw, depthTestEnable)
    }
  }
  /// [`vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `depthWriteEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetDepthWriteEnableEXT(&self, depthWriteEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthWriteEnableEXT.unwrap_unchecked()(self.raw, depthWriteEnable)
    }
  }
  /// [`vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `frontFace`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetFrontFaceEXT(&self, frontFace: VkFrontFace) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetFrontFaceEXT.unwrap_unchecked()(self.raw, frontFace)
    }
  }
  /// [`vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `primitiveTopology`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetPrimitiveTopologyEXT(&self, primitiveTopology: VkPrimitiveTopology) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetPrimitiveTopologyEXT.unwrap_unchecked()(self.raw, primitiveTopology)
    }
  }
  /// [`vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
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
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetScissorWithCountEXT(&self, pScissors: &[VkRect2D]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetScissorWithCountEXT.unwrap_unchecked()(
        self.raw,
        pScissors.len() as u32,
        pScissors.as_ptr(),
      )
    }
  }
  /// [`vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
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
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetStencilOpEXT(
    &self,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilOpEXT.unwrap_unchecked()(
        self.raw,
        faceMask,
        failOp,
        passOp,
        depthFailOp,
        compareOp,
      )
    }
  }
  /// [`vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `stencilTestEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetStencilTestEnableEXT(&self, stencilTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilTestEnableEXT.unwrap_unchecked()(self.raw, stencilTestEnable)
    }
  }
  /// [`vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state`
  /// - `VK_EXT_shader_object`
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
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetViewportWithCountEXT(&self, pViewports: &[VkViewport]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetViewportWithCountEXT.unwrap_unchecked()(
        self.raw,
        pViewports.len() as u32,
        pViewports.as_ptr(),
      )
    }
  }
  /// [`vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state2`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `depthBiasEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetDepthBiasEnableEXT(&self, depthBiasEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthBiasEnableEXT.unwrap_unchecked()(self.raw, depthBiasEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLogicOpEXT(&self, logicOp: VkLogicOp) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLogicOpEXT.unwrap_unchecked()(self.raw, logicOp)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPatchControlPointsEXT(&self, patchControlPoints: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetPatchControlPointsEXT
        .unwrap_unchecked()(self.raw, patchControlPoints)
    }
  }
  /// [`vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state2`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `primitiveRestartEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetPrimitiveRestartEnableEXT(&self, primitiveRestartEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetPrimitiveRestartEnableEXT
        .unwrap_unchecked()(self.raw, primitiveRestartEnable)
    }
  }
  /// [`vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html)
  ///
  /// Provided by:
  /// - `VK_EXT_extended_dynamic_state2`
  /// - `VK_EXT_shader_object`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `rasterizerDiscardEnable`
  #[cfg(any(
    feature = "VK_EXT_extended_dynamic_state2",
    feature = "VK_EXT_shader_object"
  ))]
  #[inline(always)]
  pub fn vkCmdSetRasterizerDiscardEnableEXT(&self, rasterizerDiscardEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRasterizerDiscardEnableEXT
        .unwrap_unchecked()(self.raw, rasterizerDiscardEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetAlphaToCoverageEnableEXT(&self, alphaToCoverageEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetAlphaToCoverageEnableEXT
        .unwrap_unchecked()(self.raw, alphaToCoverageEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetAlphaToOneEnableEXT(&self, alphaToOneEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetAlphaToOneEnableEXT.unwrap_unchecked()(self.raw, alphaToOneEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetColorBlendAdvancedEXT(
    &self,
    firstAttachment: u32,
    pColorBlendAdvanced: &[VkColorBlendAdvancedEXT],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetColorBlendAdvancedEXT
        .unwrap_unchecked()(
        self.raw,
        firstAttachment,
        pColorBlendAdvanced.len() as u32,
        pColorBlendAdvanced.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetColorBlendEnableEXT(&self, firstAttachment: u32, pColorBlendEnables: &[VkBool32]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetColorBlendEnableEXT.unwrap_unchecked()(
        self.raw,
        firstAttachment,
        pColorBlendEnables.len() as u32,
        pColorBlendEnables.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetColorBlendEquationEXT(
    &self,
    firstAttachment: u32,
    pColorBlendEquations: &[VkColorBlendEquationEXT],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetColorBlendEquationEXT
        .unwrap_unchecked()(
        self.raw,
        firstAttachment,
        pColorBlendEquations.len() as u32,
        pColorBlendEquations.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetColorWriteMaskEXT(
    &self,
    firstAttachment: u32,
    attachmentCount: u32,
    pColorWriteMasks: *const VkColorComponentFlags,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetColorWriteMaskEXT.unwrap_unchecked()(
        self.raw,
        firstAttachment,
        attachmentCount,
        pColorWriteMasks,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetConservativeRasterizationModeEXT(
    &self,
    conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetConservativeRasterizationModeEXT
        .unwrap_unchecked()(self.raw, conservativeRasterizationMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageModulationModeNV(
    &self,
    coverageModulationMode: VkCoverageModulationModeNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageModulationModeNV
        .unwrap_unchecked()(self.raw, coverageModulationMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageModulationTableEnableNV(&self, coverageModulationTableEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageModulationTableEnableNV
        .unwrap_unchecked()(self.raw, coverageModulationTableEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageModulationTableNV(&self, pCoverageModulationTable: &[f32]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageModulationTableNV
        .unwrap_unchecked()(
        self.raw,
        pCoverageModulationTable.len() as u32,
        pCoverageModulationTable.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageReductionModeNV(&self, coverageReductionMode: VkCoverageReductionModeNV) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageReductionModeNV
        .unwrap_unchecked()(self.raw, coverageReductionMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageToColorEnableNV(&self, coverageToColorEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageToColorEnableNV
        .unwrap_unchecked()(self.raw, coverageToColorEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoverageToColorLocationNV(&self, coverageToColorLocation: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetCoverageToColorLocationNV
        .unwrap_unchecked()(self.raw, coverageToColorLocation)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthClampEnableEXT(&self, depthClampEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthClampEnableEXT.unwrap_unchecked()(self.raw, depthClampEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthClipEnableEXT(&self, depthClipEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthClipEnableEXT.unwrap_unchecked()(self.raw, depthClipEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthClipNegativeOneToOneEXT(&self, negativeOneToOne: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDepthClipNegativeOneToOneEXT
        .unwrap_unchecked()(self.raw, negativeOneToOne)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetExtraPrimitiveOverestimationSizeEXT(&self, extraPrimitiveOverestimationSize: f32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetExtraPrimitiveOverestimationSizeEXT
        .unwrap_unchecked()(self.raw, extraPrimitiveOverestimationSize)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLineRasterizationModeEXT(
    &self,
    lineRasterizationMode: VkLineRasterizationModeEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetLineRasterizationModeEXT
        .unwrap_unchecked()(self.raw, lineRasterizationMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLineStippleEnableEXT(&self, stippledLineEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLineStippleEnableEXT.unwrap_unchecked()(self.raw, stippledLineEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLogicOpEnableEXT(&self, logicOpEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLogicOpEnableEXT.unwrap_unchecked()(self.raw, logicOpEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPolygonModeEXT(&self, polygonMode: VkPolygonMode) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetPolygonModeEXT.unwrap_unchecked()(self.raw, polygonMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetProvokingVertexModeEXT(&self, provokingVertexMode: VkProvokingVertexModeEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetProvokingVertexModeEXT
        .unwrap_unchecked()(self.raw, provokingVertexMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRasterizationSamplesEXT(&self, rasterizationSamples: VkSampleCountFlagBits) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRasterizationSamplesEXT
        .unwrap_unchecked()(self.raw, rasterizationSamples)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRasterizationStreamEXT(&self, rasterizationStream: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRasterizationStreamEXT
        .unwrap_unchecked()(self.raw, rasterizationStream)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRepresentativeFragmentTestEnableNV(
    &self,
    representativeFragmentTestEnable: VkBool32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRepresentativeFragmentTestEnableNV
        .unwrap_unchecked()(self.raw, representativeFragmentTestEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetSampleLocationsEnableEXT(&self, sampleLocationsEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetSampleLocationsEnableEXT
        .unwrap_unchecked()(self.raw, sampleLocationsEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetSampleMaskEXT(
    &self,
    samples: VkSampleCountFlagBits,
    pSampleMask: *const VkSampleMask,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetSampleMaskEXT.unwrap_unchecked()(self.raw, samples, pSampleMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetShadingRateImageEnableNV(&self, shadingRateImageEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetShadingRateImageEnableNV
        .unwrap_unchecked()(self.raw, shadingRateImageEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetTessellationDomainOriginEXT(&self, domainOrigin: VkTessellationDomainOrigin) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetTessellationDomainOriginEXT
        .unwrap_unchecked()(self.raw, domainOrigin)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewportSwizzleNV(
    &self,
    firstViewport: u32,
    pViewportSwizzles: &[VkViewportSwizzleNV],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetViewportSwizzleNV.unwrap_unchecked()(
        self.raw,
        firstViewport,
        pViewportSwizzles.len() as u32,
        pViewportSwizzles.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewportWScalingEnableNV(&self, viewportWScalingEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetViewportWScalingEnableNV
        .unwrap_unchecked()(self.raw, viewportWScalingEnable)
    }
  }
  /// [`vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html)
  ///
  /// Provided by:
  /// - `VK_EXT_fragment_density_map_offset`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Inside
  /// - **Tasks:** Action, State
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pRenderingEndInfo`: optional: true
  #[cfg(feature = "VK_EXT_fragment_density_map_offset")]
  #[inline(always)]
  pub fn vkCmdEndRendering2EXT(&self, pRenderingEndInfo: *const VkRenderingEndInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRendering2EXT.unwrap_unchecked()(self.raw, pRenderingEndInfo)
    }
  }
  /// [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)
  ///
  /// Provided by:
  /// - `VK_EXT_line_rasterization`
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
  #[cfg(feature = "VK_EXT_line_rasterization")]
  #[inline(always)]
  pub fn vkCmdSetLineStippleEXT(&self, lineStippleFactor: u32, lineStipplePattern: u16) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLineStippleEXT.unwrap_unchecked()(
        self.raw,
        lineStippleFactor,
        lineStipplePattern,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDecompressMemoryEXT(&self, pDecompressMemoryInfoEXT: &VkDecompressMemoryInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDecompressMemoryEXT.unwrap_unchecked()(self.raw, pDecompressMemoryInfoEXT)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDecompressMemoryIndirectCountEXT(
    &self,
    decompressionMethod: VkMemoryDecompressionMethodFlagsEXT,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    maxDecompressionCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDecompressMemoryIndirectCountEXT
        .unwrap_unchecked()(
        self.raw,
        decompressionMethod,
        indirectCommandsAddress,
        indirectCommandsCountAddress,
        maxDecompressionCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksEXT(&self, groupCountX: u32, groupCountY: u32, groupCountZ: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawMeshTasksEXT.unwrap_unchecked()(
        self.raw,
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirectCountEXT(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawMeshTasksIndirectCountEXT
        .unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirectEXT(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawMeshTasksIndirectEXT
        .unwrap_unchecked()(self.raw, buffer, offset, drawCount, stride)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMultiEXT(
    &self,
    pVertexInfo: &[VkMultiDrawInfoEXT],
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawMultiEXT.unwrap_unchecked()(
        self.raw,
        pVertexInfo.len() as u32,
        pVertexInfo.as_ptr(),
        instanceCount,
        firstInstance,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMultiIndexedEXT(
    &self,
    pIndexInfo: &[VkMultiDrawIndexedInfoEXT],
    instanceCount: u32,
    firstInstance: u32,
    stride: u32,
    pVertexOffset: *const i32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawMultiIndexedEXT.unwrap_unchecked()(
        self.raw,
        pIndexInfo.len() as u32,
        pIndexInfo.as_ptr(),
        instanceCount,
        firstInstance,
        stride,
        pVertexOffset,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildMicromapsEXT(&self, pInfos: &[VkMicromapBuildInfoEXT]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBuildMicromapsEXT.unwrap_unchecked()(
        self.raw,
        pInfos.len() as u32,
        pInfos.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryToMicromapEXT(&self, pInfo: &VkCopyMemoryToMicromapInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMemoryToMicromapEXT.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMicromapEXT(&self, pInfo: &VkCopyMicromapInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMicromapEXT.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMicromapToMemoryEXT(&self, pInfo: &VkCopyMicromapToMemoryInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMicromapToMemoryEXT.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteMicromapsPropertiesEXT(
    &self,
    pMicromaps: &[VkMicromapEXT],
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdWriteMicromapsPropertiesEXT
        .unwrap_unchecked()(
        self.raw,
        pMicromaps.len() as u32,
        pMicromaps.as_ptr(),
        queryType,
        queryPool,
        firstQuery,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPrimitiveRestartIndexEXT(&self, primitiveRestartIndex: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetPrimitiveRestartIndexEXT
        .unwrap_unchecked()(self.raw, primitiveRestartIndex)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetSampleLocationsEXT(&self, pSampleLocationsInfo: &VkSampleLocationsInfoEXT) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetSampleLocationsEXT.unwrap_unchecked()(self.raw, pSampleLocationsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindShadersEXT(
    &self,
    pStages: &[VkShaderStageFlagBits],
    pShaders: *const VkShaderEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindShadersEXT.unwrap_unchecked()(
        self.raw,
        pStages.len() as u32,
        pStages.as_ptr(),
        pShaders,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetVertexInputEXT(
    &self,
    pVertexBindingDescriptions: &[VkVertexInputBindingDescription2EXT],
    pVertexAttributeDescriptions: &[VkVertexInputAttributeDescription2EXT],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetVertexInputEXT.unwrap_unchecked()(
        self.raw,
        pVertexBindingDescriptions.len() as u32,
        pVertexBindingDescriptions.as_ptr(),
        pVertexAttributeDescriptions.len() as u32,
        pVertexAttributeDescriptions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginQueryIndexedEXT(
    &self,
    queryPool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
    index: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginQueryIndexedEXT.unwrap_unchecked()(
        self.raw, queryPool, query, flags, index,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginTransformFeedbackEXT(
    &self,
    firstCounterBuffer: u32,
    pCounterBuffers: &[VkBuffer],
    pCounterBufferOffsets: *const VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginTransformFeedbackEXT
        .unwrap_unchecked()(
        self.raw,
        firstCounterBuffer,
        pCounterBuffers.len() as u32,
        pCounterBuffers.as_ptr(),
        pCounterBufferOffsets,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindTransformFeedbackBuffersEXT(
    &self,
    firstBinding: u32,
    pBuffers: &[VkBuffer],
    pOffsets: &[VkDeviceSize],
    pSizes: *const VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindTransformFeedbackBuffersEXT
        .unwrap_unchecked()(
        self.raw,
        firstBinding,
        pOffsets.len() as u32,
        pBuffers.as_ptr(),
        pOffsets.as_ptr(),
        pSizes,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirectByteCountEXT(
    &self,
    instanceCount: u32,
    firstInstance: u32,
    counterBuffer: VkBuffer,
    counterBufferOffset: VkDeviceSize,
    counterOffset: u32,
    vertexStride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndirectByteCountEXT
        .unwrap_unchecked()(
        self.raw,
        instanceCount,
        firstInstance,
        counterBuffer,
        counterBufferOffset,
        counterOffset,
        vertexStride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndQueryIndexedEXT(&self, queryPool: VkQueryPool, query: u32, index: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndQueryIndexedEXT.unwrap_unchecked()(self.raw, queryPool, query, index)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndTransformFeedbackEXT(
    &self,
    firstCounterBuffer: u32,
    pCounterBuffers: &[VkBuffer],
    pCounterBufferOffsets: *const VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndTransformFeedbackEXT.unwrap_unchecked()(
        self.raw,
        firstCounterBuffer,
        pCounterBuffers.len() as u32,
        pCounterBuffers.as_ptr(),
        pCounterBufferOffsets,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginRenderPass(
    &self,
    pRenderPassBegin: &VkRenderPassBeginInfo,
    contents: VkSubpassContents,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginRenderPass.unwrap_unchecked()(self.raw, pRenderPassBegin, contents)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindIndexBuffer(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    indexType: VkIndexType,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindIndexBuffer.unwrap_unchecked()(self.raw, buffer, offset, indexType)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindVertexBuffers(
    &self,
    firstBinding: u32,
    pBuffers: *const VkBuffer,
    pOffsets: &[VkDeviceSize],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindVertexBuffers.unwrap_unchecked()(
        self.raw,
        firstBinding,
        pOffsets.len() as u32,
        pBuffers,
        pOffsets.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBlitImage(
    &self,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pRegions: &[VkImageBlit],
    filter: VkFilter,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBlitImage.unwrap_unchecked()(
        self.raw,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        pRegions.len() as u32,
        pRegions.as_ptr(),
        filter,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdClearAttachments(&self, pAttachments: &[VkClearAttachment], pRects: &[VkClearRect]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdClearAttachments.unwrap_unchecked()(
        self.raw,
        pAttachments.len() as u32,
        pAttachments.as_ptr(),
        pRects.len() as u32,
        pRects.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdClearDepthStencilImage(
    &self,
    image: VkImage,
    imageLayout: VkImageLayout,
    pDepthStencil: &VkClearDepthStencilValue,
    pRanges: &[VkImageSubresourceRange],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdClearDepthStencilImage.unwrap_unchecked()(
        self.raw,
        image,
        imageLayout,
        pDepthStencil,
        pRanges.len() as u32,
        pRanges.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDraw(
    &self,
    vertexCount: u32,
    instanceCount: u32,
    firstVertex: u32,
    firstInstance: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDraw.unwrap_unchecked()(
        self.raw,
        vertexCount,
        instanceCount,
        firstVertex,
        firstInstance,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndexed(
    &self,
    indexCount: u32,
    instanceCount: u32,
    firstIndex: u32,
    vertexOffset: i32,
    firstInstance: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndexed.unwrap_unchecked()(
        self.raw,
        indexCount,
        instanceCount,
        firstIndex,
        vertexOffset,
        firstInstance,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirect(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndexedIndirect.unwrap_unchecked()(
        self.raw, buffer, offset, drawCount, stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirect(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirect.unwrap_unchecked()(self.raw, buffer, offset, drawCount, stride)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndRenderPass(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRenderPass.unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdNextSubpass(&self, contents: VkSubpassContents) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdNextSubpass.unwrap_unchecked()(self.raw, contents)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdResolveImage(
    &self,
    srcImage: VkImage,
    srcImageLayout: VkImageLayout,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pRegions: &[VkImageResolve],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResolveImage.unwrap_unchecked()(
        self.raw,
        srcImage,
        srcImageLayout,
        dstImage,
        dstImageLayout,
        pRegions.len() as u32,
        pRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetBlendConstants(&self, blendConstants: [f32; 4]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetBlendConstants.unwrap_unchecked()(self.raw, blendConstants)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthBias(
    &self,
    depthBiasConstantFactor: f32,
    depthBiasClamp: f32,
    depthBiasSlopeFactor: f32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthBias.unwrap_unchecked()(
        self.raw,
        depthBiasConstantFactor,
        depthBiasClamp,
        depthBiasSlopeFactor,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthBounds(&self, minDepthBounds: f32, maxDepthBounds: f32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthBounds.unwrap_unchecked()(self.raw, minDepthBounds, maxDepthBounds)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLineWidth(&self, lineWidth: f32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLineWidth.unwrap_unchecked()(self.raw, lineWidth)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetScissor(&self, firstScissor: u32, pScissors: &[VkRect2D]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetScissor.unwrap_unchecked()(
        self.raw,
        firstScissor,
        pScissors.len() as u32,
        pScissors.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetStencilCompareMask(&self, faceMask: VkStencilFaceFlags, compareMask: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilCompareMask.unwrap_unchecked()(self.raw, faceMask, compareMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetStencilReference(&self, faceMask: VkStencilFaceFlags, reference: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilReference.unwrap_unchecked()(self.raw, faceMask, reference)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetStencilWriteMask(&self, faceMask: VkStencilFaceFlags, writeMask: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilWriteMask.unwrap_unchecked()(self.raw, faceMask, writeMask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewport(&self, firstViewport: u32, pViewports: &[VkViewport]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetViewport.unwrap_unchecked()(
        self.raw,
        firstViewport,
        pViewports.len() as u32,
        pViewports.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginRenderPass2(
    &self,
    pRenderPassBegin: &VkRenderPassBeginInfo,
    pSubpassBeginInfo: &VkSubpassBeginInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginRenderPass2.unwrap_unchecked()(
        self.raw,
        pRenderPassBegin,
        pSubpassBeginInfo,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirectCount(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndexedIndirectCount
        .unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirectCount(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirectCount.unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndRenderPass2(&self, pSubpassEndInfo: &VkSubpassEndInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRenderPass2.unwrap_unchecked()(self.raw, pSubpassEndInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdNextSubpass2(
    &self,
    pSubpassBeginInfo: &VkSubpassBeginInfo,
    pSubpassEndInfo: &VkSubpassEndInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdNextSubpass2.unwrap_unchecked()(
        self.raw,
        pSubpassBeginInfo,
        pSubpassEndInfo,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginRendering(&self, pRenderingInfo: &VkRenderingInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginRendering.unwrap_unchecked()(self.raw, pRenderingInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindVertexBuffers2(
    &self,
    firstBinding: u32,
    pBuffers: *const VkBuffer,
    pOffsets: &[VkDeviceSize],
    pSizes: *const VkDeviceSize,
    pStrides: *const VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindVertexBuffers2.unwrap_unchecked()(
        self.raw,
        firstBinding,
        pOffsets.len() as u32,
        pBuffers,
        pOffsets.as_ptr(),
        pSizes,
        pStrides,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBlitImage2(&self, pBlitImageInfo: &VkBlitImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBlitImage2.unwrap_unchecked()(self.raw, pBlitImageInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndRendering(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRendering.unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdResolveImage2(&self, pResolveImageInfo: &VkResolveImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResolveImage2.unwrap_unchecked()(self.raw, pResolveImageInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCullMode(&self, cullMode: VkCullModeFlags) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetCullMode.unwrap_unchecked()(self.raw, cullMode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthBiasEnable(&self, depthBiasEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthBiasEnable.unwrap_unchecked()(self.raw, depthBiasEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthBoundsTestEnable(&self, depthBoundsTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDepthBoundsTestEnable
        .unwrap_unchecked()(self.raw, depthBoundsTestEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthCompareOp(&self, depthCompareOp: VkCompareOp) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthCompareOp.unwrap_unchecked()(self.raw, depthCompareOp)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthTestEnable(&self, depthTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthTestEnable.unwrap_unchecked()(self.raw, depthTestEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDepthWriteEnable(&self, depthWriteEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDepthWriteEnable.unwrap_unchecked()(self.raw, depthWriteEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetFrontFace(&self, frontFace: VkFrontFace) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetFrontFace.unwrap_unchecked()(self.raw, frontFace)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPrimitiveRestartEnable(&self, primitiveRestartEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetPrimitiveRestartEnable
        .unwrap_unchecked()(self.raw, primitiveRestartEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPrimitiveTopology(&self, primitiveTopology: VkPrimitiveTopology) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetPrimitiveTopology.unwrap_unchecked()(self.raw, primitiveTopology)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRasterizerDiscardEnable(&self, rasterizerDiscardEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRasterizerDiscardEnable
        .unwrap_unchecked()(self.raw, rasterizerDiscardEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetScissorWithCount(&self, pScissors: &[VkRect2D]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetScissorWithCount.unwrap_unchecked()(
        self.raw,
        pScissors.len() as u32,
        pScissors.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetStencilOp(
    &self,
    faceMask: VkStencilFaceFlags,
    failOp: VkStencilOp,
    passOp: VkStencilOp,
    depthFailOp: VkStencilOp,
    compareOp: VkCompareOp,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilOp.unwrap_unchecked()(
        self.raw,
        faceMask,
        failOp,
        passOp,
        depthFailOp,
        compareOp,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetStencilTestEnable(&self, stencilTestEnable: VkBool32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetStencilTestEnable.unwrap_unchecked()(self.raw, stencilTestEnable)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewportWithCount(&self, pViewports: &[VkViewport]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetViewportWithCount.unwrap_unchecked()(
        self.raw,
        pViewports.len() as u32,
        pViewports.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindIndexBuffer2(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindIndexBuffer2.unwrap_unchecked()(
        self.raw, buffer, offset, size, indexType,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetLineStipple(&self, lineStippleFactor: u32, lineStipplePattern: u16) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLineStipple.unwrap_unchecked()(
        self.raw,
        lineStippleFactor,
        lineStipplePattern,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRenderingAttachmentLocations(
    &self,
    pLocationInfo: &VkRenderingAttachmentLocationInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRenderingAttachmentLocations
        .unwrap_unchecked()(self.raw, pLocationInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRenderingInputAttachmentIndices(
    &self,
    pInputAttachmentIndexInfo: &VkRenderingInputAttachmentIndexInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRenderingInputAttachmentIndices
        .unwrap_unchecked()(self.raw, pInputAttachmentIndexInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawClusterHUAWEI(&self, groupCountX: u32, groupCountY: u32, groupCountZ: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawClusterHUAWEI.unwrap_unchecked()(
        self.raw,
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawClusterIndirectHUAWEI(&self, buffer: VkBuffer, offset: VkDeviceSize) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawClusterIndirectHUAWEI
        .unwrap_unchecked()(self.raw, buffer, offset)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindInvocationMaskHUAWEI(&self, imageView: VkImageView, imageLayout: VkImageLayout) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindInvocationMaskHUAWEI
        .unwrap_unchecked()(self.raw, imageView, imageLayout)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSubpassShadingHUAWEI(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSubpassShadingHUAWEI.unwrap_unchecked()(self.raw)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPerformanceMarkerINTEL(
    &self,
    pMarkerInfo: &VkPerformanceMarkerInfoINTEL,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCmdSetPerformanceMarkerINTEL
        .unwrap_unchecked()(self.raw, pMarkerInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPerformanceOverrideINTEL(
    &self,
    pOverrideInfo: &VkPerformanceOverrideInfoINTEL,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCmdSetPerformanceOverrideINTEL
        .unwrap_unchecked()(self.raw, pOverrideInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetPerformanceStreamMarkerINTEL(
    &self,
    pMarkerInfo: &VkPerformanceStreamMarkerInfoINTEL,
  ) -> Result<VkResult, VkResult> {
    let r = unsafe {
      (self.table)
        .vkCmdSetPerformanceStreamMarkerINTEL
        .unwrap_unchecked()(self.raw, pMarkerInfo)
    };
    if r >= VkResult::VK_SUCCESS {
      Ok(r)
    } else {
      Err(r)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildAccelerationStructuresIndirectKHR(
    &self,
    pInfos: &[VkAccelerationStructureBuildGeometryInfoKHR],
    pIndirectDeviceAddresses: &[VkDeviceAddress],
    pIndirectStrides: &[u32],
    ppMaxPrimitiveCounts: *const *const u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBuildAccelerationStructuresIndirectKHR
        .unwrap_unchecked()(
        self.raw,
        pIndirectStrides.len() as u32,
        pInfos.as_ptr(),
        pIndirectDeviceAddresses.as_ptr(),
        pIndirectStrides.as_ptr(),
        ppMaxPrimitiveCounts,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildAccelerationStructuresKHR(
    &self,
    pInfos: &[VkAccelerationStructureBuildGeometryInfoKHR],
    ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBuildAccelerationStructuresKHR
        .unwrap_unchecked()(
        self.raw,
        pInfos.len() as u32,
        pInfos.as_ptr(),
        ppBuildRangeInfos,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyAccelerationStructureKHR(&self, pInfo: &VkCopyAccelerationStructureInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyAccelerationStructureKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyAccelerationStructureToMemoryKHR(
    &self,
    pInfo: &VkCopyAccelerationStructureToMemoryInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyAccelerationStructureToMemoryKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryToAccelerationStructureKHR(
    &self,
    pInfo: &VkCopyMemoryToAccelerationStructureInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyMemoryToAccelerationStructureKHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteAccelerationStructuresPropertiesKHR(
    &self,
    pAccelerationStructures: &[VkAccelerationStructureKHR],
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdWriteAccelerationStructuresPropertiesKHR
        .unwrap_unchecked()(
        self.raw,
        pAccelerationStructures.len() as u32,
        pAccelerationStructures.as_ptr(),
        queryType,
        queryPool,
        firstQuery,
      )
    }
  }
  /// [`vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pBlitImageInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[inline(always)]
  pub fn vkCmdBlitImage2KHR(&self, pBlitImageInfo: &VkBlitImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBlitImage2KHR.unwrap_unchecked()(self.raw, pBlitImageInfo)
    }
  }
  /// [`vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Transfer, Graphics, Compute
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pCopyBufferInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[deprecated(note = "superseded by `vkCmdCopyMemoryKHR`")]
  #[inline(always)]
  pub fn vkCmdCopyBuffer2KHR(&self, pCopyBufferInfo: &VkCopyBufferInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBuffer2KHR.unwrap_unchecked()(self.raw, pCopyBufferInfo)
    }
  }
  /// [`vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Transfer, Graphics, Compute
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pCopyBufferToImageInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[deprecated(note = "superseded by `vkCmdCopyMemoryToImageKHR`")]
  #[inline(always)]
  pub fn vkCmdCopyBufferToImage2KHR(&self, pCopyBufferToImageInfo: &VkCopyBufferToImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyBufferToImage2KHR.unwrap_unchecked()(self.raw, pCopyBufferToImageInfo)
    }
  }
  /// [`vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Transfer, Graphics, Compute
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pCopyImageInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[inline(always)]
  pub fn vkCmdCopyImage2KHR(&self, pCopyImageInfo: &VkCopyImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImage2KHR.unwrap_unchecked()(self.raw, pCopyImageInfo)
    }
  }
  /// [`vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Transfer, Graphics, Compute
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pCopyImageToBufferInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[deprecated(note = "superseded by `vkCmdCopyImageToMemoryKHR`")]
  #[inline(always)]
  pub fn vkCmdCopyImageToBuffer2KHR(&self, pCopyImageToBufferInfo: &VkCopyImageToBufferInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImageToBuffer2KHR.unwrap_unchecked()(self.raw, pCopyImageToBufferInfo)
    }
  }
  /// [`vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_copy_commands2`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pResolveImageInfo`
  #[cfg(feature = "VK_KHR_copy_commands2")]
  #[inline(always)]
  pub fn vkCmdResolveImage2KHR(&self, pResolveImageInfo: &VkResolveImageInfo2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResolveImage2KHR.unwrap_unchecked()(self.raw, pResolveImageInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryIndirectKHR(&self, pCopyMemoryIndirectInfo: &VkCopyMemoryIndirectInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMemoryIndirectKHR.unwrap_unchecked()(self.raw, pCopyMemoryIndirectInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryToImageIndirectKHR(
    &self,
    pCopyMemoryToImageIndirectInfo: &VkCopyMemoryToImageIndirectInfoKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyMemoryToImageIndirectKHR
        .unwrap_unchecked()(self.raw, pCopyMemoryToImageIndirectInfo)
    }
  }
  /// [`vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_create_renderpass2`
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
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  #[inline(always)]
  pub fn vkCmdBeginRenderPass2KHR(
    &self,
    pRenderPassBegin: &VkRenderPassBeginInfo,
    pSubpassBeginInfo: &VkSubpassBeginInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginRenderPass2KHR.unwrap_unchecked()(
        self.raw,
        pRenderPassBegin,
        pSubpassBeginInfo,
      )
    }
  }
  /// [`vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_create_renderpass2`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Inside
  /// - **Tasks:** Action, State, Synchronization
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pSubpassEndInfo`
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  #[inline(always)]
  pub fn vkCmdEndRenderPass2KHR(&self, pSubpassEndInfo: &VkSubpassEndInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRenderPass2KHR.unwrap_unchecked()(self.raw, pSubpassEndInfo)
    }
  }
  /// [`vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_create_renderpass2`
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
  #[cfg(feature = "VK_KHR_create_renderpass2")]
  #[inline(always)]
  pub fn vkCmdNextSubpass2KHR(
    &self,
    pSubpassBeginInfo: &VkSubpassBeginInfo,
    pSubpassEndInfo: &VkSubpassEndInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdNextSubpass2KHR.unwrap_unchecked()(
        self.raw,
        pSubpassBeginInfo,
        pSubpassEndInfo,
      )
    }
  }
  /// [`vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html)
  ///
  /// Provided by:
  /// - `VK_KHR_descriptor_update_template`
  /// - `VK_KHR_push_descriptor`
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
  #[cfg(any(
    feature = "VK_KHR_descriptor_update_template",
    feature = "VK_KHR_push_descriptor"
  ))]
  #[inline(always)]
  pub fn vkCmdPushDescriptorSetWithTemplateKHR(
    &self,
    descriptorUpdateTemplate: VkDescriptorUpdateTemplate,
    layout: VkPipelineLayout,
    set: u32,
    pData: *const core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPushDescriptorSetWithTemplateKHR
        .unwrap_unchecked()(self.raw, descriptorUpdateTemplate, layout, set, pData)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginConditionalRendering2EXT(
    &self,
    pConditionalRenderingBegin: &VkConditionalRenderingBeginInfo2EXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginConditionalRendering2EXT
        .unwrap_unchecked()(self.raw, pConditionalRenderingBegin)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginTransformFeedback2EXT(
    &self,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginTransformFeedback2EXT
        .unwrap_unchecked()(
        self.raw,
        firstCounterRange,
        counterRangeCount,
        pCounterInfos,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindIndexBuffer3KHR(&self, pInfo: &VkBindIndexBuffer3InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindIndexBuffer3KHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindTransformFeedbackBuffers2EXT(
    &self,
    firstBinding: u32,
    bindingCount: u32,
    pBindingInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindTransformFeedbackBuffers2EXT
        .unwrap_unchecked()(self.raw, firstBinding, bindingCount, pBindingInfos)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindVertexBuffers3KHR(
    &self,
    firstBinding: u32,
    pBindingInfos: &[VkBindVertexBuffer3InfoKHR],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindVertexBuffers3KHR.unwrap_unchecked()(
        self.raw,
        firstBinding,
        pBindingInfos.len() as u32,
        pBindingInfos.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyImageToMemoryKHR(&self, pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyImageToMemoryKHR.unwrap_unchecked()(self.raw, pCopyMemoryInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryKHR(&self, pCopyMemoryInfo: *const VkCopyDeviceMemoryInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMemoryKHR.unwrap_unchecked()(self.raw, pCopyMemoryInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryToImageKHR(&self, pCopyMemoryInfo: *const VkCopyDeviceMemoryImageInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMemoryToImageKHR.unwrap_unchecked()(self.raw, pCopyMemoryInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyQueryPoolResultsToMemoryKHR(
    &self,
    queryPool: VkQueryPool,
    firstQuery: u32,
    queryCount: u32,
    pDstRange: &VkStridedDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    queryResultFlags: VkQueryResultFlags,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyQueryPoolResultsToMemoryKHR
        .unwrap_unchecked()(
        self.raw,
        queryPool,
        firstQuery,
        queryCount,
        pDstRange,
        dstFlags,
        queryResultFlags,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchIndirect2KHR(&self, pInfo: &VkDispatchIndirect2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchIndirect2KHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirect2KHR(&self, pInfo: &VkDrawIndirect2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndexedIndirect2KHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirectCount2KHR(&self, pInfo: &VkDrawIndirectCount2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndexedIndirectCount2KHR
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirect2KHR(&self, pInfo: &VkDrawIndirect2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirect2KHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirectByteCount2EXT(
    &self,
    instanceCount: u32,
    firstInstance: u32,
    pCounterInfo: &VkBindTransformFeedbackBuffer2InfoEXT,
    counterOffset: u32,
    vertexStride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndirectByteCount2EXT
        .unwrap_unchecked()(
        self.raw,
        instanceCount,
        firstInstance,
        pCounterInfo,
        counterOffset,
        vertexStride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawIndirectCount2KHR(&self, pInfo: &VkDrawIndirectCount2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirectCount2KHR.unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirect2EXT(&self, pInfo: &VkDrawIndirect2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawMeshTasksIndirect2EXT
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirectCount2EXT(&self, pInfo: &VkDrawIndirectCount2InfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawMeshTasksIndirectCount2EXT
        .unwrap_unchecked()(self.raw, pInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndTransformFeedback2EXT(
    &self,
    firstCounterRange: u32,
    counterRangeCount: u32,
    pCounterInfos: *const VkBindTransformFeedbackBuffer2InfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdEndTransformFeedback2EXT
        .unwrap_unchecked()(
        self.raw,
        firstCounterRange,
        counterRangeCount,
        pCounterInfos,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdFillMemoryKHR(
    &self,
    pDstRange: &VkDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    data: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdFillMemoryKHR.unwrap_unchecked()(self.raw, pDstRange, dstFlags, data)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdUpdateMemoryKHR(
    &self,
    pDstRange: &VkDeviceAddressRangeKHR,
    dstFlags: VkAddressCommandFlagsKHR,
    dataSize: VkDeviceSize,
    pData: *const core::ffi::c_void,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdUpdateMemoryKHR.unwrap_unchecked()(
        self.raw, pDstRange, dstFlags, dataSize, pData,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteMarkerToMemoryAMD(&self, pInfo: &VkMemoryMarkerInfoAMD) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteMarkerToMemoryAMD.unwrap_unchecked()(self.raw, pInfo)
    }
  }
  /// [`vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
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
  #[cfg(feature = "VK_KHR_device_group")]
  #[inline(always)]
  pub fn vkCmdDispatchBaseKHR(
    &self,
    baseGroupX: u32,
    baseGroupY: u32,
    baseGroupZ: u32,
    groupCountX: u32,
    groupCountY: u32,
    groupCountZ: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchBaseKHR.unwrap_unchecked()(
        self.raw,
        baseGroupX,
        baseGroupY,
        baseGroupZ,
        groupCountX,
        groupCountY,
        groupCountZ,
      )
    }
  }
  /// [`vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html)
  ///
  /// Provided by:
  /// - `VK_KHR_device_group`
  ///
  /// - **Queues:** Graphics, Compute, Transfer
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan, VulkanSC
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `deviceMask`
  #[cfg(feature = "VK_KHR_device_group")]
  #[inline(always)]
  pub fn vkCmdSetDeviceMaskKHR(&self, deviceMask: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetDeviceMaskKHR.unwrap_unchecked()(self.raw, deviceMask)
    }
  }
  /// [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html)
  ///
  /// Provided by:
  /// - `VK_KHR_draw_indirect_count`
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
  #[cfg(feature = "VK_KHR_draw_indirect_count")]
  #[deprecated(note = "superseded by `vkCmdDrawIndexedIndirectCount2KHR`")]
  #[inline(always)]
  pub fn vkCmdDrawIndexedIndirectCountKHR(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawIndexedIndirectCountKHR
        .unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
  /// [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html)
  ///
  /// Provided by:
  /// - `VK_KHR_draw_indirect_count`
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
  #[cfg(feature = "VK_KHR_draw_indirect_count")]
  #[deprecated(note = "superseded by `vkCmdDrawIndirectCount2KHR`")]
  #[inline(always)]
  pub fn vkCmdDrawIndirectCountKHR(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawIndirectCountKHR.unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
  /// [`vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html)
  ///
  /// Provided by:
  /// - `VK_KHR_dynamic_rendering`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Outside
  /// - **Tasks:** Action, State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pRenderingInfo`
  #[cfg(feature = "VK_KHR_dynamic_rendering")]
  #[inline(always)]
  pub fn vkCmdBeginRenderingKHR(&self, pRenderingInfo: &VkRenderingInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginRenderingKHR.unwrap_unchecked()(self.raw, pRenderingInfo)
    }
  }
  /// [`vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html)
  ///
  /// Provided by:
  /// - `VK_KHR_dynamic_rendering`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Inside
  /// - **Tasks:** Action, State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  #[cfg(feature = "VK_KHR_dynamic_rendering")]
  #[inline(always)]
  pub fn vkCmdEndRenderingKHR(&self) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRenderingKHR.unwrap_unchecked()(self.raw)
    }
  }
  /// [`vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html)
  ///
  /// Provided by:
  /// - `VK_KHR_dynamic_rendering_local_read`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Inside
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pLocationInfo`
  #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
  #[inline(always)]
  pub fn vkCmdSetRenderingAttachmentLocationsKHR(
    &self,
    pLocationInfo: &VkRenderingAttachmentLocationInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRenderingAttachmentLocationsKHR
        .unwrap_unchecked()(self.raw, pLocationInfo)
    }
  }
  /// [`vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html)
  ///
  /// Provided by:
  /// - `VK_KHR_dynamic_rendering_local_read`
  ///
  /// - **Queues:** Graphics
  /// - **Render Pass:** Inside
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pInputAttachmentIndexInfo`
  #[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
  #[inline(always)]
  pub fn vkCmdSetRenderingInputAttachmentIndicesKHR(
    &self,
    pInputAttachmentIndexInfo: &VkRenderingInputAttachmentIndexInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRenderingInputAttachmentIndicesKHR
        .unwrap_unchecked()(self.raw, pInputAttachmentIndexInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetFragmentShadingRateKHR(
    &self,
    pFragmentSize: &VkExtent2D,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetFragmentShadingRateKHR
        .unwrap_unchecked()(self.raw, pFragmentSize, combinerOps)
    }
  }
  /// [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html)
  ///
  /// Provided by:
  /// - `VK_KHR_line_rasterization`
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
  #[cfg(feature = "VK_KHR_line_rasterization")]
  #[inline(always)]
  pub fn vkCmdSetLineStippleKHR(&self, lineStippleFactor: u32, lineStipplePattern: u16) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetLineStippleKHR.unwrap_unchecked()(
        self.raw,
        lineStippleFactor,
        lineStipplePattern,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndRendering2KHR(&self, pRenderingEndInfo: *const VkRenderingEndInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndRendering2KHR.unwrap_unchecked()(self.raw, pRenderingEndInfo)
    }
  }
  /// [`vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance5`
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
  #[cfg(feature = "VK_KHR_maintenance5")]
  #[deprecated(note = "superseded by `vkCmdBindIndexBuffer3KHR`")]
  #[inline(always)]
  pub fn vkCmdBindIndexBuffer2KHR(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    indexType: VkIndexType,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindIndexBuffer2KHR.unwrap_unchecked()(
        self.raw, buffer, offset, size, indexType,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindDescriptorBufferEmbeddedSamplers2EXT(
    &self,
    pBindDescriptorBufferEmbeddedSamplersInfo: &VkBindDescriptorBufferEmbeddedSamplersInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindDescriptorBufferEmbeddedSamplers2EXT
        .unwrap_unchecked()(self.raw, pBindDescriptorBufferEmbeddedSamplersInfo)
    }
  }
  /// [`vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance6`
  ///
  /// - **Queues:** Graphics, Compute
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pBindDescriptorSetsInfo`
  #[cfg(feature = "VK_KHR_maintenance6")]
  #[inline(always)]
  pub fn vkCmdBindDescriptorSets2KHR(&self, pBindDescriptorSetsInfo: &VkBindDescriptorSetsInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindDescriptorSets2KHR.unwrap_unchecked()(self.raw, pBindDescriptorSetsInfo)
    }
  }
  /// [`vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance6`
  ///
  /// - **Queues:** Graphics, Compute
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pPushConstantsInfo`
  #[cfg(feature = "VK_KHR_maintenance6")]
  #[inline(always)]
  pub fn vkCmdPushConstants2KHR(&self, pPushConstantsInfo: &VkPushConstantsInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushConstants2KHR.unwrap_unchecked()(self.raw, pPushConstantsInfo)
    }
  }
  /// [`vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance6`
  ///
  /// - **Queues:** Graphics, Compute
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pPushDescriptorSetInfo`
  #[cfg(feature = "VK_KHR_maintenance6")]
  #[inline(always)]
  pub fn vkCmdPushDescriptorSet2KHR(&self, pPushDescriptorSetInfo: &VkPushDescriptorSetInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushDescriptorSet2KHR.unwrap_unchecked()(self.raw, pPushDescriptorSetInfo)
    }
  }
  /// [`vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_maintenance6`
  ///
  /// - **Queues:** Graphics, Compute
  /// - **Render Pass:** Both
  /// - **Tasks:** State
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pPushDescriptorSetWithTemplateInfo`
  #[cfg(feature = "VK_KHR_maintenance6")]
  #[inline(always)]
  pub fn vkCmdPushDescriptorSetWithTemplate2KHR(
    &self,
    pPushDescriptorSetWithTemplateInfo: &VkPushDescriptorSetWithTemplateInfo,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPushDescriptorSetWithTemplate2KHR
        .unwrap_unchecked()(self.raw, pPushDescriptorSetWithTemplateInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetDescriptorBufferOffsets2EXT(
    &self,
    pSetDescriptorBufferOffsetsInfo: &VkSetDescriptorBufferOffsetsInfoEXT,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetDescriptorBufferOffsets2EXT
        .unwrap_unchecked()(self.raw, pSetDescriptorBufferOffsetsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdRefreshObjectsKHR(&self, pRefreshObjects: &VkRefreshObjectListKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdRefreshObjectsKHR.unwrap_unchecked()(self.raw, pRefreshObjects)
    }
  }
  /// [`vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html)
  ///
  /// Provided by:
  /// - `VK_KHR_push_descriptor`
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
  #[cfg(feature = "VK_KHR_push_descriptor")]
  #[inline(always)]
  pub fn vkCmdPushDescriptorSetKHR(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    set: u32,
    pDescriptorWrites: &[VkWriteDescriptorSet],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPushDescriptorSetKHR.unwrap_unchecked()(
        self.raw,
        pipelineBindPoint,
        layout,
        set,
        pDescriptorWrites.len() as u32,
        pDescriptorWrites.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdTraceRaysIndirect2KHR(&self, indirectDeviceAddress: VkDeviceAddress) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdTraceRaysIndirect2KHR.unwrap_unchecked()(self.raw, indirectDeviceAddress)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetRayTracingPipelineStackSizeKHR(&self, pipelineStackSize: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetRayTracingPipelineStackSizeKHR
        .unwrap_unchecked()(self.raw, pipelineStackSize)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdTraceRaysIndirectKHR(
    &self,
    pRaygenShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    indirectDeviceAddress: VkDeviceAddress,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdTraceRaysIndirectKHR.unwrap_unchecked()(
        self.raw,
        pRaygenShaderBindingTable,
        pMissShaderBindingTable,
        pHitShaderBindingTable,
        pCallableShaderBindingTable,
        indirectDeviceAddress,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdTraceRaysKHR(
    &self,
    pRaygenShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pMissShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pHitShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    pCallableShaderBindingTable: &VkStridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdTraceRaysKHR.unwrap_unchecked()(
        self.raw,
        pRaygenShaderBindingTable,
        pMissShaderBindingTable,
        pHitShaderBindingTable,
        pCallableShaderBindingTable,
        width,
        height,
        depth,
      )
    }
  }
  /// [`vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_synchronization2`
  ///
  /// - **Queues:** Transfer, Graphics, Compute, VideoDecodeKHR, VideoEncodeKHR
  /// - **Render Pass:** Both
  /// - **Tasks:** Synchronization
  /// - **Export Scopes:** Vulkan
  ///
  /// # Parameters
  /// - `commandBuffer`
  /// - `pDependencyInfo`
  #[cfg(feature = "VK_KHR_synchronization2")]
  #[inline(always)]
  pub fn vkCmdPipelineBarrier2KHR(&self, pDependencyInfo: &VkDependencyInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdPipelineBarrier2KHR.unwrap_unchecked()(self.raw, pDependencyInfo)
    }
  }
  /// [`vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_synchronization2`
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
  #[cfg(feature = "VK_KHR_synchronization2")]
  #[inline(always)]
  pub fn vkCmdResetEvent2KHR(&self, event: VkEvent, stageMask: VkPipelineStageFlags2) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdResetEvent2KHR.unwrap_unchecked()(self.raw, event, stageMask)
    }
  }
  /// [`vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_synchronization2`
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
  #[cfg(feature = "VK_KHR_synchronization2")]
  #[inline(always)]
  pub fn vkCmdSetEvent2KHR(&self, event: VkEvent, pDependencyInfo: &VkDependencyInfo) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetEvent2KHR.unwrap_unchecked()(self.raw, event, pDependencyInfo)
    }
  }
  /// [`vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_synchronization2`
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
  #[cfg(feature = "VK_KHR_synchronization2")]
  #[inline(always)]
  pub fn vkCmdWaitEvents2KHR(&self, pEvents: &[VkEvent], pDependencyInfos: &[VkDependencyInfo]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWaitEvents2KHR.unwrap_unchecked()(
        self.raw,
        pDependencyInfos.len() as u32,
        pEvents.as_ptr(),
        pDependencyInfos.as_ptr(),
      )
    }
  }
  /// [`vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html)
  ///
  /// Provided by:
  /// - `VK_KHR_synchronization2`
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
  #[cfg(feature = "VK_KHR_synchronization2")]
  #[inline(always)]
  pub fn vkCmdWriteTimestamp2KHR(
    &self,
    stage: VkPipelineStageFlags2,
    queryPool: VkQueryPool,
    query: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdWriteTimestamp2KHR.unwrap_unchecked()(self.raw, stage, queryPool, query)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDecodeVideoKHR(&self, pDecodeInfo: &VkVideoDecodeInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDecodeVideoKHR.unwrap_unchecked()(self.raw, pDecodeInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEncodeVideoKHR(&self, pEncodeInfo: &VkVideoEncodeInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEncodeVideoKHR.unwrap_unchecked()(self.raw, pEncodeInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginVideoCodingKHR(&self, pBeginInfo: &VkVideoBeginCodingInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBeginVideoCodingKHR.unwrap_unchecked()(self.raw, pBeginInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdControlVideoCodingKHR(&self, pCodingControlInfo: &VkVideoCodingControlInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdControlVideoCodingKHR.unwrap_unchecked()(self.raw, pCodingControlInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndVideoCodingKHR(&self, pEndCodingInfo: &VkVideoEndCodingInfoKHR) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndVideoCodingKHR.unwrap_unchecked()(self.raw, pEndCodingInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCuLaunchKernelNVX(&self, pLaunchInfo: &VkCuLaunchInfoNVX) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCuLaunchKernelNVX.unwrap_unchecked()(self.raw, pLaunchInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewportWScalingNV(
    &self,
    firstViewport: u32,
    pViewportWScalings: &[VkViewportWScalingNV],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetViewportWScalingNV.unwrap_unchecked()(
        self.raw,
        firstViewport,
        pViewportWScalings.len() as u32,
        pViewportWScalings.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildClusterAccelerationStructureIndirectNV(
    &self,
    pCommandInfos: &VkClusterAccelerationStructureCommandsInfoNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBuildClusterAccelerationStructureIndirectNV
        .unwrap_unchecked()(self.raw, pCommandInfos)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetComputeOccupancyPriorityNV(
    &self,
    pParameters: &VkComputeOccupancyPriorityParametersNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetComputeOccupancyPriorityNV
        .unwrap_unchecked()(self.raw, pParameters)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdConvertCooperativeVectorMatrixNV(
    &self,
    pInfos: &[VkConvertCooperativeVectorMatrixInfoNV],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdConvertCooperativeVectorMatrixNV
        .unwrap_unchecked()(self.raw, pInfos.len() as u32, pInfos.as_ptr())
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryIndirectNV(
    &self,
    copyBufferAddress: VkDeviceAddress,
    copyCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCopyMemoryIndirectNV.unwrap_unchecked()(
        self.raw,
        copyBufferAddress,
        copyCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyMemoryToImageIndirectNV(
    &self,
    copyBufferAddress: VkDeviceAddress,
    stride: u32,
    dstImage: VkImage,
    dstImageLayout: VkImageLayout,
    pImageSubresources: &[VkImageSubresourceLayers],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyMemoryToImageIndirectNV
        .unwrap_unchecked()(
        self.raw,
        copyBufferAddress,
        pImageSubresources.len() as u32,
        stride,
        dstImage,
        dstImageLayout,
        pImageSubresources.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCudaLaunchKernelNV(&self, pLaunchInfo: &VkCudaLaunchInfoNV) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdCudaLaunchKernelNV.unwrap_unchecked()(self.raw, pLaunchInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCheckpointNV(&self, pCheckpointMarker: *const core::ffi::c_void) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetCheckpointNV.unwrap_unchecked()(self.raw, pCheckpointMarker)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindPipelineShaderGroupNV(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
    groupIndex: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBindPipelineShaderGroupNV
        .unwrap_unchecked()(self.raw, pipelineBindPoint, pipeline, groupIndex)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdExecuteGeneratedCommandsNV(
    &self,
    isPreprocessed: VkBool32,
    pGeneratedCommandsInfo: &VkGeneratedCommandsInfoNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdExecuteGeneratedCommandsNV
        .unwrap_unchecked()(self.raw, isPreprocessed, pGeneratedCommandsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdPreprocessGeneratedCommandsNV(
    &self,
    pGeneratedCommandsInfo: &VkGeneratedCommandsInfoNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdPreprocessGeneratedCommandsNV
        .unwrap_unchecked()(self.raw, pGeneratedCommandsInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdUpdatePipelineIndirectBufferNV(
    &self,
    pipelineBindPoint: VkPipelineBindPoint,
    pipeline: VkPipeline,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdUpdatePipelineIndirectBufferNV
        .unwrap_unchecked()(self.raw, pipelineBindPoint, pipeline)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetFragmentShadingRateEnumNV(
    &self,
    shadingRate: VkFragmentShadingRateNV,
    combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetFragmentShadingRateEnumNV
        .unwrap_unchecked()(self.raw, shadingRate, combinerOps)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDecompressMemoryIndirectCountNV(
    &self,
    indirectCommandsAddress: VkDeviceAddress,
    indirectCommandsCountAddress: VkDeviceAddress,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDecompressMemoryIndirectCountNV
        .unwrap_unchecked()(
        self.raw,
        indirectCommandsAddress,
        indirectCommandsCountAddress,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDecompressMemoryNV(&self, pDecompressMemoryRegions: &[VkDecompressMemoryRegionNV]) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDecompressMemoryNV.unwrap_unchecked()(
        self.raw,
        pDecompressMemoryRegions.len() as u32,
        pDecompressMemoryRegions.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirectCountNV(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    countBuffer: VkBuffer,
    countBufferOffset: VkDeviceSize,
    maxDrawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdDrawMeshTasksIndirectCountNV
        .unwrap_unchecked()(
        self.raw,
        buffer,
        offset,
        countBuffer,
        countBufferOffset,
        maxDrawCount,
        stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksIndirectNV(
    &self,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    drawCount: u32,
    stride: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawMeshTasksIndirectNV.unwrap_unchecked()(
        self.raw, buffer, offset, drawCount, stride,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDrawMeshTasksNV(&self, taskCount: u32, firstTask: u32) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDrawMeshTasksNV.unwrap_unchecked()(self.raw, taskCount, firstTask)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdOpticalFlowExecuteNV(
    &self,
    session: VkOpticalFlowSessionNV,
    pExecuteInfo: &VkOpticalFlowExecuteInfoNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdOpticalFlowExecuteNV.unwrap_unchecked()(self.raw, session, pExecuteInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildPartitionedAccelerationStructuresNV(
    &self,
    pBuildInfo: &VkBuildPartitionedAccelerationStructureInfoNV,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBuildPartitionedAccelerationStructuresNV
        .unwrap_unchecked()(self.raw, pBuildInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBuildAccelerationStructureNV(
    &self,
    pInfo: &VkAccelerationStructureInfoNV,
    instanceData: VkBuffer,
    instanceOffset: VkDeviceSize,
    update: VkBool32,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    scratch: VkBuffer,
    scratchOffset: VkDeviceSize,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBuildAccelerationStructureNV
        .unwrap_unchecked()(
        self.raw,
        pInfo,
        instanceData,
        instanceOffset,
        update,
        dst,
        src,
        scratch,
        scratchOffset,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdCopyAccelerationStructureNV(
    &self,
    dst: VkAccelerationStructureNV,
    src: VkAccelerationStructureNV,
    mode: VkCopyAccelerationStructureModeKHR,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdCopyAccelerationStructureNV
        .unwrap_unchecked()(self.raw, dst, src, mode)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdTraceRaysNV(
    &self,
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
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdTraceRaysNV.unwrap_unchecked()(
        self.raw,
        raygenShaderBindingTableBuffer,
        raygenShaderBindingOffset,
        missShaderBindingTableBuffer,
        missShaderBindingOffset,
        missShaderBindingStride,
        hitShaderBindingTableBuffer,
        hitShaderBindingOffset,
        hitShaderBindingStride,
        callableShaderBindingTableBuffer,
        callableShaderBindingOffset,
        callableShaderBindingStride,
        width,
        height,
        depth,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdWriteAccelerationStructuresPropertiesNV(
    &self,
    pAccelerationStructures: &[VkAccelerationStructureNV],
    queryType: VkQueryType,
    queryPool: VkQueryPool,
    firstQuery: u32,
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdWriteAccelerationStructuresPropertiesNV
        .unwrap_unchecked()(
        self.raw,
        pAccelerationStructures.len() as u32,
        pAccelerationStructures.as_ptr(),
        queryType,
        queryPool,
        firstQuery,
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetExclusiveScissorEnableNV(
    &self,
    firstExclusiveScissor: u32,
    pExclusiveScissorEnables: &[VkBool32],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetExclusiveScissorEnableNV
        .unwrap_unchecked()(
        self.raw,
        firstExclusiveScissor,
        pExclusiveScissorEnables.len() as u32,
        pExclusiveScissorEnables.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetExclusiveScissorNV(
    &self,
    firstExclusiveScissor: u32,
    pExclusiveScissors: &[VkRect2D],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetExclusiveScissorNV.unwrap_unchecked()(
        self.raw,
        firstExclusiveScissor,
        pExclusiveScissors.len() as u32,
        pExclusiveScissors.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindShadingRateImageNV(&self, imageView: VkImageView, imageLayout: VkImageLayout) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindShadingRateImageNV.unwrap_unchecked()(self.raw, imageView, imageLayout)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetCoarseSampleOrderNV(
    &self,
    sampleOrderType: VkCoarseSampleOrderTypeNV,
    pCustomSampleOrders: &[VkCoarseSampleOrderCustomNV],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdSetCoarseSampleOrderNV.unwrap_unchecked()(
        self.raw,
        sampleOrderType,
        pCustomSampleOrders.len() as u32,
        pCustomSampleOrders.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdSetViewportShadingRatePaletteNV(
    &self,
    firstViewport: u32,
    pShadingRatePalettes: &[VkShadingRatePaletteNV],
  ) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdSetViewportShadingRatePaletteNV
        .unwrap_unchecked()(
        self.raw,
        firstViewport,
        pShadingRatePalettes.len() as u32,
        pShadingRatePalettes.as_ptr(),
      )
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBindTileMemoryQCOM(&self, pTileMemoryBindInfo: *const VkTileMemoryBindInfoQCOM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdBindTileMemoryQCOM.unwrap_unchecked()(self.raw, pTileMemoryBindInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdBeginPerTileExecutionQCOM(&self, pPerTileBeginInfo: &VkPerTileBeginInfoQCOM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table)
        .vkCmdBeginPerTileExecutionQCOM
        .unwrap_unchecked()(self.raw, pPerTileBeginInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdDispatchTileQCOM(&self, pDispatchTileInfo: &VkDispatchTileInfoQCOM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdDispatchTileQCOM.unwrap_unchecked()(self.raw, pDispatchTileInfo)
    }
  }
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
  #[inline(always)]
  pub fn vkCmdEndPerTileExecutionQCOM(&self, pPerTileEndInfo: &VkPerTileEndInfoQCOM) {
    unsafe {
      // SAFETY: table is fully loaded at creation.
      (self.table).vkCmdEndPerTileExecutionQCOM.unwrap_unchecked()(self.raw, pPerTileEndInfo)
    }
  }
}
