use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;

use wayland_backend::client::ObjectId;
use wayland_client::globals::GlobalList;
use wayland_client::protocol::{wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_surface};
use wayland_client::{Connection, Proxy};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
use windsurf_core::{Event, LogicalSize, WindowId};

use crate::xkb::XkbState;

pub(crate) struct SharedDisplay {
    pub(crate) connection: Connection,
    pub(crate) globals: GlobalList,
    pub(crate) compositor: wl_compositor::WlCompositor,
    pub(crate) wm_base: xdg_wm_base::XdgWmBase,
    pub(crate) decoration_manager: Option<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
    pub(crate) next_window_id: AtomicU64,
    pub(crate) pump: Mutex<PumpState>,
}

pub(crate) struct PumpState {
    pub(crate) event_queue: wayland_client::EventQueue<State>,
    pub(crate) state: State,
}

pub(crate) struct State {
    pub(crate) pending_events: VecDeque<Event>,
    pub(crate) compositor: wl_compositor::WlCompositor,
    pub(crate) windows: HashMap<WindowId, WindowState>,
    pub(crate) surface_to_window: HashMap<ObjectId, WindowId>,
    pub(crate) pointer_focus: Option<WindowId>,
    pub(crate) keyboard_focus: Option<WindowId>,
    pub(crate) seat: Option<wl_seat::WlSeat>,
    pub(crate) pointer: Option<wl_pointer::WlPointer>,
    pub(crate) keyboard: Option<wl_keyboard::WlKeyboard>,
    pub(crate) xkb: Option<XkbState>,
}

pub(crate) struct WindowState {
    pub(crate) surface: wl_surface::WlSurface,
    pub(crate) size: LogicalSize,
    pub(crate) scale_factor: f64,
    pub(crate) needs_redraw: bool,
    pub(crate) transparent: bool,
}
impl State {
    pub(crate) fn new(compositor: wl_compositor::WlCompositor) -> Self {
        Self {
            pending_events: VecDeque::new(),
            compositor,
            windows: HashMap::new(),
            surface_to_window: HashMap::new(),
            pointer_focus: None,
            keyboard_focus: None,
            seat: None,
            pointer: None,
            keyboard: None,
            xkb: XkbState::new(),
        }
    }

    pub(crate) fn window_for_surface(&self, surface: &wl_surface::WlSurface) -> Option<WindowId> {
        self.surface_to_window.get(&surface.id()).copied()
    }

    pub(crate) fn push(&mut self, event: Event) {
        self.pending_events.push_back(event);
    }
}

pub(crate) type SharedDisplayRef = Rc<SharedDisplay>;
