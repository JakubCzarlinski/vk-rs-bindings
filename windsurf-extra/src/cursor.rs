use alloc::vec::Vec;
use windsurf_core::WindowId;

extern crate alloc;

/// Stock cursor icon mapped to the platform's native cursor set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Pointer,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    Grab,
    Grabbing,
    Move,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    ZoomIn,
    ZoomOut,
}

/// Cursor visibility or pointer-capture mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorMode {
    Normal,
    Hidden,
    Confined,
    Locked,
}

/// Cursor source chosen by the application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorSource {
    Icon(CursorIcon),
    Rgba {
        width: u16,
        height: u16,
        hotspot_x: u16,
        hotspot_y: u16,
        pixels: Vec<u8>,
    },
}

/// Cursor-specific backend events.
#[derive(Debug, Clone, PartialEq)]
pub enum CursorEvent {
    Moved { id: WindowId, x: f64, y: f64 },
    ModeChanged { id: WindowId, mode: CursorMode },
    VisibilityChanged { id: WindowId, visible: bool },
}
