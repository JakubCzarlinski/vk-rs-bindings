use core::error;

use alloc::fmt;

extern crate alloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectError {
    NotMainThread,
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
pub enum PumpError {
    NotMainThread,
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
pub enum WindowError {
    NotMainThread,
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
