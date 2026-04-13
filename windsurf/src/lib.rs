#![doc = include_str!("../README.md")]

pub use windsurf_core::*;

#[cfg(feature = "extras")]
pub use windsurf_extra::*;

#[cfg(feature = "wayland")]
pub use windsurf_wayland::*;

#[cfg(feature = "extras")]
pub mod extras {
    pub use windsurf_extra::*;
}

#[cfg(feature = "wayland")]
pub mod wayland {
    pub use windsurf_wayland::*;
}
