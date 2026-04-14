#![doc = include_str!("../README.md")]

mod error;

pub use crate::error::{ConnectError, PumpError, WindowError};

#[cfg(target_os = "macos")]
mod display;
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
