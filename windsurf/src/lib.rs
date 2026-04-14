#![doc = include_str!("../README.md")]

pub use windsurf_core::*;

#[cfg(feature = "extras")]
pub use windsurf_extra::*;

#[cfg(all(feature = "macos", target_os = "macos"))]
pub use windsurf_macos::{
    ConnectError, Display, PumpError, RawDisplay, RawWindow, Window, WindowError,
};

#[cfg(all(feature = "wayland", target_os = "linux"))]
pub use windsurf_wayland::{
    ConnectError, Display, PumpError, RawDisplay, RawWindow, Window, WindowError,
};

#[cfg(feature = "extras")]
pub mod extras {
    pub use windsurf_extra::*;
}

#[cfg(feature = "macos")]
pub mod macos {
    pub use windsurf_macos::*;
}

#[cfg(feature = "wayland")]
pub mod wayland {
    pub use windsurf_wayland::*;
}
