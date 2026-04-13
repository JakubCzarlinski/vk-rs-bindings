use std::fmt;

use wayland_client::globals::BindError;

#[derive(Debug)]
pub enum ConnectError {
    WaylandConnect(wayland_client::ConnectError),
    Registry(wayland_client::globals::GlobalError),
    MissingGlobal(&'static str),
    Bind(BindError),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaylandConnect(err) => write!(f, "failed to connect to wayland: {err}"),
            Self::Registry(err) => write!(f, "failed to initialize the wayland registry: {err}"),
            Self::MissingGlobal(name) => write!(f, "required wayland global `{name}` is missing"),
            Self::Bind(err) => write!(f, "failed to bind a wayland global: {err}"),
        }
    }
}

impl std::error::Error for ConnectError {}

#[derive(Debug)]
pub enum PumpError {
    Wayland(wayland_client::DispatchError),
    Io(std::io::Error),
}

impl fmt::Display for PumpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wayland(err) => write!(f, "wayland dispatch failed: {err}"),
            Self::Io(err) => write!(f, "wayland socket read failed: {err}"),
        }
    }
}

impl std::error::Error for PumpError {}

#[derive(Debug)]
pub enum WindowError {
    MissingShell,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingShell => write!(f, "wayland compositor does not expose xdg_wm_base"),
        }
    }
}

impl std::error::Error for WindowError {}
