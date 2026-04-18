use crate::xkb::XkbState;
use alloc::rc::Rc;
use parking_lot::Mutex;
use std::collections::HashMap;
use wayland_backend::client::ObjectId;
use wayland_client::globals::GlobalList;
use wayland_client::protocol::{wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_surface};
use wayland_client::{Connection, Proxy};
use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1, wp_cursor_shape_manager_v1,
};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
use windsurf_core::{
    CursorIcon, CursorMode, CursorSource, Event, EventQueue, WindowHandle, WindowHandleAllocator,
};

extern crate alloc;

pub(crate) struct SharedDisplay {
    pub(crate) connection: Connection,
    pub(crate) globals: GlobalList,
    pub(crate) compositor: wl_compositor::WlCompositor,
    pub(crate) wm_base: xdg_wm_base::XdgWmBase,
    pub(crate) decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    pub(crate) pump: Mutex<PumpState>,
}

pub(crate) struct PumpState {
    pub(crate) event_queue: wayland_client::EventQueue<State>,
    pub(crate) state: State,
}

pub(crate) struct State {
    pub(crate) pending_events: EventQueue<1024>,
    pub(crate) compositor: wl_compositor::WlCompositor,
    pub(crate) handles: WindowHandleAllocator,
    pub(crate) windows: HashMap<WindowHandle, WindowState>,
    pub(crate) surface_to_window: HashMap<ObjectId, WindowHandle>,
    pub(crate) pointer_focus: Option<WindowHandle>,
    pub(crate) keyboard_focus: Option<WindowHandle>,
    pub(crate) seat: Option<wl_seat::WlSeat>,
    pub(crate) pointer: Option<wl_pointer::WlPointer>,
    pub(crate) cursor_shape_manager: Option<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1>,
    pub(crate) cursor_shape_device: Option<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1>,
    pub(crate) pointer_enter_serial: Option<u32>,
    pub(crate) keyboard: Option<wl_keyboard::WlKeyboard>,
    pub(crate) xkb: Option<XkbState>,
}

pub(crate) struct WindowState {
    pub(crate) surface: wl_surface::WlSurface,
    pub(crate) size: windsurf_core::LogicalSize,
    pub(crate) scale_factor: f64,
    pub(crate) needs_redraw: bool,
    pub(crate) transparent: bool,
    pub(crate) ime_enabled: bool,
    pub(crate) cursor_mode: CursorMode,
    pub(crate) cursor_visible: bool,
    pub(crate) cursor_icon: CursorIcon,
}

impl State {
    pub(crate) fn new(
        compositor: wl_compositor::WlCompositor,
        cursor_shape_manager: Option<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1>,
    ) -> Self {
        Self {
            pending_events: EventQueue::new(),
            compositor,
            handles: WindowHandleAllocator::new(),
            windows: HashMap::new(),
            surface_to_window: HashMap::new(),
            pointer_focus: None,
            keyboard_focus: None,
            seat: None,
            pointer: None,
            cursor_shape_manager,
            cursor_shape_device: None,
            pointer_enter_serial: None,
            keyboard: None,
            xkb: XkbState::new(),
        }
    }

    pub(crate) fn window_for_surface(
        &self,
        surface: &wl_surface::WlSurface,
    ) -> Option<WindowHandle> {
        self.surface_to_window.get(&surface.id()).copied()
    }

    pub(crate) fn push_window(&mut self, window: WindowHandle, event: Event) {
        let _ = self.pending_events.push(Some(window), event);
    }

    pub(crate) fn pop_event(&mut self) -> Option<(Option<WindowHandle>, Event)> {
        self.pending_events.pop()
    }

    pub(crate) fn take_overflow_count(&mut self) -> usize {
        self.pending_events.take_dropped_count()
    }

    pub(crate) fn apply_cursor(&mut self, window: WindowHandle) {
        let Some(window_state) = self.windows.get(&window) else {
            return;
        };
        let Some(serial) = self.pointer_enter_serial else {
            return;
        };

        if !window_state.cursor_visible || matches!(window_state.cursor_mode, CursorMode::Hidden) {
            if let Some(pointer) = self.pointer.as_ref() {
                pointer.set_cursor(serial, None, 0, 0);
            }
            return;
        }

        let Some(device) = self.cursor_shape_device.as_ref() else {
            return;
        };
        device.set_shape(serial, map_cursor_shape(window_state.cursor_icon));
    }
}

pub(crate) type SharedDisplayRef = Rc<SharedDisplay>;

pub(crate) fn icon_from_source(source: &CursorSource) -> CursorIcon {
    match source {
        CursorSource::Icon(icon) => *icon,
        CursorSource::Rgba { .. } => CursorIcon::Default,
    }
}

fn map_cursor_shape(icon: CursorIcon) -> wp_cursor_shape_device_v1::Shape {
    match icon {
        CursorIcon::Default => wp_cursor_shape_device_v1::Shape::Default,
        CursorIcon::Crosshair => wp_cursor_shape_device_v1::Shape::Crosshair,
        CursorIcon::Pointer => wp_cursor_shape_device_v1::Shape::Pointer,
        CursorIcon::Text => wp_cursor_shape_device_v1::Shape::Text,
        CursorIcon::Wait => wp_cursor_shape_device_v1::Shape::Wait,
        CursorIcon::Help => wp_cursor_shape_device_v1::Shape::Help,
        CursorIcon::Progress => wp_cursor_shape_device_v1::Shape::Progress,
        CursorIcon::NotAllowed => wp_cursor_shape_device_v1::Shape::NotAllowed,
        CursorIcon::Grab => wp_cursor_shape_device_v1::Shape::Grab,
        CursorIcon::Grabbing => wp_cursor_shape_device_v1::Shape::Grabbing,
        CursorIcon::Move => wp_cursor_shape_device_v1::Shape::Move,
        CursorIcon::EwResize => wp_cursor_shape_device_v1::Shape::EwResize,
        CursorIcon::NsResize => wp_cursor_shape_device_v1::Shape::NsResize,
        CursorIcon::NeswResize => wp_cursor_shape_device_v1::Shape::NeswResize,
        CursorIcon::NwseResize => wp_cursor_shape_device_v1::Shape::NwseResize,
        CursorIcon::ColResize => wp_cursor_shape_device_v1::Shape::ColResize,
        CursorIcon::RowResize => wp_cursor_shape_device_v1::Shape::RowResize,
        CursorIcon::ZoomIn => wp_cursor_shape_device_v1::Shape::ZoomIn,
        CursorIcon::ZoomOut => wp_cursor_shape_device_v1::Shape::ZoomOut,
    }
}
