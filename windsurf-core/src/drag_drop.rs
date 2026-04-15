use crate::WindowId;
use alloc::string::String;
use alloc::vec::Vec;

extern crate alloc;

/// Requested drag-and-drop action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragAction {
    Copy,
    Move,
    Link,
}

/// Logical drag position in window coordinates.
///
/// Uses `f32` to keep drag payload events compact.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragPosition {
    pub x: f32,
    pub y: f32,
}

impl DragPosition {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Transfer payload accepted by the drag-and-drop abstraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DragData {
    Text(String),
    /// UTF-8 file paths as provided by the backend.
    Files(Vec<String>),
    Bytes {
        mime_type: String,
        data: Vec<u8>,
    },
}

/// Drag payload initiated by the local application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragSource {
    pub items: Vec<DragData>,
    pub action: DragAction,
}

/// Drag-and-drop events emitted by a backend.
#[derive(Debug, Clone, PartialEq)]
pub enum DragDropEvent {
    Entered {
        id: WindowId,
        position: DragPosition,
        offered: Vec<String>,
    },
    Moved {
        id: WindowId,
        position: DragPosition,
    },
    Left {
        id: WindowId,
    },
    Dropped {
        id: WindowId,
        position: DragPosition,
        data: Vec<DragData>,
        action: DragAction,
    },
}

impl DragDropEvent {
    pub const fn window_id(&self) -> WindowId {
        match self {
            Self::Entered { id, .. }
            | Self::Moved { id, .. }
            | Self::Left { id }
            | Self::Dropped { id, .. } => *id,
        }
    }
}
