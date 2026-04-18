use alloc::fmt;
use wayland_client::globals::BindError;

extern crate alloc;

#[derive(Debug)]
pub enum ConnectError {
    WaylandConnect(wayland_client::ConnectError),
    Registry(wayland_client::globals::GlobalError),
    MissingGlobal(&'static str),
    Bind(BindError),
    Epoll(std::io::Error),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WaylandConnect(err) => write!(f, "failed to connect to wayland: {err}"),
            Self::Registry(err) => write!(f, "failed to initialize the wayland registry: {err}"),
            Self::MissingGlobal(name) => write!(f, "required wayland global `{name}` is missing"),
            Self::Bind(err) => write!(f, "failed to bind a wayland global: {err}"),
            Self::Epoll(err) => write!(f, "failed to initialize epoll wait path: {err}"),
        }
    }
}

impl core::error::Error for ConnectError {}

#[derive(Debug)]
pub enum PollError {
    Wayland(wayland_client::DispatchError),
    Io(std::io::Error),
    Wait(std::io::Error),
    QueueOverflow { dropped: usize },
}

impl fmt::Display for PollError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wayland(err) => write!(f, "wayland dispatch failed: {err}"),
            Self::Io(err) => write!(f, "wayland socket read failed: {err}"),
            Self::Wait(err) => write!(f, "wayland wait failed: {err}"),
            Self::QueueOverflow { dropped } => {
                write!(f, "event queue overflowed; dropped {dropped} events")
            }
        }
    }
}

impl core::error::Error for PollError {}

#[derive(Debug)]
pub enum WindowError {
    MissingShell,
    NoAvailableWindowHandle,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingShell => write!(f, "wayland compositor does not expose xdg_wm_base"),
            Self::NoAvailableWindowHandle => {
                write!(f, "maximum concurrent window count (255) reached")
            }
        }
    }
}

impl core::error::Error for WindowError {}
