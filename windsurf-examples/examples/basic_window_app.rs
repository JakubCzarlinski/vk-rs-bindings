use crate::basic_window_vulkan::{
    create_command_buffers, create_command_pools, create_device, create_frame_sync,
    create_framebuffers, create_graphics_pipeline, create_instance,
    create_render_finished_semaphores, create_render_pass, create_surface, create_swapchain_state,
    draw_frame, example_error, find_graphics_present_queue_family, recreate_swapchain_state,
};
use core::error::Error;
use core::time::Duration;
use std::thread;
use windsurf::{
    CursorIcon, CursorSource, Display, DragData, DragDropEvent, Event, EventQueue, FeatureSet,
    Features, ImePurpose, ImeState, KeyCode, KeyState, PointerButton, Window, WindowAttributes,
};

const CURSOR_CYCLE: [CursorIcon; 5] = [
    CursorIcon::Pointer,
    CursorIcon::Crosshair,
    CursorIcon::Text,
    CursorIcon::Grab,
    CursorIcon::Move,
];

pub fn main() -> Result<(), Box<dyn Error>> {
    let display = Display::connect()?;
    let window = Window::new(
        &display,
        &WindowAttributes {
            title: String::from("windsurf Vulkan triangle"),
            transparent: true,
            ..WindowAttributes::default()
        },
    )?;

    let mut events = EventQueue::new();
    let features = display.supported_features();
    let mut keyboard_focused = false;
    let mut cursor_idx = 0usize;

    if features.contains(FeatureSet::CURSOR) {
        let _ = display.set_cursor(
            window.id(),
            &CursorSource::Icon(CURSOR_CYCLE[cursor_idx % CURSOR_CYCLE.len()]),
        );
    }
    if features.contains(FeatureSet::IME) {
        let _ = display.set_ime_state(
            window.id(),
            &ImeState {
                enabled: true,
                purpose: ImePurpose::Normal,
                cursor_area: None,
            },
        );
    }

    let vulkan_lib = vk::VulkanLib::load()
        .map_err(|err| example_error(format!("failed to load Vulkan loader: {err:?}")))?;
    let entry = vk::Entry::new(&vulkan_lib);
    let instance = create_instance(&entry, &window)?;
    #[cfg(target_os = "macos")]
    let mut metal_layer = None;
    let surface = create_surface(
        &instance,
        &window,
        #[cfg(target_os = "macos")]
        &mut metal_layer,
    )?;

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
        display.pump(&mut events)?;

        for event in events.drain() {
            match event {
                Event::CloseRequested { .. } => {
                    let _ = device.vkDeviceWaitIdle();
                    return Ok(());
                }
                Event::WindowCreated { .. } => {
                    window.request_redraw();
                }
                Event::PointerButton {
                    button: PointerButton::Left,
                    state: windsurf::ButtonState::Pressed,
                    ..
                } if features.contains(FeatureSet::CURSOR) => {
                    cursor_idx += 1;
                    let icon = CURSOR_CYCLE[cursor_idx % CURSOR_CYCLE.len()];
                    let _ = display.set_cursor(window.id(), &CursorSource::Icon(icon));
                    eprintln!("cursor changed: {icon:?}");
                }
                Event::KeyboardFocusIn { id } => {
                    keyboard_focused = true;
                    if features.contains(FeatureSet::IME) {
                        let _ = display.set_ime_state(
                            id,
                            &ImeState {
                                enabled: true,
                                purpose: ImePurpose::Normal,
                                cursor_area: None,
                            },
                        );
                    }
                }
                Event::KeyboardFocusOut { id } => {
                    keyboard_focused = false;
                    if features.contains(FeatureSet::IME) {
                        let _ = display.set_ime_state(
                            id,
                            &ImeState {
                                enabled: false,
                                purpose: ImePurpose::Normal,
                                cursor_area: None,
                            },
                        );
                    }
                }
                Event::Key { key, state, .. }
                    if keyboard_focused
                        && matches!(state, KeyState::Pressed | KeyState::Repeated) =>
                {
                    if !matches!(key, KeyCode::Unknown) {
                        eprintln!("key: {key:?}");
                    }
                }
                Event::TextInput { text, .. } if keyboard_focused => {
                    eprintln!("typed: {text}");
                }
                Event::WindowResized { width, height, .. } => {
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
                Event::RedrawRequested { .. } => {
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
                    }
                }
                Event::DragDrop(DragDropEvent::Dropped { data, .. }) => {
                    for item in data {
                        match item {
                            DragData::Files(paths) => {
                                for path in paths {
                                    eprintln!("drag-drop file: {path}");
                                }
                            }
                            DragData::Text(text) => {
                                eprintln!("drag-drop text: {text}");
                            }
                            DragData::Bytes { mime_type, data } => {
                                eprintln!("drag-drop bytes: mime={mime_type}, len={}", data.len());
                            }
                        }
                    }
                }
                Event::Ime(other) => eprintln!("ime event: {other:?}"),
                Event::Cursor(other) => eprintln!("cursor event: {other:?}"),
                Event::DragDrop(other) => eprintln!("drag event: {other:?}"),
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(8));
    }
}
