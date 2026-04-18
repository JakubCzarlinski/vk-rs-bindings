use crate::error::{ConnectError, PollError, WindowError};
#[cfg(feature = "cursor")]
use crate::state::icon_from_source;
use crate::state::{PumpState, SharedDisplay, SharedDisplayRef, State};
use crate::window::Window;
use alloc::rc::Rc;
use core::time::Duration;
use parking_lot::Mutex;
use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use wayland_client::Connection;
use wayland_client::globals::{BindError, GlobalList, registry_queue_init};
#[cfg(feature = "drag_drop")]
use wayland_client::protocol::wl_data_device_manager;
use wayland_client::protocol::{wl_compositor, wl_seat};
#[cfg(feature = "cursor")]
use wayland_protocols::wp::cursor_shape::v1::client::wp_cursor_shape_manager_v1;
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
#[cfg(any(feature = "ime", feature = "cursor"))]
use windsurf_core::Event;
use windsurf_core::{
    Backend, CursorMode, CursorSource, DragSource, FeatureKind, FeatureSet, ImeState, ScopedEvent,
    UnsupportedFeature, WindowAttributes, WindowHandle,
};

extern crate alloc;

pub struct WaylandBackend {
    pub(crate) shared: SharedDisplayRef,
    epoll_fd: OwnedFd,
}

pub struct RawDisplay<'a> {
    pub connection: &'a Connection,
    pub globals: &'a GlobalList,
    pub compositor: &'a wl_compositor::WlCompositor,
    pub wm_base: &'a xdg_wm_base::XdgWmBase,
}

impl WaylandBackend {
    pub fn raw_fd(&self) -> RawFd {
        self.shared.connection.as_fd().as_raw_fd()
    }

    pub fn raw_borrowed_fd(&self) -> BorrowedFd<'_> {
        self.shared.connection.as_fd()
    }

    pub fn raw(&self) -> RawDisplay<'_> {
        RawDisplay {
            connection: &self.shared.connection,
            globals: &self.shared.globals,
            compositor: &self.shared.compositor,
            wm_base: &self.shared.wm_base,
        }
    }

    fn pump_once(&mut self) -> Result<(), PollError> {
        let mut pump = self.shared.pump.lock();
        let PumpState { event_queue, state } = &mut *pump;

        let _ = event_queue.flush();
        if let Some(guard) = event_queue.prepare_read() {
            match guard.read() {
                Ok(_) => {}
                Err(wayland_backend::client::WaylandError::Io(err))
                    if err.kind() == io::ErrorKind::WouldBlock => {}
                Err(wayland_backend::client::WaylandError::Io(err)) => {
                    return Err(PollError::Io(err));
                }
                Err(err) => return Err(PollError::Wayland(err.into())),
            }
        }

        event_queue
            .dispatch_pending(state)
            .map_err(PollError::Wayland)?;

        Ok(())
    }

    fn pop_event(&mut self) -> Option<ScopedEvent> {
        let mut pump = self.shared.pump.lock();
        pump.state.pop_event()
    }

    fn wait_for_socket(&self, timeout: Option<Duration>) -> Result<bool, PollError> {
        let timeout_ms = timeout.map_or(-1, |timeout| {
            timeout.as_millis().clamp(0, i32::MAX as u128) as i32
        });
        let mut event = libc::epoll_event { events: 0, u64: 0 };

        loop {
            let ready = unsafe {
                libc::epoll_wait(self.epoll_fd.as_raw_fd(), &raw mut event, 1, timeout_ms)
            };

            if ready > 0 {
                return Ok(true);
            }
            if ready == 0 {
                return Ok(false);
            }

            let err = io::Error::last_os_error();
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            return Err(PollError::Wait(err));
        }
    }
}

impl Backend for WaylandBackend {
    type ConnectError = ConnectError;
    type PollError = PollError;
    type WindowError = WindowError;
    type BackendWindow = Window;

    fn connect() -> Result<Self, Self::ConnectError> {
        let connection = Connection::connect_to_env().map_err(ConnectError::WaylandConnect)?;
        let (globals, event_queue) =
            registry_queue_init::<State>(&connection).map_err(ConnectError::Registry)?;
        let qh = event_queue.handle();

        let compositor = globals
            .bind::<wl_compositor::WlCompositor, _, _>(&qh, 1..=6, ())
            .map_err(|err| match err {
                BindError::NotPresent => ConnectError::MissingGlobal("wl_compositor"),
                other @ BindError::UnsupportedVersion => ConnectError::Bind(other),
            })?;
        let wm_base = globals
            .bind::<xdg_wm_base::XdgWmBase, _, _>(&qh, 1..=6, ())
            .map_err(|err| match err {
                BindError::NotPresent => ConnectError::MissingGlobal("xdg_wm_base"),
                other @ BindError::UnsupportedVersion => ConnectError::Bind(other),
            })?;
        let decoration_manager = match globals
            .bind::<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, _, _>(&qh, 1..=1, ())
        {
            Ok(manager) => Some(manager),
            Err(BindError::NotPresent) => None,
            Err(other @ BindError::UnsupportedVersion) => {
                return Err(ConnectError::Bind(other));
            }
        };
        #[cfg(feature = "drag_drop")]
        let data_device_manager = match globals
            .bind::<wl_data_device_manager::WlDataDeviceManager, _, _>(&qh, 1..=3, ())
        {
            Ok(manager) => Some(manager),
            Err(BindError::NotPresent) => None,
            Err(other @ BindError::UnsupportedVersion) => {
                return Err(ConnectError::Bind(other));
            }
        };
        #[cfg(feature = "cursor")]
        let cursor_shape_manager = match globals
            .bind::<wp_cursor_shape_manager_v1::WpCursorShapeManagerV1, _, _>(&qh, 1..=1, ())
        {
            Ok(manager) => Some(manager),
            Err(BindError::NotPresent) => None,
            Err(other @ BindError::UnsupportedVersion) => {
                return Err(ConnectError::Bind(other));
            }
        };

        let mut state = State::new(
            compositor.clone(),
            #[cfg(feature = "drag_drop")]
            data_device_manager,
            #[cfg(feature = "cursor")]
            cursor_shape_manager,
        );
        for global in globals.contents().clone_list() {
            if global.interface == "wl_seat" && state.seat.is_none() {
                let version = global.version.min(7);
                let seat =
                    globals
                        .registry()
                        .bind::<wl_seat::WlSeat, _, _>(global.name, version, &qh, ());
                state.seat = Some(seat);
            }
        }

        let shared = SharedDisplay {
            connection,
            globals,
            compositor,
            wm_base,
            decoration_manager,
            pump: Mutex::new(PumpState { event_queue, state }),
        };

        let epoll_fd = create_epoll(shared.connection.as_fd().as_raw_fd())?;

        Ok(Self {
            shared: Rc::new(shared),
            epoll_fd,
        })
    }

    fn poll_event(&mut self) -> Result<Option<ScopedEvent>, Self::PollError> {
        if let Some(event) = self.pop_event() {
            return Ok(Some(event));
        }

        self.pump_once()?;
        Ok(self.pop_event())
    }

    fn wait_event(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Option<ScopedEvent>, Self::PollError> {
        if let Some(event) = self.pop_event() {
            return Ok(Some(event));
        }

        if !self.wait_for_socket(timeout)? {
            return Ok(None);
        }

        self.pump_once()?;
        Ok(self.pop_event())
    }

    fn create_window(
        &mut self,
        attrs: WindowAttributes,
    ) -> Result<(WindowHandle, Self::BackendWindow), Self::WindowError> {
        let window = Window::new(self, &attrs)?;
        Ok((window.handle(), window))
    }

    fn destroy_window(&mut self, _handle: WindowHandle, _window: &mut Self::BackendWindow) {}

    fn set_title(&mut self, _handle: WindowHandle, window: &Self::BackendWindow, title: &str) {
        window.set_title(title);
    }

    fn request_redraw(&mut self, _handle: WindowHandle, window: &Self::BackendWindow) {
        window.request_redraw();
    }

    fn inner_size(&self, _handle: WindowHandle, window: &Self::BackendWindow) -> (u32, u32) {
        window.inner_size()
    }

    fn scale_factor(&self, _handle: WindowHandle, window: &Self::BackendWindow) -> f64 {
        window.scale_factor()
    }

    fn supported_features(&self) -> FeatureSet {
        #[cfg(feature = "drag_drop")]
        {
            let mut features = FeatureSet::empty()
                .with(if cfg!(feature = "ime") {
                    FeatureSet::IME
                } else {
                    FeatureSet::empty()
                })
                .with(if cfg!(feature = "cursor") {
                    FeatureSet::CURSOR
                } else {
                    FeatureSet::empty()
                });

            if self.shared.pump.lock().state.data_device_manager.is_some() {
                features = features.with(FeatureSet::DRAG_DROP_DESTINATION);
            }
            features
        }

        #[cfg(not(feature = "drag_drop"))]
        {
            FeatureSet::empty()
                .with(if cfg!(feature = "ime") {
                    FeatureSet::IME
                } else {
                    FeatureSet::empty()
                })
                .with(if cfg!(feature = "cursor") {
                    FeatureSet::CURSOR
                } else {
                    FeatureSet::empty()
                })
        }
    }

    fn set_ime_state(
        &mut self,
        window: WindowHandle,
        state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        #[cfg(feature = "ime")]
        {
            let mut pump = self.shared.pump.lock();
            if let Some(window_state) = pump.state.get_window_mut(window)
                && window_state.ime_enabled != state.enabled
            {
                window_state.ime_enabled = state.enabled;
                if state.enabled {
                    pump.state.push_window(window, Event::ImeEnabled);
                } else {
                    pump.state.push_window(window, Event::ImeDisabled);
                }
            }
            Ok(())
        }
        #[cfg(not(feature = "ime"))]
        {
            let _ = (window, state);
            Err(UnsupportedFeature::new(FeatureKind::Ime))
        }
    }

    fn set_cursor(
        &mut self,
        window: WindowHandle,
        source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        #[cfg(feature = "cursor")]
        {
            let mut pump = self.shared.pump.lock();
            let mut emit_visibility = None;
            if let Some(window_state) = pump.state.get_window_mut(window) {
                window_state.cursor_icon = icon_from_source(source);
                emit_visibility = Some(window_state.cursor_visible);
            }
            if let Some(visible) = emit_visibility {
                pump.state
                    .push_window(window, Event::CursorVisibilityChanged { visible });
            }
            pump.state.apply_cursor(window);
            Ok(())
        }
        #[cfg(not(feature = "cursor"))]
        {
            let _ = (window, source);
            Err(UnsupportedFeature::new(FeatureKind::Cursor))
        }
    }

    fn set_cursor_mode(
        &mut self,
        window: WindowHandle,
        mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        #[cfg(feature = "cursor")]
        {
            let mut pump = self.shared.pump.lock();
            let mut mode_changed = false;
            let mut visibility_changed = None;
            if let Some(window_state) = pump.state.get_window_mut(window)
                && window_state.cursor_mode != mode
            {
                window_state.cursor_mode = mode;
                mode_changed = true;

                let visible = !matches!(mode, CursorMode::Hidden);
                if window_state.cursor_visible != visible {
                    window_state.cursor_visible = visible;
                    visibility_changed = Some(visible);
                }
            }

            if mode_changed {
                pump.state
                    .push_window(window, Event::CursorModeChanged { mode });
            }
            if let Some(visible) = visibility_changed {
                pump.state
                    .push_window(window, Event::CursorVisibilityChanged { visible });
            }
            pump.state.apply_cursor(window);
            Ok(())
        }
        #[cfg(not(feature = "cursor"))]
        {
            let _ = (window, mode);
            Err(UnsupportedFeature::new(FeatureKind::Cursor))
        }
    }

    fn start_drag(
        &mut self,
        _window: WindowHandle,
        _source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}

fn create_epoll(raw_fd: RawFd) -> Result<OwnedFd, ConnectError> {
    let epoll_raw = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
    if epoll_raw < 0 {
        return Err(ConnectError::Epoll(io::Error::last_os_error()));
    }

    let mut event = libc::epoll_event {
        events: libc::EPOLLIN as u32,
        u64: 0,
    };
    let result = unsafe { libc::epoll_ctl(epoll_raw, libc::EPOLL_CTL_ADD, raw_fd, &raw mut event) };
    if result < 0 {
        let error = io::Error::last_os_error();
        unsafe {
            libc::close(epoll_raw);
        }
        return Err(ConnectError::Epoll(error));
    }

    Ok(unsafe { OwnedFd::from_raw_fd(epoll_raw) })
}
