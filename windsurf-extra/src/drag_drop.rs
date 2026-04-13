use std::path::PathBuf;

use windsurf_core::{LogicalPosition, WindowId};

/// Requested drag-and-drop action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragAction {
    Copy,
    Move,
    Link,
}

/// Transfer payload accepted by the drag-and-drop abstraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DragData {
    Text(String),
    Files(Vec<PathBuf>),
    Bytes { mime_type: String, data: Vec<u8> },
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
        position: LogicalPosition,
        offered: Vec<String>,
    },
    Moved {
        id: WindowId,
        position: LogicalPosition,
    },
    Left {
        id: WindowId,
    },
    Dropped {
        id: WindowId,
        position: LogicalPosition,
        data: Vec<DragData>,
        action: DragAction,
    },
}
