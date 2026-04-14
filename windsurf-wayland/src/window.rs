use std::rc::Rc;
use std::sync::atomic::Ordering;

use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use wayland_client::Proxy;
use wayland_client::protocol::wl_surface;
use wayland_protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1;
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel};
use windsurf_core::{Event, WindowAttributes, WindowId};

use crate::display::Display;
use crate::error::WindowError;
use crate::shell::update_opaque_region;
use crate::state::{SharedDisplayRef, WindowState};

#[derive(Clone)]
pub struct Window {
    pub(crate) shared: SharedDisplayRef,
    pub(crate) id: WindowId,
    pub(crate) surface: wl_surface::WlSurface,
    pub(crate) xdg_surface: xdg_surface::XdgSurface,
    pub(crate) toplevel: xdg_toplevel::XdgToplevel,
    pub(crate) decoration: Option<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
}

pub struct RawWindow<'a> {
    pub surface: &'a wl_surface::WlSurface,
    pub xdg_surface: &'a xdg_surface::XdgSurface,
    pub toplevel: &'a xdg_toplevel::XdgToplevel,
}

impl Window {
    pub fn new(display: &Display, attrs: &WindowAttributes) -> Result<Self, WindowError> {
        let id = WindowId::new(
            display
                .shared
                .next_window_id
                .fetch_add(1, Ordering::Relaxed),
        );
        let mut pump = display.shared.pump.lock().unwrap();
        let qh = pump.event_queue.handle();

        let surface = display.shared.compositor.create_surface(&qh, ());
        let xdg_surface = display.shared.wm_base.get_xdg_surface(&surface, &qh, id);
        let toplevel = xdg_surface.get_toplevel(&qh, id);
        let decoration = display
            .shared
            .decoration_manager
            .as_ref()
            .map(|manager| manager.get_toplevel_decoration(&toplevel, &qh, id));

        toplevel.set_title(attrs.title.clone());
        if let Some(min_size) = attrs.min_size {
            toplevel.set_min_size(min_size.width as i32, min_size.height as i32);
        }
        if let Some(max_size) = attrs.max_size {
            toplevel.set_max_size(max_size.width as i32, max_size.height as i32);
        }
        if let Some(decoration) = decoration.as_ref() {
            let mode = if attrs.decorations {
                zxdg_toplevel_decoration_v1::Mode::ServerSide
            } else {
                zxdg_toplevel_decoration_v1::Mode::ClientSide
            };
            decoration.set_mode(mode);
        }
        update_opaque_region(
            &display.shared.compositor,
            &surface,
            attrs.transparent,
            attrs.size,
            &qh,
        );

        surface.commit();

        pump.state.surface_to_window.insert(surface.id(), id);
        pump.state.windows.insert(
            id,
            WindowState {
                surface: surface.clone(),
                size: attrs.size,
                scale_factor: 1.0,
                needs_redraw: true,
                transparent: attrs.transparent,
            },
        );
        pump.state.push(Event::WindowCreated { id });
        pump.state.push(Event::RedrawRequested { id });

        Ok(Self {
            shared: Rc::clone(&display.shared),
            id,
            surface,
            xdg_surface,
            toplevel,
            decoration,
        })
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn set_title(&self, title: &str) {
        self.toplevel.set_title(String::from(title));
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let pump = self.shared.pump.lock().unwrap();
        let size = pump
            .state
            .windows
            .get(&self.id)
            .map(|window| window.size)
            .unwrap_or_default();
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f64 {
        let pump = self.shared.pump.lock().unwrap();
        pump.state
            .windows
            .get(&self.id)
            .map_or(1.0, |window| window.scale_factor)
    }

    pub fn request_redraw(&self) {
        let mut pump = self.shared.pump.lock().unwrap();
        if let Some(window) = pump.state.windows.get_mut(&self.id)
            && !window.needs_redraw
        {
            window.needs_redraw = true;
            pump.state.push(Event::RedrawRequested { id: self.id });
        }
    }

    pub fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError> {
        self.window_handle().map(Into::into)
    }

    pub fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError> {
        self.display_handle().map(Into::into)
    }

    pub fn raw(&self) -> RawWindow<'_> {
        RawWindow {
            surface: &self.surface,
            xdg_surface: &self.xdg_surface,
            toplevel: &self.toplevel,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let Ok(mut pump) = self.shared.pump.lock() else {
            return;
        };

        if pump.state.windows.remove(&self.id).is_some() {
            pump.state.surface_to_window.remove(&self.surface.id());
            if pump.state.pointer_focus == Some(self.id) {
                pump.state.pointer_focus = None;
            }
            if pump.state.keyboard_focus == Some(self.id) {
                pump.state.keyboard_focus = None;
            }
            pump.state.push(Event::WindowDestroyed { id: self.id });
        }

        if let Some(decoration) = self.decoration.take() {
            decoration.destroy();
        }
        self.toplevel.destroy();
        self.xdg_surface.destroy();
        self.surface.destroy();
    }
}
