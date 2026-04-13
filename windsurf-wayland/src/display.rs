use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
use std::rc::Rc;
use std::sync::atomic::AtomicU64;

use wayland_client::Connection;
use wayland_client::globals::{BindError, GlobalList, registry_queue_init};
use wayland_client::protocol::{wl_compositor, wl_seat};
use wayland_protocols::xdg::decoration::zv1::client::zxdg_decoration_manager_v1;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
use windsurf_core::EventQueue;

use crate::error::{ConnectError, PumpError};
use crate::state::{PumpState, SharedDisplay, SharedDisplayRef, State};

#[derive(Clone)]
pub struct Display {
    pub(crate) shared: SharedDisplayRef,
}

pub struct RawDisplay<'a> {
    pub connection: &'a Connection,
    pub globals: &'a GlobalList,
    pub compositor: &'a wl_compositor::WlCompositor,
    pub wm_base: &'a xdg_wm_base::XdgWmBase,
}

impl Display {
    pub fn connect() -> Result<Self, ConnectError> {
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

        let mut state = State::new(compositor.clone());
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
            next_window_id: AtomicU64::new(1),
            pump: std::sync::Mutex::new(PumpState { event_queue, state }),
        };

        Ok(Self {
            shared: Rc::new(shared),
        })
    }

    pub fn raw_fd(&self) -> RawFd {
        self.shared.connection.as_fd().as_raw_fd()
    }

    pub fn raw_borrowed_fd(&self) -> BorrowedFd<'_> {
        self.shared.connection.as_fd()
    }

    pub fn pump(&self, queue: &mut EventQueue) -> Result<(), PumpError> {
        let mut pump = self.shared.pump.lock().unwrap();
        let PumpState { event_queue, state } = &mut *pump;

        let _ = event_queue.flush();
        if let Some(guard) = event_queue.prepare_read() {
            match guard.read() {
                Ok(_) => {}
                Err(wayland_backend::client::WaylandError::Io(err))
                    if err.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(wayland_backend::client::WaylandError::Io(err)) => {
                    return Err(PumpError::Io(err));
                }
                Err(err) => return Err(PumpError::Wayland(err.into())),
            }
        }

        event_queue
            .dispatch_pending(state)
            .map_err(PumpError::Wayland)?;

        while let Some(event) = state.pending_events.pop_front() {
            queue.push(event);
        }

        Ok(())
    }

    pub fn raw(&self) -> RawDisplay<'_> {
        RawDisplay {
            connection: &self.shared.connection,
            globals: &self.shared.globals,
            compositor: &self.shared.compositor,
            wm_base: &self.shared.wm_base,
        }
    }
}
