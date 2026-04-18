#[cfg(target_os = "linux")]
mod basic_window_vulkan;

#[cfg(target_os = "linux")]
use crate::basic_window_vulkan::{
    create_command_buffers, create_command_pools, create_device, create_frame_sync,
    create_framebuffers, create_graphics_pipeline, create_instance,
    create_render_finished_semaphores, create_render_pass, create_surface, create_swapchain_state,
    draw_frame, example_error, find_graphics_present_queue_family, recreate_swapchain_state,
};
#[cfg(target_os = "linux")]
use core::time::Duration;
#[cfg(target_os = "linux")]
use windsurf::{Event, EventLoop, LogicalSize, Window, WindowAttributes};

#[cfg(target_os = "linux")]
const WAIT_SLICE: Duration = Duration::from_millis(16);

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut event_loop = EventLoop::connect()?;
    let window = Window::new(
        &event_loop,
        WindowAttributes {
            title: String::from("windsurf Vulkan triangle"),
            size: LogicalSize::new(800, 600),
            transparent: true,
            ..WindowAttributes::default()
        },
    )?;
    let window_handle = window.handle();

    let vulkan_lib = vk::VulkanLib::load()
        .map_err(|err| example_error(format!("failed to load Vulkan loader: {err:?}")))?;
    let entry = vk::Entry::new(&vulkan_lib);
    let instance = create_instance(&entry, &window)?;
    let surface = create_surface(&instance, &window)?;

    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|err| example_error(format!("failed to enumerate physical devices: {err:?}")))?;
    let physical_device = physical_devices
        .first()
        .ok_or_else(|| example_error("no physical device found"))?;
    let queue_family_index = find_graphics_present_queue_family(physical_device, &surface)
        .ok_or_else(|| example_error("no graphics+present queue family"))?;
    let device = create_device(physical_device, queue_family_index)?;
    let queue = device.vkGetDeviceQueue(queue_family_index, 0);

    let mut swapchain_state = create_swapchain_state(
        physical_device,
        &device,
        &surface,
        window.inner_size(),
        None,
    );
    let render_pass = create_render_pass(&device, swapchain_state.surface_format.format);
    let (pipeline_layout, pipeline) = create_graphics_pipeline(&device, render_pass.raw());
    let mut framebuffers = create_framebuffers(
        &device,
        render_pass.raw(),
        &swapchain_state.image_views,
        swapchain_state.extent,
    );
    let frame_sync = create_frame_sync(&device);
    let mut render_finished =
        create_render_finished_semaphores(&device, swapchain_state.images.len());
    let command_pools = create_command_pools(&device, queue_family_index);
    let mut command_buffers = create_command_buffers(&command_pools);
    let mut images_in_flight = vec![vk::VkFence::NULL; swapchain_state.images.len()];
    let mut current_frame = 0usize;

    window.request_redraw();
    loop {
        let Some((scope, event)) = event_loop.wait_event(Some(WAIT_SLICE))? else {
            continue;
        };
        if scope != Some(window_handle) {
            continue;
        }

        match event {
            Event::CloseRequested => {
                let _ = device.vkDeviceWaitIdle();
                return Ok(());
            }
            Event::WindowCreated => {
                window.request_redraw();
            }
            Event::WindowResized { width, height } => {
                if width > 0 && height > 0 {
                    let _ = device.vkDeviceWaitIdle();
                    recreate_swapchain_state(
                        physical_device,
                        &device,
                        &surface,
                        (width, height),
                        &mut swapchain_state,
                        &render_pass,
                        &mut framebuffers,
                        &mut render_finished,
                        &mut images_in_flight,
                    );
                    window.request_redraw();
                }
            }
            Event::RedrawRequested => match draw_frame(
                &device,
                &queue,
                &pipeline,
                &pipeline_layout,
                render_pass.raw(),
                &swapchain_state,
                &framebuffers,
                &frame_sync,
                &render_finished,
                &mut command_buffers,
                &mut images_in_flight,
                &mut current_frame,
            ) {
                Ok(()) => {}
                Err(true) => {
                    let _ = device.vkDeviceWaitIdle();
                    recreate_swapchain_state(
                        physical_device,
                        &device,
                        &surface,
                        window.inner_size(),
                        &mut swapchain_state,
                        &render_pass,
                        &mut framebuffers,
                        &mut render_finished,
                        &mut images_in_flight,
                    );
                    window.request_redraw();
                }
                Err(false) => {
                    let _ = device.vkDeviceWaitIdle();
                    return Ok(());
                }
            },
            _ => {}
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("The basic_window example currently targets Linux Wayland");
}
