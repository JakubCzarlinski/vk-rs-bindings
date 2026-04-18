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
use std::sync::Arc;
#[cfg(target_os = "linux")]
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
#[cfg(target_os = "linux")]
use std::thread::sleep;
#[cfg(target_os = "linux")]
use std::time::Instant;
#[cfg(target_os = "linux")]
use windsurf::{Event, EventLoop, LogicalSize, Window, WindowAttributes};

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    let sim_hz = read_hz("SIM_HZ", 120.0);
    let render_hz = read_hz("RENDER_HZ", 60.0);
    println!(
        "Starting windsurf spinning-triangle demo (sim {:.1} Hz, render {:.1} Hz)",
        sim_hz, render_hz
    );

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

    let angular_speed = core::f32::consts::PI * 0.75;
    let sim_step = Duration::from_secs_f64(1.0 / sim_hz);
    let render_step = Duration::from_secs_f64(1.0 / render_hz);
    let sim_start = Instant::now();
    let mut next_render_tick = sim_start;

    let shared_angle = Arc::new(AtomicU32::new(0.0f32.to_bits()));
    let sim_running = Arc::new(AtomicBool::new(true));
    let sim_angle = Arc::clone(&shared_angle);
    let sim_running_flag = Arc::clone(&sim_running);
    let sim_thread = std::thread::spawn(move || {
        let mut local_sim_step_index: u64 = 0;
        let mut local_angle: f32 = 0.0;
        while sim_running_flag.load(Ordering::Relaxed) {
            let now = Instant::now();
            advance_simulation(
                now,
                sim_start,
                sim_step,
                angular_speed,
                &mut local_sim_step_index,
                &mut local_angle,
            );
            sim_angle.store(local_angle.to_bits(), Ordering::Relaxed);
            sleep(sim_step / 2);
        }
    });

    let mut running = true;
    let mut pending_resize: Option<(u32, u32)> = None;
    while running {
        let timeout = Some(next_render_tick.saturating_duration_since(Instant::now()));
        if let Some((scope, event)) = event_loop.wait_event(timeout)?
            && scope == Some(window_handle)
        {
            match event {
                Event::CloseRequested => {
                    running = false;
                }
                Event::WindowResized { width, height } => {
                    pending_resize = Some((width, height));
                }
                Event::DragDropDropped {
                    position,
                    action,
                    data,
                } => {
                    println!(
                        "drop action={action:?} at ({:.1}, {:.1}) payload_items={}",
                        position.x,
                        position.y,
                        data.len()
                    );
                }
                _ => {}
            }
        }

        let now = Instant::now();
        if now < next_render_tick {
            continue;
        }

        let size = pending_resize.unwrap_or_else(|| window.inner_size());
        if size.0 > 0 && size.1 > 0 {
            if let Some(new_size) = pending_resize.take() {
                let _ = device.vkDeviceWaitIdle();
                recreate_swapchain_state(
                    physical_device,
                    &device,
                    &surface,
                    new_size,
                    &mut swapchain_state,
                    &render_pass,
                    &mut framebuffers,
                    &mut render_finished,
                    &mut images_in_flight,
                );
            }

            let angle = f32::from_bits(shared_angle.load(Ordering::Relaxed));
            match draw_frame(
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
                angle,
            ) {
                Ok(()) => {}
                Err(true) => {
                    let size = window.inner_size();
                    if size.0 > 0 && size.1 > 0 {
                        let _ = device.vkDeviceWaitIdle();
                        recreate_swapchain_state(
                            physical_device,
                            &device,
                            &surface,
                            size,
                            &mut swapchain_state,
                            &render_pass,
                            &mut framebuffers,
                            &mut render_finished,
                            &mut images_in_flight,
                        );
                    }
                }
                Err(false) => {
                    running = false;
                }
            }
        }

        next_render_tick += render_step;
        if now.saturating_duration_since(next_render_tick) > render_step.saturating_mul(4) {
            next_render_tick = now + render_step;
        }
    }

    sim_running.store(false, Ordering::Relaxed);
    let _ = sim_thread.join();
    let _ = device.vkDeviceWaitIdle();
    Ok(())
}

#[cfg(target_os = "linux")]
fn read_hz(key: &str, default: f64) -> f64 {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| *v > 0.0)
        .unwrap_or(default)
}

#[cfg(target_os = "linux")]
fn advance_simulation(
    now: Instant,
    sim_start: Instant,
    sim_step: Duration,
    angular_speed: f32,
    sim_step_index: &mut u64,
    angle: &mut f32,
) {
    let step_secs = sim_step.as_secs_f64();
    if step_secs <= 0.0 {
        return;
    }

    let elapsed_steps = ((now - sim_start).as_secs_f64() / step_secs).floor() as u64;
    if elapsed_steps <= *sim_step_index {
        return;
    }

    let delta_steps = elapsed_steps - *sim_step_index;
    *sim_step_index = elapsed_steps;
    *angle += delta_steps as f32 * angular_speed * sim_step.as_secs_f32();
    if *angle > core::f32::consts::TAU {
        *angle %= core::f32::consts::TAU;
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("The basic_window example currently targets Linux Wayland");
}
