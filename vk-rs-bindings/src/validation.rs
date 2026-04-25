//! Compile-time feature dependency validation.
//!
//! If you enable an extension feature without its required dependencies,
//! a `compile_error!` will fire.  When using Cargo feature implication
//! these checks are redundant but help with manual `cfg` usage.
#[cfg(all(feature = "VK_KHR_swapchain", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_swapchain` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain.html"
);
#[cfg(all(feature = "VK_KHR_display", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_display` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display.html"
);
#[cfg(all(
  feature = "VK_KHR_display_swapchain",
  not(all(feature = "VK_KHR_swapchain", feature = "VK_KHR_display"))
))]
compile_error!(
  "Feature `VK_KHR_display_swapchain` requires `VK_KHR_swapchain + VK_KHR_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display_swapchain.html"
);
#[cfg(all(feature = "VK_KHR_xlib_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_xlib_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xlib_surface.html"
);
#[cfg(all(feature = "VK_KHR_xcb_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_xcb_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xcb_surface.html"
);
#[cfg(all(feature = "VK_KHR_wayland_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_wayland_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_wayland_surface.html"
);
#[cfg(all(feature = "VK_KHR_android_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_android_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_android_surface.html"
);
#[cfg(all(feature = "VK_KHR_win32_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_KHR_win32_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_surface.html"
);
#[cfg(all(feature = "VK_EXT_debug_marker", not(feature = "VK_EXT_debug_report")))]
compile_error!(
  "Feature `VK_EXT_debug_marker` requires `VK_EXT_debug_report`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_marker.html"
);
#[cfg(all(
  feature = "VK_KHR_video_queue",
  not(any(
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_synchronization2"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_KHR_video_queue` requires `VK_VERSION_1_1 + VK_KHR_synchronization2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_queue.html"
);
#[cfg(all(
  feature = "VK_KHR_video_decode_queue",
  not(any(
    all(feature = "VK_KHR_video_queue", feature = "VK_KHR_synchronization2"),
    all(feature = "VK_KHR_video_queue", feature = "VK_VERSION_1_3")
  ))
))]
compile_error!(
  "Feature `VK_KHR_video_decode_queue` requires `VK_KHR_video_queue + VK_KHR_synchronization2 , VK_KHR_video_queue + VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_queue.html"
);
#[cfg(all(
  feature = "VK_EXT_transform_feedback",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_transform_feedback` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_transform_feedback.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_h264",
  not(feature = "VK_KHR_video_encode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_encode_h264` requires `VK_KHR_video_encode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h264.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_h265",
  not(feature = "VK_KHR_video_encode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_encode_h265` requires `VK_KHR_video_encode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h265.html"
);
#[cfg(all(
  feature = "VK_KHR_video_decode_h264",
  not(feature = "VK_KHR_video_decode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_decode_h264` requires `VK_KHR_video_decode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h264.html"
);
#[cfg(all(
  feature = "VK_AMD_texture_gather_bias_lod",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_AMD_texture_gather_bias_lod` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_texture_gather_bias_lod.html"
);
#[cfg(all(
  feature = "VK_KHR_dynamic_rendering",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_depth_stencil_resolve"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_depth_stencil_resolve"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_dynamic_rendering` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_depth_stencil_resolve , VK_VERSION_1_1 + VK_KHR_depth_stencil_resolve , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering.html"
);
#[cfg(all(
  feature = "VK_GGP_stream_descriptor_surface",
  not(feature = "VK_KHR_surface")
))]
compile_error!(
  "Feature `VK_GGP_stream_descriptor_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_stream_descriptor_surface.html"
);
#[cfg(all(
  feature = "VK_NV_corner_sampled_image",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_corner_sampled_image` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_corner_sampled_image.html"
);
#[cfg(all(
  feature = "VK_KHR_multiview",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_multiview` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_multiview.html"
);
#[cfg(all(
  feature = "VK_NV_external_memory",
  not(feature = "VK_NV_external_memory_capabilities")
))]
compile_error!(
  "Feature `VK_NV_external_memory` requires `VK_NV_external_memory_capabilities`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory.html"
);
#[cfg(all(
  feature = "VK_NV_external_memory_win32",
  not(feature = "VK_NV_external_memory")
))]
compile_error!(
  "Feature `VK_NV_external_memory_win32` requires `VK_NV_external_memory`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_win32.html"
);
#[cfg(all(
  feature = "VK_NV_win32_keyed_mutex",
  not(feature = "VK_NV_external_memory_win32")
))]
compile_error!(
  "Feature `VK_NV_win32_keyed_mutex` requires `VK_NV_external_memory_win32`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_win32_keyed_mutex.html"
);
#[cfg(all(
  feature = "VK_KHR_device_group",
  not(feature = "VK_KHR_device_group_creation")
))]
compile_error!(
  "Feature `VK_KHR_device_group` requires `VK_KHR_device_group_creation`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_group.html"
);
#[cfg(all(feature = "VK_NN_vi_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_NN_vi_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NN_vi_surface.html"
);
#[cfg(all(
  feature = "VK_EXT_texture_compression_astc_hdr",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_texture_compression_astc_hdr` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_hdr.html"
);
#[cfg(all(
  feature = "VK_EXT_astc_decode_mode",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_astc_decode_mode` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_astc_decode_mode.html"
);
#[cfg(all(
  feature = "VK_EXT_pipeline_robustness",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pipeline_robustness` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_robustness.html"
);
#[cfg(all(
  feature = "VK_KHR_external_memory_capabilities",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_external_memory_capabilities` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_capabilities.html"
);
#[cfg(all(
  feature = "VK_KHR_external_memory",
  not(any(
    feature = "VK_KHR_external_memory_capabilities",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_external_memory` requires `VK_KHR_external_memory_capabilities , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory.html"
);
#[cfg(all(
  feature = "VK_KHR_external_memory_win32",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_KHR_external_memory_win32` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_win32.html"
);
#[cfg(all(
  feature = "VK_KHR_external_memory_fd",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_KHR_external_memory_fd` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_fd.html"
);
#[cfg(all(
  feature = "VK_KHR_win32_keyed_mutex",
  not(feature = "VK_KHR_external_memory_win32")
))]
compile_error!(
  "Feature `VK_KHR_win32_keyed_mutex` requires `VK_KHR_external_memory_win32`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_keyed_mutex.html"
);
#[cfg(all(
  feature = "VK_KHR_external_semaphore_capabilities",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_external_semaphore_capabilities` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_capabilities.html"
);
#[cfg(all(
  feature = "VK_KHR_external_semaphore",
  not(feature = "VK_KHR_external_semaphore_capabilities")
))]
compile_error!(
  "Feature `VK_KHR_external_semaphore` requires `VK_KHR_external_semaphore_capabilities`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore.html"
);
#[cfg(all(
  feature = "VK_KHR_external_semaphore_win32",
  not(feature = "VK_KHR_external_semaphore")
))]
compile_error!(
  "Feature `VK_KHR_external_semaphore_win32` requires `VK_KHR_external_semaphore`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_win32.html"
);
#[cfg(all(
  feature = "VK_KHR_external_semaphore_fd",
  not(any(feature = "VK_KHR_external_semaphore", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_KHR_external_semaphore_fd` requires `VK_KHR_external_semaphore , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_fd.html"
);
#[cfg(all(
  feature = "VK_KHR_push_descriptor",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_push_descriptor` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_push_descriptor.html"
);
#[cfg(all(
  feature = "VK_EXT_conditional_rendering",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_conditional_rendering` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conditional_rendering.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_float16_int8",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_float16_int8` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float16_int8.html"
);
#[cfg(all(
  feature = "VK_KHR_16bit_storage",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_storage_buffer_storage_class"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_16bit_storage` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_storage_buffer_storage_class , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_16bit_storage.html"
);
#[cfg(all(
  feature = "VK_KHR_incremental_present",
  not(feature = "VK_KHR_swapchain")
))]
compile_error!(
  "Feature `VK_KHR_incremental_present` requires `VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_incremental_present.html"
);
#[cfg(all(
  feature = "VK_EXT_direct_mode_display",
  not(feature = "VK_KHR_display")
))]
compile_error!(
  "Feature `VK_EXT_direct_mode_display` requires `VK_KHR_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_direct_mode_display.html"
);
#[cfg(all(
  feature = "VK_EXT_acquire_xlib_display",
  not(feature = "VK_EXT_direct_mode_display")
))]
compile_error!(
  "Feature `VK_EXT_acquire_xlib_display` requires `VK_EXT_direct_mode_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_xlib_display.html"
);
#[cfg(all(
  feature = "VK_EXT_display_surface_counter",
  not(feature = "VK_KHR_display")
))]
compile_error!(
  "Feature `VK_EXT_display_surface_counter` requires `VK_KHR_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_surface_counter.html"
);
#[cfg(all(
  feature = "VK_EXT_display_control",
  not(all(
    feature = "VK_EXT_display_surface_counter",
    feature = "VK_KHR_swapchain"
  ))
))]
compile_error!(
  "Feature `VK_EXT_display_control` requires `VK_EXT_display_surface_counter + VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_control.html"
);
#[cfg(all(
  feature = "VK_GOOGLE_display_timing",
  not(feature = "VK_KHR_swapchain")
))]
compile_error!(
  "Feature `VK_GOOGLE_display_timing` requires `VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_display_timing.html"
);
#[cfg(all(
  feature = "VK_NVX_multiview_per_view_attributes",
  not(any(feature = "VK_KHR_multiview", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_NVX_multiview_per_view_attributes` requires `VK_KHR_multiview , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_multiview_per_view_attributes.html"
);
#[cfg(all(
  feature = "VK_EXT_discard_rectangles",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_discard_rectangles` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_discard_rectangles.html"
);
#[cfg(all(
  feature = "VK_EXT_conservative_rasterization",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_conservative_rasterization` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conservative_rasterization.html"
);
#[cfg(all(
  feature = "VK_EXT_depth_clip_enable",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_depth_clip_enable` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_enable.html"
);
#[cfg(all(
  feature = "VK_EXT_swapchain_colorspace",
  not(feature = "VK_KHR_surface")
))]
compile_error!(
  "Feature `VK_EXT_swapchain_colorspace` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_colorspace.html"
);
#[cfg(all(feature = "VK_EXT_hdr_metadata", not(feature = "VK_KHR_swapchain")))]
compile_error!(
  "Feature `VK_EXT_hdr_metadata` requires `VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_hdr_metadata.html"
);
#[cfg(all(
  feature = "VK_KHR_imageless_framebuffer",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_maintenance2",
      feature = "VK_KHR_image_format_list"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_image_format_list"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_imageless_framebuffer` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_maintenance2 + VK_KHR_image_format_list , VK_VERSION_1_1 + VK_KHR_image_format_list , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_imageless_framebuffer.html"
);
#[cfg(all(
  feature = "VK_KHR_create_renderpass2",
  not(any(
    all(feature = "VK_KHR_multiview", feature = "VK_KHR_maintenance2"),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_create_renderpass2` requires `VK_KHR_multiview + VK_KHR_maintenance2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_create_renderpass2.html"
);
#[cfg(all(
  feature = "VK_IMG_relaxed_line_rasterization",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_IMG_relaxed_line_rasterization` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_relaxed_line_rasterization.html"
);
#[cfg(all(
  feature = "VK_KHR_shared_presentable_image",
  not(any(
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_VERSION_1_1"
    )
  ))
))]
compile_error!(
  "Feature `VK_KHR_shared_presentable_image` requires `VK_KHR_swapchain + VK_KHR_get_surface_capabilities2 + VK_KHR_get_physical_device_properties2 , VK_KHR_swapchain + VK_KHR_get_surface_capabilities2 + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shared_presentable_image.html"
);
#[cfg(all(
  feature = "VK_KHR_external_fence_capabilities",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_external_fence_capabilities` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_capabilities.html"
);
#[cfg(all(
  feature = "VK_KHR_external_fence",
  not(feature = "VK_KHR_external_fence_capabilities")
))]
compile_error!(
  "Feature `VK_KHR_external_fence` requires `VK_KHR_external_fence_capabilities`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence.html"
);
#[cfg(all(
  feature = "VK_KHR_external_fence_win32",
  not(feature = "VK_KHR_external_fence")
))]
compile_error!(
  "Feature `VK_KHR_external_fence_win32` requires `VK_KHR_external_fence`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_win32.html"
);
#[cfg(all(
  feature = "VK_KHR_external_fence_fd",
  not(any(feature = "VK_KHR_external_fence", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_KHR_external_fence_fd` requires `VK_KHR_external_fence , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_fd.html"
);
#[cfg(all(
  feature = "VK_KHR_performance_query",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_performance_query` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_performance_query.html"
);
#[cfg(all(
  feature = "VK_KHR_get_surface_capabilities2",
  not(feature = "VK_KHR_surface")
))]
compile_error!(
  "Feature `VK_KHR_get_surface_capabilities2` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_surface_capabilities2.html"
);
#[cfg(all(
  feature = "VK_KHR_variable_pointers",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_storage_buffer_storage_class"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_variable_pointers` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_storage_buffer_storage_class , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_variable_pointers.html"
);
#[cfg(all(
  feature = "VK_KHR_get_display_properties2",
  not(feature = "VK_KHR_display")
))]
compile_error!(
  "Feature `VK_KHR_get_display_properties2` requires `VK_KHR_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_display_properties2.html"
);
#[cfg(all(feature = "VK_MVK_ios_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_MVK_ios_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_ios_surface.html"
);
#[cfg(all(feature = "VK_MVK_macos_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_MVK_macos_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_macos_surface.html"
);
#[cfg(all(
  feature = "VK_EXT_external_memory_dma_buf",
  not(feature = "VK_KHR_external_memory_fd")
))]
compile_error!(
  "Feature `VK_EXT_external_memory_dma_buf` requires `VK_KHR_external_memory_fd`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_dma_buf.html"
);
#[cfg(all(
  feature = "VK_EXT_queue_family_foreign",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_EXT_queue_family_foreign` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_queue_family_foreign.html"
);
#[cfg(all(
  feature = "VK_KHR_dedicated_allocation",
  not(any(
    feature = "VK_KHR_get_memory_requirements2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_dedicated_allocation` requires `VK_KHR_get_memory_requirements2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dedicated_allocation.html"
);
#[cfg(all(
  feature = "VK_ANDROID_external_memory_android_hardware_buffer",
  not(any(
    all(
      feature = "VK_KHR_sampler_ycbcr_conversion",
      feature = "VK_KHR_external_memory",
      feature = "VK_KHR_dedicated_allocation",
      feature = "VK_EXT_queue_family_foreign"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_EXT_queue_family_foreign")
  ))
))]
compile_error!(
  "Feature `VK_ANDROID_external_memory_android_hardware_buffer` requires `VK_KHR_sampler_ycbcr_conversion + VK_KHR_external_memory + VK_KHR_dedicated_allocation + VK_EXT_queue_family_foreign , VK_VERSION_1_1 + VK_EXT_queue_family_foreign`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_memory_android_hardware_buffer.html"
);
#[cfg(all(
  feature = "VK_EXT_sampler_filter_minmax",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_sampler_filter_minmax` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sampler_filter_minmax.html"
);
#[cfg(all(
  feature = "VK_AMDX_shader_enqueue",
  not(any(
    all(
      feature = "VK_KHR_synchronization2",
      feature = "VK_KHR_spirv_1_4",
      feature = "VK_EXT_extended_dynamic_state",
      feature = "VK_KHR_maintenance5",
      feature = "VK_KHR_pipeline_library"
    ),
    all(
      feature = "VK_VERSION_1_3",
      feature = "VK_KHR_maintenance5",
      feature = "VK_KHR_pipeline_library"
    )
  ))
))]
compile_error!(
  "Feature `VK_AMDX_shader_enqueue` requires `VK_KHR_synchronization2 + VK_KHR_spirv_1_4 + VK_EXT_extended_dynamic_state + VK_KHR_maintenance5 + VK_KHR_pipeline_library , VK_VERSION_1_3 + VK_KHR_maintenance5 + VK_KHR_pipeline_library`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_shader_enqueue.html"
);
#[cfg(all(
  feature = "VK_EXT_descriptor_heap",
  not(any(
    all(
      feature = "VK_KHR_maintenance5",
      feature = "VK_KHR_buffer_device_address"
    ),
    all(feature = "VK_KHR_maintenance5", feature = "VK_VERSION_1_2")
  ))
))]
compile_error!(
  "Feature `VK_EXT_descriptor_heap` requires `VK_KHR_maintenance5 + VK_KHR_buffer_device_address , VK_KHR_maintenance5 + VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_heap.html"
);
#[cfg(all(
  feature = "VK_EXT_inline_uniform_block",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_maintenance1"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_inline_uniform_block` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_maintenance1 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_inline_uniform_block.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_bfloat16",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_bfloat16` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_bfloat16.html"
);
#[cfg(all(
  feature = "VK_EXT_sample_locations",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_sample_locations` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sample_locations.html"
);
#[cfg(all(
  feature = "VK_EXT_blend_operation_advanced",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_blend_operation_advanced` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_blend_operation_advanced.html"
);
#[cfg(all(
  feature = "VK_KHR_acceleration_structure",
  not(any(
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_descriptor_indexing",
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_KHR_deferred_host_operations"
    ),
    all(
      feature = "VK_VERSION_1_2",
      feature = "VK_KHR_deferred_host_operations"
    )
  ))
))]
compile_error!(
  "Feature `VK_KHR_acceleration_structure` requires `VK_VERSION_1_1 + VK_EXT_descriptor_indexing + VK_KHR_buffer_device_address + VK_KHR_deferred_host_operations , VK_VERSION_1_2 + VK_KHR_deferred_host_operations`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_acceleration_structure.html"
);
#[cfg(all(
  feature = "VK_KHR_ray_tracing_pipeline",
  not(any(
    all(
      feature = "VK_KHR_spirv_1_4",
      feature = "VK_KHR_acceleration_structure"
    ),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_acceleration_structure")
  ))
))]
compile_error!(
  "Feature `VK_KHR_ray_tracing_pipeline` requires `VK_KHR_spirv_1_4 + VK_KHR_acceleration_structure , VK_VERSION_1_2 + VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_pipeline.html"
);
#[cfg(all(
  feature = "VK_KHR_ray_query",
  not(any(
    all(
      feature = "VK_KHR_spirv_1_4",
      feature = "VK_KHR_acceleration_structure"
    ),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_acceleration_structure")
  ))
))]
compile_error!(
  "Feature `VK_KHR_ray_query` requires `VK_KHR_spirv_1_4 + VK_KHR_acceleration_structure , VK_VERSION_1_2 + VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_query.html"
);
#[cfg(all(feature = "VK_NV_shader_sm_builtins", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_NV_shader_sm_builtins` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_sm_builtins.html"
);
#[cfg(all(
  feature = "VK_KHR_sampler_ycbcr_conversion",
  not(any(
    all(
      feature = "VK_KHR_maintenance1",
      feature = "VK_KHR_bind_memory2",
      feature = "VK_KHR_get_memory_requirements2",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_sampler_ycbcr_conversion` requires `VK_KHR_maintenance1 + VK_KHR_bind_memory2 + VK_KHR_get_memory_requirements2 + VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_sampler_ycbcr_conversion.html"
);
#[cfg(all(
  feature = "VK_EXT_image_drm_format_modifier",
  not(any(
    all(
      feature = "VK_KHR_bind_memory2",
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_sampler_ycbcr_conversion",
      feature = "VK_KHR_image_format_list"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_image_format_list"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_drm_format_modifier` requires `VK_KHR_bind_memory2 + VK_KHR_get_physical_device_properties2 + VK_KHR_sampler_ycbcr_conversion + VK_KHR_image_format_list , VK_VERSION_1_1 + VK_KHR_image_format_list , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_drm_format_modifier.html"
);
#[cfg(all(
  feature = "VK_EXT_descriptor_indexing",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_maintenance3"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_descriptor_indexing` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_maintenance3 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_indexing.html"
);
#[cfg(all(
  feature = "VK_KHR_portability_subset",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_portability_subset` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_portability_subset.html"
);
#[cfg(all(
  feature = "VK_NV_shading_rate_image",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_shading_rate_image` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shading_rate_image.html"
);
#[cfg(all(
  feature = "VK_NV_ray_tracing",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_get_memory_requirements2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_ray_tracing` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_get_memory_requirements2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing.html"
);
#[cfg(all(
  feature = "VK_NV_representative_fragment_test",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_representative_fragment_test` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_representative_fragment_test.html"
);
#[cfg(all(
  feature = "VK_KHR_maintenance3",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_maintenance3` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance3.html"
);
#[cfg(all(
  feature = "VK_QCOM_cooperative_matrix_conversion",
  not(feature = "VK_KHR_cooperative_matrix")
))]
compile_error!(
  "Feature `VK_QCOM_cooperative_matrix_conversion` requires `VK_KHR_cooperative_matrix`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_cooperative_matrix_conversion.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_subgroup_extended_types",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_KHR_shader_subgroup_extended_types` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_extended_types.html"
);
#[cfg(all(
  feature = "VK_KHR_8bit_storage",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_storage_buffer_storage_class"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_8bit_storage` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_storage_buffer_storage_class , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_8bit_storage.html"
);
#[cfg(all(
  feature = "VK_EXT_external_memory_host",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_EXT_external_memory_host` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_host.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_atomic_int64",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_atomic_int64` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_atomic_int64.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_clock",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_clock` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_clock.html"
);
#[cfg(all(
  feature = "VK_EXT_calibrated_timestamps",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_calibrated_timestamps` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_calibrated_timestamps.html"
);
#[cfg(all(
  feature = "VK_AMD_shader_core_properties",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_AMD_shader_core_properties` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties.html"
);
#[cfg(all(
  feature = "VK_KHR_video_decode_h265",
  not(feature = "VK_KHR_video_decode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_decode_h265` requires `VK_KHR_video_decode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h265.html"
);
#[cfg(all(
  feature = "VK_KHR_global_priority",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_global_priority` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_global_priority.html"
);
#[cfg(all(
  feature = "VK_EXT_vertex_attribute_divisor",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_vertex_attribute_divisor` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_divisor.html"
);
#[cfg(all(
  feature = "VK_GGP_frame_token",
  not(all(
    feature = "VK_KHR_swapchain",
    feature = "VK_GGP_stream_descriptor_surface"
  ))
))]
compile_error!(
  "Feature `VK_GGP_frame_token` requires `VK_KHR_swapchain + VK_GGP_stream_descriptor_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_frame_token.html"
);
#[cfg(all(
  feature = "VK_KHR_driver_properties",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_driver_properties` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_driver_properties.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_float_controls",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_float_controls` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls.html"
);
#[cfg(all(
  feature = "VK_NV_shader_subgroup_partitioned",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_NV_shader_subgroup_partitioned` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_subgroup_partitioned.html"
);
#[cfg(all(
  feature = "VK_KHR_depth_stencil_resolve",
  not(any(feature = "VK_KHR_create_renderpass2", feature = "VK_VERSION_1_2"))
))]
compile_error!(
  "Feature `VK_KHR_depth_stencil_resolve` requires `VK_KHR_create_renderpass2 , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_stencil_resolve.html"
);
#[cfg(all(
  feature = "VK_KHR_swapchain_mutable_format",
  not(any(
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_maintenance2",
      feature = "VK_KHR_image_format_list"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_maintenance2",
      feature = "VK_VERSION_1_2"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_image_format_list"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_VERSION_1_1",
      feature = "VK_VERSION_1_2"
    )
  ))
))]
compile_error!(
  "Feature `VK_KHR_swapchain_mutable_format` requires `VK_KHR_swapchain + VK_KHR_maintenance2 + VK_KHR_image_format_list , VK_KHR_swapchain + VK_KHR_maintenance2 + VK_VERSION_1_2 , VK_KHR_swapchain + VK_VERSION_1_1 + VK_KHR_image_format_list , VK_KHR_swapchain + VK_VERSION_1_1 + VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_mutable_format.html"
);
#[cfg(all(
  feature = "VK_NV_compute_shader_derivatives",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_compute_shader_derivatives` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_shader_derivatives.html"
);
#[cfg(all(
  feature = "VK_NV_mesh_shader",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_mesh_shader` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_mesh_shader.html"
);
#[cfg(all(
  feature = "VK_NV_fragment_shader_barycentric",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_fragment_shader_barycentric` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shader_barycentric.html"
);
#[cfg(all(
  feature = "VK_NV_shader_image_footprint",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_shader_image_footprint` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_image_footprint.html"
);
#[cfg(all(
  feature = "VK_NV_scissor_exclusive",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_scissor_exclusive` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_scissor_exclusive.html"
);
#[cfg(all(
  feature = "VK_NV_device_diagnostic_checkpoints",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_device_diagnostic_checkpoints` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostic_checkpoints.html"
);
#[cfg(all(
  feature = "VK_KHR_timeline_semaphore",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_timeline_semaphore` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_timeline_semaphore.html"
);
#[cfg(all(
  feature = "VK_EXT_present_timing",
  not(all(
    feature = "VK_KHR_swapchain",
    feature = "VK_KHR_present_id2",
    feature = "VK_KHR_get_surface_capabilities2",
    feature = "VK_KHR_calibrated_timestamps"
  ))
))]
compile_error!(
  "Feature `VK_EXT_present_timing` requires `VK_KHR_swapchain + VK_KHR_present_id2 + VK_KHR_get_surface_capabilities2 + VK_KHR_calibrated_timestamps`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_timing.html"
);
#[cfg(all(
  feature = "VK_INTEL_shader_integer_functions2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_INTEL_shader_integer_functions2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_shader_integer_functions2.html"
);
#[cfg(all(
  feature = "VK_KHR_vulkan_memory_model",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_vulkan_memory_model` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vulkan_memory_model.html"
);
#[cfg(all(
  feature = "VK_EXT_pci_bus_info",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pci_bus_info` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pci_bus_info.html"
);
#[cfg(all(
  feature = "VK_AMD_display_native_hdr",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    )
  ))
))]
compile_error!(
  "Feature `VK_AMD_display_native_hdr` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain , VK_VERSION_1_1 + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_display_native_hdr.html"
);
#[cfg(all(
  feature = "VK_FUCHSIA_imagepipe_surface",
  not(feature = "VK_KHR_surface")
))]
compile_error!(
  "Feature `VK_FUCHSIA_imagepipe_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_imagepipe_surface.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_terminate_invocation",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_terminate_invocation` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_terminate_invocation.html"
);
#[cfg(all(feature = "VK_EXT_metal_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_EXT_metal_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_metal_surface.html"
);
#[cfg(all(
  feature = "VK_EXT_fragment_density_map",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_fragment_density_map` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map.html"
);
#[cfg(all(
  feature = "VK_EXT_scalar_block_layout",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_scalar_block_layout` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_scalar_block_layout.html"
);
#[cfg(all(
  feature = "VK_EXT_subgroup_size_control",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_EXT_subgroup_size_control` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subgroup_size_control.html"
);
#[cfg(all(
  feature = "VK_KHR_fragment_shading_rate",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_create_renderpass2"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_create_renderpass2"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_fragment_shading_rate` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_create_renderpass2 , VK_VERSION_1_1 + VK_KHR_create_renderpass2 , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shading_rate.html"
);
#[cfg(all(
  feature = "VK_AMD_shader_core_properties2",
  not(feature = "VK_AMD_shader_core_properties")
))]
compile_error!(
  "Feature `VK_AMD_shader_core_properties2` requires `VK_AMD_shader_core_properties`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties2.html"
);
#[cfg(all(
  feature = "VK_AMD_device_coherent_memory",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_AMD_device_coherent_memory` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_device_coherent_memory.html"
);
#[cfg(all(
  feature = "VK_KHR_dynamic_rendering_local_read",
  not(any(feature = "VK_KHR_dynamic_rendering", feature = "VK_VERSION_1_3"))
))]
compile_error!(
  "Feature `VK_KHR_dynamic_rendering_local_read` requires `VK_KHR_dynamic_rendering , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering_local_read.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_abort",
  not(all(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_KHR_device_fault",
    feature = "VK_KHR_shader_constant_data"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_abort` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_device_fault + VK_KHR_shader_constant_data`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_abort.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_image_atomic_int64",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_image_atomic_int64` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_image_atomic_int64.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_quad_control",
  not(any(
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_vulkan_memory_model",
      feature = "VK_KHR_shader_maximal_reconvergence"
    ),
    all(
      feature = "VK_VERSION_1_2",
      feature = "VK_KHR_shader_maximal_reconvergence"
    )
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_quad_control` requires `VK_VERSION_1_1 + VK_KHR_vulkan_memory_model + VK_KHR_shader_maximal_reconvergence , VK_VERSION_1_2 + VK_KHR_shader_maximal_reconvergence`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_quad_control.html"
);
#[cfg(all(
  feature = "VK_KHR_spirv_1_4",
  not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_shader_float_controls"))
))]
compile_error!(
  "Feature `VK_KHR_spirv_1_4` requires `VK_VERSION_1_1 + VK_KHR_shader_float_controls`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_spirv_1_4.html"
);
#[cfg(all(
  feature = "VK_EXT_memory_budget",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_memory_budget` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_budget.html"
);
#[cfg(all(
  feature = "VK_EXT_memory_priority",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_memory_priority` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_priority.html"
);
#[cfg(all(
  feature = "VK_KHR_surface_protected_capabilities",
  not(all(
    feature = "VK_VERSION_1_1",
    feature = "VK_KHR_get_surface_capabilities2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_surface_protected_capabilities` requires `VK_VERSION_1_1 + VK_KHR_get_surface_capabilities2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_protected_capabilities.html"
);
#[cfg(all(
  feature = "VK_NV_dedicated_allocation_image_aliasing",
  not(any(
    all(
      feature = "VK_KHR_dedicated_allocation",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_dedicated_allocation_image_aliasing` requires `VK_KHR_dedicated_allocation + VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_dedicated_allocation_image_aliasing.html"
);
#[cfg(all(
  feature = "VK_KHR_separate_depth_stencil_layouts",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_create_renderpass2"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_create_renderpass2"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_separate_depth_stencil_layouts` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_create_renderpass2 , VK_VERSION_1_1 + VK_KHR_create_renderpass2 , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_separate_depth_stencil_layouts.html"
);
#[cfg(all(
  feature = "VK_EXT_buffer_device_address",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_buffer_device_address` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_buffer_device_address.html"
);
#[cfg(all(
  feature = "VK_KHR_present_wait",
  not(all(feature = "VK_KHR_swapchain", feature = "VK_KHR_present_id"))
))]
compile_error!(
  "Feature `VK_KHR_present_wait` requires `VK_KHR_swapchain + VK_KHR_present_id`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait.html"
);
#[cfg(all(
  feature = "VK_NV_cooperative_matrix",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_cooperative_matrix` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix.html"
);
#[cfg(all(
  feature = "VK_NV_coverage_reduction_mode",
  not(any(
    all(
      feature = "VK_NV_framebuffer_mixed_samples",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(
      feature = "VK_NV_framebuffer_mixed_samples",
      feature = "VK_VERSION_1_1"
    )
  ))
))]
compile_error!(
  "Feature `VK_NV_coverage_reduction_mode` requires `VK_NV_framebuffer_mixed_samples + VK_KHR_get_physical_device_properties2 , VK_NV_framebuffer_mixed_samples + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_coverage_reduction_mode.html"
);
#[cfg(all(
  feature = "VK_EXT_fragment_shader_interlock",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_fragment_shader_interlock` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_shader_interlock.html"
);
#[cfg(all(
  feature = "VK_EXT_ycbcr_image_arrays",
  not(any(
    feature = "VK_KHR_sampler_ycbcr_conversion",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_ycbcr_image_arrays` requires `VK_KHR_sampler_ycbcr_conversion , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_image_arrays.html"
);
#[cfg(all(
  feature = "VK_KHR_uniform_buffer_standard_layout",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_uniform_buffer_standard_layout` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_uniform_buffer_standard_layout.html"
);
#[cfg(all(
  feature = "VK_EXT_provoking_vertex",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_provoking_vertex` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_provoking_vertex.html"
);
#[cfg(all(
  feature = "VK_EXT_full_screen_exclusive",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_surface",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_surface",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    )
  ))
))]
compile_error!(
  "Feature `VK_EXT_full_screen_exclusive` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_surface + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain , VK_VERSION_1_1 + VK_KHR_surface + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_full_screen_exclusive.html"
);
#[cfg(all(feature = "VK_EXT_headless_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_EXT_headless_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_headless_surface.html"
);
#[cfg(all(
  feature = "VK_KHR_buffer_device_address",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_device_group"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_buffer_device_address` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_device_group , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_buffer_device_address.html"
);
#[cfg(all(
  feature = "VK_EXT_line_rasterization",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_line_rasterization` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_line_rasterization.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_atomic_float",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_atomic_float` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float.html"
);
#[cfg(all(
  feature = "VK_EXT_host_query_reset",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_host_query_reset` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_query_reset.html"
);
#[cfg(all(
  feature = "VK_EXT_index_type_uint8",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_index_type_uint8` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_index_type_uint8.html"
);
#[cfg(all(
  feature = "VK_EXT_extended_dynamic_state",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_extended_dynamic_state` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state.html"
);
#[cfg(all(
  feature = "VK_KHR_pipeline_executable_properties",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_pipeline_executable_properties` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_executable_properties.html"
);
#[cfg(all(
  feature = "VK_EXT_host_image_copy",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_copy_commands2",
      feature = "VK_KHR_format_feature_flags2"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_copy_commands2",
      feature = "VK_KHR_format_feature_flags2"
    ),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_host_image_copy` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_copy_commands2 + VK_KHR_format_feature_flags2 , VK_VERSION_1_1 + VK_KHR_copy_commands2 + VK_KHR_format_feature_flags2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_image_copy.html"
);
#[cfg(all(
  feature = "VK_EXT_map_memory_placed",
  not(any(feature = "VK_KHR_map_memory2", feature = "VK_VERSION_1_4"))
))]
compile_error!(
  "Feature `VK_EXT_map_memory_placed` requires `VK_KHR_map_memory2 , VK_VERSION_1_4`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_map_memory_placed.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_atomic_float2",
  not(feature = "VK_EXT_shader_atomic_float")
))]
compile_error!(
  "Feature `VK_EXT_shader_atomic_float2` requires `VK_EXT_shader_atomic_float`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float2.html"
);
#[cfg(all(
  feature = "VK_EXT_surface_maintenance1",
  not(all(
    feature = "VK_KHR_surface",
    feature = "VK_KHR_get_surface_capabilities2"
  ))
))]
compile_error!(
  "Feature `VK_EXT_surface_maintenance1` requires `VK_KHR_surface + VK_KHR_get_surface_capabilities2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_surface_maintenance1.html"
);
#[cfg(all(
  feature = "VK_EXT_swapchain_maintenance1",
  not(any(
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_EXT_surface_maintenance1",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_EXT_surface_maintenance1",
      feature = "VK_VERSION_1_1"
    )
  ))
))]
compile_error!(
  "Feature `VK_EXT_swapchain_maintenance1` requires `VK_KHR_swapchain + VK_EXT_surface_maintenance1 + VK_KHR_get_physical_device_properties2 , VK_KHR_swapchain + VK_EXT_surface_maintenance1 + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_maintenance1.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_demote_to_helper_invocation",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_demote_to_helper_invocation` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_demote_to_helper_invocation.html"
);
#[cfg(all(
  feature = "VK_NV_device_generated_commands",
  not(any(
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_buffer_device_address"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_NV_device_generated_commands` requires `VK_VERSION_1_1 + VK_KHR_buffer_device_address , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands.html"
);
#[cfg(all(
  feature = "VK_NV_inherited_viewport_scissor",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_inherited_viewport_scissor` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_inherited_viewport_scissor.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_integer_dot_product",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_integer_dot_product` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_integer_dot_product.html"
);
#[cfg(all(
  feature = "VK_EXT_texel_buffer_alignment",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_texel_buffer_alignment` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texel_buffer_alignment.html"
);
#[cfg(all(
  feature = "VK_EXT_depth_bias_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_depth_bias_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_bias_control.html"
);
#[cfg(all(
  feature = "VK_EXT_device_memory_report",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_device_memory_report` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_memory_report.html"
);
#[cfg(all(
  feature = "VK_EXT_acquire_drm_display",
  not(feature = "VK_EXT_direct_mode_display")
))]
compile_error!(
  "Feature `VK_EXT_acquire_drm_display` requires `VK_EXT_direct_mode_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_drm_display.html"
);
#[cfg(all(
  feature = "VK_EXT_robustness2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_robustness2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_robustness2.html"
);
#[cfg(all(
  feature = "VK_EXT_custom_border_color",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_custom_border_color` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_border_color.html"
);
#[cfg(all(
  feature = "VK_EXT_texture_compression_astc_3d",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_texture_compression_astc_3d` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_3d.html"
);
#[cfg(all(
  feature = "VK_NV_present_barrier",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_surface",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_surface",
      feature = "VK_KHR_get_surface_capabilities2",
      feature = "VK_KHR_swapchain"
    )
  ))
))]
compile_error!(
  "Feature `VK_NV_present_barrier` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_surface + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain , VK_VERSION_1_1 + VK_KHR_surface + VK_KHR_get_surface_capabilities2 + VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_barrier.html"
);
#[cfg(all(
  feature = "VK_KHR_present_id",
  not(any(
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(feature = "VK_KHR_swapchain", feature = "VK_VERSION_1_1")
  ))
))]
compile_error!(
  "Feature `VK_KHR_present_id` requires `VK_KHR_swapchain + VK_KHR_get_physical_device_properties2 , VK_KHR_swapchain + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id.html"
);
#[cfg(all(
  feature = "VK_EXT_private_data",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_private_data` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_private_data.html"
);
#[cfg(all(
  feature = "VK_EXT_pipeline_creation_cache_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pipeline_creation_cache_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_creation_cache_control.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_queue",
  not(any(
    all(feature = "VK_KHR_video_queue", feature = "VK_KHR_synchronization2"),
    all(feature = "VK_KHR_video_queue", feature = "VK_VERSION_1_3")
  ))
))]
compile_error!(
  "Feature `VK_KHR_video_encode_queue` requires `VK_KHR_video_queue + VK_KHR_synchronization2 , VK_KHR_video_queue + VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_queue.html"
);
#[cfg(all(
  feature = "VK_NV_device_diagnostics_config",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_device_diagnostics_config` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostics_config.html"
);
#[cfg(all(
  feature = "VK_QCOM_queue_perf_hint",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_queue_perf_hint` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_queue_perf_hint.html"
);
#[cfg(all(
  feature = "VK_NV_cuda_kernel_launch",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_cuda_kernel_launch` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cuda_kernel_launch.html"
);
#[cfg(all(
  feature = "VK_QCOM_tile_shading",
  not(feature = "VK_QCOM_tile_properties")
))]
compile_error!(
  "Feature `VK_QCOM_tile_shading` requires `VK_QCOM_tile_properties`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_shading.html"
);
#[cfg(all(
  feature = "VK_KHR_synchronization2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_synchronization2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_synchronization2.html"
);
#[cfg(all(
  feature = "VK_EXT_descriptor_buffer",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_EXT_descriptor_indexing",
      feature = "VK_KHR_synchronization2"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_EXT_descriptor_indexing",
      feature = "VK_KHR_synchronization2"
    ),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_synchronization2"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_descriptor_buffer` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address + VK_EXT_descriptor_indexing + VK_KHR_synchronization2 , VK_VERSION_1_1 + VK_KHR_buffer_device_address + VK_EXT_descriptor_indexing + VK_KHR_synchronization2 , VK_VERSION_1_2 + VK_KHR_synchronization2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_buffer.html"
);
#[cfg(all(
  feature = "VK_KHR_device_address_commands",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_KHR_synchronization2",
      feature = "VK_EXT_extended_dynamic_state"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_KHR_synchronization2",
      feature = "VK_EXT_extended_dynamic_state"
    ),
    all(
      feature = "VK_VERSION_1_2",
      feature = "VK_KHR_synchronization2",
      feature = "VK_EXT_extended_dynamic_state"
    ),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_KHR_device_address_commands` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address + VK_KHR_synchronization2 + VK_EXT_extended_dynamic_state , VK_VERSION_1_1 + VK_KHR_buffer_device_address + VK_KHR_synchronization2 + VK_EXT_extended_dynamic_state , VK_VERSION_1_2 + VK_KHR_synchronization2 + VK_EXT_extended_dynamic_state , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_address_commands.html"
);
#[cfg(all(
  feature = "VK_EXT_graphics_pipeline_library",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_pipeline_library"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_pipeline_library")
  ))
))]
compile_error!(
  "Feature `VK_EXT_graphics_pipeline_library` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_pipeline_library , VK_VERSION_1_1 + VK_KHR_pipeline_library`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_graphics_pipeline_library.html"
);
#[cfg(all(
  feature = "VK_AMD_shader_early_and_late_fragment_tests",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_AMD_shader_early_and_late_fragment_tests` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_early_and_late_fragment_tests.html"
);
#[cfg(all(
  feature = "VK_KHR_fragment_shader_barycentric",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_fragment_shader_barycentric` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shader_barycentric.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_subgroup_uniform_control_flow",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_KHR_shader_subgroup_uniform_control_flow` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_uniform_control_flow.html"
);
#[cfg(all(
  feature = "VK_KHR_zero_initialize_workgroup_memory",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_zero_initialize_workgroup_memory` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_zero_initialize_workgroup_memory.html"
);
#[cfg(all(
  feature = "VK_NV_fragment_shading_rate_enums",
  not(feature = "VK_KHR_fragment_shading_rate")
))]
compile_error!(
  "Feature `VK_NV_fragment_shading_rate_enums` requires `VK_KHR_fragment_shading_rate`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shading_rate_enums.html"
);
#[cfg(all(
  feature = "VK_NV_ray_tracing_motion_blur",
  not(feature = "VK_KHR_ray_tracing_pipeline")
))]
compile_error!(
  "Feature `VK_NV_ray_tracing_motion_blur` requires `VK_KHR_ray_tracing_pipeline`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_motion_blur.html"
);
#[cfg(all(
  feature = "VK_EXT_mesh_shader",
  not(any(feature = "VK_KHR_spirv_1_4", feature = "VK_VERSION_1_2"))
))]
compile_error!(
  "Feature `VK_EXT_mesh_shader` requires `VK_KHR_spirv_1_4 , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mesh_shader.html"
);
#[cfg(all(
  feature = "VK_EXT_ycbcr_2plane_444_formats",
  not(any(
    feature = "VK_KHR_sampler_ycbcr_conversion",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_ycbcr_2plane_444_formats` requires `VK_KHR_sampler_ycbcr_conversion , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_2plane_444_formats.html"
);
#[cfg(all(
  feature = "VK_EXT_fragment_density_map2",
  not(feature = "VK_EXT_fragment_density_map")
))]
compile_error!(
  "Feature `VK_EXT_fragment_density_map2` requires `VK_EXT_fragment_density_map`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map2.html"
);
#[cfg(all(
  feature = "VK_QCOM_rotated_copy_commands",
  not(any(feature = "VK_KHR_copy_commands2", feature = "VK_VERSION_1_3"))
))]
compile_error!(
  "Feature `VK_QCOM_rotated_copy_commands` requires `VK_KHR_copy_commands2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_rotated_copy_commands.html"
);
#[cfg(all(
  feature = "VK_EXT_image_robustness",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_robustness` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_robustness.html"
);
#[cfg(all(
  feature = "VK_KHR_workgroup_memory_explicit_layout",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_workgroup_memory_explicit_layout` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_workgroup_memory_explicit_layout.html"
);
#[cfg(all(
  feature = "VK_KHR_copy_commands2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_copy_commands2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_commands2.html"
);
#[cfg(all(
  feature = "VK_EXT_image_compression_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_compression_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control.html"
);
#[cfg(all(
  feature = "VK_EXT_attachment_feedback_loop_layout",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_attachment_feedback_loop_layout` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_layout.html"
);
#[cfg(all(
  feature = "VK_EXT_4444_formats",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_4444_formats` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_4444_formats.html"
);
#[cfg(all(
  feature = "VK_EXT_device_fault",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_device_fault` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_fault.html"
);
#[cfg(all(
  feature = "VK_ARM_rasterization_order_attachment_access",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_ARM_rasterization_order_attachment_access` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_rasterization_order_attachment_access.html"
);
#[cfg(all(
  feature = "VK_EXT_rgba10x6_formats",
  not(any(
    feature = "VK_KHR_sampler_ycbcr_conversion",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_rgba10x6_formats` requires `VK_KHR_sampler_ycbcr_conversion , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rgba10x6_formats.html"
);
#[cfg(all(
  feature = "VK_NV_acquire_winrt_display",
  not(feature = "VK_EXT_direct_mode_display")
))]
compile_error!(
  "Feature `VK_NV_acquire_winrt_display` requires `VK_EXT_direct_mode_display`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_acquire_winrt_display.html"
);
#[cfg(all(feature = "VK_EXT_directfb_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_EXT_directfb_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_directfb_surface.html"
);
#[cfg(all(
  feature = "VK_VALVE_mutable_descriptor_type",
  not(feature = "VK_KHR_maintenance3")
))]
compile_error!(
  "Feature `VK_VALVE_mutable_descriptor_type` requires `VK_KHR_maintenance3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_mutable_descriptor_type.html"
);
#[cfg(all(
  feature = "VK_EXT_vertex_input_dynamic_state",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_vertex_input_dynamic_state` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_input_dynamic_state.html"
);
#[cfg(all(
  feature = "VK_EXT_physical_device_drm",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_physical_device_drm` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_physical_device_drm.html"
);
#[cfg(all(
  feature = "VK_EXT_device_address_binding_report",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_debug_utils"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_EXT_debug_utils")
  ))
))]
compile_error!(
  "Feature `VK_EXT_device_address_binding_report` requires `VK_KHR_get_physical_device_properties2 + VK_EXT_debug_utils , VK_VERSION_1_1 + VK_EXT_debug_utils`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_address_binding_report.html"
);
#[cfg(all(
  feature = "VK_EXT_depth_clip_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_depth_clip_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_control.html"
);
#[cfg(all(
  feature = "VK_EXT_primitive_topology_list_restart",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_primitive_topology_list_restart` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_topology_list_restart.html"
);
#[cfg(all(
  feature = "VK_KHR_format_feature_flags2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_format_feature_flags2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_format_feature_flags2.html"
);
#[cfg(all(
  feature = "VK_EXT_present_mode_fifo_latest_ready",
  not(feature = "VK_KHR_swapchain")
))]
compile_error!(
  "Feature `VK_EXT_present_mode_fifo_latest_ready` requires `VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_mode_fifo_latest_ready.html"
);
#[cfg(all(
  feature = "VK_FUCHSIA_external_memory",
  not(any(
    all(
      feature = "VK_KHR_external_memory_capabilities",
      feature = "VK_KHR_external_memory"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_FUCHSIA_external_memory` requires `VK_KHR_external_memory_capabilities + VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_memory.html"
);
#[cfg(all(
  feature = "VK_FUCHSIA_external_semaphore",
  not(all(
    feature = "VK_KHR_external_semaphore_capabilities",
    feature = "VK_KHR_external_semaphore"
  ))
))]
compile_error!(
  "Feature `VK_FUCHSIA_external_semaphore` requires `VK_KHR_external_semaphore_capabilities + VK_KHR_external_semaphore`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_semaphore.html"
);
#[cfg(all(
  feature = "VK_FUCHSIA_buffer_collection",
  not(any(
    all(
      feature = "VK_FUCHSIA_external_memory",
      feature = "VK_KHR_sampler_ycbcr_conversion"
    ),
    all(feature = "VK_FUCHSIA_external_memory", feature = "VK_VERSION_1_1")
  ))
))]
compile_error!(
  "Feature `VK_FUCHSIA_buffer_collection` requires `VK_FUCHSIA_external_memory + VK_KHR_sampler_ycbcr_conversion , VK_FUCHSIA_external_memory + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_buffer_collection.html"
);
#[cfg(all(
  feature = "VK_HUAWEI_subpass_shading",
  not(any(
    all(
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_KHR_synchronization2"
    ),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_synchronization2"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_HUAWEI_subpass_shading` requires `VK_KHR_create_renderpass2 + VK_KHR_synchronization2 , VK_VERSION_1_2 + VK_KHR_synchronization2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_subpass_shading.html"
);
#[cfg(all(
  feature = "VK_HUAWEI_invocation_mask",
  not(any(
    all(
      feature = "VK_KHR_ray_tracing_pipeline",
      feature = "VK_KHR_synchronization2"
    ),
    all(feature = "VK_KHR_ray_tracing_pipeline", feature = "VK_VERSION_1_3")
  ))
))]
compile_error!(
  "Feature `VK_HUAWEI_invocation_mask` requires `VK_KHR_ray_tracing_pipeline + VK_KHR_synchronization2 , VK_KHR_ray_tracing_pipeline + VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_invocation_mask.html"
);
#[cfg(all(
  feature = "VK_NV_external_memory_rdma",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_NV_external_memory_rdma` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_rdma.html"
);
#[cfg(all(
  feature = "VK_EXT_pipeline_properties",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pipeline_properties` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_properties.html"
);
#[cfg(all(feature = "VK_NV_external_sci_sync", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_NV_external_sci_sync` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_sci_sync.html"
);
#[cfg(all(
  feature = "VK_NV_external_memory_sci_buf",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_NV_external_memory_sci_buf` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_sci_buf.html"
);
#[cfg(all(
  feature = "VK_EXT_frame_boundary",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_frame_boundary` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_frame_boundary.html"
);
#[cfg(all(
  feature = "VK_EXT_multisampled_render_to_single_sampled",
  not(any(
    all(
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_KHR_depth_stencil_resolve"
    ),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_EXT_multisampled_render_to_single_sampled` requires `VK_KHR_create_renderpass2 + VK_KHR_depth_stencil_resolve , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multisampled_render_to_single_sampled.html"
);
#[cfg(all(
  feature = "VK_EXT_extended_dynamic_state2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_extended_dynamic_state2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state2.html"
);
#[cfg(all(feature = "VK_QNX_screen_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_QNX_screen_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_screen_surface.html"
);
#[cfg(all(
  feature = "VK_EXT_color_write_enable",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_color_write_enable` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_color_write_enable.html"
);
#[cfg(all(
  feature = "VK_EXT_primitives_generated_query",
  not(feature = "VK_EXT_transform_feedback")
))]
compile_error!(
  "Feature `VK_EXT_primitives_generated_query` requires `VK_EXT_transform_feedback`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitives_generated_query.html"
);
#[cfg(all(
  feature = "VK_KHR_ray_tracing_maintenance1",
  not(feature = "VK_KHR_acceleration_structure")
))]
compile_error!(
  "Feature `VK_KHR_ray_tracing_maintenance1` requires `VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_maintenance1.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_untyped_pointers",
  not(feature = "VK_KHR_get_physical_device_properties2")
))]
compile_error!(
  "Feature `VK_KHR_shader_untyped_pointers` requires `VK_KHR_get_physical_device_properties2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_untyped_pointers.html"
);
#[cfg(all(
  feature = "VK_EXT_global_priority_query",
  not(any(
    all(
      feature = "VK_EXT_global_priority",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(feature = "VK_EXT_global_priority", feature = "VK_VERSION_1_1")
  ))
))]
compile_error!(
  "Feature `VK_EXT_global_priority_query` requires `VK_EXT_global_priority + VK_KHR_get_physical_device_properties2 , VK_EXT_global_priority + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_global_priority_query.html"
);
#[cfg(all(
  feature = "VK_VALVE_video_encode_rgb_conversion",
  not(any(
    all(
      feature = "VK_KHR_video_encode_queue",
      feature = "VK_KHR_sampler_ycbcr_conversion"
    ),
    all(feature = "VK_KHR_video_encode_queue", feature = "VK_VERSION_1_1")
  ))
))]
compile_error!(
  "Feature `VK_VALVE_video_encode_rgb_conversion` requires `VK_KHR_video_encode_queue + VK_KHR_sampler_ycbcr_conversion , VK_KHR_video_encode_queue + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_video_encode_rgb_conversion.html"
);
#[cfg(all(
  feature = "VK_EXT_image_view_min_lod",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_view_min_lod` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_view_min_lod.html"
);
#[cfg(all(
  feature = "VK_EXT_multi_draw",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_multi_draw` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multi_draw.html"
);
#[cfg(all(
  feature = "VK_EXT_image_2d_view_of_3d",
  not(any(
    all(
      feature = "VK_KHR_maintenance1",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_2d_view_of_3d` requires `VK_KHR_maintenance1 + VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_2d_view_of_3d.html"
);
#[cfg(all(feature = "VK_EXT_shader_tile_image", not(feature = "VK_VERSION_1_3")))]
compile_error!(
  "Feature `VK_EXT_shader_tile_image` requires `VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_tile_image.html"
);
#[cfg(all(
  feature = "VK_EXT_opacity_micromap",
  not(any(
    all(
      feature = "VK_KHR_acceleration_structure",
      feature = "VK_KHR_synchronization2"
    ),
    all(feature = "VK_KHR_acceleration_structure", feature = "VK_VERSION_1_3")
  ))
))]
compile_error!(
  "Feature `VK_EXT_opacity_micromap` requires `VK_KHR_acceleration_structure + VK_KHR_synchronization2 , VK_KHR_acceleration_structure + VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_opacity_micromap.html"
);
#[cfg(all(
  feature = "VK_NV_displacement_micromap",
  not(feature = "VK_EXT_opacity_micromap")
))]
compile_error!(
  "Feature `VK_NV_displacement_micromap` requires `VK_EXT_opacity_micromap`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_displacement_micromap.html"
);
#[cfg(all(
  feature = "VK_HUAWEI_cluster_culling_shader",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_HUAWEI_cluster_culling_shader` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_cluster_culling_shader.html"
);
#[cfg(all(
  feature = "VK_EXT_border_color_swizzle",
  not(feature = "VK_EXT_custom_border_color")
))]
compile_error!(
  "Feature `VK_EXT_border_color_swizzle` requires `VK_EXT_custom_border_color`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_border_color_swizzle.html"
);
#[cfg(all(
  feature = "VK_EXT_pageable_device_local_memory",
  not(feature = "VK_EXT_memory_priority")
))]
compile_error!(
  "Feature `VK_EXT_pageable_device_local_memory` requires `VK_EXT_memory_priority`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pageable_device_local_memory.html"
);
#[cfg(all(feature = "VK_KHR_maintenance4", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_KHR_maintenance4` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance4.html"
);
#[cfg(all(
  feature = "VK_ARM_shader_core_properties",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_ARM_shader_core_properties` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_properties.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_subgroup_rotate",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_subgroup_rotate` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_rotate.html"
);
#[cfg(all(
  feature = "VK_ARM_scheduling_controls",
  not(feature = "VK_ARM_shader_core_builtins")
))]
compile_error!(
  "Feature `VK_ARM_scheduling_controls` requires `VK_ARM_shader_core_builtins`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_scheduling_controls.html"
);
#[cfg(all(
  feature = "VK_EXT_image_sliced_view_of_3d",
  not(any(
    all(
      feature = "VK_KHR_maintenance1",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_image_sliced_view_of_3d` requires `VK_KHR_maintenance1 + VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_sliced_view_of_3d.html"
);
#[cfg(all(
  feature = "VK_VALVE_descriptor_set_host_mapping",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_VALVE_descriptor_set_host_mapping` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_descriptor_set_host_mapping.html"
);
#[cfg(all(
  feature = "VK_EXT_depth_clamp_zero_one",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_depth_clamp_zero_one` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_zero_one.html"
);
#[cfg(all(
  feature = "VK_EXT_non_seamless_cube_map",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_non_seamless_cube_map` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_non_seamless_cube_map.html"
);
#[cfg(all(
  feature = "VK_ARM_render_pass_striped",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_synchronization2"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_synchronization2"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_ARM_render_pass_striped` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_synchronization2 , VK_VERSION_1_1 + VK_KHR_synchronization2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_render_pass_striped.html"
);
#[cfg(all(
  feature = "VK_QCOM_fragment_density_map_offset",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_fragment_density_map"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_EXT_fragment_density_map")
  ))
))]
compile_error!(
  "Feature `VK_QCOM_fragment_density_map_offset` requires `VK_KHR_get_physical_device_properties2 + VK_EXT_fragment_density_map , VK_VERSION_1_1 + VK_EXT_fragment_density_map`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_fragment_density_map_offset.html"
);
#[cfg(all(
  feature = "VK_NV_copy_memory_indirect",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_buffer_device_address"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_buffer_device_address"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_NV_copy_memory_indirect` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address , VK_VERSION_1_1 + VK_KHR_buffer_device_address , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_copy_memory_indirect.html"
);
#[cfg(all(
  feature = "VK_NV_memory_decompression",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_buffer_device_address"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_buffer_device_address"),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_NV_memory_decompression` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address , VK_VERSION_1_1 + VK_KHR_buffer_device_address , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_memory_decompression.html"
);
#[cfg(all(
  feature = "VK_NV_device_generated_commands_compute",
  not(feature = "VK_NV_device_generated_commands")
))]
compile_error!(
  "Feature `VK_NV_device_generated_commands_compute` requires `VK_NV_device_generated_commands`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands_compute.html"
);
#[cfg(all(
  feature = "VK_NV_ray_tracing_linear_swept_spheres",
  not(feature = "VK_KHR_ray_tracing_pipeline")
))]
compile_error!(
  "Feature `VK_NV_ray_tracing_linear_swept_spheres` requires `VK_KHR_ray_tracing_pipeline`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_linear_swept_spheres.html"
);
#[cfg(all(
  feature = "VK_NV_linear_color_attachment",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_linear_color_attachment` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_linear_color_attachment.html"
);
#[cfg(all(
  feature = "VK_GOOGLE_surfaceless_query",
  not(feature = "VK_KHR_surface")
))]
compile_error!(
  "Feature `VK_GOOGLE_surfaceless_query` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_surfaceless_query.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_maximal_reconvergence",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_KHR_shader_maximal_reconvergence` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_maximal_reconvergence.html"
);
#[cfg(all(
  feature = "VK_EXT_image_compression_control_swapchain",
  not(feature = "VK_EXT_image_compression_control")
))]
compile_error!(
  "Feature `VK_EXT_image_compression_control_swapchain` requires `VK_EXT_image_compression_control`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control_swapchain.html"
);
#[cfg(all(
  feature = "VK_QCOM_image_processing",
  not(any(feature = "VK_KHR_format_feature_flags2", feature = "VK_VERSION_1_3"))
))]
compile_error!(
  "Feature `VK_QCOM_image_processing` requires `VK_KHR_format_feature_flags2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing.html"
);
#[cfg(all(
  feature = "VK_EXT_nested_command_buffer",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_nested_command_buffer` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_nested_command_buffer.html"
);
#[cfg(all(
  feature = "VK_OHOS_external_memory",
  not(any(
    all(
      feature = "VK_KHR_sampler_ycbcr_conversion",
      feature = "VK_KHR_external_memory",
      feature = "VK_KHR_dedicated_allocation",
      feature = "VK_EXT_queue_family_foreign"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_EXT_queue_family_foreign")
  ))
))]
compile_error!(
  "Feature `VK_OHOS_external_memory` requires `VK_KHR_sampler_ycbcr_conversion + VK_KHR_external_memory + VK_KHR_dedicated_allocation + VK_EXT_queue_family_foreign , VK_VERSION_1_1 + VK_EXT_queue_family_foreign`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_external_memory.html"
);
#[cfg(all(
  feature = "VK_EXT_external_memory_acquire_unmodified",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_EXT_external_memory_acquire_unmodified` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_acquire_unmodified.html"
);
#[cfg(all(
  feature = "VK_EXT_extended_dynamic_state3",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_extended_dynamic_state3` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state3.html"
);
#[cfg(all(
  feature = "VK_EXT_subpass_merge_feedback",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_subpass_merge_feedback` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subpass_merge_feedback.html"
);
#[cfg(all(feature = "VK_ARM_tensors", not(feature = "VK_VERSION_1_3")))]
compile_error!(
  "Feature `VK_ARM_tensors` requires `VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_tensors.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_module_identifier",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_pipeline_creation_cache_control"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_pipeline_creation_cache_control"
    ),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_module_identifier` requires `VK_KHR_get_physical_device_properties2 + VK_EXT_pipeline_creation_cache_control , VK_VERSION_1_1 + VK_EXT_pipeline_creation_cache_control , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_module_identifier.html"
);
#[cfg(all(
  feature = "VK_EXT_rasterization_order_attachment_access",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_rasterization_order_attachment_access` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rasterization_order_attachment_access.html"
);
#[cfg(all(
  feature = "VK_NV_optical_flow",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_format_feature_flags2",
      feature = "VK_KHR_synchronization2"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_format_feature_flags2",
      feature = "VK_KHR_synchronization2"
    ),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_NV_optical_flow` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_format_feature_flags2 + VK_KHR_synchronization2 , VK_VERSION_1_1 + VK_KHR_format_feature_flags2 + VK_KHR_synchronization2 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_optical_flow.html"
);
#[cfg(all(
  feature = "VK_EXT_legacy_dithering",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_legacy_dithering` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_dithering.html"
);
#[cfg(all(
  feature = "VK_EXT_pipeline_protected_access",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pipeline_protected_access` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_protected_access.html"
);
#[cfg(all(
  feature = "VK_ANDROID_external_format_resolve",
  not(feature = "VK_ANDROID_external_memory_android_hardware_buffer")
))]
compile_error!(
  "Feature `VK_ANDROID_external_format_resolve` requires `VK_ANDROID_external_memory_android_hardware_buffer`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_format_resolve.html"
);
#[cfg(all(
  feature = "VK_KHR_maintenance5",
  not(any(
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_dynamic_rendering"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_KHR_maintenance5` requires `VK_VERSION_1_1 + VK_KHR_dynamic_rendering , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance5.html"
);
#[cfg(all(
  feature = "VK_AMD_anti_lag",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_AMD_anti_lag` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_anti_lag.html"
);
#[cfg(all(
  feature = "VK_AMDX_dense_geometry_format",
  not(any(
    all(
      feature = "VK_KHR_acceleration_structure",
      feature = "VK_KHR_maintenance5"
    ),
    all(feature = "VK_KHR_acceleration_structure", feature = "VK_VERSION_1_4")
  ))
))]
compile_error!(
  "Feature `VK_AMDX_dense_geometry_format` requires `VK_KHR_acceleration_structure + VK_KHR_maintenance5 , VK_KHR_acceleration_structure + VK_VERSION_1_4`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_dense_geometry_format.html"
);
#[cfg(all(
  feature = "VK_KHR_present_id2",
  not(all(
    feature = "VK_KHR_get_surface_capabilities2",
    feature = "VK_KHR_surface",
    feature = "VK_KHR_swapchain"
  ))
))]
compile_error!(
  "Feature `VK_KHR_present_id2` requires `VK_KHR_get_surface_capabilities2 + VK_KHR_surface + VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id2.html"
);
#[cfg(all(
  feature = "VK_KHR_present_wait2",
  not(all(
    feature = "VK_KHR_get_surface_capabilities2",
    feature = "VK_KHR_surface",
    feature = "VK_KHR_swapchain",
    feature = "VK_KHR_present_id2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_present_wait2` requires `VK_KHR_get_surface_capabilities2 + VK_KHR_surface + VK_KHR_swapchain + VK_KHR_present_id2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait2.html"
);
#[cfg(all(
  feature = "VK_KHR_ray_tracing_position_fetch",
  not(feature = "VK_KHR_acceleration_structure")
))]
compile_error!(
  "Feature `VK_KHR_ray_tracing_position_fetch` requires `VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_position_fetch.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_object",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_dynamic_rendering"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_object` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_dynamic_rendering , VK_VERSION_1_1 + VK_KHR_dynamic_rendering , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_object.html"
);
#[cfg(all(
  feature = "VK_KHR_pipeline_binary",
  not(any(feature = "VK_KHR_maintenance5", feature = "VK_VERSION_1_4"))
))]
compile_error!(
  "Feature `VK_KHR_pipeline_binary` requires `VK_KHR_maintenance5 , VK_VERSION_1_4`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_binary.html"
);
#[cfg(all(
  feature = "VK_QCOM_tile_properties",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_tile_properties` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_properties.html"
);
#[cfg(all(
  feature = "VK_SEC_amigo_profiling",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_SEC_amigo_profiling` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_amigo_profiling.html"
);
#[cfg(all(
  feature = "VK_KHR_surface_maintenance1",
  not(all(
    feature = "VK_KHR_surface",
    feature = "VK_KHR_get_surface_capabilities2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_surface_maintenance1` requires `VK_KHR_surface + VK_KHR_get_surface_capabilities2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_maintenance1.html"
);
#[cfg(all(
  feature = "VK_KHR_swapchain_maintenance1",
  not(any(
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_surface_maintenance1",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    all(
      feature = "VK_KHR_swapchain",
      feature = "VK_KHR_surface_maintenance1",
      feature = "VK_VERSION_1_1"
    )
  ))
))]
compile_error!(
  "Feature `VK_KHR_swapchain_maintenance1` requires `VK_KHR_swapchain + VK_KHR_surface_maintenance1 + VK_KHR_get_physical_device_properties2 , VK_KHR_swapchain + VK_KHR_surface_maintenance1 + VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_maintenance1.html"
);
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_viewports",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_multiview_per_view_viewports` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_viewports.html"
);
#[cfg(all(feature = "VK_NV_external_sci_sync2", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_NV_external_sci_sync2` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_sci_sync2.html"
);
#[cfg(all(
  feature = "VK_NV_ray_tracing_invocation_reorder",
  not(feature = "VK_KHR_ray_tracing_pipeline")
))]
compile_error!(
  "Feature `VK_NV_ray_tracing_invocation_reorder` requires `VK_KHR_ray_tracing_pipeline`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_invocation_reorder.html"
);
#[cfg(all(
  feature = "VK_NV_cooperative_vector",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_cooperative_vector` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_vector.html"
);
#[cfg(all(
  feature = "VK_NV_extended_sparse_address_space",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_extended_sparse_address_space` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_extended_sparse_address_space.html"
);
#[cfg(all(
  feature = "VK_EXT_mutable_descriptor_type",
  not(any(feature = "VK_KHR_maintenance3", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_EXT_mutable_descriptor_type` requires `VK_KHR_maintenance3 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mutable_descriptor_type.html"
);
#[cfg(all(
  feature = "VK_EXT_legacy_vertex_attributes",
  not(feature = "VK_EXT_vertex_input_dynamic_state")
))]
compile_error!(
  "Feature `VK_EXT_legacy_vertex_attributes` requires `VK_EXT_vertex_input_dynamic_state`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_vertex_attributes.html"
);
#[cfg(all(
  feature = "VK_ARM_shader_core_builtins",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_ARM_shader_core_builtins` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_builtins.html"
);
#[cfg(all(
  feature = "VK_EXT_pipeline_library_group_handles",
  not(all(
    feature = "VK_KHR_ray_tracing_pipeline",
    feature = "VK_KHR_pipeline_library"
  ))
))]
compile_error!(
  "Feature `VK_EXT_pipeline_library_group_handles` requires `VK_KHR_ray_tracing_pipeline + VK_KHR_pipeline_library`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_library_group_handles.html"
);
#[cfg(all(
  feature = "VK_EXT_dynamic_rendering_unused_attachments",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_dynamic_rendering"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_dynamic_rendering_unused_attachments` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_dynamic_rendering , VK_VERSION_1_1 + VK_KHR_dynamic_rendering , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_dynamic_rendering_unused_attachments.html"
);
#[cfg(all(
  feature = "VK_KHR_internally_synchronized_queues",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_KHR_internally_synchronized_queues` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_internally_synchronized_queues.html"
);
#[cfg(all(
  feature = "VK_NV_low_latency2",
  not(any(
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_present_id"),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_present_id2"),
    all(feature = "VK_KHR_timeline_semaphore", feature = "VK_KHR_present_id"),
    all(feature = "VK_KHR_timeline_semaphore", feature = "VK_KHR_present_id2")
  ))
))]
compile_error!(
  "Feature `VK_NV_low_latency2` requires `VK_VERSION_1_2 + VK_KHR_present_id , VK_VERSION_1_2 + VK_KHR_present_id2 , VK_KHR_timeline_semaphore + VK_KHR_present_id , VK_KHR_timeline_semaphore + VK_KHR_present_id2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency2.html"
);
#[cfg(all(
  feature = "VK_KHR_cooperative_matrix",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_cooperative_matrix` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_cooperative_matrix.html"
);
#[cfg(all(
  feature = "VK_ARM_data_graph",
  not(all(
    feature = "VK_VERSION_1_3",
    feature = "VK_KHR_maintenance5",
    feature = "VK_KHR_deferred_host_operations"
  ))
))]
compile_error!(
  "Feature `VK_ARM_data_graph` requires `VK_VERSION_1_3 + VK_KHR_maintenance5 + VK_KHR_deferred_host_operations`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph.html"
);
#[cfg(all(
  feature = "VK_ARM_data_graph_instruction_set_tosa",
  not(feature = "VK_ARM_data_graph")
))]
compile_error!(
  "Feature `VK_ARM_data_graph_instruction_set_tosa` requires `VK_ARM_data_graph`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_instruction_set_tosa.html"
);
#[cfg(all(
  feature = "VK_QCOM_multiview_per_view_render_areas",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_multiview_per_view_render_areas` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_render_areas.html"
);
#[cfg(all(
  feature = "VK_KHR_compute_shader_derivatives",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_compute_shader_derivatives` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_compute_shader_derivatives.html"
);
#[cfg(all(
  feature = "VK_KHR_video_decode_av1",
  not(feature = "VK_KHR_video_decode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_decode_av1` requires `VK_KHR_video_decode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_av1.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_av1",
  not(feature = "VK_KHR_video_encode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_encode_av1` requires `VK_KHR_video_encode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_av1.html"
);
#[cfg(all(
  feature = "VK_KHR_video_decode_vp9",
  not(feature = "VK_KHR_video_decode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_decode_vp9` requires `VK_KHR_video_decode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_vp9.html"
);
#[cfg(all(
  feature = "VK_KHR_video_maintenance1",
  not(feature = "VK_KHR_video_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_maintenance1` requires `VK_KHR_video_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance1.html"
);
#[cfg(all(
  feature = "VK_NV_per_stage_descriptor_set",
  not(any(feature = "VK_KHR_maintenance6", feature = "VK_VERSION_1_4"))
))]
compile_error!(
  "Feature `VK_NV_per_stage_descriptor_set` requires `VK_KHR_maintenance6 , VK_VERSION_1_4`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_per_stage_descriptor_set.html"
);
#[cfg(all(
  feature = "VK_QCOM_image_processing2",
  not(feature = "VK_QCOM_image_processing")
))]
compile_error!(
  "Feature `VK_QCOM_image_processing2` requires `VK_QCOM_image_processing`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing2.html"
);
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_weights",
  not(feature = "VK_EXT_filter_cubic")
))]
compile_error!(
  "Feature `VK_QCOM_filter_cubic_weights` requires `VK_EXT_filter_cubic`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_weights.html"
);
#[cfg(all(
  feature = "VK_QCOM_ycbcr_degamma",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_ycbcr_degamma` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_ycbcr_degamma.html"
);
#[cfg(all(
  feature = "VK_QCOM_filter_cubic_clamp",
  not(any(
    all(feature = "VK_EXT_filter_cubic", feature = "VK_VERSION_1_2"),
    all(
      feature = "VK_EXT_filter_cubic",
      feature = "VK_EXT_sampler_filter_minmax"
    )
  ))
))]
compile_error!(
  "Feature `VK_QCOM_filter_cubic_clamp` requires `VK_EXT_filter_cubic + VK_VERSION_1_2 , VK_EXT_filter_cubic + VK_EXT_sampler_filter_minmax`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_clamp.html"
);
#[cfg(all(
  feature = "VK_EXT_attachment_feedback_loop_dynamic_state",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_attachment_feedback_loop_layout"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_attachment_feedback_loop_layout"
    )
  ))
))]
compile_error!(
  "Feature `VK_EXT_attachment_feedback_loop_dynamic_state` requires `VK_KHR_get_physical_device_properties2 + VK_EXT_attachment_feedback_loop_layout , VK_VERSION_1_1 + VK_EXT_attachment_feedback_loop_layout`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_dynamic_state.html"
);
#[cfg(all(
  feature = "VK_KHR_vertex_attribute_divisor",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_vertex_attribute_divisor` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vertex_attribute_divisor.html"
);
#[cfg(all(
  feature = "VK_KHR_unified_image_layouts",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_unified_image_layouts` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_unified_image_layouts.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_float_controls2",
  not(all(feature = "VK_VERSION_1_1", feature = "VK_KHR_shader_float_controls"))
))]
compile_error!(
  "Feature `VK_KHR_shader_float_controls2` requires `VK_VERSION_1_1 + VK_KHR_shader_float_controls`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls2.html"
);
#[cfg(all(
  feature = "VK_QNX_external_memory_screen_buffer",
  not(any(
    all(
      feature = "VK_KHR_sampler_ycbcr_conversion",
      feature = "VK_KHR_external_memory",
      feature = "VK_KHR_dedicated_allocation",
      feature = "VK_EXT_queue_family_foreign"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_EXT_queue_family_foreign")
  ))
))]
compile_error!(
  "Feature `VK_QNX_external_memory_screen_buffer` requires `VK_KHR_sampler_ycbcr_conversion + VK_KHR_external_memory + VK_KHR_dedicated_allocation + VK_EXT_queue_family_foreign , VK_VERSION_1_1 + VK_EXT_queue_family_foreign`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_external_memory_screen_buffer.html"
);
#[cfg(all(
  feature = "VK_MSFT_layered_driver",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_MSFT_layered_driver` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_MSFT_layered_driver.html"
);
#[cfg(all(
  feature = "VK_KHR_index_type_uint8",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_index_type_uint8` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_index_type_uint8.html"
);
#[cfg(all(
  feature = "VK_KHR_line_rasterization",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_line_rasterization` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_line_rasterization.html"
);
#[cfg(all(
  feature = "VK_KHR_calibrated_timestamps",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_calibrated_timestamps` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_calibrated_timestamps.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_expect_assume",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_expect_assume` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_expect_assume.html"
);
#[cfg(all(feature = "VK_KHR_maintenance6", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_KHR_maintenance6` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance6.html"
);
#[cfg(all(
  feature = "VK_NV_descriptor_pool_overallocation",
  not(feature = "VK_VERSION_1_1")
))]
compile_error!(
  "Feature `VK_NV_descriptor_pool_overallocation` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_descriptor_pool_overallocation.html"
);
#[cfg(all(
  feature = "VK_QCOM_tile_memory_heap",
  not(any(
    all(
      feature = "VK_KHR_get_memory_requirements2",
      feature = "VK_KHR_get_physical_device_properties2"
    ),
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_QCOM_tile_memory_heap` requires `VK_KHR_get_memory_requirements2 + VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_memory_heap.html"
);
#[cfg(all(
  feature = "VK_KHR_copy_memory_indirect",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_buffer_device_address"
    ),
    feature = "VK_VERSION_1_2"
  ))
))]
compile_error!(
  "Feature `VK_KHR_copy_memory_indirect` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address , VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_memory_indirect.html"
);
#[cfg(all(
  feature = "VK_EXT_memory_decompression",
  not(all(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_KHR_buffer_device_address"
  ))
))]
compile_error!(
  "Feature `VK_EXT_memory_decompression` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_buffer_device_address`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_decompression.html"
);
#[cfg(all(
  feature = "VK_NV_display_stereo",
  not(all(feature = "VK_KHR_display", feature = "VK_KHR_get_display_properties2"))
))]
compile_error!(
  "Feature `VK_NV_display_stereo` requires `VK_KHR_display + VK_KHR_get_display_properties2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_display_stereo.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_intra_refresh",
  not(feature = "VK_KHR_video_encode_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_encode_intra_refresh` requires `VK_KHR_video_encode_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_intra_refresh.html"
);
#[cfg(all(
  feature = "VK_KHR_video_encode_quantization_map",
  not(any(
    all(
      feature = "VK_KHR_video_encode_queue",
      feature = "VK_KHR_format_feature_flags2"
    ),
    all(feature = "VK_KHR_video_encode_queue", feature = "VK_VERSION_1_3")
  ))
))]
compile_error!(
  "Feature `VK_KHR_video_encode_quantization_map` requires `VK_KHR_video_encode_queue + VK_KHR_format_feature_flags2 , VK_KHR_video_encode_queue + VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_quantization_map.html"
);
#[cfg(all(
  feature = "VK_NV_raw_access_chains",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_raw_access_chains` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_raw_access_chains.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_relaxed_extended_instruction",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_relaxed_extended_instruction` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_relaxed_extended_instruction.html"
);
#[cfg(all(
  feature = "VK_NV_command_buffer_inheritance",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_command_buffer_inheritance` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_command_buffer_inheritance.html"
);
#[cfg(all(feature = "VK_KHR_maintenance7", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_KHR_maintenance7` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance7.html"
);
#[cfg(all(
  feature = "VK_NV_shader_atomic_float16_vector",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_shader_atomic_float16_vector` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_atomic_float16_vector.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_replicated_composites",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_replicated_composites` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_replicated_composites.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_float8",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_float8` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_float8.html"
);
#[cfg(all(
  feature = "VK_NV_ray_tracing_validation",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_ray_tracing_validation` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_validation.html"
);
#[cfg(all(
  feature = "VK_NV_cluster_acceleration_structure",
  not(feature = "VK_KHR_acceleration_structure")
))]
compile_error!(
  "Feature `VK_NV_cluster_acceleration_structure` requires `VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cluster_acceleration_structure.html"
);
#[cfg(all(
  feature = "VK_NV_partitioned_acceleration_structure",
  not(feature = "VK_KHR_acceleration_structure")
))]
compile_error!(
  "Feature `VK_NV_partitioned_acceleration_structure` requires `VK_KHR_acceleration_structure`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_partitioned_acceleration_structure.html"
);
#[cfg(all(
  feature = "VK_EXT_device_generated_commands",
  not(any(
    all(
      feature = "VK_KHR_buffer_device_address",
      feature = "VK_KHR_maintenance5"
    ),
    all(feature = "VK_VERSION_1_2", feature = "VK_KHR_maintenance5"),
    feature = "VK_VERSION_1_3"
  ))
))]
compile_error!(
  "Feature `VK_EXT_device_generated_commands` requires `VK_KHR_buffer_device_address + VK_KHR_maintenance5 , VK_VERSION_1_2 + VK_KHR_maintenance5 , VK_VERSION_1_3`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_generated_commands.html"
);
#[cfg(all(
  feature = "VK_KHR_device_fault",
  not(feature = "VK_KHR_get_physical_device_properties2")
))]
compile_error!(
  "Feature `VK_KHR_device_fault` requires `VK_KHR_get_physical_device_properties2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_fault.html"
);
#[cfg(all(feature = "VK_KHR_maintenance8", not(feature = "VK_VERSION_1_1")))]
compile_error!(
  "Feature `VK_KHR_maintenance8` requires `VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance8.html"
);
#[cfg(all(
  feature = "VK_MESA_image_alignment_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_MESA_image_alignment_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_MESA_image_alignment_control.html"
);
#[cfg(all(
  feature = "VK_KHR_shader_fma",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_shader_fma` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_fma.html"
);
#[cfg(all(
  feature = "VK_EXT_ray_tracing_invocation_reorder",
  not(feature = "VK_KHR_ray_tracing_pipeline")
))]
compile_error!(
  "Feature `VK_EXT_ray_tracing_invocation_reorder` requires `VK_KHR_ray_tracing_pipeline`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ray_tracing_invocation_reorder.html"
);
#[cfg(all(
  feature = "VK_EXT_depth_clamp_control",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_depth_clamp_control` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_control.html"
);
#[cfg(all(
  feature = "VK_KHR_maintenance9",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_maintenance9` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance9.html"
);
#[cfg(all(
  feature = "VK_KHR_video_maintenance2",
  not(feature = "VK_KHR_video_queue")
))]
compile_error!(
  "Feature `VK_KHR_video_maintenance2` requires `VK_KHR_video_queue`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance2.html"
);
#[cfg(all(feature = "VK_OHOS_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_OHOS_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_surface.html"
);
#[cfg(all(
  feature = "VK_HUAWEI_hdr_vivid",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_swapchain",
      feature = "VK_EXT_hdr_metadata"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_KHR_swapchain",
      feature = "VK_EXT_hdr_metadata"
    )
  ))
))]
compile_error!(
  "Feature `VK_HUAWEI_hdr_vivid` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_swapchain + VK_EXT_hdr_metadata , VK_VERSION_1_1 + VK_KHR_swapchain + VK_EXT_hdr_metadata`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_hdr_vivid.html"
);
#[cfg(all(
  feature = "VK_NV_cooperative_matrix2",
  not(feature = "VK_KHR_cooperative_matrix")
))]
compile_error!(
  "Feature `VK_NV_cooperative_matrix2` requires `VK_KHR_cooperative_matrix`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix2.html"
);
#[cfg(all(
  feature = "VK_ARM_pipeline_opacity_micromap",
  not(feature = "VK_EXT_opacity_micromap")
))]
compile_error!(
  "Feature `VK_ARM_pipeline_opacity_micromap` requires `VK_EXT_opacity_micromap`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_pipeline_opacity_micromap.html"
);
#[cfg(all(
  feature = "VK_EXT_external_memory_metal",
  not(any(feature = "VK_KHR_external_memory", feature = "VK_VERSION_1_1"))
))]
compile_error!(
  "Feature `VK_EXT_external_memory_metal` requires `VK_KHR_external_memory , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_metal.html"
);
#[cfg(all(
  feature = "VK_KHR_depth_clamp_zero_one",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_depth_clamp_zero_one` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_clamp_zero_one.html"
);
#[cfg(all(
  feature = "VK_ARM_performance_counters_by_region",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_ARM_performance_counters_by_region` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_performance_counters_by_region.html"
);
#[cfg(all(
  feature = "VK_ARM_shader_instrumentation",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_ARM_shader_instrumentation` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_instrumentation.html"
);
#[cfg(all(
  feature = "VK_EXT_vertex_attribute_robustness",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_vertex_attribute_robustness` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_robustness.html"
);
#[cfg(all(
  feature = "VK_ARM_format_pack",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_ARM_format_pack` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_format_pack.html"
);
#[cfg(all(
  feature = "VK_VALVE_fragment_density_map_layered",
  not(any(
    all(
      feature = "VK_KHR_maintenance5",
      feature = "VK_EXT_fragment_density_map"
    ),
    all(feature = "VK_VERSION_1_4", feature = "VK_EXT_fragment_density_map")
  ))
))]
compile_error!(
  "Feature `VK_VALVE_fragment_density_map_layered` requires `VK_KHR_maintenance5 + VK_EXT_fragment_density_map , VK_VERSION_1_4 + VK_EXT_fragment_density_map`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_fragment_density_map_layered.html"
);
#[cfg(all(
  feature = "VK_KHR_robustness2",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_robustness2` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_robustness2.html"
);
#[cfg(all(
  feature = "VK_NV_present_metering",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_present_metering` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_metering.html"
);
#[cfg(all(
  feature = "VK_EXT_fragment_density_map_offset",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_VERSION_1_2",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_VERSION_1_2",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_KHR_create_renderpass2",
      feature = "VK_KHR_dynamic_rendering"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_VERSION_1_2",
      feature = "VK_VERSION_1_3"
    ),
    all(
      feature = "VK_VERSION_1_1",
      feature = "VK_EXT_fragment_density_map",
      feature = "VK_VERSION_1_2",
      feature = "VK_KHR_dynamic_rendering"
    )
  ))
))]
compile_error!(
  "Feature `VK_EXT_fragment_density_map_offset` requires `VK_KHR_get_physical_device_properties2 + VK_EXT_fragment_density_map + VK_KHR_create_renderpass2 + VK_VERSION_1_3 , VK_KHR_get_physical_device_properties2 + VK_EXT_fragment_density_map + VK_KHR_create_renderpass2 + VK_KHR_dynamic_rendering , VK_KHR_get_physical_device_properties2 + VK_EXT_fragment_density_map + VK_VERSION_1_2 + VK_VERSION_1_3 , VK_KHR_get_physical_device_properties2 + VK_EXT_fragment_density_map + VK_VERSION_1_2 + VK_KHR_dynamic_rendering , VK_VERSION_1_1 + VK_EXT_fragment_density_map + VK_KHR_create_renderpass2 + VK_VERSION_1_3 , VK_VERSION_1_1 + VK_EXT_fragment_density_map + VK_KHR_create_renderpass2 + VK_KHR_dynamic_rendering , VK_VERSION_1_1 + VK_EXT_fragment_density_map + VK_VERSION_1_2 + VK_VERSION_1_3 , VK_VERSION_1_1 + VK_EXT_fragment_density_map + VK_VERSION_1_2 + VK_KHR_dynamic_rendering`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map_offset.html"
);
#[cfg(all(
  feature = "VK_EXT_zero_initialize_device_memory",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_zero_initialize_device_memory` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_zero_initialize_device_memory.html"
);
#[cfg(all(
  feature = "VK_KHR_present_mode_fifo_latest_ready",
  not(feature = "VK_KHR_swapchain")
))]
compile_error!(
  "Feature `VK_KHR_present_mode_fifo_latest_ready` requires `VK_KHR_swapchain`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_mode_fifo_latest_ready.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_64bit_indexing",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_64bit_indexing` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_64bit_indexing.html"
);
#[cfg(all(
  feature = "VK_EXT_custom_resolve",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_custom_resolve` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_resolve.html"
);
#[cfg(all(
  feature = "VK_QCOM_data_graph_model",
  not(feature = "VK_ARM_data_graph")
))]
compile_error!(
  "Feature `VK_QCOM_data_graph_model` requires `VK_ARM_data_graph`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_data_graph_model.html"
);
#[cfg(all(
  feature = "VK_KHR_maintenance10",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_KHR_maintenance10` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance10.html"
);
#[cfg(all(
  feature = "VK_ARM_data_graph_optical_flow",
  not(feature = "VK_ARM_data_graph")
))]
compile_error!(
  "Feature `VK_ARM_data_graph_optical_flow` requires `VK_ARM_data_graph`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_optical_flow.html"
);
#[cfg(all(feature = "VK_EXT_shader_long_vector", not(feature = "VK_VERSION_1_2")))]
compile_error!(
  "Feature `VK_EXT_shader_long_vector` requires `VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_long_vector.html"
);
#[cfg(all(
  feature = "VK_SEC_pipeline_cache_incremental_mode",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_SEC_pipeline_cache_incremental_mode` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_pipeline_cache_incremental_mode.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_uniform_buffer_unsized_array",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_uniform_buffer_unsized_array` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_uniform_buffer_unsized_array.html"
);
#[cfg(all(
  feature = "VK_NV_compute_occupancy_priority",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_NV_compute_occupancy_priority` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_occupancy_priority.html"
);
#[cfg(all(
  feature = "VK_EXT_shader_subgroup_partitioned",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_shader_subgroup_partitioned` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_partitioned.html"
);
#[cfg(all(feature = "VK_SEC_ubm_surface", not(feature = "VK_KHR_surface")))]
compile_error!(
  "Feature `VK_SEC_ubm_surface` requires `VK_KHR_surface`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_ubm_surface.html"
);
#[cfg(all(
  feature = "VK_VALVE_shader_mixed_float_dot_product",
  not(any(
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_KHR_shader_float16_int8"
    ),
    all(
      feature = "VK_KHR_get_physical_device_properties2",
      feature = "VK_VERSION_1_2"
    ),
    all(feature = "VK_VERSION_1_1", feature = "VK_KHR_shader_float16_int8"),
    all(feature = "VK_VERSION_1_1", feature = "VK_VERSION_1_2")
  ))
))]
compile_error!(
  "Feature `VK_VALVE_shader_mixed_float_dot_product` requires `VK_KHR_get_physical_device_properties2 + VK_KHR_shader_float16_int8 , VK_KHR_get_physical_device_properties2 + VK_VERSION_1_2 , VK_VERSION_1_1 + VK_KHR_shader_float16_int8 , VK_VERSION_1_1 + VK_VERSION_1_2`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_shader_mixed_float_dot_product.html"
);
#[cfg(all(
  feature = "VK_EXT_primitive_restart_index",
  not(any(
    feature = "VK_KHR_get_physical_device_properties2",
    feature = "VK_VERSION_1_1"
  ))
))]
compile_error!(
  "Feature `VK_EXT_primitive_restart_index` requires `VK_KHR_get_physical_device_properties2 , VK_VERSION_1_1`.\nAdd the required features to Cargo.toml.\nSpec: https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_restart_index.html"
);
