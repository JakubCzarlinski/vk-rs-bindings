#![doc = include_str!("../README.md")]
#![cfg_attr(
    target_os = "linux",
    doc = r"
## Wayland, XKB, and `raw-window-handle`

This backend sits on top of three separate layers that solve different
problems:

- Wayland is the transport and object model. It gives us the connection,
  registry, compositor, surfaces, seats, and XDG shell objects that are used to
  create windows and receive input.
- XKB sits on top of `wl_keyboard`. Wayland key events are just hardware-ish
  keycodes plus a compositor-provided keymap. `xkbcommon` turns that keymap into
  a state machine that can translate those keycodes into logical keys and text.
- `raw-window-handle` does not participate in event handling at all. It is only
  the escape hatch that lets renderer crates borrow the native `wl_display*` and
  `wl_surface*` pointers that already exist underneath the Wayland client types.

In practice the flow is:

1. `Display::pump` drains the Wayland socket and dispatches protocol events.
2. Protocol handlers translate those events into `windsurf_core::Event`.
3. Keyboard handlers feed Wayland keymap/modifier/key events through XKB.
4. Renderers use `raw-window-handle` to create Vulkan/EGL/other surfaces against
   the same Wayland objects.

## Performance Notes

The backend keeps the hot path simple:

- the event pump does not allocate while dispatching and forwarding events
- keyboard text extraction uses a stack buffer first and only falls back to a
  heap allocation for unusually long composed text
- compositor keymaps are memory-mapped during load instead of copied into a
  temporary `Vec`

The remaining unavoidable allocations come from textual input payloads in
`Event::TextInput { text: String, .. }`.
"
)]

#[cfg(target_os = "linux")]
mod display;
#[cfg(target_os = "linux")]
mod error;
#[cfg(target_os = "linux")]
mod input;
#[cfg(target_os = "linux")]
mod raw;
#[cfg(target_os = "linux")]
mod shell;
#[cfg(target_os = "linux")]
mod state;
#[cfg(target_os = "linux")]
mod util;
#[cfg(target_os = "linux")]
mod window;
#[cfg(target_os = "linux")]
mod xkb;

#[cfg(target_os = "linux")]
pub use crate::display::{Display, RawDisplay};
#[cfg(target_os = "linux")]
pub use crate::error::{ConnectError, PumpError, WindowError};
#[cfg(target_os = "linux")]
pub use crate::window::{RawWindow, Window};

#[cfg(not(target_os = "linux"))]
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
#[cfg(not(target_os = "linux"))]
use windsurf_core::{
    CursorMode, CursorSource, DragSource, EventQueue, FeatureKind, FeatureSet, Features, ImeState,
    UnsupportedFeature, WindowAttributes, WindowId,
};

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectError;

#[cfg(not(target_os = "linux"))]
impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl std::error::Error for ConnectError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PumpError;

#[cfg(not(target_os = "linux"))]
impl std::fmt::Display for PumpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl std::error::Error for PumpError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowError;

#[cfg(not(target_os = "linux"))]
impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl std::error::Error for WindowError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Display;

#[cfg(not(target_os = "linux"))]
pub struct RawDisplay;

#[cfg(not(target_os = "linux"))]
impl Display {
    pub fn connect() -> Result<Self, ConnectError> {
        Err(ConnectError)
    }

    pub fn pump(&self, _queue: &mut EventQueue) -> Result<(), PumpError> {
        Err(PumpError)
    }
}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Window;

#[cfg(not(target_os = "linux"))]
pub struct RawWindow;

#[cfg(not(target_os = "linux"))]
impl Window {
    pub fn new(_display: &Display, _attrs: WindowAttributes) -> Result<Self, WindowError> {
        Err(WindowError)
    }

    pub fn id(&self) -> WindowId {
        WindowId::new(0)
    }

    pub fn set_title(&self, _title: &str) {}

    pub fn inner_size(&self) -> (u32, u32) {
        (0, 0)
    }

    pub fn scale_factor(&self) -> f64 {
        1.0
    }

    pub fn request_redraw(&self) {}

    pub fn raw_window_handle(&self) -> Result<raw_window_handle::RawWindowHandle, HandleError> {
        self.window_handle().map(Into::into)
    }

    pub fn raw_display_handle(&self) -> Result<raw_window_handle::RawDisplayHandle, HandleError> {
        self.display_handle().map(Into::into)
    }
}

#[cfg(not(target_os = "linux"))]
impl raw_window_handle::HasDisplayHandle for Display {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

#[cfg(not(target_os = "linux"))]
impl raw_window_handle::HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

#[cfg(not(target_os = "linux"))]
impl raw_window_handle::HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

#[cfg(not(target_os = "linux"))]
impl Features for Display {
    fn supported_features(&self) -> FeatureSet {
        FeatureSet::empty()
    }

    fn set_ime_state(
        &self,
        _window: WindowId,
        _state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Ime))
    }

    fn set_cursor(
        &self,
        _window: WindowId,
        _source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn set_cursor_mode(
        &self,
        _window: WindowId,
        _mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn start_drag(&self, _window: WindowId, _source: DragSource) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}
