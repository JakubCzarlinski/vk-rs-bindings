//! Vulkan API constants, version helpers, and extension version/name constants.
/// [VK_MAKE_VIDEO_STD_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_VIDEO_STD_VERSION.html)
#[inline]
pub const fn VK_MAKE_VIDEO_STD_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
/// [VK_MAX_PHYSICAL_DEVICE_NAME_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_NAME_SIZE.html)
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
/// [VK_UUID_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_UUID_SIZE.html)
pub const VK_UUID_SIZE: u32 = 16;
/// [VK_LUID_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUID_SIZE.html)
pub const VK_LUID_SIZE: u32 = 8;
/// [VK_MAX_EXTENSION_NAME_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_EXTENSION_NAME_SIZE.html)
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
/// [VK_MAX_DESCRIPTION_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DESCRIPTION_SIZE.html)
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
/// [VK_MAX_MEMORY_TYPES](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_TYPES.html)
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
/// [VK_MAX_MEMORY_HEAPS](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_MEMORY_HEAPS.html)
///
/// The maximum number of unique memory heaps, each of which supporting 1 or more memory types
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
/// [VK_LOD_CLAMP_NONE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LOD_CLAMP_NONE.html)
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0f32;
/// [VK_REMAINING_MIP_LEVELS](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_MIP_LEVELS.html)
pub const VK_REMAINING_MIP_LEVELS: u32 = u32::MAX;
/// [VK_REMAINING_ARRAY_LAYERS](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_ARRAY_LAYERS.html)
pub const VK_REMAINING_ARRAY_LAYERS: u32 = u32::MAX;
/// [VK_REMAINING_3D_SLICES_EXT](https://docs.vulkan.org/refpages/latest/refpages/source/VK_REMAINING_3D_SLICES_EXT.html)
pub const VK_REMAINING_3D_SLICES_EXT: u32 = u32::MAX;
/// [VK_WHOLE_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_WHOLE_SIZE.html)
pub const VK_WHOLE_SIZE: u64 = u64::MAX;
/// [VK_ATTACHMENT_UNUSED](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ATTACHMENT_UNUSED.html)
pub const VK_ATTACHMENT_UNUSED: u32 = u32::MAX;
/// [VK_TRUE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_TRUE.html)
pub const VK_TRUE: u32 = 1;
/// [VK_FALSE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FALSE.html)
pub const VK_FALSE: u32 = 0;
/// [VK_QUEUE_FAMILY_IGNORED](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_IGNORED.html)
pub const VK_QUEUE_FAMILY_IGNORED: u32 = u32::MAX;
/// [VK_QUEUE_FAMILY_EXTERNAL](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_EXTERNAL.html)
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = u32::MAX;
/// [VK_QUEUE_FAMILY_FOREIGN_EXT](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QUEUE_FAMILY_FOREIGN_EXT.html)
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = u32::MAX;
/// [VK_SUBPASS_EXTERNAL](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SUBPASS_EXTERNAL.html)
pub const VK_SUBPASS_EXTERNAL: u32 = u32::MAX;
/// [VK_MAX_DEVICE_GROUP_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DEVICE_GROUP_SIZE.html)
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;
/// [VK_MAX_DRIVER_NAME_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_NAME_SIZE.html)
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
/// [VK_MAX_DRIVER_INFO_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_DRIVER_INFO_SIZE.html)
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
/// [VK_SHADER_UNUSED_KHR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_UNUSED_KHR.html)
pub const VK_SHADER_UNUSED_KHR: u32 = u32::MAX;
/// [VK_MAX_GLOBAL_PRIORITY_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_GLOBAL_PRIORITY_SIZE.html)
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
/// [VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT.html)
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
/// [VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR.html)
pub const VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
/// [VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR.html)
pub const VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
/// [VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR.html)
pub const VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
/// [VK_SHADER_INDEX_UNUSED_AMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SHADER_INDEX_UNUSED_AMDX.html)
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = u32::MAX;
/// [VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV](https://docs.vulkan.org/refpages/latest/refpages/source/VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV.html)
pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = u32::MAX;
/// [VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX.html)
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
/// [VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX.html)
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
/// [VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM.html)
pub const VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
/// [VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM](https://docs.vulkan.org/refpages/latest/refpages/source/VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM.html)
pub const VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
/// [VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV.html)
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25f32;
/// [VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV.html)
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50f32;
/// [VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV](https://docs.vulkan.org/refpages/latest/refpages/source/VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV.html)
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75f32;
/// [VKSC_API_VARIANT](https://docs.vulkan.org/refpages/latest/refpages/source/VKSC_API_VARIANT.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
pub const VKSC_API_VARIANT: u32 = 1u32;
/// [VKSC_API_VERSION_1_0](https://docs.vulkan.org/refpages/latest/refpages/source/VKSC_API_VERSION_1_0.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
pub const VKSC_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 0u32, 0u32);
/// [VK_HEADER_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
pub const VK_HEADER_VERSION: u32 = 20u32;
/// [VK_HEADER_VERSION_COMPLETE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION_COMPLETE.html)
#[cfg(feature = "VKSC_VERSION_1_0")]
pub const VK_HEADER_VERSION_COMPLETE: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 0u32, 0u32);
/// [VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION.html)
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
pub const VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION: u32 = 1;
/// [VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMDX_dense_geometry_format")]
pub const VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMDX_dense_geometry_format";
/// [VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub const VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION: u32 = 2;
/// [VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMDX_shader_enqueue")]
pub const VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMDX_shader_enqueue";
/// [VK_AMD_ANTI_LAG_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_ANTI_LAG_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_anti_lag")]
pub const VK_AMD_ANTI_LAG_SPEC_VERSION: u32 = 1;
/// [VK_AMD_ANTI_LAG_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_ANTI_LAG_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_anti_lag")]
pub const VK_AMD_ANTI_LAG_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_AMD_anti_lag";
/// [VK_AMD_BUFFER_MARKER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_BUFFER_MARKER_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_buffer_marker")]
pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
/// [VK_AMD_BUFFER_MARKER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_BUFFER_MARKER_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_buffer_marker")]
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_AMD_buffer_marker";
/// [VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_device_coherent_memory")]
pub const VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_device_coherent_memory")]
pub const VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_device_coherent_memory";
/// [VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_display_native_hdr")]
pub const VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
/// [VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_display_native_hdr")]
pub const VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_display_native_hdr";
/// [VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
/// [VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_draw_indirect_count")]
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_draw_indirect_count";
/// [VK_AMD_GCN_SHADER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GCN_SHADER_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_gcn_shader")]
pub const VK_AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;
/// [VK_AMD_GCN_SHADER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GCN_SHADER_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_gcn_shader")]
pub const VK_AMD_GCN_SHADER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_AMD_gcn_shader";
/// [VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;
/// [VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_gpu_shader_half_float")]
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_gpu_shader_half_float";
/// [VK_AMD_GPU_SHADER_INT16_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GPU_SHADER_INT16_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
pub const VK_AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 2;
/// [VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_gpu_shader_int16")]
pub const VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_gpu_shader_int16";
/// [VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
pub const VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
/// [VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_memory_overallocation_behavior")]
pub const VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_memory_overallocation_behavior";
/// [VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;
/// [VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_mixed_attachment_samples")]
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_mixed_attachment_samples";
/// [VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_negative_viewport_height")]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: u32 = 1;
/// [VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_negative_viewport_height")]
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_negative_viewport_height";
/// [VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
pub const VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_pipeline_compiler_control")]
pub const VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_pipeline_compiler_control";
/// [VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_rasterization_order")]
pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
/// [VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_rasterization_order")]
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_rasterization_order";
/// [VK_AMD_SHADER_BALLOT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_BALLOT_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_ballot")]
pub const VK_AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_BALLOT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_BALLOT_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_ballot")]
pub const VK_AMD_SHADER_BALLOT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_AMD_shader_ballot";
/// [VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_core_properties")]
pub const VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
/// [VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_core_properties")]
pub const VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_core_properties";
/// [VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_core_properties2")]
pub const VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_core_properties2")]
pub const VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_core_properties2";
/// [VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
pub const VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_early_and_late_fragment_tests")]
pub const VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_early_and_late_fragment_tests";
/// [VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_explicit_vertex_parameter")]
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_explicit_vertex_parameter";
/// [VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
pub const VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_fragment_mask")]
pub const VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_fragment_mask";
/// [VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_image_load_store_lod")]
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_image_load_store_lod";
/// [VK_AMD_SHADER_INFO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_INFO_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_info")]
pub const VK_AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_INFO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_INFO_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_info")]
pub const VK_AMD_SHADER_INFO_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_AMD_shader_info";
/// [VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
pub const VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;
/// [VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_shader_trinary_minmax")]
pub const VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_shader_trinary_minmax";
/// [VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION.html)
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
/// [VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME.html)
#[cfg(feature = "VK_AMD_texture_gather_bias_lod")]
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_AMD_texture_gather_bias_lod";
/// [VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION.html)
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
pub const VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION: u32 = 1;
/// [VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_ANDROID_external_format_resolve")]
pub const VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ANDROID_external_format_resolve";
/// [VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION.html)
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub const VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;
/// [VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME.html)
#[cfg(feature = "VK_ANDROID_external_memory_android_hardware_buffer")]
pub const VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME:
    &'static core::ffi::CStr = c"VK_ANDROID_external_memory_android_hardware_buffer";
/// [VK_ARM_DATA_GRAPH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_DATA_GRAPH_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_data_graph")]
pub const VK_ARM_DATA_GRAPH_SPEC_VERSION: u32 = 1;
/// [VK_ARM_DATA_GRAPH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_DATA_GRAPH_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_data_graph")]
pub const VK_ARM_DATA_GRAPH_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_ARM_data_graph";
/// [VK_ARM_FORMAT_PACK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_FORMAT_PACK_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_format_pack")]
pub const VK_ARM_FORMAT_PACK_SPEC_VERSION: u32 = 1;
/// [VK_ARM_FORMAT_PACK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_FORMAT_PACK_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_format_pack")]
pub const VK_ARM_FORMAT_PACK_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_ARM_format_pack";
/// [VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
pub const VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION: u32 = 1;
/// [VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_performance_counters_by_region")]
pub const VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_performance_counters_by_region";
/// [VK_ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
pub const VK_ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION: u32 = 1;
/// [VK_ARM_PIPELINE_OPACITY_MICROMAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_PIPELINE_OPACITY_MICROMAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_pipeline_opacity_micromap")]
pub const VK_ARM_PIPELINE_OPACITY_MICROMAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_pipeline_opacity_micromap";
/// [VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
pub const VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
/// [VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_rasterization_order_attachment_access")]
pub const VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_rasterization_order_attachment_access";
/// [VK_ARM_RENDER_PASS_STRIPED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_RENDER_PASS_STRIPED_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_render_pass_striped")]
pub const VK_ARM_RENDER_PASS_STRIPED_SPEC_VERSION: u32 = 1;
/// [VK_ARM_RENDER_PASS_STRIPED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_RENDER_PASS_STRIPED_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_render_pass_striped")]
pub const VK_ARM_RENDER_PASS_STRIPED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_render_pass_striped";
/// [VK_ARM_SCHEDULING_CONTROLS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SCHEDULING_CONTROLS_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_scheduling_controls")]
pub const VK_ARM_SCHEDULING_CONTROLS_SPEC_VERSION: u32 = 1;
/// [VK_ARM_SCHEDULING_CONTROLS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SCHEDULING_CONTROLS_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_scheduling_controls")]
pub const VK_ARM_SCHEDULING_CONTROLS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_scheduling_controls";
/// [VK_ARM_SHADER_CORE_BUILTINS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_CORE_BUILTINS_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_shader_core_builtins")]
pub const VK_ARM_SHADER_CORE_BUILTINS_SPEC_VERSION: u32 = 2;
/// [VK_ARM_SHADER_CORE_BUILTINS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_CORE_BUILTINS_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_shader_core_builtins")]
pub const VK_ARM_SHADER_CORE_BUILTINS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_shader_core_builtins";
/// [VK_ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_shader_core_properties")]
pub const VK_ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 1;
/// [VK_ARM_SHADER_CORE_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_CORE_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_shader_core_properties")]
pub const VK_ARM_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_shader_core_properties";
/// [VK_ARM_SHADER_INSTRUMENTATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_INSTRUMENTATION_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub const VK_ARM_SHADER_INSTRUMENTATION_SPEC_VERSION: u32 = 1;
/// [VK_ARM_SHADER_INSTRUMENTATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_SHADER_INSTRUMENTATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub const VK_ARM_SHADER_INSTRUMENTATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_ARM_shader_instrumentation";
/// [VK_ARM_TENSORS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_TENSORS_SPEC_VERSION.html)
#[cfg(feature = "VK_ARM_tensors")]
pub const VK_ARM_TENSORS_SPEC_VERSION: u32 = 2;
/// [VK_ARM_TENSORS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_TENSORS_EXTENSION_NAME.html)
#[cfg(feature = "VK_ARM_tensors")]
pub const VK_ARM_TENSORS_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_ARM_tensors";
/// [VK_MAKE_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_VERSION.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
/// [VK_VERSION_MAJOR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VERSION_MAJOR.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_VERSION_MAJOR(version: u32) -> u32 {
    version >> 22
}
/// [VK_VERSION_MINOR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VERSION_MINOR.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}
/// [VK_VERSION_PATCH](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VERSION_PATCH.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_VERSION_PATCH(version: u32) -> u32 {
    version & 0xFFF
}
/// [VK_MAKE_API_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_API_VERSION.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}
/// [VK_API_VERSION_VARIANT](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_VARIANT.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_API_VERSION_VARIANT(version: u32) -> u32 {
    version >> 29
}
/// [VK_API_VERSION_MAJOR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MAJOR.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_API_VERSION_MAJOR(version: u32) -> u32 {
    (version >> 22) & 0x7F
}
/// [VK_API_VERSION_MINOR](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MINOR.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_API_VERSION_MINOR(version: u32) -> u32 {
    (version >> 12) & 0x3FF
}
/// [VK_API_VERSION_PATCH](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_PATCH.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[inline]
pub const fn VK_API_VERSION_PATCH(version: u32) -> u32 {
    version & 0xFFF
}
/// [VK_API_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub const VK_API_VERSION: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 0u32, 0u32);
/// [VK_API_VERSION_1_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_0.html)
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub const VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 0u32, 0u32);
/// [VK_HEADER_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
pub const VK_HEADER_VERSION: u32 = 346u32;
/// [VK_HEADER_VERSION_COMPLETE](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION_COMPLETE.html)
#[cfg(all(feature = "VK_BASE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
pub const VK_HEADER_VERSION_COMPLETE: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 4u32, 0u32);
/// [VK_API_VERSION_1_1](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_1.html)
#[cfg(feature = "VK_BASE_VERSION_1_1")]
pub const VK_API_VERSION_1_1: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 1u32, 0u32);
/// [VK_API_VERSION_1_2](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_2.html)
#[cfg(feature = "VK_BASE_VERSION_1_2")]
pub const VK_API_VERSION_1_2: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 2u32, 0u32);
/// [VK_API_VERSION_1_3](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_3.html)
#[cfg(feature = "VK_BASE_VERSION_1_3")]
pub const VK_API_VERSION_1_3: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 3u32, 0u32);
/// [VK_API_VERSION_1_4](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_4.html)
#[cfg(feature = "VK_BASE_VERSION_1_4")]
pub const VK_API_VERSION_1_4: u32 = VK_MAKE_API_VERSION(0u32, 1u32, 4u32, 0u32);
/// [VK_EXT_4444_FORMATS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_4444_FORMATS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_4444_formats")]
pub const VK_EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_4444_FORMATS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_4444_FORMATS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_4444_formats")]
pub const VK_EXT_4444_FORMATS_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_4444_formats";
/// [VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_acquire_drm_display")]
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_acquire_drm_display")]
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_acquire_drm_display";
/// [VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_acquire_xlib_display")]
pub const VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_acquire_xlib_display";
/// [VK_EXT_APPLICATION_PARAMETERS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_APPLICATION_PARAMETERS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_application_parameters")]
pub const VK_EXT_APPLICATION_PARAMETERS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_APPLICATION_PARAMETERS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_APPLICATION_PARAMETERS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_application_parameters")]
pub const VK_EXT_APPLICATION_PARAMETERS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_application_parameters";
/// [VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_astc_decode_mode")]
pub const VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_astc_decode_mode")]
pub const VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_astc_decode_mode";
/// [VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
pub const VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_attachment_feedback_loop_dynamic_state")]
pub const VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_attachment_feedback_loop_dynamic_state";
/// [VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_attachment_feedback_loop_layout")]
pub const VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION: u32 = 2;
/// [VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_attachment_feedback_loop_layout")]
pub const VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_attachment_feedback_loop_layout";
/// [VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
/// [VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_blend_operation_advanced")]
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_blend_operation_advanced";
/// [VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_border_color_swizzle")]
pub const VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_border_color_swizzle")]
pub const VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_border_color_swizzle";
/// [VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_buffer_device_address")]
pub const VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
/// [VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_buffer_device_address")]
pub const VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_buffer_device_address";
/// [VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub const VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
/// [VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_calibrated_timestamps")]
pub const VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_calibrated_timestamps";
/// [VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_color_write_enable")]
pub const VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_color_write_enable")]
pub const VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_color_write_enable";
/// [VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_conditional_rendering")]
pub const VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
/// [VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_conditional_rendering")]
pub const VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_conditional_rendering";
/// [VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
/// [VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_conservative_rasterization")]
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_conservative_rasterization";
/// [VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_custom_border_color")]
pub const VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
/// [VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_custom_border_color")]
pub const VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_custom_border_color";
/// [VK_EXT_CUSTOM_RESOLVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CUSTOM_RESOLVE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_custom_resolve")]
pub const VK_EXT_CUSTOM_RESOLVE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_CUSTOM_RESOLVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_CUSTOM_RESOLVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_custom_resolve")]
pub const VK_EXT_CUSTOM_RESOLVE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_custom_resolve";
/// [VK_EXT_DEBUG_MARKER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_MARKER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_debug_marker")]
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
/// [VK_EXT_DEBUG_MARKER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_MARKER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_debug_marker")]
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_debug_marker";
/// [VK_EXT_DEBUG_REPORT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_REPORT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
/// [VK_EXT_DEBUG_REPORT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_REPORT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_debug_report")]
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_debug_report";
/// [VK_EXT_DEBUG_UTILS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_UTILS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_debug_utils")]
pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
/// [VK_EXT_DEBUG_UTILS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEBUG_UTILS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_debug_utils")]
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_debug_utils";
/// [VK_EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_bias_control")]
pub const VK_EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_BIAS_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_BIAS_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_bias_control")]
pub const VK_EXT_DEPTH_BIAS_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_bias_control";
/// [VK_EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_clamp_control")]
pub const VK_EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_CLAMP_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLAMP_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_clamp_control")]
pub const VK_EXT_DEPTH_CLAMP_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_clamp_control";
/// [VK_EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_clamp_zero_one")]
pub const VK_EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_clamp_zero_one")]
pub const VK_EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_clamp_zero_one";
/// [VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_clip_control")]
pub const VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_clip_control")]
pub const VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_clip_control";
/// [VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_clip_enable")]
pub const VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_clip_enable")]
pub const VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_clip_enable";
/// [VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_depth_range_unrestricted")]
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_depth_range_unrestricted";
/// [VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub const VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_descriptor_buffer")]
pub const VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_descriptor_buffer";
/// [VK_EXT_DESCRIPTOR_HEAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_HEAP_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub const VK_EXT_DESCRIPTOR_HEAP_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DESCRIPTOR_HEAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_HEAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_descriptor_heap")]
pub const VK_EXT_DESCRIPTOR_HEAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_descriptor_heap";
/// [VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_descriptor_indexing")]
pub const VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
/// [VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_descriptor_indexing")]
pub const VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_descriptor_indexing";
/// [VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_device_address_binding_report")]
pub const VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_device_address_binding_report")]
pub const VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_device_address_binding_report";
/// [VK_EXT_DEVICE_FAULT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_FAULT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_device_fault")]
pub const VK_EXT_DEVICE_FAULT_SPEC_VERSION: u32 = 2;
/// [VK_EXT_DEVICE_FAULT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_FAULT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_device_fault")]
pub const VK_EXT_DEVICE_FAULT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_device_fault";
/// [VK_EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub const VK_EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_device_generated_commands")]
pub const VK_EXT_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_device_generated_commands";
/// [VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_device_memory_report")]
pub const VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
/// [VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_device_memory_report")]
pub const VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_device_memory_report";
/// [VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_direct_mode_display")]
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_direct_mode_display";
/// [VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_directfb_surface")]
pub const VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_directfb_surface")]
pub const VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_directfb_surface";
/// [VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 2;
/// [VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_discard_rectangles")]
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_discard_rectangles";
/// [VK_EXT_DISPLAY_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISPLAY_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_display_control")]
pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_display_control")]
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_display_control";
/// [VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_display_surface_counter")]
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_display_surface_counter";
/// [VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
pub const VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_dynamic_rendering_unused_attachments")]
pub const VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_dynamic_rendering_unused_attachments";
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_extended_dynamic_state";
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state2")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_extended_dynamic_state2";
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION: u32 = 2;
/// [VK_EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_extended_dynamic_state3")]
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_extended_dynamic_state3";
/// [VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
pub const VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_external_memory_acquire_unmodified")]
pub const VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_external_memory_acquire_unmodified";
/// [VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_external_memory_dma_buf")]
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_external_memory_dma_buf";
/// [VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_external_memory_host")]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_external_memory_host")]
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_external_memory_host";
/// [VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub const VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_external_memory_metal")]
pub const VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_external_memory_metal";
/// [VK_EXT_FILTER_CUBIC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FILTER_CUBIC_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_filter_cubic")]
pub const VK_EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
/// [VK_EXT_FILTER_CUBIC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FILTER_CUBIC_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_filter_cubic")]
pub const VK_EXT_FILTER_CUBIC_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_filter_cubic";
/// [VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_fragment_density_map")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 3;
/// [VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_fragment_density_map")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_fragment_density_map";
/// [VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_fragment_density_map2")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
/// [VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_fragment_density_map2")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_fragment_density_map2";
/// [VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
/// [VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_fragment_density_map_offset")]
pub const VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_fragment_density_map_offset";
/// [VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
pub const VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
/// [VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_fragment_shader_interlock")]
pub const VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_fragment_shader_interlock";
/// [VK_EXT_FRAME_BOUNDARY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAME_BOUNDARY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_frame_boundary")]
pub const VK_EXT_FRAME_BOUNDARY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_FRAME_BOUNDARY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FRAME_BOUNDARY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_frame_boundary")]
pub const VK_EXT_FRAME_BOUNDARY_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_frame_boundary";
/// [VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
/// [VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub const VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_full_screen_exclusive";
/// [VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_global_priority")]
pub const VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
/// [VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_global_priority")]
pub const VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_global_priority";
/// [VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_global_priority_query")]
pub const VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_global_priority_query")]
pub const VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_global_priority_query";
/// [VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
pub const VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_graphics_pipeline_library")]
pub const VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_graphics_pipeline_library";
/// [VK_EXT_HDR_METADATA_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HDR_METADATA_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub const VK_EXT_HDR_METADATA_SPEC_VERSION: u32 = 3;
/// [VK_EXT_HDR_METADATA_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HDR_METADATA_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_hdr_metadata")]
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_hdr_metadata";
/// [VK_EXT_HEADLESS_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HEADLESS_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_headless_surface")]
pub const VK_EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_headless_surface")]
pub const VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_headless_surface";
/// [VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_host_image_copy")]
pub const VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_host_image_copy")]
pub const VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_host_image_copy";
/// [VK_EXT_HOST_QUERY_RESET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HOST_QUERY_RESET_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_host_query_reset")]
pub const VK_EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
/// [VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_host_query_reset")]
pub const VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_host_query_reset";
/// [VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
pub const VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_2d_view_of_3d")]
pub const VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_2d_view_of_3d";
/// [VK_EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_compression_control")]
pub const VK_EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_compression_control")]
pub const VK_EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_compression_control";
/// [VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
pub const VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_compression_control_swapchain")]
pub const VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_compression_control_swapchain";
/// [VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
/// [VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_drm_format_modifier")]
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_drm_format_modifier";
/// [VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_robustness")]
pub const VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_robustness")]
pub const VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_robustness";
/// [VK_EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
pub const VK_EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_SLICED_VIEW_OF_3D_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_SLICED_VIEW_OF_3D_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_sliced_view_of_3d")]
pub const VK_EXT_IMAGE_SLICED_VIEW_OF_3D_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_sliced_view_of_3d";
/// [VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_image_view_min_lod")]
pub const VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
/// [VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_image_view_min_lod")]
pub const VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_image_view_min_lod";
/// [VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_index_type_uint8")]
pub const VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
/// [VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_index_type_uint8")]
pub const VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_index_type_uint8";
/// [VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_inline_uniform_block")]
pub const VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;
/// [VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_inline_uniform_block")]
pub const VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_inline_uniform_block";
/// [VK_EXT_LAYER_SETTINGS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LAYER_SETTINGS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_layer_settings")]
pub const VK_EXT_LAYER_SETTINGS_SPEC_VERSION: u32 = 2;
/// [VK_EXT_LAYER_SETTINGS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LAYER_SETTINGS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_layer_settings")]
pub const VK_EXT_LAYER_SETTINGS_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_layer_settings";
/// [VK_EXT_LEGACY_DITHERING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LEGACY_DITHERING_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_legacy_dithering")]
pub const VK_EXT_LEGACY_DITHERING_SPEC_VERSION: u32 = 2;
/// [VK_EXT_LEGACY_DITHERING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LEGACY_DITHERING_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_legacy_dithering")]
pub const VK_EXT_LEGACY_DITHERING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_legacy_dithering";
/// [VK_EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
pub const VK_EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION: u32 = 1;
/// [VK_EXT_LEGACY_VERTEX_ATTRIBUTES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LEGACY_VERTEX_ATTRIBUTES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_legacy_vertex_attributes")]
pub const VK_EXT_LEGACY_VERTEX_ATTRIBUTES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_legacy_vertex_attributes";
/// [VK_EXT_LINE_RASTERIZATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LINE_RASTERIZATION_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_line_rasterization")]
pub const VK_EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
/// [VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_line_rasterization")]
pub const VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_line_rasterization";
/// [VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_load_store_op_none")]
pub const VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_load_store_op_none")]
pub const VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_load_store_op_none";
/// [VK_EXT_MAP_MEMORY_PLACED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MAP_MEMORY_PLACED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_map_memory_placed")]
pub const VK_EXT_MAP_MEMORY_PLACED_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MAP_MEMORY_PLACED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MAP_MEMORY_PLACED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_map_memory_placed")]
pub const VK_EXT_MAP_MEMORY_PLACED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_map_memory_placed";
/// [VK_EXT_MEMORY_BUDGET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_BUDGET_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_memory_budget")]
pub const VK_EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MEMORY_BUDGET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_BUDGET_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_memory_budget")]
pub const VK_EXT_MEMORY_BUDGET_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_memory_budget";
/// [VK_EXT_MEMORY_DECOMPRESSION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_DECOMPRESSION_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_memory_decompression")]
pub const VK_EXT_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MEMORY_DECOMPRESSION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_DECOMPRESSION_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_memory_decompression")]
pub const VK_EXT_MEMORY_DECOMPRESSION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_memory_decompression";
/// [VK_EXT_MEMORY_PRIORITY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_PRIORITY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_memory_priority")]
pub const VK_EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_memory_priority")]
pub const VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_memory_priority";
/// [VK_EXT_MESH_SHADER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MESH_SHADER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_mesh_shader")]
pub const VK_EXT_MESH_SHADER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MESH_SHADER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MESH_SHADER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_mesh_shader")]
pub const VK_EXT_MESH_SHADER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_mesh_shader";
/// [VK_EXT_METAL_OBJECTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_METAL_OBJECTS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_metal_objects")]
pub const VK_EXT_METAL_OBJECTS_SPEC_VERSION: u32 = 2;
/// [VK_EXT_METAL_OBJECTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_METAL_OBJECTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_metal_objects")]
pub const VK_EXT_METAL_OBJECTS_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_metal_objects";
/// [VK_EXT_METAL_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_METAL_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_metal_surface")]
pub const VK_EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_METAL_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_METAL_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_metal_surface")]
pub const VK_EXT_METAL_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_metal_surface";
/// [VK_EXT_MULTI_DRAW_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MULTI_DRAW_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_multi_draw")]
pub const VK_EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MULTI_DRAW_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MULTI_DRAW_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_multi_draw")]
pub const VK_EXT_MULTI_DRAW_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_multi_draw";
/// [VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
pub const VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_multisampled_render_to_single_sampled")]
pub const VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_multisampled_render_to_single_sampled";
/// [VK_EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
pub const VK_EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_mutable_descriptor_type")]
pub const VK_EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_mutable_descriptor_type";
/// [VK_EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_nested_command_buffer")]
pub const VK_EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_NESTED_COMMAND_BUFFER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_NESTED_COMMAND_BUFFER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_nested_command_buffer")]
pub const VK_EXT_NESTED_COMMAND_BUFFER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_nested_command_buffer";
/// [VK_EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_non_seamless_cube_map")]
pub const VK_EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION: u32 = 1;
/// [VK_EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_non_seamless_cube_map")]
pub const VK_EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_non_seamless_cube_map";
/// [VK_EXT_OPACITY_MICROMAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_OPACITY_MICROMAP_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub const VK_EXT_OPACITY_MICROMAP_SPEC_VERSION: u32 = 2;
/// [VK_EXT_OPACITY_MICROMAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_OPACITY_MICROMAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub const VK_EXT_OPACITY_MICROMAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_opacity_micromap";
/// [VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
pub const VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pageable_device_local_memory")]
pub const VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pageable_device_local_memory";
/// [VK_EXT_PCI_BUS_INFO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PCI_BUS_INFO_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pci_bus_info")]
pub const VK_EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
/// [VK_EXT_PCI_BUS_INFO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PCI_BUS_INFO_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pci_bus_info")]
pub const VK_EXT_PCI_BUS_INFO_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_pci_bus_info";
/// [VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_physical_device_drm")]
pub const VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_physical_device_drm")]
pub const VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_physical_device_drm";
/// [VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
pub const VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
/// [VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_creation_cache_control")]
pub const VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_creation_cache_control";
/// [VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
pub const VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_creation_feedback")]
pub const VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_creation_feedback";
/// [VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_library_group_handles")]
pub const VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_library_group_handles")]
pub const VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_library_group_handles";
/// [VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_properties")]
pub const VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_properties")]
pub const VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_properties";
/// [VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_protected_access")]
pub const VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_protected_access";
/// [VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_pipeline_robustness")]
pub const VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_pipeline_robustness")]
pub const VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_pipeline_robustness";
/// [VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_post_depth_coverage")]
pub const VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_post_depth_coverage")]
pub const VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_post_depth_coverage";
/// [VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_present_mode_fifo_latest_ready")]
pub const VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_present_mode_fifo_latest_ready")]
pub const VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_present_mode_fifo_latest_ready";
/// [VK_EXT_PRESENT_TIMING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRESENT_TIMING_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_present_timing")]
pub const VK_EXT_PRESENT_TIMING_SPEC_VERSION: u32 = 3;
/// [VK_EXT_PRESENT_TIMING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRESENT_TIMING_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_present_timing")]
pub const VK_EXT_PRESENT_TIMING_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_present_timing";
/// [VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
pub const VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_primitive_topology_list_restart")]
pub const VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_primitive_topology_list_restart";
/// [VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_primitives_generated_query")]
pub const VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_primitives_generated_query")]
pub const VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_primitives_generated_query";
/// [VK_EXT_PRIVATE_DATA_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIVATE_DATA_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_private_data")]
pub const VK_EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PRIVATE_DATA_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PRIVATE_DATA_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_private_data")]
pub const VK_EXT_PRIVATE_DATA_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_private_data";
/// [VK_EXT_PROVOKING_VERTEX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PROVOKING_VERTEX_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_provoking_vertex")]
pub const VK_EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
/// [VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_provoking_vertex")]
pub const VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_provoking_vertex";
/// [VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
/// [VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_queue_family_foreign")]
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_queue_family_foreign";
/// [VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
pub const VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_rasterization_order_attachment_access")]
pub const VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_rasterization_order_attachment_access";
/// [VK_EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
pub const VK_EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_ray_tracing_invocation_reorder")]
pub const VK_EXT_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_ray_tracing_invocation_reorder";
/// [VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
pub const VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_rgba10x6_formats")]
pub const VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_rgba10x6_formats";
/// [VK_EXT_ROBUSTNESS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ROBUSTNESS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_robustness2")]
pub const VK_EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ROBUSTNESS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ROBUSTNESS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_robustness2")]
pub const VK_EXT_ROBUSTNESS_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_robustness2";
/// [VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_sample_locations")]
pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_sample_locations")]
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_sample_locations";
/// [VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
/// [VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_sampler_filter_minmax")]
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_sampler_filter_minmax";
/// [VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_scalar_block_layout")]
pub const VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_scalar_block_layout")]
pub const VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_scalar_block_layout";
/// [VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_separate_stencil_usage")]
pub const VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_separate_stencil_usage")]
pub const VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_separate_stencil_usage";
/// [VK_EXT_SHADER_64BIT_INDEXING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_64BIT_INDEXING_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_64bit_indexing")]
pub const VK_EXT_SHADER_64BIT_INDEXING_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_64BIT_INDEXING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_64BIT_INDEXING_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_64bit_indexing")]
pub const VK_EXT_SHADER_64BIT_INDEXING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_64bit_indexing";
/// [VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_atomic_float")]
pub const VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_atomic_float")]
pub const VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_atomic_float";
/// [VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
pub const VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_atomic_float2")]
pub const VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_atomic_float2";
/// [VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
pub const VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_demote_to_helper_invocation")]
pub const VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_demote_to_helper_invocation";
/// [VK_EXT_SHADER_FLOAT8_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_FLOAT8_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_float8")]
pub const VK_EXT_SHADER_FLOAT8_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_FLOAT8_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_FLOAT8_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_float8")]
pub const VK_EXT_SHADER_FLOAT8_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_shader_float8";
/// [VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
pub const VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_image_atomic_int64")]
pub const VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_image_atomic_int64";
/// [VK_EXT_SHADER_LONG_VECTOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_LONG_VECTOR_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_long_vector")]
pub const VK_EXT_SHADER_LONG_VECTOR_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_LONG_VECTOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_LONG_VECTOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_long_vector")]
pub const VK_EXT_SHADER_LONG_VECTOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_long_vector";
/// [VK_EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_module_identifier")]
pub const VK_EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_module_identifier")]
pub const VK_EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_module_identifier";
/// [VK_EXT_SHADER_OBJECT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_OBJECT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_object")]
pub const VK_EXT_SHADER_OBJECT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_OBJECT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_OBJECT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_object")]
pub const VK_EXT_SHADER_OBJECT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_shader_object";
/// [VK_EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_replicated_composites")]
pub const VK_EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_REPLICATED_COMPOSITES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_REPLICATED_COMPOSITES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_replicated_composites")]
pub const VK_EXT_SHADER_REPLICATED_COMPOSITES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_replicated_composites";
/// [VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_stencil_export")]
pub const VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_stencil_export")]
pub const VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_stencil_export";
/// [VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_subgroup_ballot")]
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_subgroup_ballot";
/// [VK_EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
pub const VK_EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_subgroup_partitioned")]
pub const VK_EXT_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_subgroup_partitioned";
/// [VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_subgroup_vote")]
pub const VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_subgroup_vote";
/// [VK_EXT_SHADER_TILE_IMAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_TILE_IMAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_tile_image")]
pub const VK_EXT_SHADER_TILE_IMAGE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_TILE_IMAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_TILE_IMAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_tile_image")]
pub const VK_EXT_SHADER_TILE_IMAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_tile_image";
/// [VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
pub const VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_uniform_buffer_unsized_array")]
pub const VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_uniform_buffer_unsized_array";
/// [VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_shader_viewport_index_layer")]
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_shader_viewport_index_layer";
/// [VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_subgroup_size_control")]
pub const VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
/// [VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_subgroup_size_control")]
pub const VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_subgroup_size_control";
/// [VK_EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
pub const VK_EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION: u32 = 2;
/// [VK_EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_subpass_merge_feedback")]
pub const VK_EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_subpass_merge_feedback";
/// [VK_EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_surface_maintenance1")]
pub const VK_EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_surface_maintenance1")]
pub const VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_surface_maintenance1";
/// [VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 5;
/// [VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_swapchain_colorspace")]
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_swapchain_colorspace";
/// [VK_EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
pub const VK_EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_EXT_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_swapchain_maintenance1")]
pub const VK_EXT_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_swapchain_maintenance1";
/// [VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
pub const VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
/// [VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_texel_buffer_alignment")]
pub const VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_texel_buffer_alignment";
/// [VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION: u32 = 1;
/// [VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_texture_compression_astc_3d")]
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_texture_compression_astc_3d";
/// [VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
/// [VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_texture_compression_astc_hdr")]
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_texture_compression_astc_hdr";
/// [VK_EXT_TOOLING_INFO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TOOLING_INFO_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_tooling_info")]
pub const VK_EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
/// [VK_EXT_TOOLING_INFO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TOOLING_INFO_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_tooling_info")]
pub const VK_EXT_TOOLING_INFO_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_EXT_tooling_info";
/// [VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_transform_feedback")]
pub const VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
/// [VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_transform_feedback")]
pub const VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_transform_feedback";
/// [VK_EXT_VALIDATION_CACHE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_CACHE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_validation_cache")]
pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
/// [VK_EXT_VALIDATION_CACHE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_CACHE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_validation_cache")]
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_validation_cache";
/// [VK_EXT_VALIDATION_FEATURES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_FEATURES_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_validation_features")]
pub const VK_EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 6;
/// [VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_validation_features")]
pub const VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_validation_features";
/// [VK_EXT_VALIDATION_FLAGS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_FLAGS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_validation_flags")]
pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 3;
/// [VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_validation_flags")]
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_validation_flags";
/// [VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
/// [VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_vertex_attribute_divisor")]
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_vertex_attribute_divisor";
/// [VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
pub const VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_vertex_attribute_robustness")]
pub const VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_vertex_attribute_robustness";
/// [VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
pub const VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
/// [VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_vertex_input_dynamic_state")]
pub const VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_vertex_input_dynamic_state";
/// [VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
pub const VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_ycbcr_2plane_444_formats")]
pub const VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_ycbcr_2plane_444_formats";
/// [VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
pub const VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
/// [VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_ycbcr_image_arrays")]
pub const VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_ycbcr_image_arrays";
/// [VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
pub const VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_EXT_zero_initialize_device_memory")]
pub const VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_EXT_zero_initialize_device_memory";
/// [VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub const VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;
/// [VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME.html)
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub const VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_FUCHSIA_buffer_collection";
/// [VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub const VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_FUCHSIA_external_memory")]
pub const VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_FUCHSIA_external_memory";
/// [VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION.html)
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub const VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
/// [VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME.html)
#[cfg(feature = "VK_FUCHSIA_external_semaphore")]
pub const VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_FUCHSIA_external_semaphore";
/// [VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub const VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
pub const VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_FUCHSIA_imagepipe_surface";
/// [VK_GGP_FRAME_TOKEN_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_FRAME_TOKEN_SPEC_VERSION.html)
#[cfg(feature = "VK_GGP_frame_token")]
pub const VK_GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;
/// [VK_GGP_FRAME_TOKEN_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_FRAME_TOKEN_EXTENSION_NAME.html)
#[cfg(feature = "VK_GGP_frame_token")]
pub const VK_GGP_FRAME_TOKEN_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_GGP_frame_token";
/// [VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub const VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_GGP_stream_descriptor_surface")]
pub const VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_GGP_stream_descriptor_surface";
/// [VK_GOOGLE_DECORATE_STRING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_DECORATE_STRING_SPEC_VERSION.html)
#[cfg(feature = "VK_GOOGLE_decorate_string")]
pub const VK_GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;
/// [VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME.html)
#[cfg(feature = "VK_GOOGLE_decorate_string")]
pub const VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_GOOGLE_decorate_string";
/// [VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION.html)
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
/// [VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME.html)
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_GOOGLE_display_timing";
/// [VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION.html)
#[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
pub const VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;
/// [VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_GOOGLE_hlsl_functionality1")]
pub const VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_GOOGLE_hlsl_functionality1";
/// [VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_GOOGLE_surfaceless_query")]
pub const VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION: u32 = 2;
/// [VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_GOOGLE_surfaceless_query")]
pub const VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_GOOGLE_surfaceless_query";
/// [VK_GOOGLE_USER_TYPE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_USER_TYPE_SPEC_VERSION.html)
#[cfg(feature = "VK_GOOGLE_user_type")]
pub const VK_GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;
/// [VK_GOOGLE_USER_TYPE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_USER_TYPE_EXTENSION_NAME.html)
#[cfg(feature = "VK_GOOGLE_user_type")]
pub const VK_GOOGLE_USER_TYPE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_GOOGLE_user_type";
/// [VK_HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION.html)
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
pub const VK_HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION: u32 = 3;
/// [VK_HUAWEI_CLUSTER_CULLING_SHADER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_CLUSTER_CULLING_SHADER_EXTENSION_NAME.html)
#[cfg(feature = "VK_HUAWEI_cluster_culling_shader")]
pub const VK_HUAWEI_CLUSTER_CULLING_SHADER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_HUAWEI_cluster_culling_shader";
/// [VK_HUAWEI_HDR_VIVID_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_HDR_VIVID_SPEC_VERSION.html)
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
pub const VK_HUAWEI_HDR_VIVID_SPEC_VERSION: u32 = 1;
/// [VK_HUAWEI_HDR_VIVID_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_HDR_VIVID_EXTENSION_NAME.html)
#[cfg(feature = "VK_HUAWEI_hdr_vivid")]
pub const VK_HUAWEI_HDR_VIVID_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_HUAWEI_hdr_vivid";
/// [VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION.html)
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
pub const VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
/// [VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME.html)
#[cfg(feature = "VK_HUAWEI_invocation_mask")]
pub const VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_HUAWEI_invocation_mask";
/// [VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION.html)
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
pub const VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 3;
/// [VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME.html)
#[cfg(feature = "VK_HUAWEI_subpass_shading")]
pub const VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_HUAWEI_subpass_shading";
/// [VK_IMG_FILTER_CUBIC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_FILTER_CUBIC_SPEC_VERSION.html)
#[cfg(feature = "VK_IMG_filter_cubic")]
pub const VK_IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;
/// [VK_IMG_FILTER_CUBIC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_FILTER_CUBIC_EXTENSION_NAME.html)
#[cfg(feature = "VK_IMG_filter_cubic")]
pub const VK_IMG_FILTER_CUBIC_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_IMG_filter_cubic";
/// [VK_IMG_FORMAT_PVRTC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_FORMAT_PVRTC_SPEC_VERSION.html)
#[cfg(feature = "VK_IMG_format_pvrtc")]
pub const VK_IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
/// [VK_IMG_FORMAT_PVRTC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_FORMAT_PVRTC_EXTENSION_NAME.html)
#[cfg(feature = "VK_IMG_format_pvrtc")]
pub const VK_IMG_FORMAT_PVRTC_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_IMG_format_pvrtc";
/// [VK_IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION.html)
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
pub const VK_IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
/// [VK_IMG_RELAXED_LINE_RASTERIZATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_RELAXED_LINE_RASTERIZATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_IMG_relaxed_line_rasterization")]
pub const VK_IMG_RELAXED_LINE_RASTERIZATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_IMG_relaxed_line_rasterization";
/// [VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_INTEL_performance_query")]
pub const VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
/// [VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_INTEL_performance_query")]
pub const VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_INTEL_performance_query";
/// [VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
pub const VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
/// [VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_INTEL_shader_integer_functions2")]
pub const VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_INTEL_shader_integer_functions2";
/// [VK_KHR_16BIT_STORAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_16BIT_STORAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_16bit_storage")]
pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_16BIT_STORAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_16BIT_STORAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_16bit_storage")]
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_16bit_storage";
/// [VK_KHR_8BIT_STORAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_8BIT_STORAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_8bit_storage")]
pub const VK_KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_8BIT_STORAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_8BIT_STORAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_8bit_storage")]
pub const VK_KHR_8BIT_STORAGE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_8bit_storage";
/// [VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub const VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
/// [VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_acceleration_structure")]
pub const VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_acceleration_structure";
/// [VK_KHR_ANDROID_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ANDROID_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_android_surface")]
pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;
/// [VK_KHR_ANDROID_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ANDROID_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_android_surface")]
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_android_surface";
/// [VK_KHR_BIND_MEMORY_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_BIND_MEMORY_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_bind_memory2")]
pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_BIND_MEMORY_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_BIND_MEMORY_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_bind_memory2")]
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_bind_memory2";
/// [VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_buffer_device_address")]
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_buffer_device_address";
/// [VK_KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
pub const VK_KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_CALIBRATED_TIMESTAMPS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_CALIBRATED_TIMESTAMPS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_calibrated_timestamps")]
pub const VK_KHR_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_calibrated_timestamps";
/// [VK_KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
pub const VK_KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_compute_shader_derivatives")]
pub const VK_KHR_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_compute_shader_derivatives";
/// [VK_KHR_COOPERATIVE_MATRIX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COOPERATIVE_MATRIX_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_cooperative_matrix")]
pub const VK_KHR_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 2;
/// [VK_KHR_COOPERATIVE_MATRIX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COOPERATIVE_MATRIX_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_cooperative_matrix")]
pub const VK_KHR_COOPERATIVE_MATRIX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_cooperative_matrix";
/// [VK_KHR_COPY_COMMANDS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COPY_COMMANDS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_copy_commands2")]
pub const VK_KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_copy_commands2")]
pub const VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_copy_commands2";
/// [VK_KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
pub const VK_KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_COPY_MEMORY_INDIRECT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_COPY_MEMORY_INDIRECT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_copy_memory_indirect")]
pub const VK_KHR_COPY_MEMORY_INDIRECT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_copy_memory_indirect";
/// [VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub const VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_create_renderpass2")]
pub const VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_create_renderpass2";
/// [VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
/// [VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_dedicated_allocation")]
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_dedicated_allocation";
/// [VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub const VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
/// [VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_deferred_host_operations")]
pub const VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_deferred_host_operations";
/// [VK_KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
pub const VK_KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_depth_clamp_zero_one")]
pub const VK_KHR_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_depth_clamp_zero_one";
/// [VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_depth_stencil_resolve")]
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_depth_stencil_resolve";
/// [VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_descriptor_update_template")]
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_descriptor_update_template";
/// [VK_KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_device_address_commands")]
pub const VK_KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DEVICE_ADDRESS_COMMANDS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_ADDRESS_COMMANDS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_device_address_commands")]
pub const VK_KHR_DEVICE_ADDRESS_COMMANDS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_device_address_commands";
/// [VK_KHR_DEVICE_GROUP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_GROUP_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_device_group")]
pub const VK_KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
/// [VK_KHR_DEVICE_GROUP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_GROUP_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_device_group")]
pub const VK_KHR_DEVICE_GROUP_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_device_group";
/// [VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_device_group_creation")]
pub const VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_device_group_creation")]
pub const VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_device_group_creation";
/// [VK_KHR_DISPLAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DISPLAY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_display")]
pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 23;
/// [VK_KHR_DISPLAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DISPLAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_display")]
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_display";
/// [VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_display_swapchain")]
pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
/// [VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_display_swapchain")]
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_display_swapchain";
/// [VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub const VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_draw_indirect_count")]
pub const VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_draw_indirect_count";
/// [VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_driver_properties")]
pub const VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_driver_properties")]
pub const VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_driver_properties";
/// [VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub const VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_dynamic_rendering")]
pub const VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_dynamic_rendering";
/// [VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION: u32 = 1;
/// [VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_dynamic_rendering_local_read")]
pub const VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_dynamic_rendering_local_read";
/// [VK_KHR_EXTERNAL_FENCE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_fence")]
pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_fence")]
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_external_fence";
/// [VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_fence_capabilities")]
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_fence_capabilities";
/// [VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_fence_fd")]
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_fence_fd";
/// [VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_fence_win32")]
pub const VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_fence_win32";
/// [VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_memory")]
pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_memory")]
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_memory";
/// [VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_memory_capabilities")]
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_memory_capabilities";
/// [VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_memory_fd")]
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_memory_fd";
/// [VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_memory_win32")]
pub const VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_memory_win32";
/// [VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_semaphore")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_semaphore")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_semaphore";
/// [VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_semaphore_capabilities")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_semaphore_capabilities";
/// [VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_semaphore_fd")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_semaphore_fd";
/// [VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
/// [VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_external_semaphore_win32")]
pub const VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_external_semaphore_win32";
/// [VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_format_feature_flags2")]
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 2;
/// [VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_format_feature_flags2")]
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_format_feature_flags2";
/// [VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
pub const VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
/// [VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_fragment_shader_barycentric")]
pub const VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_fragment_shader_barycentric";
/// [VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
pub const VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
/// [VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_fragment_shading_rate")]
pub const VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_fragment_shading_rate";
/// [VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub const VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_get_display_properties2")]
pub const VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_get_display_properties2";
/// [VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_get_memory_requirements2")]
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_get_memory_requirements2";
/// [VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;
/// [VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_get_physical_device_properties2")]
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_get_physical_device_properties2";
/// [VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_get_surface_capabilities2")]
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_get_surface_capabilities2";
/// [VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_global_priority")]
pub const VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_global_priority")]
pub const VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_global_priority";
/// [VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_image_format_list")]
pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
/// [VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_image_format_list")]
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_image_format_list";
/// [VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
pub const VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
/// [VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_imageless_framebuffer")]
pub const VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_imageless_framebuffer";
/// [VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_incremental_present")]
pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
/// [VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_incremental_present")]
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_incremental_present";
/// [VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_index_type_uint8")]
pub const VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
/// [VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_index_type_uint8")]
pub const VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_index_type_uint8";
/// [VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_internally_synchronized_queues")]
pub const VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_internally_synchronized_queues")]
pub const VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_internally_synchronized_queues";
/// [VK_KHR_LINE_RASTERIZATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_LINE_RASTERIZATION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_line_rasterization")]
pub const VK_KHR_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
/// [VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_line_rasterization")]
pub const VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_line_rasterization";
/// [VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_load_store_op_none")]
pub const VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_load_store_op_none")]
pub const VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_load_store_op_none";
/// [VK_KHR_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance1")]
pub const VK_KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;
/// [VK_KHR_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance1")]
pub const VK_KHR_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance1";
/// [VK_KHR_MAINTENANCE_10_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_10_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance10")]
pub const VK_KHR_MAINTENANCE_10_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_10_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_10_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance10")]
pub const VK_KHR_MAINTENANCE_10_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance10";
/// [VK_KHR_MAINTENANCE_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance2")]
pub const VK_KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance2")]
pub const VK_KHR_MAINTENANCE_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance2";
/// [VK_KHR_MAINTENANCE_3_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_3_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance3")]
pub const VK_KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_3_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_3_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance3")]
pub const VK_KHR_MAINTENANCE_3_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance3";
/// [VK_KHR_MAINTENANCE_4_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_4_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance4")]
pub const VK_KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;
/// [VK_KHR_MAINTENANCE_4_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_4_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance4")]
pub const VK_KHR_MAINTENANCE_4_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance4";
/// [VK_KHR_MAINTENANCE_5_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_5_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance5")]
pub const VK_KHR_MAINTENANCE_5_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_5_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_5_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance5")]
pub const VK_KHR_MAINTENANCE_5_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance5";
/// [VK_KHR_MAINTENANCE_6_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_6_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance6")]
pub const VK_KHR_MAINTENANCE_6_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_6_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_6_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance6")]
pub const VK_KHR_MAINTENANCE_6_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance6";
/// [VK_KHR_MAINTENANCE_7_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_7_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance7")]
pub const VK_KHR_MAINTENANCE_7_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_7_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_7_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance7")]
pub const VK_KHR_MAINTENANCE_7_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance7";
/// [VK_KHR_MAINTENANCE_8_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_8_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance8")]
pub const VK_KHR_MAINTENANCE_8_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_8_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_8_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance8")]
pub const VK_KHR_MAINTENANCE_8_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance8";
/// [VK_KHR_MAINTENANCE_9_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_9_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_maintenance9")]
pub const VK_KHR_MAINTENANCE_9_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAINTENANCE_9_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAINTENANCE_9_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_maintenance9")]
pub const VK_KHR_MAINTENANCE_9_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_maintenance9";
/// [VK_KHR_MAP_MEMORY_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAP_MEMORY_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_map_memory2")]
pub const VK_KHR_MAP_MEMORY_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MAP_MEMORY_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MAP_MEMORY_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_map_memory2")]
pub const VK_KHR_MAP_MEMORY_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_map_memory2";
/// [VK_KHR_MULTIVIEW_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MULTIVIEW_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_multiview")]
pub const VK_KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;
/// [VK_KHR_MULTIVIEW_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_MULTIVIEW_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_multiview")]
pub const VK_KHR_MULTIVIEW_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_multiview";
/// [VK_KHR_OBJECT_REFRESH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_OBJECT_REFRESH_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_object_refresh")]
pub const VK_KHR_OBJECT_REFRESH_SPEC_VERSION: u32 = 1;
/// [VK_KHR_OBJECT_REFRESH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_OBJECT_REFRESH_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_object_refresh")]
pub const VK_KHR_OBJECT_REFRESH_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_object_refresh";
/// [VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_performance_query")]
pub const VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_performance_query")]
pub const VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_performance_query";
/// [VK_KHR_PIPELINE_BINARY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_BINARY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub const VK_KHR_PIPELINE_BINARY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PIPELINE_BINARY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_BINARY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_pipeline_binary")]
pub const VK_KHR_PIPELINE_BINARY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_pipeline_binary";
/// [VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub const VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_pipeline_executable_properties")]
pub const VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_pipeline_executable_properties";
/// [VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_pipeline_library")]
pub const VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_pipeline_library")]
pub const VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_pipeline_library";
/// [VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_portability_enumeration")]
pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_portability_enumeration")]
pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_portability_enumeration";
/// [VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_portability_subset")]
pub const VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_portability_subset")]
pub const VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_portability_subset";
/// [VK_KHR_PRESENT_ID_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_ID_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_present_id")]
pub const VK_KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PRESENT_ID_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_ID_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_present_id")]
pub const VK_KHR_PRESENT_ID_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_present_id";
/// [VK_KHR_PRESENT_ID_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_ID_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_present_id2")]
pub const VK_KHR_PRESENT_ID_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PRESENT_ID_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_ID_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_present_id2")]
pub const VK_KHR_PRESENT_ID_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_present_id2";
/// [VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
pub const VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_present_mode_fifo_latest_ready")]
pub const VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_present_mode_fifo_latest_ready";
/// [VK_KHR_PRESENT_WAIT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_WAIT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_present_wait")]
pub const VK_KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PRESENT_WAIT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_WAIT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_present_wait")]
pub const VK_KHR_PRESENT_WAIT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_present_wait";
/// [VK_KHR_PRESENT_WAIT_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_WAIT_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_present_wait2")]
pub const VK_KHR_PRESENT_WAIT_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_PRESENT_WAIT_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PRESENT_WAIT_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_present_wait2")]
pub const VK_KHR_PRESENT_WAIT_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_present_wait2";
/// [VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
/// [VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_push_descriptor")]
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_push_descriptor";
/// [VK_KHR_RAY_QUERY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_QUERY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_ray_query")]
pub const VK_KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_RAY_QUERY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_QUERY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_ray_query")]
pub const VK_KHR_RAY_QUERY_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_ray_query";
/// [VK_KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
pub const VK_KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_ray_tracing_maintenance1")]
pub const VK_KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_ray_tracing_maintenance1";
/// [VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub const VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_ray_tracing_pipeline")]
pub const VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_ray_tracing_pipeline";
/// [VK_KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
pub const VK_KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION: u32 = 1;
/// [VK_KHR_RAY_TRACING_POSITION_FETCH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RAY_TRACING_POSITION_FETCH_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_ray_tracing_position_fetch")]
pub const VK_KHR_RAY_TRACING_POSITION_FETCH_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_ray_tracing_position_fetch";
/// [VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_relaxed_block_layout")]
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_relaxed_block_layout";
/// [VK_KHR_ROBUSTNESS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ROBUSTNESS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_robustness2")]
pub const VK_KHR_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_ROBUSTNESS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ROBUSTNESS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_robustness2")]
pub const VK_KHR_ROBUSTNESS_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_robustness2";
/// [VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;
/// [VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_sampler_mirror_clamp_to_edge")]
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_sampler_mirror_clamp_to_edge";
/// [VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
/// [VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_sampler_ycbcr_conversion")]
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_sampler_ycbcr_conversion";
/// [VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
pub const VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_separate_depth_stencil_layouts")]
pub const VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_separate_depth_stencil_layouts";
/// [VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_atomic_int64")]
pub const VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_atomic_int64")]
pub const VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_atomic_int64";
/// [VK_KHR_SHADER_BFLOAT16_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_BFLOAT16_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_bfloat16")]
pub const VK_KHR_SHADER_BFLOAT16_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_BFLOAT16_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_BFLOAT16_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_bfloat16")]
pub const VK_KHR_SHADER_BFLOAT16_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_bfloat16";
/// [VK_KHR_SHADER_CLOCK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_CLOCK_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_clock")]
pub const VK_KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_CLOCK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_CLOCK_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_clock")]
pub const VK_KHR_SHADER_CLOCK_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_shader_clock";
/// [VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_draw_parameters")]
pub const VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_draw_parameters";
/// [VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_expect_assume")]
pub const VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_expect_assume")]
pub const VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_expect_assume";
/// [VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_float16_int8")]
pub const VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_float16_int8")]
pub const VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_float16_int8";
/// [VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_float_controls")]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;
/// [VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_float_controls")]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_float_controls";
/// [VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_float_controls2")]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_float_controls2")]
pub const VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_float_controls2";
/// [VK_KHR_SHADER_FMA_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FMA_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_fma")]
pub const VK_KHR_SHADER_FMA_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_FMA_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_FMA_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_fma")]
pub const VK_KHR_SHADER_FMA_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_shader_fma";
/// [VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_integer_dot_product")]
pub const VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_integer_dot_product")]
pub const VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_integer_dot_product";
/// [VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
pub const VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_maximal_reconvergence")]
pub const VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_maximal_reconvergence";
/// [VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_non_semantic_info")]
pub const VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_non_semantic_info")]
pub const VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_non_semantic_info";
/// [VK_KHR_SHADER_QUAD_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_QUAD_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_quad_control")]
pub const VK_KHR_SHADER_QUAD_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_QUAD_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_QUAD_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_quad_control")]
pub const VK_KHR_SHADER_QUAD_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_quad_control";
/// [VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
pub const VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_relaxed_extended_instruction")]
pub const VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_relaxed_extended_instruction";
/// [VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
pub const VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_subgroup_extended_types")]
pub const VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_subgroup_extended_types";
/// [VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
pub const VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION: u32 = 2;
/// [VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_subgroup_rotate")]
pub const VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_subgroup_rotate";
/// [VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
pub const VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_subgroup_uniform_control_flow")]
pub const VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_subgroup_uniform_control_flow";
/// [VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_terminate_invocation")]
pub const VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_terminate_invocation")]
pub const VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_terminate_invocation";
/// [VK_KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shader_untyped_pointers")]
pub const VK_KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHADER_UNTYPED_POINTERS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHADER_UNTYPED_POINTERS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shader_untyped_pointers")]
pub const VK_KHR_SHADER_UNTYPED_POINTERS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shader_untyped_pointers";
/// [VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_shared_presentable_image")]
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_shared_presentable_image";
/// [VK_KHR_SPIRV_1_4_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SPIRV_1_4_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_spirv_1_4")]
pub const VK_KHR_SPIRV_1_4_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SPIRV_1_4_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SPIRV_1_4_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_spirv_1_4")]
pub const VK_KHR_SPIRV_1_4_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_spirv_1_4";
/// [VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_storage_buffer_storage_class")]
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_storage_buffer_storage_class";
/// [VK_KHR_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_surface")]
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
/// [VK_KHR_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_surface")]
pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_surface";
/// [VK_KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_surface_maintenance1")]
pub const VK_KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SURFACE_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_surface_maintenance1")]
pub const VK_KHR_SURFACE_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_surface_maintenance1";
/// [VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
pub const VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
pub const VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_surface_protected_capabilities";
/// [VK_KHR_SWAPCHAIN_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_swapchain")]
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
/// [VK_KHR_SWAPCHAIN_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_swapchain")]
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_swapchain";
/// [VK_KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
pub const VK_KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_swapchain_maintenance1")]
pub const VK_KHR_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_swapchain_maintenance1";
/// [VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_swapchain_mutable_format")]
pub const VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_swapchain_mutable_format")]
pub const VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_swapchain_mutable_format";
/// [VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_synchronization2")]
pub const VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_synchronization2")]
pub const VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_synchronization2";
/// [VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub const VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
/// [VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_timeline_semaphore")]
pub const VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_timeline_semaphore";
/// [VK_KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_unified_image_layouts")]
pub const VK_KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_UNIFIED_IMAGE_LAYOUTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_UNIFIED_IMAGE_LAYOUTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_unified_image_layouts")]
pub const VK_KHR_UNIFIED_IMAGE_LAYOUTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_unified_image_layouts";
/// [VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
pub const VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_uniform_buffer_standard_layout")]
pub const VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_uniform_buffer_standard_layout";
/// [VK_KHR_VARIABLE_POINTERS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VARIABLE_POINTERS_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_variable_pointers")]
pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_variable_pointers")]
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_variable_pointers";
/// [VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_vertex_attribute_divisor")]
pub const VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_vertex_attribute_divisor";
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_decode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_DECODE_AV1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_AV1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_av1")]
pub const VK_KHR_VIDEO_DECODE_AV1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_DECODE_AV1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_AV1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_av1")]
pub const VK_KHR_VIDEO_DECODE_AV1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_decode_av1";
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_av1_decode";
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_DECODE_H264_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_H264_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const VK_KHR_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 9;
/// [VK_KHR_VIDEO_DECODE_H264_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_H264_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const VK_KHR_VIDEO_DECODE_H264_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_decode_h264";
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_h264_decode";
/// [STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_decode_h264")]
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_DECODE_H265_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_H265_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const VK_KHR_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 8;
/// [VK_KHR_VIDEO_DECODE_H265_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_H265_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const VK_KHR_VIDEO_DECODE_H265_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_decode_h265";
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_h265_decode";
/// [STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_decode_h265")]
pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;
/// [VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_queue")]
pub const VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 8;
/// [VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_queue")]
pub const VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_decode_queue";
/// [VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_decode_vp9")]
pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_DECODE_VP9_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_VP9_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_vp9")]
pub const VK_KHR_VIDEO_DECODE_VP9_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_DECODE_VP9_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_DECODE_VP9_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_vp9")]
pub const VK_KHR_VIDEO_DECODE_VP9_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_decode_vp9";
/// [VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_decode_vp9")]
pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_decode_vp9")]
pub const VK_STD_VULKAN_VIDEO_CODEC_VP9_DECODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_vp9_decode";
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_encode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_ENCODE_AV1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_AV1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_av1")]
pub const VK_KHR_VIDEO_ENCODE_AV1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_ENCODE_AV1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_AV1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_av1")]
pub const VK_KHR_VIDEO_ENCODE_AV1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_av1";
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_av1")]
pub const VK_STD_VULKAN_VIDEO_CODEC_AV1_ENCODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_av1_encode";
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_encode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_ENCODE_H264_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_H264_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_h264")]
pub const VK_KHR_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 14;
/// [VK_KHR_VIDEO_ENCODE_H264_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_H264_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_h264")]
pub const VK_KHR_VIDEO_ENCODE_H264_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_h264";
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_h264")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_h264_encode";
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0.html)
#[cfg(feature = "VK_KHR_video_encode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0: u32 =
    VK_MAKE_VIDEO_STD_VERSION(1u32, 0u32, 0u32);
/// [VK_KHR_VIDEO_ENCODE_H265_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_H265_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_h265")]
pub const VK_KHR_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 14;
/// [VK_KHR_VIDEO_ENCODE_H265_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_H265_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_h265")]
pub const VK_KHR_VIDEO_ENCODE_H265_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_h265";
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_SPEC_VERSION: u32 =
    VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_API_VERSION_1_0_0;
/// [VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_h265")]
pub const VK_STD_VULKAN_VIDEO_CODEC_H265_ENCODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_STD_vulkan_video_codec_h265_encode";
/// [VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
pub const VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_intra_refresh")]
pub const VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_intra_refresh";
/// [VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
pub const VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION: u32 = 2;
/// [VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_quantization_map")]
pub const VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_quantization_map";
/// [VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub const VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 12;
/// [VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_encode_queue")]
pub const VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_encode_queue";
/// [VK_KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_maintenance1")]
pub const VK_KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_MAINTENANCE_1_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_MAINTENANCE_1_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_maintenance1")]
pub const VK_KHR_VIDEO_MAINTENANCE_1_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_maintenance1";
/// [VK_KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_maintenance2")]
pub const VK_KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION: u32 = 1;
/// [VK_KHR_VIDEO_MAINTENANCE_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_MAINTENANCE_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_maintenance2")]
pub const VK_KHR_VIDEO_MAINTENANCE_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_video_maintenance2";
/// [VK_KHR_VIDEO_QUEUE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_QUEUE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const VK_KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 8;
/// [VK_KHR_VIDEO_QUEUE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VIDEO_QUEUE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const VK_KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_video_queue";
/// [STD_VIDEO_H264_CPB_CNT_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_CPB_CNT_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;
/// [STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
/// [STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
/// [STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
/// [STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
/// [STD_VIDEO_H264_MAX_NUM_LIST_REF](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_MAX_NUM_LIST_REF.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;
/// [STD_VIDEO_H264_MAX_CHROMA_PLANES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_MAX_CHROMA_PLANES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;
/// [STD_VIDEO_H264_NO_REFERENCE_PICTURE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H264_NO_REFERENCE_PICTURE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u32 = 0xFF;
/// [STD_VIDEO_H265_CPB_CNT_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CPB_CNT_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;
/// [STD_VIDEO_H265_SUBLAYERS_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SUBLAYERS_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;
/// [STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
/// [STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
/// [STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
/// [STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
/// [STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
/// [STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
/// [STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
/// [STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
/// [STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
/// [STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
/// [STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
/// [STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
/// [STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
/// [STD_VIDEO_H265_MAX_NUM_LIST_REF](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_NUM_LIST_REF.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;
/// [STD_VIDEO_H265_MAX_CHROMA_PLANES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_CHROMA_PLANES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;
/// [STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
/// [STD_VIDEO_H265_MAX_DPB_SIZE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_DPB_SIZE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;
/// [STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
/// [STD_VIDEO_H265_MAX_LONG_TERM_PICS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_LONG_TERM_PICS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;
/// [STD_VIDEO_H265_MAX_DELTA_POC](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_MAX_DELTA_POC.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;
/// [STD_VIDEO_H265_NO_REFERENCE_PICTURE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_H265_NO_REFERENCE_PICTURE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u32 = 0xFF;
/// [STD_VIDEO_VP9_NUM_REF_FRAMES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_NUM_REF_FRAMES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
/// [STD_VIDEO_VP9_REFS_PER_FRAME](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_REFS_PER_FRAME.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
/// [STD_VIDEO_VP9_MAX_REF_FRAMES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_REF_FRAMES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;
/// [STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
/// [STD_VIDEO_VP9_MAX_SEGMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;
/// [STD_VIDEO_VP9_SEG_LVL_MAX](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_SEG_LVL_MAX.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;
/// [STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
/// [STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
/// [STD_VIDEO_AV1_NUM_REF_FRAMES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_NUM_REF_FRAMES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_NUM_REF_FRAMES: u32 = 8;
/// [STD_VIDEO_AV1_REFS_PER_FRAME](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_REFS_PER_FRAME.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_REFS_PER_FRAME: u32 = 7;
/// [STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: u32 = 8;
/// [STD_VIDEO_AV1_MAX_TILE_COLS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_TILE_COLS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;
/// [STD_VIDEO_AV1_MAX_TILE_ROWS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_TILE_ROWS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;
/// [STD_VIDEO_AV1_MAX_SEGMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_SEGMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_SEGMENTS: u32 = 8;
/// [STD_VIDEO_AV1_SEG_LVL_MAX](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SEG_LVL_MAX.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_SEG_LVL_MAX: u32 = 8;
/// [STD_VIDEO_AV1_PRIMARY_REF_NONE](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_PRIMARY_REF_NONE.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u32 = 7;
/// [STD_VIDEO_AV1_SELECT_INTEGER_MV](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SELECT_INTEGER_MV.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u32 = 2;
/// [STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
/// [STD_VIDEO_AV1_SKIP_MODE_FRAMES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_SKIP_MODE_FRAMES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: u32 = 2;
/// [STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;
/// [STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
/// [STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;
/// [STD_VIDEO_AV1_MAX_NUM_PLANES](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_PLANES.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_PLANES: u32 = 3;
/// [STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: u32 = 6;
/// [STD_VIDEO_AV1_MAX_NUM_Y_POINTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_Y_POINTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: u32 = 14;
/// [STD_VIDEO_AV1_MAX_NUM_CB_POINTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_CB_POINTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: u32 = 10;
/// [STD_VIDEO_AV1_MAX_NUM_CR_POINTS](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_CR_POINTS.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: u32 = 10;
/// [STD_VIDEO_AV1_MAX_NUM_POS_LUMA](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_POS_LUMA.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: u32 = 24;
/// [STD_VIDEO_AV1_MAX_NUM_POS_CHROMA](https://docs.vulkan.org/refpages/latest/refpages/source/STD_VIDEO_AV1_MAX_NUM_POS_CHROMA.html)
#[cfg(feature = "VK_KHR_video_queue")]
pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: u32 = 25;
/// [VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_vulkan_memory_model")]
pub const VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
/// [VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_vulkan_memory_model")]
pub const VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_vulkan_memory_model";
/// [VK_KHR_WAYLAND_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WAYLAND_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_wayland_surface")]
pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
/// [VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_wayland_surface")]
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_wayland_surface";
/// [VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
pub const VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
/// [VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_win32_keyed_mutex")]
pub const VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_win32_keyed_mutex";
/// [VK_KHR_WIN32_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WIN32_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_win32_surface")]
pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
/// [VK_KHR_WIN32_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WIN32_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_win32_surface")]
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_win32_surface";
/// [VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
pub const VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
/// [VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_workgroup_memory_explicit_layout")]
pub const VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_workgroup_memory_explicit_layout";
/// [VK_KHR_XCB_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_XCB_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_xcb_surface")]
pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
/// [VK_KHR_XCB_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_XCB_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_xcb_surface")]
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_xcb_surface";
/// [VK_KHR_XLIB_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_XLIB_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_xlib_surface")]
pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;
/// [VK_KHR_XLIB_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_XLIB_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_xlib_surface")]
pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_KHR_xlib_surface";
/// [VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
pub const VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_KHR_zero_initialize_workgroup_memory")]
pub const VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_KHR_zero_initialize_workgroup_memory";
/// [VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION.html)
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
pub const VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION: u32 = 1;
/// [VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME.html)
#[cfg(feature = "VK_LUNARG_direct_driver_loading")]
pub const VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_LUNARG_direct_driver_loading";
/// [VK_MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION.html)
#[cfg(feature = "VK_MESA_image_alignment_control")]
pub const VK_MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION: u32 = 1;
/// [VK_MESA_IMAGE_ALIGNMENT_CONTROL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MESA_IMAGE_ALIGNMENT_CONTROL_EXTENSION_NAME.html)
#[cfg(feature = "VK_MESA_image_alignment_control")]
pub const VK_MESA_IMAGE_ALIGNMENT_CONTROL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_MESA_image_alignment_control";
/// [VK_MSFT_LAYERED_DRIVER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MSFT_LAYERED_DRIVER_SPEC_VERSION.html)
#[cfg(feature = "VK_MSFT_layered_driver")]
pub const VK_MSFT_LAYERED_DRIVER_SPEC_VERSION: u32 = 1;
/// [VK_MSFT_LAYERED_DRIVER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MSFT_LAYERED_DRIVER_EXTENSION_NAME.html)
#[cfg(feature = "VK_MSFT_layered_driver")]
pub const VK_MSFT_LAYERED_DRIVER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_MSFT_layered_driver";
/// [VK_MVK_IOS_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_IOS_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_MVK_ios_surface")]
pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;
/// [VK_MVK_IOS_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_IOS_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_MVK_ios_surface")]
pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_MVK_ios_surface";
/// [VK_MVK_MACOS_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_MACOS_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_MVK_macos_surface")]
pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;
/// [VK_MVK_MACOS_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_MACOS_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_MVK_macos_surface")]
pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_MVK_macos_surface";
/// [VK_NN_VI_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NN_VI_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_NN_vi_surface")]
pub const VK_NN_VI_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_NN_VI_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NN_VI_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NN_vi_surface")]
pub const VK_NN_VI_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NN_vi_surface";
/// [VK_NVX_BINARY_IMPORT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_BINARY_IMPORT_SPEC_VERSION.html)
#[cfg(feature = "VK_NVX_binary_import")]
pub const VK_NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 2;
/// [VK_NVX_BINARY_IMPORT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_BINARY_IMPORT_EXTENSION_NAME.html)
#[cfg(feature = "VK_NVX_binary_import")]
pub const VK_NVX_BINARY_IMPORT_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NVX_binary_import";
/// [VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION.html)
#[cfg(feature = "VK_NVX_image_view_handle")]
pub const VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 4;
/// [VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NVX_image_view_handle")]
pub const VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NVX_image_view_handle";
/// [VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION.html)
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
/// [VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME.html)
#[cfg(feature = "VK_NVX_multiview_per_view_attributes")]
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NVX_multiview_per_view_attributes";
/// [VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_acquire_winrt_display")]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
/// [VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_acquire_winrt_display")]
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_acquire_winrt_display";
/// [VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
/// [VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_clip_space_w_scaling")]
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_clip_space_w_scaling";
/// [VK_NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub const VK_NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 4;
/// [VK_NV_CLUSTER_ACCELERATION_STRUCTURE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CLUSTER_ACCELERATION_STRUCTURE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_cluster_acceleration_structure")]
pub const VK_NV_CLUSTER_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_cluster_acceleration_structure";
/// [VK_NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
pub const VK_NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION: u32 = 1;
/// [VK_NV_COMMAND_BUFFER_INHERITANCE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMMAND_BUFFER_INHERITANCE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_command_buffer_inheritance")]
pub const VK_NV_COMMAND_BUFFER_INHERITANCE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_command_buffer_inheritance";
/// [VK_NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
pub const VK_NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION: u32 = 1;
/// [VK_NV_COMPUTE_OCCUPANCY_PRIORITY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMPUTE_OCCUPANCY_PRIORITY_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_compute_occupancy_priority")]
pub const VK_NV_COMPUTE_OCCUPANCY_PRIORITY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_compute_occupancy_priority";
/// [VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
pub const VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
/// [VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_compute_shader_derivatives")]
pub const VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_compute_shader_derivatives";
/// [VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_cooperative_matrix")]
pub const VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
/// [VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_cooperative_matrix")]
pub const VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_cooperative_matrix";
/// [VK_NV_COOPERATIVE_MATRIX_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_MATRIX_2_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_cooperative_matrix2")]
pub const VK_NV_COOPERATIVE_MATRIX_2_SPEC_VERSION: u32 = 1;
/// [VK_NV_COOPERATIVE_MATRIX_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_MATRIX_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_cooperative_matrix2")]
pub const VK_NV_COOPERATIVE_MATRIX_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_cooperative_matrix2";
/// [VK_NV_COOPERATIVE_VECTOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_VECTOR_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_cooperative_vector")]
pub const VK_NV_COOPERATIVE_VECTOR_SPEC_VERSION: u32 = 4;
/// [VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_cooperative_vector")]
pub const VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_cooperative_vector";
/// [VK_NV_COPY_MEMORY_INDIRECT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COPY_MEMORY_INDIRECT_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_copy_memory_indirect")]
pub const VK_NV_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;
/// [VK_NV_COPY_MEMORY_INDIRECT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COPY_MEMORY_INDIRECT_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_copy_memory_indirect")]
pub const VK_NV_COPY_MEMORY_INDIRECT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_copy_memory_indirect";
/// [VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_corner_sampled_image")]
pub const VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
/// [VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_corner_sampled_image")]
pub const VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_corner_sampled_image";
/// [VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
pub const VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
/// [VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_coverage_reduction_mode")]
pub const VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_coverage_reduction_mode";
/// [VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub const VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION: u32 = 2;
/// [VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_cuda_kernel_launch")]
pub const VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_cuda_kernel_launch";
/// [VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
/// [VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_dedicated_allocation")]
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_dedicated_allocation";
/// [VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
pub const VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
/// [VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_dedicated_allocation_image_aliasing")]
pub const VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_dedicated_allocation_image_aliasing";
/// [VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
pub const VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION: u32 = 1;
/// [VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_descriptor_pool_overallocation")]
pub const VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_descriptor_pool_overallocation";
/// [VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub const VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
/// [VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_device_diagnostic_checkpoints")]
pub const VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_device_diagnostic_checkpoints";
/// [VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_device_diagnostics_config")]
pub const VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 2;
/// [VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_device_diagnostics_config")]
pub const VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_device_diagnostics_config";
/// [VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
pub const VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
/// [VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_device_generated_commands")]
pub const VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_device_generated_commands";
/// [VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
pub const VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION: u32 = 2;
/// [VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_device_generated_commands_compute")]
pub const VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_device_generated_commands_compute";
/// [VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_displacement_micromap")]
pub const VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION: u32 = 2;
/// [VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_displacement_micromap")]
pub const VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_displacement_micromap";
/// [VK_NV_DISPLAY_STEREO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DISPLAY_STEREO_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_display_stereo")]
pub const VK_NV_DISPLAY_STEREO_SPEC_VERSION: u32 = 1;
/// [VK_NV_DISPLAY_STEREO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_DISPLAY_STEREO_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_display_stereo")]
pub const VK_NV_DISPLAY_STEREO_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_display_stereo";
/// [VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
pub const VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_extended_sparse_address_space")]
pub const VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_extended_sparse_address_space";
/// [VK_NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_compute_queue")]
pub const VK_NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_COMPUTE_QUEUE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_COMPUTE_QUEUE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_compute_queue")]
pub const VK_NV_EXTERNAL_COMPUTE_QUEUE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_compute_queue";
/// [VK_NV_EXTERNAL_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_memory")]
pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_memory")]
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_external_memory";
/// [VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_memory_capabilities")]
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_memory_capabilities";
/// [VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub const VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_memory_rdma")]
pub const VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_memory_rdma";
/// [VK_NV_EXTERNAL_MEMORY_SCI_BUF_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_SCI_BUF_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub const VK_NV_EXTERNAL_MEMORY_SCI_BUF_SPEC_VERSION: u32 = 2;
/// [VK_NV_EXTERNAL_MEMORY_SCI_BUF_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_SCI_BUF_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_memory_sci_buf")]
pub const VK_NV_EXTERNAL_MEMORY_SCI_BUF_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_memory_sci_buf";
/// [VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_memory_win32")]
pub const VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_memory_win32")]
pub const VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_memory_win32";
/// [VK_NV_EXTERNAL_SCI_SYNC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_SCI_SYNC_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_sci_sync")]
pub const VK_NV_EXTERNAL_SCI_SYNC_SPEC_VERSION: u32 = 2;
/// [VK_NV_EXTERNAL_SCI_SYNC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_SCI_SYNC_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_sci_sync")]
pub const VK_NV_EXTERNAL_SCI_SYNC_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_sci_sync";
/// [VK_NV_EXTERNAL_SCI_SYNC_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_SCI_SYNC_2_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub const VK_NV_EXTERNAL_SCI_SYNC_2_SPEC_VERSION: u32 = 1;
/// [VK_NV_EXTERNAL_SCI_SYNC_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_EXTERNAL_SCI_SYNC_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_external_sci_sync2")]
pub const VK_NV_EXTERNAL_SCI_SYNC_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_external_sci_sync2";
/// [VK_NV_FILL_RECTANGLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FILL_RECTANGLE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_fill_rectangle")]
pub const VK_NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;
/// [VK_NV_FILL_RECTANGLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FILL_RECTANGLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_fill_rectangle")]
pub const VK_NV_FILL_RECTANGLE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_fill_rectangle";
/// [VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
/// [VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_fragment_coverage_to_color")]
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_fragment_coverage_to_color";
/// [VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
pub const VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
/// [VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_fragment_shader_barycentric")]
pub const VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_fragment_shader_barycentric";
/// [VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
pub const VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
/// [VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_fragment_shading_rate_enums")]
pub const VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_fragment_shading_rate_enums";
/// [VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
/// [VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_framebuffer_mixed_samples")]
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_framebuffer_mixed_samples";
/// [VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;
/// [VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_geometry_shader_passthrough")]
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_geometry_shader_passthrough";
/// [VK_NV_GLSL_SHADER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_GLSL_SHADER_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_glsl_shader")]
pub const VK_NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;
/// [VK_NV_GLSL_SHADER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_GLSL_SHADER_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_glsl_shader")]
pub const VK_NV_GLSL_SHADER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_glsl_shader";
/// [VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
pub const VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
/// [VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_inherited_viewport_scissor")]
pub const VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_inherited_viewport_scissor";
/// [VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_linear_color_attachment")]
pub const VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
/// [VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_linear_color_attachment")]
pub const VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_linear_color_attachment";
/// [VK_NV_LOW_LATENCY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LOW_LATENCY_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_low_latency")]
pub const VK_NV_LOW_LATENCY_SPEC_VERSION: u32 = 1;
/// [VK_NV_LOW_LATENCY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LOW_LATENCY_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_low_latency")]
pub const VK_NV_LOW_LATENCY_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_low_latency";
/// [VK_NV_LOW_LATENCY_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LOW_LATENCY_2_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_low_latency2")]
pub const VK_NV_LOW_LATENCY_2_SPEC_VERSION: u32 = 2;
/// [VK_NV_LOW_LATENCY_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_LOW_LATENCY_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_low_latency2")]
pub const VK_NV_LOW_LATENCY_2_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_low_latency2";
/// [VK_NV_MEMORY_DECOMPRESSION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_MEMORY_DECOMPRESSION_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_memory_decompression")]
pub const VK_NV_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;
/// [VK_NV_MEMORY_DECOMPRESSION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_MEMORY_DECOMPRESSION_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_memory_decompression")]
pub const VK_NV_MEMORY_DECOMPRESSION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_memory_decompression";
/// [VK_NV_MESH_SHADER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_MESH_SHADER_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_mesh_shader")]
pub const VK_NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
/// [VK_NV_MESH_SHADER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_MESH_SHADER_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_mesh_shader")]
pub const VK_NV_MESH_SHADER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_mesh_shader";
/// [VK_NV_OPTICAL_FLOW_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_OPTICAL_FLOW_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub const VK_NV_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;
/// [VK_NV_OPTICAL_FLOW_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_OPTICAL_FLOW_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_optical_flow")]
pub const VK_NV_OPTICAL_FLOW_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_optical_flow";
/// [VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
pub const VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 1;
/// [VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_partitioned_acceleration_structure")]
pub const VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_partitioned_acceleration_structure";
/// [VK_NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
pub const VK_NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION: u32 = 1;
/// [VK_NV_PER_STAGE_DESCRIPTOR_SET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PER_STAGE_DESCRIPTOR_SET_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_per_stage_descriptor_set")]
pub const VK_NV_PER_STAGE_DESCRIPTOR_SET_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_per_stage_descriptor_set";
/// [VK_NV_PRESENT_BARRIER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRESENT_BARRIER_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_present_barrier")]
pub const VK_NV_PRESENT_BARRIER_SPEC_VERSION: u32 = 1;
/// [VK_NV_PRESENT_BARRIER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRESENT_BARRIER_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_present_barrier")]
pub const VK_NV_PRESENT_BARRIER_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_present_barrier";
/// [VK_NV_PRESENT_METERING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRESENT_METERING_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_present_metering")]
pub const VK_NV_PRESENT_METERING_SPEC_VERSION: u32 = 1;
/// [VK_NV_PRESENT_METERING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRESENT_METERING_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_present_metering")]
pub const VK_NV_PRESENT_METERING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_present_metering";
/// [VK_NV_PRIVATE_VENDOR_INFO_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRIVATE_VENDOR_INFO_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_private_vendor_info")]
pub const VK_NV_PRIVATE_VENDOR_INFO_SPEC_VERSION: u32 = 2;
/// [VK_NV_PRIVATE_VENDOR_INFO_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PRIVATE_VENDOR_INFO_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_private_vendor_info")]
pub const VK_NV_PRIVATE_VENDOR_INFO_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_private_vendor_info";
/// [VK_NV_PUSH_CONSTANT_BANK_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PUSH_CONSTANT_BANK_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_push_constant_bank")]
pub const VK_NV_PUSH_CONSTANT_BANK_SPEC_VERSION: u32 = 1;
/// [VK_NV_PUSH_CONSTANT_BANK_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_PUSH_CONSTANT_BANK_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_push_constant_bank")]
pub const VK_NV_PUSH_CONSTANT_BANK_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_push_constant_bank";
/// [VK_NV_RAW_ACCESS_CHAINS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAW_ACCESS_CHAINS_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_raw_access_chains")]
pub const VK_NV_RAW_ACCESS_CHAINS_SPEC_VERSION: u32 = 1;
/// [VK_NV_RAW_ACCESS_CHAINS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAW_ACCESS_CHAINS_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_raw_access_chains")]
pub const VK_NV_RAW_ACCESS_CHAINS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_raw_access_chains";
/// [VK_NV_RAY_TRACING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_ray_tracing")]
pub const VK_NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
/// [VK_NV_RAY_TRACING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_ray_tracing")]
pub const VK_NV_RAY_TRACING_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_NV_ray_tracing";
/// [VK_NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
pub const VK_NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 1;
/// [VK_NV_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_ray_tracing_invocation_reorder")]
pub const VK_NV_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_ray_tracing_invocation_reorder";
/// [VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
pub const VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION: u32 = 1;
/// [VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_ray_tracing_linear_swept_spheres")]
pub const VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_ray_tracing_linear_swept_spheres";
/// [VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
pub const VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
/// [VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
pub const VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_ray_tracing_motion_blur";
/// [VK_NV_RAY_TRACING_VALIDATION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_VALIDATION_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_ray_tracing_validation")]
pub const VK_NV_RAY_TRACING_VALIDATION_SPEC_VERSION: u32 = 1;
/// [VK_NV_RAY_TRACING_VALIDATION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_RAY_TRACING_VALIDATION_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_ray_tracing_validation")]
pub const VK_NV_RAY_TRACING_VALIDATION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_ray_tracing_validation";
/// [VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_representative_fragment_test")]
pub const VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
/// [VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_representative_fragment_test")]
pub const VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_representative_fragment_test";
/// [VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;
/// [VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_sample_mask_override_coverage")]
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_sample_mask_override_coverage";
/// [VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_scissor_exclusive")]
pub const VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 2;
/// [VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_scissor_exclusive")]
pub const VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_scissor_exclusive";
/// [VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
pub const VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION: u32 = 1;
/// [VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_shader_atomic_float16_vector")]
pub const VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_shader_atomic_float16_vector";
/// [VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_shader_image_footprint")]
pub const VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
/// [VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_shader_image_footprint")]
pub const VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_shader_image_footprint";
/// [VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_shader_sm_builtins")]
pub const VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
/// [VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_shader_sm_builtins")]
pub const VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_shader_sm_builtins";
/// [VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
pub const VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;
/// [VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_shader_subgroup_partitioned")]
pub const VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_shader_subgroup_partitioned";
/// [VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_shading_rate_image")]
pub const VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
/// [VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_shading_rate_image")]
pub const VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_shading_rate_image";
/// [VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_viewport_array2")]
pub const VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;
/// [VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_viewport_array2")]
pub const VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_viewport_array2";
/// [VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
/// [VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_viewport_swizzle")]
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_viewport_swizzle";
/// [VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION.html)
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
pub const VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
/// [VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME.html)
#[cfg(feature = "VK_NV_win32_keyed_mutex")]
pub const VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_NV_win32_keyed_mutex";
/// [VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION.html)
#[cfg(feature = "VK_OHOS_external_memory")]
pub const VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
/// [VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME.html)
#[cfg(feature = "VK_OHOS_external_memory")]
pub const VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_OHOS_external_memory";
/// [VK_OHOS_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_OHOS_surface")]
pub const VK_OHOS_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_OHOS_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_OHOS_surface")]
pub const VK_OHOS_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_OHOS_surface";
/// [VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
pub const VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_cooperative_matrix_conversion")]
pub const VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_cooperative_matrix_conversion";
/// [VK_QCOM_DATA_GRAPH_MODEL_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_DATA_GRAPH_MODEL_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_data_graph_model")]
pub const VK_QCOM_DATA_GRAPH_MODEL_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_DATA_GRAPH_MODEL_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_DATA_GRAPH_MODEL_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_data_graph_model")]
pub const VK_QCOM_DATA_GRAPH_MODEL_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_data_graph_model";
/// [VK_QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
pub const VK_QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_FILTER_CUBIC_CLAMP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FILTER_CUBIC_CLAMP_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_filter_cubic_clamp")]
pub const VK_QCOM_FILTER_CUBIC_CLAMP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_filter_cubic_clamp";
/// [VK_QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
pub const VK_QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_FILTER_CUBIC_WEIGHTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FILTER_CUBIC_WEIGHTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_filter_cubic_weights")]
pub const VK_QCOM_FILTER_CUBIC_WEIGHTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_filter_cubic_weights";
/// [VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
pub const VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 3;
/// [VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_fragment_density_map_offset")]
pub const VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_fragment_density_map_offset";
/// [VK_QCOM_IMAGE_PROCESSING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_IMAGE_PROCESSING_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_image_processing")]
pub const VK_QCOM_IMAGE_PROCESSING_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_IMAGE_PROCESSING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_IMAGE_PROCESSING_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_image_processing")]
pub const VK_QCOM_IMAGE_PROCESSING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_image_processing";
/// [VK_QCOM_IMAGE_PROCESSING_2_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_IMAGE_PROCESSING_2_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_image_processing2")]
pub const VK_QCOM_IMAGE_PROCESSING_2_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_IMAGE_PROCESSING_2_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_IMAGE_PROCESSING_2_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_image_processing2")]
pub const VK_QCOM_IMAGE_PROCESSING_2_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_image_processing2";
/// [VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
pub const VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_multiview_per_view_render_areas")]
pub const VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_multiview_per_view_render_areas";
/// [VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
pub const VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_multiview_per_view_viewports")]
pub const VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_multiview_per_view_viewports";
/// [VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
pub const VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION: u32 = 4;
/// [VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_render_pass_shader_resolve")]
pub const VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_render_pass_shader_resolve";
/// [VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_render_pass_store_ops")]
pub const VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION: u32 = 2;
/// [VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_render_pass_store_ops")]
pub const VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_render_pass_store_ops";
/// [VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_render_pass_transform")]
pub const VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 5;
/// [VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_render_pass_transform")]
pub const VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_render_pass_transform";
/// [VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
pub const VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 2;
/// [VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_rotated_copy_commands")]
pub const VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_rotated_copy_commands";
/// [VK_QCOM_TILE_MEMORY_HEAP_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_MEMORY_HEAP_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
pub const VK_QCOM_TILE_MEMORY_HEAP_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_TILE_MEMORY_HEAP_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_MEMORY_HEAP_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_tile_memory_heap")]
pub const VK_QCOM_TILE_MEMORY_HEAP_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_tile_memory_heap";
/// [VK_QCOM_TILE_PROPERTIES_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_PROPERTIES_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_tile_properties")]
pub const VK_QCOM_TILE_PROPERTIES_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_TILE_PROPERTIES_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_PROPERTIES_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_tile_properties")]
pub const VK_QCOM_TILE_PROPERTIES_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QCOM_tile_properties";
/// [VK_QCOM_TILE_SHADING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_SHADING_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
pub const VK_QCOM_TILE_SHADING_SPEC_VERSION: u32 = 2;
/// [VK_QCOM_TILE_SHADING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_TILE_SHADING_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_tile_shading")]
pub const VK_QCOM_TILE_SHADING_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_QCOM_tile_shading";
/// [VK_QCOM_YCBCR_DEGAMMA_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_YCBCR_DEGAMMA_SPEC_VERSION.html)
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
pub const VK_QCOM_YCBCR_DEGAMMA_SPEC_VERSION: u32 = 1;
/// [VK_QCOM_YCBCR_DEGAMMA_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_YCBCR_DEGAMMA_EXTENSION_NAME.html)
#[cfg(feature = "VK_QCOM_ycbcr_degamma")]
pub const VK_QCOM_YCBCR_DEGAMMA_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_QCOM_ycbcr_degamma";
/// [VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION.html)
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
pub const VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION: u32 = 1;
/// [VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME.html)
#[cfg(feature = "VK_QNX_external_memory_screen_buffer")]
pub const VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_QNX_external_memory_screen_buffer";
/// [VK_QNX_SCREEN_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_SCREEN_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_QNX_screen_surface")]
pub const VK_QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_QNX_SCREEN_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_SCREEN_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_QNX_screen_surface")]
pub const VK_QNX_SCREEN_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_QNX_screen_surface";
/// [VK_SEC_AMIGO_PROFILING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_AMIGO_PROFILING_SPEC_VERSION.html)
#[cfg(feature = "VK_SEC_amigo_profiling")]
pub const VK_SEC_AMIGO_PROFILING_SPEC_VERSION: u32 = 1;
/// [VK_SEC_AMIGO_PROFILING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_AMIGO_PROFILING_EXTENSION_NAME.html)
#[cfg(feature = "VK_SEC_amigo_profiling")]
pub const VK_SEC_AMIGO_PROFILING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_SEC_amigo_profiling";
/// [VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION.html)
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
pub const VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION: u32 = 1;
/// [VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_EXTENSION_NAME.html)
#[cfg(feature = "VK_SEC_pipeline_cache_incremental_mode")]
pub const VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_SEC_pipeline_cache_incremental_mode";
/// [VK_SEC_UBM_SURFACE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_UBM_SURFACE_SPEC_VERSION.html)
#[cfg(feature = "VK_SEC_ubm_surface")]
pub const VK_SEC_UBM_SURFACE_SPEC_VERSION: u32 = 1;
/// [VK_SEC_UBM_SURFACE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_UBM_SURFACE_EXTENSION_NAME.html)
#[cfg(feature = "VK_SEC_ubm_surface")]
pub const VK_SEC_UBM_SURFACE_EXTENSION_NAME: &'static core::ffi::CStr = c"VK_SEC_ubm_surface";
/// [VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION.html)
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
pub const VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
/// [VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME.html)
#[cfg(feature = "VK_VALVE_descriptor_set_host_mapping")]
pub const VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_VALVE_descriptor_set_host_mapping";
/// [VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION.html)
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
pub const VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION: u32 = 1;
/// [VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_EXTENSION_NAME.html)
#[cfg(feature = "VK_VALVE_fragment_density_map_layered")]
pub const VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_VALVE_fragment_density_map_layered";
/// [VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION.html)
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
pub const VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
/// [VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME.html)
#[cfg(feature = "VK_VALVE_mutable_descriptor_type")]
pub const VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_VALVE_mutable_descriptor_type";
/// [VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION.html)
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
pub const VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
/// [VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_EXTENSION_NAME.html)
#[cfg(feature = "VK_VALVE_shader_mixed_float_dot_product")]
pub const VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_VALVE_shader_mixed_float_dot_product";
/// [VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION.html)
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
pub const VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION: u32 = 1;
/// [VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_EXTENSION_NAME](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_EXTENSION_NAME.html)
#[cfg(feature = "VK_VALVE_video_encode_rgb_conversion")]
pub const VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_EXTENSION_NAME: &'static core::ffi::CStr =
    c"VK_VALVE_video_encode_rgb_conversion";
