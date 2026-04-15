#![doc = include_str!("../README.md")]
#![no_std]
mod error;

pub use crate::error::{ConnectError, PumpError, WindowError};
#[cfg(not(target_os = "macos"))]
use windsurf_core::{
    CursorMode, CursorSource, DragSource, EventQueue, FeatureKind, FeatureSet, Features, ImeState,
    UnsupportedFeature,
};

#[cfg(target_os = "macos")]
mod app;
#[cfg(target_os = "macos")]
mod cursor;
#[cfg(target_os = "macos")]
mod display;
#[cfg(target_os = "macos")]
mod drag;
#[cfg(target_os = "macos")]
mod input;
#[cfg(target_os = "macos")]
mod keymap;
#[cfg(target_os = "macos")]
mod raw;
#[cfg(target_os = "macos")]
mod state;
#[cfg(target_os = "macos")]
mod window;

#[cfg(target_os = "macos")]
pub use crate::display::{Display, RawDisplay};
#[cfg(target_os = "macos")]
pub use crate::window::{RawWindow, Window};

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Display;

#[cfg(not(target_os = "macos"))]
impl Display {
    pub fn connect() -> Result<Self, ConnectError> {
        Err(ConnectError::UnsupportedTarget)
    }

    pub fn pump(&self, _queue: &mut EventQueue) -> Result<(), PumpError> {
        Err(PumpError::UnsupportedTarget)
    }
}

#[cfg(not(target_os = "macos"))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Window;

#[cfg(not(target_os = "macos"))]
impl Window {
    pub fn new(
        _display: &Display,
        _attributes: windsurf_core::WindowAttributes,
    ) -> Result<Self, WindowError> {
        Err(WindowError::UnsupportedTarget)
    }
}

#[cfg(not(target_os = "macos"))]
pub struct RawDisplay;

#[cfg(not(target_os = "macos"))]
pub struct RawWindow;

#[cfg(not(target_os = "macos"))]
impl Features for Display {
    fn supported_features(&self) -> FeatureSet {
        FeatureSet::empty()
    }

    fn set_ime_state(
        &self,
        _window: windsurf_core::WindowId,
        _state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Ime))
    }

    fn set_cursor(
        &self,
        _window: windsurf_core::WindowId,
        _source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn set_cursor_mode(
        &self,
        _window: windsurf_core::WindowId,
        _mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn start_drag(
        &self,
        _window: windsurf_core::WindowId,
        _source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}
