use wayland_client::globals::GlobalListContents;
use wayland_client::protocol::{wl_compositor, wl_region, wl_registry, wl_seat, wl_surface};
use wayland_client::{Connection, Dispatch, QueueHandle, delegate_noop};
use wayland_protocols::xdg::decoration::zv1::client::{
    zxdg_decoration_manager_v1, zxdg_toplevel_decoration_v1,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};
use windsurf_core::{Event, LogicalSize, WindowId};

use crate::state::State;
use crate::util::{logical_size_to_i32, nonzero_or};

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
            && interface == "wl_seat"
            && state.seat.is_none()
        {
            let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, version.min(7), qh, ());
            state.seat = Some(seat);
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for State {
    fn event(
        _state: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, WindowId> for State {
    fn event(
        state: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        window_id: &WindowId,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            xdg_surface.ack_configure(serial);
            if let Some(window) = state.windows.get_mut(window_id) {
                window.needs_redraw = false;
            }
            state.push(Event::RedrawRequested { id: *window_id });
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, WindowId> for State {
    fn event(
        state: &mut Self,
        _toplevel: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        window_id: &WindowId,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        match event {
            xdg_toplevel::Event::Configure { width, height, .. } => {
                if let Some(window) = state.windows.get_mut(window_id) {
                    let width = nonzero_or(window.size.width, width as u32);
                    let height = nonzero_or(window.size.height, height as u32);

                    if window.size.width != width || window.size.height != height {
                        window.size = LogicalSize::new(width, height);
                        update_opaque_region(
                            &state.compositor,
                            &window.surface,
                            window.transparent,
                            window.size,
                            qh,
                        );
                        state.push(Event::WindowResized {
                            id: *window_id,
                            width,
                            height,
                        });
                    }
                }
            }
            xdg_toplevel::Event::Close => {
                state.push(Event::CloseRequested { id: *window_id });
            }
            _ => {}
        }
    }
}

impl Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, ()> for State {
    fn event(
        _state: &mut Self,
        _manager: &zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
        _event: zxdg_decoration_manager_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, WindowId> for State {
    fn event(
        _state: &mut Self,
        _decoration: &zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
        _event: zxdg_toplevel_decoration_v1::Event,
        _window_id: &WindowId,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

delegate_noop!(State: ignore wl_compositor::WlCompositor);
delegate_noop!(State: ignore wl_region::WlRegion);
delegate_noop!(State: ignore wl_surface::WlSurface);

pub(crate) fn update_opaque_region(
    compositor: &wl_compositor::WlCompositor,
    surface: &wl_surface::WlSurface,
    transparent: bool,
    size: LogicalSize,
    qh: &QueueHandle<State>,
) {
    if transparent || size.width == 0 || size.height == 0 {
        surface.set_opaque_region(None);
        return;
    }

    let region = compositor.create_region(qh, ());
    region.add(
        0,
        0,
        logical_size_to_i32(size.width),
        logical_size_to_i32(size.height),
    );
    surface.set_opaque_region(Some(&region));
    region.destroy();
}
