use alloc::fmt;
use core::error;

extern crate alloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Error returned by [`crate::Display::connect`].
pub enum ConnectError {
    /// The API was called from a non-main thread.
    NotMainThread,
    /// The crate was compiled on an unsupported target.
    UnsupportedTarget,
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotMainThread => f.write_str("windsurf-macos must be used from the main thread"),
            Self::UnsupportedTarget => f.write_str("windsurf-macos only supports macOS targets"),
        }
    }
}

impl error::Error for ConnectError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Error returned by [`crate::Display::pump`] and [`crate::Display::pump_extras`].
pub enum PumpError {
    /// The API was called from a non-main thread.
    NotMainThread,
    /// The crate was compiled on an unsupported target.
    UnsupportedTarget,
}

impl fmt::Display for PumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotMainThread => f.write_str("windsurf-macos pump must run on the main thread"),
            Self::UnsupportedTarget => f.write_str("windsurf-macos only supports macOS targets"),
        }
    }
}

impl error::Error for PumpError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Error returned by [`crate::Window::new`].
pub enum WindowError {
    /// The API was called from a non-main thread.
    NotMainThread,
    /// The crate was compiled on an unsupported target.
    UnsupportedTarget,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotMainThread => {
                f.write_str("windsurf-macos windows must be created on the main thread")
            }
            Self::UnsupportedTarget => f.write_str("windsurf-macos only supports macOS targets"),
        }
    }
}

impl error::Error for WindowError {}
