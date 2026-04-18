use crate::display::WaylandBackend;
use crate::error::WindowError;
use crate::ext_background_effect::ext_background_effect_surface_v1;
use crate::shell::update_opaque_region;
use crate::state::{SharedDisplayRef, WindowState};
use alloc::rc::Rc;
use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use wayland_client::Proxy;
use wayland_client::protocol::wl_surface;
use wayland_protocols::xdg::decoration::zv1::client::zxdg_toplevel_decoration_v1;
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel};
use wayland_protocols_plasma::blur::client::org_kde_kwin_blur;
#[cfg(feature = "cursor")]
use windsurf_core::{CursorIcon, CursorMode};
use windsurf_core::{Event, WindowAttributes, WindowHandle};

extern crate alloc;

#[derive(Clone)]
pub struct Window {
    pub(crate) shared: SharedDisplayRef,
    pub(crate) handle: WindowHandle,
    pub(crate) surface: wl_surface::WlSurface,
    pub(crate) xdg_surface: xdg_surface::XdgSurface,
    pub(crate) toplevel: xdg_toplevel::XdgToplevel,
    pub(crate) decoration: Option<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>,
    pub(crate) blur: Option<BlurObject>,
}

#[derive(Clone)]
pub(crate) enum BlurObject {
    Ext(ext_background_effect_surface_v1::ExtBackgroundEffectSurfaceV1),
    Kde(org_kde_kwin_blur::OrgKdeKwinBlur),
}

pub struct RawWindow<'a> {
    pub surface: &'a wl_surface::WlSurface,
    pub xdg_surface: &'a xdg_surface::XdgSurface,
    pub toplevel: &'a xdg_toplevel::XdgToplevel,
}

impl Window {
    pub fn new(backend: &WaylandBackend, attrs: &WindowAttributes) -> Result<Self, WindowError> {
        let mut pump = backend.shared.pump.lock();
        let handle = pump
            .state
            .handles
            .allocate()
            .ok_or(WindowError::NoAvailableWindowHandle)?;
        let qh = pump.event_queue.handle();

        let surface = backend.shared.compositor.create_surface(&qh, ());
        let xdg_surface = backend
            .shared
            .wm_base
            .get_xdg_surface(&surface, &qh, handle);
        let toplevel = xdg_surface.get_toplevel(&qh, handle);
        let decoration = backend
            .shared
            .decoration_manager
            .as_ref()
            .map(|manager| manager.get_toplevel_decoration(&toplevel, &qh, handle));

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
            &backend.shared.compositor,
            &surface,
            attrs.transparent,
            attrs.size,
            &qh,
        );
        let blur = if attrs.transparent {
            if let Some(manager) = backend.shared.ext_blur_manager.as_ref() {
                if pump.state.ext_blur_capable {
                    let effect = manager.get_background_effect(&surface, &qh, ());
                    effect.set_blur_region(None);
                    Some(BlurObject::Ext(effect))
                } else if let Some(manager) = backend.shared.kde_blur_manager.as_ref() {
                    let blur = manager.create(&surface, &qh, ());
                    blur.set_region(None);
                    blur.commit();
                    Some(BlurObject::Kde(blur))
                } else {
                    None
                }
            } else if let Some(manager) = backend.shared.kde_blur_manager.as_ref() {
                let blur = manager.create(&surface, &qh, ());
                blur.set_region(None);
                blur.commit();
                Some(BlurObject::Kde(blur))
            } else {
                None
            }
        } else {
            None
        };

        surface.commit();

        pump.state.map_surface(surface.id(), handle);
        pump.state.insert_window(
            handle,
            WindowState {
                surface: surface.clone(),
                size: attrs.size,
                scale_factor: 1.0,
                needs_redraw: true,
                transparent: attrs.transparent,
                #[cfg(feature = "ime")]
                ime_enabled: false,
                #[cfg(feature = "cursor")]
                cursor_mode: CursorMode::Normal,
                #[cfg(feature = "cursor")]
                cursor_visible: true,
                #[cfg(feature = "cursor")]
                cursor_icon: CursorIcon::Default,
            },
        );
        pump.state.push_window(handle, Event::WindowCreated);
        pump.state.push_window(handle, Event::RedrawRequested);

        Ok(Self {
            shared: Rc::clone(&backend.shared),
            handle,
            surface,
            xdg_surface,
            toplevel,
            decoration,
            blur,
        })
    }

    pub fn handle(&self) -> WindowHandle {
        self.handle
    }

    pub fn set_title(&self, title: &str) {
        self.toplevel.set_title(String::from(title));
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let pump = self.shared.pump.lock();
        let size = pump
            .state
            .get_window(self.handle)
            .map(|window| window.size)
            .unwrap_or_default();
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f64 {
        let pump = self.shared.pump.lock();
        pump.state
            .get_window(self.handle)
            .map_or(1.0, |window| window.scale_factor)
    }

    pub fn request_redraw(&self) {
        let mut pump = self.shared.pump.lock();
        if let Some(window) = pump.state.get_window_mut(self.handle)
            && !window.needs_redraw
        {
            window.needs_redraw = true;
            pump.state.push_window(self.handle, Event::RedrawRequested);
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
        let mut pump = self.shared.pump.lock();

        if pump.state.remove_window(self.handle).is_some() {
            let surface_id = self.surface.id();
            let _ = pump.state.unmap_surface(&surface_id);
            if pump.state.pointer_focus == Some(self.handle) {
                pump.state.pointer_focus = None;
            }
            if pump.state.keyboard_focus == Some(self.handle) {
                pump.state.keyboard_focus = None;
            }
            pump.state.handles.release(self.handle);
            pump.state.push_window(self.handle, Event::WindowDestroyed);
        }

        if let Some(decoration) = self.decoration.take() {
            decoration.destroy();
        }
        if let Some(blur) = self.blur.take() {
            match blur {
                BlurObject::Ext(effect) => effect.destroy(),
                BlurObject::Kde(effect) => effect.release(),
            }
        }
        self.toplevel.destroy();
        self.xdg_surface.destroy();
        self.surface.destroy();
    }
}
