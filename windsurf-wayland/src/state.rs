use crate::xkb::XkbState;
use alloc::rc::Rc;
#[cfg(feature = "drag_drop")]
use alloc::string::String;
use parking_lot::Mutex;
use smallvec::SmallVec;
use wayland_backend::client::ObjectId;
use wayland_client::globals::GlobalList;
use wayland_client::protocol::{wl_compositor, wl_keyboard, wl_pointer, wl_seat, wl_surface};
#[cfg(feature = "drag_drop")]
use wayland_client::protocol::{wl_data_device, wl_data_device_manager, wl_data_offer};
use wayland_client::{Connection, Proxy};
#[cfg(feature = "cursor")]
use wayland_protocols::wp::cursor_shape::v1::client::{
    wp_cursor_shape_device_v1, wp_cursor_shape_manager_v1,
};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
#[cfg(feature = "cursor")]
use windsurf_core::CursorSource;
#[cfg(feature = "cursor")]
use windsurf_core::{CursorIcon, CursorMode};
#[cfg(feature = "drag_drop")]
use windsurf_core::{DragAction, DragPosition};
use windsurf_core::{Event, EventQueue, WindowHandle, WindowHandleAllocator};

extern crate alloc;

pub(crate) const PENDING_EVENT_QUEUE_CAPACITY: usize = 16;
pub(crate) type PendingEventQueue = EventQueue;

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
    pub(crate) pending_events: PendingEventQueue,
    pub(crate) compositor: wl_compositor::WlCompositor,
    pub(crate) handles: WindowHandleAllocator,
    windows: SmallVec<[(WindowHandle, WindowState); 2]>,
    surface_to_window: SmallVec<[(ObjectId, WindowHandle); 2]>,
    pub(crate) pointer_focus: Option<WindowHandle>,
    pub(crate) keyboard_focus: Option<WindowHandle>,
    pub(crate) seat: Option<wl_seat::WlSeat>,
    pub(crate) pointer: Option<wl_pointer::WlPointer>,
    #[cfg(feature = "cursor")]
    pub(crate) cursor_shape_manager: Option<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1>,
    #[cfg(feature = "cursor")]
    pub(crate) cursor_shape_device: Option<wp_cursor_shape_device_v1::WpCursorShapeDeviceV1>,
    pub(crate) pointer_enter_serial: Option<u32>,
    pub(crate) keyboard: Option<wl_keyboard::WlKeyboard>,
    pub(crate) xkb: Option<XkbState>,
    #[cfg(feature = "drag_drop")]
    pub(crate) data_device_manager: Option<wl_data_device_manager::WlDataDeviceManager>,
    #[cfg(feature = "drag_drop")]
    pub(crate) data_device: Option<wl_data_device::WlDataDevice>,
    #[cfg(feature = "drag_drop")]
    drag_offers: SmallVec<[(ObjectId, DragOfferState); 2]>,
    #[cfg(feature = "drag_drop")]
    pub(crate) current_drag: Option<CurrentDrag>,
}

#[cfg(feature = "drag_drop")]
pub(crate) struct DragOfferState {
    pub(crate) offer: wl_data_offer::WlDataOffer,
    pub(crate) offered: SmallVec<[String; 4]>,
    pub(crate) action: DragAction,
}

#[cfg(feature = "drag_drop")]
pub(crate) struct CurrentDrag {
    pub(crate) window: WindowHandle,
    pub(crate) offer_id: ObjectId,
    pub(crate) position: DragPosition,
}

pub(crate) struct WindowState {
    pub(crate) surface: wl_surface::WlSurface,
    pub(crate) size: windsurf_core::LogicalSize,
    pub(crate) scale_factor: f64,
    pub(crate) needs_redraw: bool,
    pub(crate) transparent: bool,
    #[cfg(feature = "ime")]
    pub(crate) ime_enabled: bool,
    #[cfg(feature = "cursor")]
    pub(crate) cursor_mode: CursorMode,
    #[cfg(feature = "cursor")]
    pub(crate) cursor_visible: bool,
    #[cfg(feature = "cursor")]
    pub(crate) cursor_icon: CursorIcon,
}

impl State {
    pub(crate) fn new(
        compositor: wl_compositor::WlCompositor,
        #[cfg(feature = "drag_drop")] data_device_manager: Option<
            wl_data_device_manager::WlDataDeviceManager,
        >,
        #[cfg(feature = "cursor")] cursor_shape_manager: Option<
            wp_cursor_shape_manager_v1::WpCursorShapeManagerV1,
        >,
    ) -> Self {
        Self {
            pending_events: PendingEventQueue::with_capacity(PENDING_EVENT_QUEUE_CAPACITY),
            compositor,
            handles: WindowHandleAllocator::new(),
            windows: SmallVec::new(),
            surface_to_window: SmallVec::new(),
            pointer_focus: None,
            keyboard_focus: None,
            seat: None,
            pointer: None,
            #[cfg(feature = "cursor")]
            cursor_shape_manager,
            #[cfg(feature = "cursor")]
            cursor_shape_device: None,
            pointer_enter_serial: None,
            keyboard: None,
            xkb: XkbState::new(),
            #[cfg(feature = "drag_drop")]
            data_device_manager,
            #[cfg(feature = "drag_drop")]
            data_device: None,
            #[cfg(feature = "drag_drop")]
            drag_offers: SmallVec::new(),
            #[cfg(feature = "drag_drop")]
            current_drag: None,
        }
    }

    pub(crate) fn window_for_surface(
        &self,
        surface: &wl_surface::WlSurface,
    ) -> Option<WindowHandle> {
        self.surface_to_window
            .iter()
            .find(|(object_id, _)| *object_id == surface.id())
            .map(|(_, handle)| *handle)
    }

    pub(crate) fn map_surface(&mut self, object_id: ObjectId, handle: WindowHandle) {
        self.unmap_surface(&object_id);
        self.surface_to_window.push((object_id, handle));
    }

    pub(crate) fn unmap_surface(&mut self, object_id: &ObjectId) -> Option<WindowHandle> {
        let index = self
            .surface_to_window
            .iter()
            .position(|(mapped_id, _)| mapped_id == object_id)?;
        Some(self.surface_to_window.swap_remove(index).1)
    }

    pub(crate) fn get_window(&self, handle: WindowHandle) -> Option<&WindowState> {
        self.windows
            .iter()
            .find(|(candidate, _)| *candidate == handle)
            .map(|(_, state)| state)
    }

    pub(crate) fn get_window_mut(&mut self, handle: WindowHandle) -> Option<&mut WindowState> {
        self.windows
            .iter_mut()
            .find(|(candidate, _)| *candidate == handle)
            .map(|(_, state)| state)
    }

    pub(crate) fn insert_window(&mut self, handle: WindowHandle, state: WindowState) {
        if let Some(window_state) = self.get_window_mut(handle) {
            *window_state = state;
            return;
        }
        self.windows.push((handle, state));
    }

    pub(crate) fn remove_window(&mut self, handle: WindowHandle) -> Option<WindowState> {
        let index = self
            .windows
            .iter()
            .position(|(candidate, _)| *candidate == handle)?;
        Some(self.windows.swap_remove(index).1)
    }

    pub(crate) fn push_window(&mut self, window: WindowHandle, event: Event) {
        self.pending_events.push(Some(window), event);
    }

    pub(crate) fn pop_event(&mut self) -> Option<(Option<WindowHandle>, Event)> {
        self.pending_events.pop()
    }

    #[cfg(feature = "drag_drop")]
    pub(crate) fn upsert_drag_offer(&mut self, offer: wl_data_offer::WlDataOffer) {
        let offer_id = offer.id();
        if let Some((_, state)) = self
            .drag_offers
            .iter_mut()
            .find(|(existing_id, _)| *existing_id == offer_id)
        {
            state.offer = offer;
            return;
        }

        self.drag_offers.push((
            offer_id,
            DragOfferState {
                offer,
                offered: SmallVec::new(),
                action: DragAction::Copy,
            },
        ));
    }

    #[cfg(feature = "drag_drop")]
    pub(crate) fn drag_offer_mut(&mut self, offer_id: &ObjectId) -> Option<&mut DragOfferState> {
        self.drag_offers
            .iter_mut()
            .find(|(existing_id, _)| existing_id == offer_id)
            .map(|(_, state)| state)
    }

    #[cfg(feature = "drag_drop")]
    pub(crate) fn drag_offer(&self, offer_id: &ObjectId) -> Option<&DragOfferState> {
        self.drag_offers
            .iter()
            .find(|(existing_id, _)| existing_id == offer_id)
            .map(|(_, state)| state)
    }

    #[cfg(feature = "drag_drop")]
    pub(crate) fn drop_drag_offer(&mut self, offer_id: &ObjectId) {
        if let Some(index) = self
            .drag_offers
            .iter()
            .position(|(existing_id, _)| existing_id == offer_id)
        {
            let (_, state) = self.drag_offers.swap_remove(index);
            state.offer.destroy();
        }
    }

    pub(crate) fn apply_cursor(&mut self, window: WindowHandle) {
        #[cfg(not(feature = "cursor"))]
        {
            let _ = window;
            return;
        }

        #[cfg(feature = "cursor")]
        {
            let Some(window_state) = self.get_window(window) else {
                return;
            };
            let Some(serial) = self.pointer_enter_serial else {
                return;
            };

            if !window_state.cursor_visible
                || matches!(window_state.cursor_mode, CursorMode::Hidden)
            {
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
}

pub(crate) type SharedDisplayRef = Rc<SharedDisplay>;

#[cfg(feature = "cursor")]
pub(crate) const fn icon_from_source(source: &CursorSource) -> CursorIcon {
    match source {
        CursorSource::Icon(icon) => *icon,
        CursorSource::Rgba { .. } => CursorIcon::Default,
    }
}

#[cfg(feature = "cursor")]
const fn map_cursor_shape(icon: CursorIcon) -> wp_cursor_shape_device_v1::Shape {
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
