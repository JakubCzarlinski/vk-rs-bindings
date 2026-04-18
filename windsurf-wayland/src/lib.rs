#![doc = include_str!("../README.md")]

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
pub use crate::display::{RawDisplay, WaylandBackend};
#[cfg(target_os = "linux")]
pub use crate::error::{ConnectError, PollError, WindowError};
#[cfg(target_os = "linux")]
pub use crate::window::{RawWindow, Window};

#[cfg(not(target_os = "linux"))]
use core::time::Duration;
#[cfg(not(target_os = "linux"))]
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
#[cfg(not(target_os = "linux"))]
use windsurf_core::{
    CursorMode, CursorSource, DragSource, Event, FeatureKind, FeatureSet, ImeState, LoopBackend,
    ScopedEvent, UnsupportedFeature, WindowAttributes, WindowHandle as CoreWindowHandle,
};

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectError;

#[cfg(not(target_os = "linux"))]
impl core::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl core::error::Error for ConnectError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollError;

#[cfg(not(target_os = "linux"))]
impl core::fmt::Display for PollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl core::error::Error for PollError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowError;

#[cfg(not(target_os = "linux"))]
impl core::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("windsurf-wayland only supports Linux targets")
    }
}

#[cfg(not(target_os = "linux"))]
impl core::error::Error for WindowError {}

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Default)]
pub struct WaylandBackend;

#[cfg(not(target_os = "linux"))]
#[derive(Debug, Default)]
pub struct Window;

#[cfg(not(target_os = "linux"))]
pub struct RawDisplay;

#[cfg(not(target_os = "linux"))]
pub struct RawWindow;

#[cfg(not(target_os = "linux"))]
impl LoopBackend for WaylandBackend {
    type ConnectError = ConnectError;
    type PollError = PollError;
    type WindowError = WindowError;
    type BackendWindow = Window;

    fn connect() -> Result<Self, Self::ConnectError> {
        Err(ConnectError)
    }

    fn poll_event(&mut self) -> Result<Option<ScopedEvent>, Self::PollError> {
        Err(PollError)
    }

    fn wait_event(
        &mut self,
        _timeout: Option<Duration>,
    ) -> Result<Option<ScopedEvent>, Self::PollError> {
        Err(PollError)
    }

    fn create_window(
        &mut self,
        _attrs: WindowAttributes,
    ) -> Result<(CoreWindowHandle, Self::BackendWindow), Self::WindowError> {
        Err(WindowError)
    }

    fn destroy_window(&mut self, _handle: CoreWindowHandle, _window: &mut Self::BackendWindow) {}

    fn set_title(
        &mut self,
        _handle: CoreWindowHandle,
        _window: &Self::BackendWindow,
        _title: &str,
    ) {
    }

    fn request_redraw(&mut self, _handle: CoreWindowHandle, _window: &Self::BackendWindow) {}

    fn inner_size(&self, _handle: CoreWindowHandle, _window: &Self::BackendWindow) -> (u32, u32) {
        (0, 0)
    }

    fn scale_factor(&self, _handle: CoreWindowHandle, _window: &Self::BackendWindow) -> f64 {
        1.0
    }

    fn supported_features(&self) -> FeatureSet {
        FeatureSet::empty()
    }

    fn set_ime_state(
        &mut self,
        _window: CoreWindowHandle,
        _state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Ime))
    }

    fn set_cursor(
        &mut self,
        _window: CoreWindowHandle,
        _source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn set_cursor_mode(
        &mut self,
        _window: CoreWindowHandle,
        _mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn start_drag(
        &mut self,
        _window: CoreWindowHandle,
        _source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}

#[cfg(not(target_os = "linux"))]
impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

#[cfg(not(target_os = "linux"))]
impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}

#[cfg(not(target_os = "linux"))]
impl HasDisplayHandle for WaylandBackend {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::Unavailable)
    }
}
