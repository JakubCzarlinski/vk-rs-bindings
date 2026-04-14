use alloc::collections::{BTreeMap, VecDeque};
use alloc::rc::{Rc, Weak};
use core::cell::{Cell, RefCell};
use objc2::rc::Retained;
use objc2_app_kit::{NSView, NSWindow};
use objc2_quartz_core::CAMetalLayer;
use windsurf_core::{Event, LogicalSize, WindowId};
use windsurf_extra::{CursorIcon, CursorMode, ExtraEvent};

extern crate alloc;

pub(crate) type SharedDisplayRef = Rc<RefCell<SharedState>>;

pub(crate) struct SharedState {
    pub(crate) next_window_id: Cell<u64>,
    pub(crate) pending_events: VecDeque<Event>,
    pub(crate) pending_extra_events: VecDeque<ExtraEvent>,
    pub(crate) keyboard_focus: Option<WindowId>,
    pub(crate) pointer_focus: Option<WindowId>,
    pub(crate) cursor_hidden: bool,
    pub(crate) windows: BTreeMap<WindowId, WindowState>,
}

impl SharedState {
    pub(crate) fn push(&mut self, event: Event) {
        self.pending_events.push_back(event);
    }

    pub(crate) fn push_extra(&mut self, event: ExtraEvent) {
        self.pending_extra_events.push_back(event);
    }
}

pub(crate) struct WindowInner {
    pub(crate) window: Retained<NSWindow>,
    pub(crate) view: Retained<NSView>,
    pub(crate) layer: Retained<CAMetalLayer>,
}

pub(crate) struct WindowState {
    pub(crate) inner: Weak<WindowInner>,
    pub(crate) size: LogicalSize,
    pub(crate) scale_factor: f64,
    pub(crate) needs_redraw: bool,
    pub(crate) close_requested: bool,
    pub(crate) ime_enabled: bool,
    pub(crate) cursor_mode: CursorMode,
    pub(crate) cursor_visible: bool,
    pub(crate) cursor_icon: CursorIcon,
}
