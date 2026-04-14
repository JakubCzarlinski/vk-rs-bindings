#![allow(
    clippy::borrow_as_ptr,
    clippy::cast_precision_loss,
    clippy::cast_ptr_alignment,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

use core::error::Error;
use core::ffi::{c_char, c_void};
use core::ptr::{null, null_mut};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
#[cfg(target_os = "macos")]
use raw_window_metal::Layer as MetalLayer;
use vk::*;
use windsurf::Window;

const VERT_SHADER_SPV: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/windsurf_triangle.vert.spv"));
const FRAG_SHADER_SPV: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/windsurf_triangle.frag.spv"));

const FRAMES_IN_FLIGHT: usize = 2;

pub fn example_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(std::io::Error::other(message.into()))
}
pub fn create_instance<'a>(
    entry: &'a Entry<'a>,
    window: &Window,
) -> Result<Instance<'a>, Box<dyn Error>> {
    // const VALIDATION_LAYER: &std::ffi::CStr = c"VK_LAYER_KHRONOS_validation";
    // const LAYER_NAMES: [*const i8; 1] = [VALIDATION_LAYER.as_ptr()];
    const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
        .with_apiVersion(VK_API_VERSION_1_4)
        .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
        .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
        .with_pEngineName(c"windsurf-examples".as_ptr())
        .with_pApplicationName(c"windsurf Vulkan Triangle".as_ptr());

    let instance_extensions = required_instance_extensions(window);
    let create_info = VkInstanceCreateInfo::DEFAULT
        .with_pApplicationInfo(&APP_INFO)
        // .with_enabledLayerCount(LAYER_NAMES.len() as u32)
        // .with_ppEnabledLayerNames(LAYER_NAMES.as_ptr())
        .with_enabledExtensionCount(instance_extensions.len() as u32)
        .with_ppEnabledExtensionNames(instance_extensions.as_ptr());

    entry
        .vkCreateInstance(&create_info, null())
        .map_err(|err| example_error(format!("vkCreateInstance failed: {err:?}")))
}

fn required_instance_extensions(window: &Window) -> Vec<*const c_char> {
    let mut extensions = vec![VK_KHR_SURFACE_EXTENSION_NAME.as_ptr()];
    let window_handle = window
        .window_handle()
        .expect("window handle unavailable")
        .as_raw();

    match window_handle {
        #[cfg(target_os = "linux")]
        RawWindowHandle::Wayland(_) => {
            extensions.push(VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME.as_ptr());
        }
        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(_) => {
            extensions.push(VK_EXT_METAL_SURFACE_EXTENSION_NAME.as_ptr());
        }
        other => panic!("unsupported window backend for Vulkan example: {other:?}"),
    }

    extensions
}

pub fn create_surface<'a>(
    instance: &'a Instance<'a>,
    window: &Window,
    #[cfg(target_os = "macos")] metal_layer: &mut Option<MetalLayer>,
) -> Result<SurfaceKHR<'a>, Box<dyn Error>> {
    let window_handle = window
        .window_handle()
        .expect("window handle unavailable")
        .as_raw();
    let display_handle = window
        .display_handle()
        .expect("display handle unavailable")
        .as_raw();

    match (window_handle, display_handle) {
        #[cfg(target_os = "linux")]
        (RawWindowHandle::Wayland(wayland_window), RawDisplayHandle::Wayland(wayland_display)) => {
            let create_info = VkWaylandSurfaceCreateInfoKHR::DEFAULT
                .with_display(wayland_display.display.as_ptr().cast::<wl_display>())
                .with_surface(wayland_window.surface.as_ptr().cast::<wl_surface>());
            instance
                .vkCreateWaylandSurfaceKHR(&create_info, null())
                .map_err(|err| example_error(format!("vkCreateWaylandSurfaceKHR failed: {err:?}")))
        }
        #[cfg(target_os = "macos")]
        (RawWindowHandle::AppKit(appkit_window), RawDisplayHandle::AppKit(_)) => {
            let layer = unsafe { MetalLayer::from_ns_view(appkit_window.ns_view) };
            let create_info = VkMetalSurfaceCreateInfoEXT::DEFAULT
                .with_pLayer(layer.as_ptr().as_ptr().cast::<CAMetalLayer>());
            let surface = instance
                .vkCreateMetalSurfaceEXT(&create_info, null())
                .map_err(|err| example_error(format!("vkCreateMetalSurfaceEXT failed: {err:?}")))?;
            *metal_layer = Some(layer);
            Ok(surface)
        }
        (other_window, other_display) => Err(example_error(format!(
            "unsupported window/display backend combination: {other_window:?} / {other_display:?}"
        ))),
    }
}

pub fn find_graphics_present_queue_family(
    physical_device: &PhysicalDevice<'_>,
    surface: &SurfaceKHR<'_>,
) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());

    let mut props = vec![VkQueueFamilyProperties2::DEFAULT; count as usize];
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, props.as_mut_ptr());

    props.iter().enumerate().find_map(|(index, family)| {
        let is_graphics = (family.queueFamilyProperties.queueFlags
            & VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT.0)
            != 0;
        if !is_graphics {
            return None;
        }

        let mut present_supported = 0u32;
        physical_device
            .vkGetPhysicalDeviceSurfaceSupportKHR(
                index as u32,
                surface.raw(),
                &mut present_supported,
            )
            .ok()?;
        (present_supported != 0).then_some(index as u32)
    })
}

pub fn create_device<'a>(
    physical_device: &'a PhysicalDevice<'a>,
    queue_family_index: u32,
) -> Result<Device<'a>, Box<dyn Error>> {
    const PRIORITIES: [f32; 1] = [1.0];
    const ENABLED_EXTENSIONS: [*const i8; 1] = [VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr()];
    const VULKAN_13_FEATURES: VkPhysicalDeviceVulkan13Features =
        VkPhysicalDeviceVulkan13Features::DEFAULT.with_synchronization2(1);
    let queue_info = VkDeviceQueueCreateInfo::DEFAULT
        .with_queueCount(1)
        .with_pQueuePriorities(PRIORITIES.as_ptr())
        .with_queueFamilyIndex(queue_family_index);
    let device_info = VkDeviceCreateInfo::DEFAULT
        .with_queueCreateInfoCount(1)
        .with_enabledExtensionCount(ENABLED_EXTENSIONS.len() as u32)
        .with_ppEnabledExtensionNames(ENABLED_EXTENSIONS.as_ptr())
        .with_pNext(core::ptr::from_ref(&VULKAN_13_FEATURES).cast::<c_void>())
        .with_pQueueCreateInfos(&queue_info);

    physical_device
        .vkCreateDevice(&device_info, null())
        .map_err(|err| example_error(format!("vkCreateDevice failed: {err:?}")))
}

pub struct SwapchainState<'a> {
    swapchain: SwapchainKHR<'a>,
    pub images: Vec<VkImage>,
    pub image_views: Vec<ImageView<'a>>,
    pub surface_format: VkSurfaceFormatKHR,
    pub extent: VkExtent2D,
}

pub fn create_swapchain_state<'a>(
    physical_device: &'a PhysicalDevice<'a>,
    device: &'a Device<'a>,
    surface: &SurfaceKHR<'a>,
    window_size: (u32, u32),
    old_swapchain: Option<&SwapchainKHR<'a>>,
) -> SwapchainState<'a> {
    let surface_format = pick_surface_format(physical_device, surface);
    let present_mode = pick_present_mode(physical_device, surface);

    let mut caps = VkSurfaceCapabilitiesKHR::DEFAULT;
    physical_device
        .vkGetPhysicalDeviceSurfaceCapabilitiesKHR(surface.raw(), &mut caps)
        .expect("vkGetPhysicalDeviceSurfaceCapabilitiesKHR failed");

    let mut image_count = caps.minImageCount + 1;
    if caps.maxImageCount > 0 && image_count > caps.maxImageCount {
        image_count = caps.maxImageCount;
    }

    let extent = choose_extent(caps, window_size);
    let composite_alpha = choose_composite_alpha(caps.supportedCompositeAlpha);

    let mut create_info = VkSwapchainCreateInfoKHR::DEFAULT
        .with_surface(surface.raw())
        .with_minImageCount(image_count)
        .with_imageFormat(surface_format.format)
        .with_imageColorSpace(surface_format.colorSpace)
        .with_imageExtent(extent)
        .with_imageArrayLayers(1)
        .with_imageUsage(VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT.0)
        .with_imageSharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_preTransform(caps.currentTransform)
        .with_compositeAlpha(composite_alpha)
        .with_presentMode(present_mode)
        .with_clipped(1);
    create_info = create_info
        .with_oldSwapchain(old_swapchain.map_or(VkSwapchainKHR::NULL, SwapchainKHR::raw));

    let swapchain = device
        .vkCreateSwapchainKHR(&create_info, null())
        .expect("vkCreateSwapchainKHR failed");

    let images = get_swapchain_images(&swapchain);
    let image_views = images
        .iter()
        .map(|image| {
            let range = VkImageSubresourceRange::DEFAULT
                .with_aspectMask(VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT.0)
                .with_baseMipLevel(0)
                .with_levelCount(1)
                .with_baseArrayLayer(0)
                .with_layerCount(1);

            let view_info = VkImageViewCreateInfo::DEFAULT
                .with_image(*image)
                .with_viewType(VkImageViewType::VK_IMAGE_VIEW_TYPE_2D)
                .with_format(surface_format.format)
                .with_subresourceRange(range);

            device
                .vkCreateImageView(&view_info, null())
                .expect("vkCreateImageView failed")
        })
        .collect::<Vec<_>>();

    SwapchainState {
        swapchain,
        images,
        image_views,
        surface_format,
        extent,
    }
}

fn get_swapchain_images(swapchain: &SwapchainKHR<'_>) -> Vec<VkImage> {
    let mut count = 0;
    swapchain
        .vkGetSwapchainImagesKHR(&mut count, null_mut())
        .expect("vkGetSwapchainImagesKHR(count) failed");
    let mut images = vec![VkImage::NULL; count as usize];
    swapchain
        .vkGetSwapchainImagesKHR(&mut count, images.as_mut_ptr())
        .expect("vkGetSwapchainImagesKHR(list) failed");
    images.truncate(count as usize);
    images
}

fn pick_surface_format(
    physical_device: &PhysicalDevice<'_>,
    surface: &SurfaceKHR<'_>,
) -> VkSurfaceFormatKHR {
    let mut count = 0;
    physical_device
        .vkGetPhysicalDeviceSurfaceFormatsKHR(surface.raw(), &mut count, null_mut())
        .expect("vkGetPhysicalDeviceSurfaceFormatsKHR(count) failed");
    let mut formats = vec![VkSurfaceFormatKHR::DEFAULT; count as usize];
    physical_device
        .vkGetPhysicalDeviceSurfaceFormatsKHR(surface.raw(), &mut count, formats.as_mut_ptr())
        .expect("vkGetPhysicalDeviceSurfaceFormatsKHR(list) failed");

    formats
        .iter()
        .copied()
        .find(|f| {
            f.format == VkFormat::VK_FORMAT_R8G8B8A8_SRGB
                && f.colorSpace == VkColorSpaceKHR::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR
        })
        .unwrap_or(formats[0])
}

fn pick_present_mode(
    physical_device: &PhysicalDevice<'_>,
    surface: &SurfaceKHR<'_>,
) -> VkPresentModeKHR {
    let mut count = 0;
    physical_device
        .vkGetPhysicalDeviceSurfacePresentModesKHR(surface.raw(), &mut count, null_mut())
        .expect("vkGetPhysicalDeviceSurfacePresentModesKHR(count) failed");
    let mut modes = vec![VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR; count as usize];
    physical_device
        .vkGetPhysicalDeviceSurfacePresentModesKHR(surface.raw(), &mut count, modes.as_mut_ptr())
        .expect("vkGetPhysicalDeviceSurfacePresentModesKHR(list) failed");
    modes
        .into_iter()
        .find(|mode| *mode == VkPresentModeKHR::VK_PRESENT_MODE_MAILBOX_KHR)
        .unwrap_or(VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR)
}

fn choose_extent(caps: VkSurfaceCapabilitiesKHR, window_size: (u32, u32)) -> VkExtent2D {
    if caps.currentExtent.width != u32::MAX {
        return caps.currentExtent;
    }

    let width = window_size
        .0
        .clamp(caps.minImageExtent.width, caps.maxImageExtent.width);
    let height = window_size
        .1
        .clamp(caps.minImageExtent.height, caps.maxImageExtent.height);
    VkExtent2D::DEFAULT.with_width(width).with_height(height)
}

fn choose_composite_alpha(supported: VkCompositeAlphaFlagsKHR) -> VkCompositeAlphaFlagBitsKHR {
    const OPTIONS: [VkCompositeAlphaFlagBitsKHR; 4] = [
        VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR,
        VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR,
        VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR,
        VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
    ];
    OPTIONS
        .into_iter()
        .find(|flag| (supported & flag.0) != 0)
        .unwrap_or(VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR)
}

pub fn create_render_pass<'a>(device: &'a Device<'a>, color_format: VkFormat) -> RenderPass<'a> {
    let color_attachment = VkAttachmentDescription2::DEFAULT
        .with_format(color_format)
        .with_samples(VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT)
        .with_loadOp(VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_CLEAR)
        .with_storeOp(VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_STORE)
        .with_stencilLoadOp(VkAttachmentLoadOp::VK_ATTACHMENT_LOAD_OP_DONT_CARE)
        .with_stencilStoreOp(VkAttachmentStoreOp::VK_ATTACHMENT_STORE_OP_DONT_CARE)
        .with_initialLayout(VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED)
        .with_finalLayout(VkImageLayout::VK_IMAGE_LAYOUT_PRESENT_SRC_KHR);

    let color_ref = VkAttachmentReference2::DEFAULT
        .with_attachment(0)
        .with_layout(VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL);

    let subpass = VkSubpassDescription2::DEFAULT
        .with_pipelineBindPoint(VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS)
        .with_colorAttachmentCount(1)
        .with_pColorAttachments(&color_ref);

    let dependency = VkSubpassDependency2::DEFAULT
        .with_srcSubpass(VK_SUBPASS_EXTERNAL)
        .with_dstSubpass(0)
        .with_srcStageMask(
            VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT.0 as u32,
        )
        .with_dstStageMask(
            VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT.0 as u32,
        )
        .with_srcAccessMask(0)
        .with_dstAccessMask(VkAccessFlagBits2::VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT.0 as u32);

    let info = VkRenderPassCreateInfo2::DEFAULT
        .with_attachmentCount(1)
        .with_pAttachments(&color_attachment)
        .with_subpassCount(1)
        .with_pSubpasses(&subpass)
        .with_dependencyCount(1)
        .with_pDependencies(&dependency);

    device
        .vkCreateRenderPass2(&info, null())
        .expect("vkCreateRenderPass2 failed")
}

pub fn create_graphics_pipeline<'a>(
    device: &'a Device<'a>,
    render_pass: VkRenderPass,
) -> (PipelineLayout<'a>, Pipeline<'a>) {
    let vert_module = create_shader_module(device, VERT_SHADER_SPV);
    let frag_module = create_shader_module(device, FRAG_SHADER_SPV);

    let shader_stages = [
        VkPipelineShaderStageCreateInfo::DEFAULT
            .with_stage(VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT)
            .with_module(vert_module.raw())
            .with_pName(c"main".as_ptr()),
        VkPipelineShaderStageCreateInfo::DEFAULT
            .with_stage(VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT)
            .with_module(frag_module.raw())
            .with_pName(c"main".as_ptr()),
    ];

    let vertex_input = VkPipelineVertexInputStateCreateInfo::DEFAULT;
    let input_assembly = VkPipelineInputAssemblyStateCreateInfo::DEFAULT
        .with_topology(VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST)
        .with_primitiveRestartEnable(0);
    let viewport_state = VkPipelineViewportStateCreateInfo::DEFAULT
        .with_viewportCount(1)
        .with_scissorCount(1);
    let rasterization = VkPipelineRasterizationStateCreateInfo::DEFAULT
        .with_depthClampEnable(0)
        .with_rasterizerDiscardEnable(0)
        .with_polygonMode(VkPolygonMode::VK_POLYGON_MODE_FILL)
        .with_lineWidth(1.0)
        .with_cullMode(VkCullModeFlagBits::VK_CULL_MODE_NONE.0)
        .with_frontFace(VkFrontFace::VK_FRONT_FACE_COUNTER_CLOCKWISE)
        .with_depthBiasEnable(0);
    let multisample = VkPipelineMultisampleStateCreateInfo::DEFAULT
        .with_sampleShadingEnable(0)
        .with_rasterizationSamples(VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT);
    let color_blend_attachment = VkPipelineColorBlendAttachmentState::DEFAULT
        .with_colorWriteMask(
            VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT.0
                | VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT.0
                | VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT.0
                | VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT.0,
        )
        .with_blendEnable(0);
    let color_blend = VkPipelineColorBlendStateCreateInfo::DEFAULT
        .with_logicOpEnable(0)
        .with_attachmentCount(1)
        .with_pAttachments(&color_blend_attachment);
    let dynamic_states = [
        VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT,
        VkDynamicState::VK_DYNAMIC_STATE_SCISSOR,
    ];
    let dynamic = VkPipelineDynamicStateCreateInfo::DEFAULT
        .with_dynamicStateCount(dynamic_states.len() as u32)
        .with_pDynamicStates(dynamic_states.as_ptr());
    let layout_info = VkPipelineLayoutCreateInfo::DEFAULT;
    let pipeline_layout = device
        .vkCreatePipelineLayout(&layout_info, null())
        .expect("vkCreatePipelineLayout failed");

    let pipeline_info = VkGraphicsPipelineCreateInfo::DEFAULT
        .with_stageCount(shader_stages.len() as u32)
        .with_pStages(shader_stages.as_ptr())
        .with_pVertexInputState(&vertex_input)
        .with_pInputAssemblyState(&input_assembly)
        .with_pViewportState(&viewport_state)
        .with_pRasterizationState(&rasterization)
        .with_pMultisampleState(&multisample)
        .with_pColorBlendState(&color_blend)
        .with_pDynamicState(&dynamic)
        .with_layout(pipeline_layout.raw())
        .with_renderPass(render_pass)
        .with_subpass(0);

    let pipeline = device
        .vkCreateGraphicsPipelines(VkPipelineCache::NULL, 1, &pipeline_info, null())
        .expect("vkCreateGraphicsPipelines failed")
        .into_iter()
        .next()
        .expect("no pipeline returned");

    (pipeline_layout, pipeline)
}

fn create_shader_module<'a>(device: &'a Device<'a>, bytes: &[u8]) -> ShaderModule<'a> {
    let info = VkShaderModuleCreateInfo::DEFAULT
        .with_codeSize(bytes.len())
        .with_pCode(bytes.as_ptr().cast::<u32>());
    device
        .vkCreateShaderModule(&info, null())
        .expect("vkCreateShaderModule failed")
}

pub fn create_framebuffers<'a>(
    device: &'a Device<'a>,
    render_pass: VkRenderPass,
    image_views: &[ImageView<'a>],
    extent: VkExtent2D,
) -> Vec<Framebuffer<'a>> {
    image_views
        .iter()
        .map(|view| {
            let attachments = [view.raw()];
            let info = VkFramebufferCreateInfo::DEFAULT
                .with_renderPass(render_pass)
                .with_attachmentCount(1)
                .with_pAttachments(attachments.as_ptr())
                .with_width(extent.width)
                .with_height(extent.height)
                .with_layers(1);
            device
                .vkCreateFramebuffer(&info, null())
                .expect("vkCreateFramebuffer failed")
        })
        .collect()
}

pub struct FrameSync<'a> {
    image_available: Semaphore<'a>,
    in_flight_fence: Fence<'a>,
}

pub fn create_frame_sync<'a>(device: &'a Device<'a>) -> Vec<FrameSync<'a>> {
    let mut frames = Vec::with_capacity(FRAMES_IN_FLIGHT);
    for _ in 0..FRAMES_IN_FLIGHT {
        let semaphore_info = VkSemaphoreCreateInfo::DEFAULT;
        let fence_info = VkFenceCreateInfo::DEFAULT
            .with_flags(VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT.0);
        frames.push(FrameSync {
            image_available: device
                .vkCreateSemaphore(&semaphore_info, null())
                .expect("vkCreateSemaphore(image_available) failed"),
            in_flight_fence: device
                .vkCreateFence(&fence_info, null())
                .expect("vkCreateFence failed"),
        });
    }
    frames
}

pub fn create_render_finished_semaphores<'a>(
    device: &'a Device<'a>,
    count: usize,
) -> Vec<Semaphore<'a>> {
    let semaphore_info = VkSemaphoreCreateInfo::DEFAULT;
    let mut semaphores = Vec::with_capacity(count);
    for _ in 0..count {
        semaphores.push(
            device
                .vkCreateSemaphore(&semaphore_info, null())
                .expect("vkCreateSemaphore(render_finished) failed"),
        );
    }
    semaphores
}

pub fn create_command_pools<'a>(
    device: &'a Device<'a>,
    queue_family_index: u32,
) -> Vec<CommandPool<'a>> {
    let mut pools = Vec::with_capacity(FRAMES_IN_FLIGHT);
    for _ in 0..FRAMES_IN_FLIGHT {
        let pool_info = VkCommandPoolCreateInfo::DEFAULT
            .with_queueFamilyIndex(queue_family_index)
            .with_flags(
                VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT.0,
            );
        pools.push(
            device
                .vkCreateCommandPool(&pool_info, null())
                .expect("vkCreateCommandPool failed"),
        );
    }
    pools
}

pub fn create_command_buffers<'p>(command_pools: &'p [CommandPool<'_>]) -> Vec<CommandBuffer<'p>> {
    let mut result = Vec::with_capacity(command_pools.len());
    for pool in command_pools {
        let alloc_info = VkCommandBufferAllocateInfo::DEFAULT
            .with_commandPool(pool.raw())
            .with_level(VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
            .with_commandBufferCount(1);
        let mut command_buffers = pool
            .vkAllocateCommandBuffers(&alloc_info)
            .expect("vkAllocateCommandBuffers failed");
        result.push(command_buffers.pop().expect("no command buffer allocated"));
    }
    result
}

#[allow(clippy::too_many_arguments)]
pub fn draw_frame(
    device: &Device<'_>,
    queue: &Queue<'_>,
    pipeline: &Pipeline<'_>,
    pipeline_layout: &PipelineLayout<'_>,
    render_pass: VkRenderPass,
    swapchain_state: &SwapchainState<'_>,
    framebuffers: &[Framebuffer<'_>],
    frame_sync: &[FrameSync<'_>],
    render_finished: &[Semaphore<'_>],
    command_buffers: &mut [CommandBuffer<'_>],
    images_in_flight: &mut [VkFence],
    current_frame: &mut usize,
) -> Result<(), bool> {
    let sync = &frame_sync[*current_frame];
    let command_buffer = &mut command_buffers[*current_frame];

    device
        .vkWaitForFences(1, &sync.in_flight_fence.raw(), 1, u64::MAX)
        .expect("vkWaitForFences(frame fence) failed");

    let mut image_index = 0u32;
    match swapchain_state.swapchain.vkAcquireNextImageKHR(
        u64::MAX,
        sync.image_available.raw(),
        VkFence::NULL,
        &mut image_index,
    ) {
        Ok(VkResult::VK_SUCCESS | VkResult::VK_SUBOPTIMAL_KHR) => {}
        Err(VkResult::VK_ERROR_OUT_OF_DATE_KHR) => return Err(true),
        Err(err) => {
            eprintln!("vkAcquireNextImageKHR failed: {err:?}");
            return Err(false);
        }
        Ok(other) => {
            eprintln!("vkAcquireNextImageKHR unexpected result: {other:?}");
            return Err(false);
        }
    }

    let image_index_usize = image_index as usize;
    let render_finished_semaphore = &render_finished[image_index_usize];
    if images_in_flight[image_index_usize] != VkFence::NULL {
        device
            .vkWaitForFences(1, &images_in_flight[image_index_usize], 1, u64::MAX)
            .expect("vkWaitForFences(image fence) failed");
    }
    images_in_flight[image_index_usize] = sync.in_flight_fence.raw();

    device
        .vkResetFences(1, &sync.in_flight_fence.raw())
        .expect("vkResetFences failed");
    command_buffer
        .vkResetCommandBuffer(
            VkCommandBufferResetFlagBits::VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT.0,
        )
        .expect("vkResetCommandBuffer failed");

    record_command_buffer(
        command_buffer,
        render_pass,
        framebuffers[image_index_usize].raw(),
        swapchain_state.extent,
        pipeline.raw(),
        pipeline_layout.raw(),
    );

    let wait_semaphore = VkSemaphoreSubmitInfo::DEFAULT
        .with_semaphore(sync.image_available.raw())
        .with_stageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT.0)
        .with_deviceIndex(0)
        .with_value(0);
    let signal_semaphore = VkSemaphoreSubmitInfo::DEFAULT
        .with_semaphore(render_finished_semaphore.raw())
        .with_stageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT.0)
        .with_deviceIndex(0)
        .with_value(0);
    let cmd_info = VkCommandBufferSubmitInfo::DEFAULT
        .with_commandBuffer(command_buffer.raw())
        .with_deviceMask(0);
    let submit_info = VkSubmitInfo2::DEFAULT
        .with_waitSemaphoreInfoCount(1)
        .with_pWaitSemaphoreInfos(&wait_semaphore)
        .with_commandBufferInfoCount(1)
        .with_pCommandBufferInfos(&cmd_info)
        .with_signalSemaphoreInfoCount(1)
        .with_pSignalSemaphoreInfos(&signal_semaphore);

    queue
        .vkQueueSubmit2(1, &submit_info, sync.in_flight_fence.raw())
        .expect("vkQueueSubmit2 failed");

    let wait_semaphores = [render_finished_semaphore.raw()];
    let swapchains = [swapchain_state.swapchain.raw()];
    let image_indices = [image_index];
    let present_info = VkPresentInfoKHR::DEFAULT
        .with_waitSemaphoreCount(1)
        .with_pWaitSemaphores(wait_semaphores.as_ptr())
        .with_swapchainCount(1)
        .with_pSwapchains(swapchains.as_ptr())
        .with_pImageIndices(image_indices.as_ptr());

    match queue.vkQueuePresentKHR(&present_info) {
        Ok(VkResult::VK_SUCCESS) => {}
        Ok(VkResult::VK_SUBOPTIMAL_KHR) | Err(VkResult::VK_ERROR_OUT_OF_DATE_KHR) => {
            *current_frame = (*current_frame + 1) % FRAMES_IN_FLIGHT;
            return Err(true);
        }
        Err(err) => {
            eprintln!("vkQueuePresentKHR failed: {err:?}");
            return Err(false);
        }
        Ok(other) => {
            eprintln!("vkQueuePresentKHR unexpected result: {other:?}");
            return Err(false);
        }
    }

    *current_frame = (*current_frame + 1) % FRAMES_IN_FLIGHT;
    Ok(())
}

fn record_command_buffer(
    command_buffer: &CommandBuffer<'_>,
    render_pass: VkRenderPass,
    framebuffer: VkFramebuffer,
    extent: VkExtent2D,
    pipeline: VkPipeline,
    _pipeline_layout: VkPipelineLayout,
) {
    command_buffer
        .vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .expect("vkBeginCommandBuffer failed");

    let clear_color = VkClearColorValue {
        float32: [0.05, 0.06, 0.09, 0.5],
    };
    let clear_value = VkClearValue { color: clear_color };
    let clear_values = [clear_value];

    let render_pass_begin = VkRenderPassBeginInfo::DEFAULT
        .with_renderPass(render_pass)
        .with_framebuffer(framebuffer)
        .with_renderArea(
            VkRect2D::DEFAULT
                .with_offset(VkOffset2D::DEFAULT.with_x(0).with_y(0))
                .with_extent(extent),
        )
        .with_clearValueCount(1)
        .with_pClearValues(clear_values.as_ptr());
    let subpass_begin =
        VkSubpassBeginInfo::DEFAULT.with_contents(VkSubpassContents::VK_SUBPASS_CONTENTS_INLINE);
    let subpass_end = VkSubpassEndInfo::DEFAULT;

    command_buffer.vkCmdBeginRenderPass2(&render_pass_begin, &subpass_begin);
    command_buffer.vkCmdBindPipeline(
        VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
        pipeline,
    );

    let viewport = VkViewport::DEFAULT
        .with_x(0.0)
        .with_y(0.0)
        .with_width(extent.width as f32)
        .with_height(extent.height as f32)
        .with_minDepth(0.0)
        .with_maxDepth(1.0);
    command_buffer.vkCmdSetViewport(0, 1, &viewport);

    let scissor = VkRect2D::DEFAULT
        .with_offset(VkOffset2D::DEFAULT.with_x(0).with_y(0))
        .with_extent(extent);
    command_buffer.vkCmdSetScissor(0, 1, &scissor);

    command_buffer.vkCmdDraw(3, 1, 0, 0);
    command_buffer.vkCmdEndRenderPass2(&subpass_end);
    command_buffer
        .vkEndCommandBuffer()
        .expect("vkEndCommandBuffer failed");
}

pub fn recreate_swapchain_state<'a>(
    physical_device: &'a PhysicalDevice<'a>,
    device: &'a Device<'a>,
    surface: &SurfaceKHR<'a>,
    size: (u32, u32),
    swapchain_state: &mut SwapchainState<'a>,
    render_pass: &RenderPass<'a>,
    framebuffers: &mut Vec<Framebuffer<'a>>,
    render_finished: &mut Vec<Semaphore<'a>>,
    images_in_flight: &mut Vec<VkFence>,
) {
    let new_state = create_swapchain_state(
        physical_device,
        device,
        surface,
        size,
        Some(&swapchain_state.swapchain),
    );
    *framebuffers = create_framebuffers(
        device,
        render_pass.raw(),
        &new_state.image_views,
        new_state.extent,
    );
    *render_finished = create_render_finished_semaphores(device, new_state.images.len());
    *images_in_flight = vec![VkFence::NULL; new_state.images.len()];
    *swapchain_state = new_state;
}
