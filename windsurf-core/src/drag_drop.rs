//! Drag-and-drop related types and traits.
use alloc::string::String;
use alloc::sync::Arc;

extern crate alloc;

/// Requested drag-and-drop action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragAction {
    Copy,
    Move,
    Link,
}

/// Logical drag position in window coordinates.
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
    /// A UTF-8 string, typically from a text selection or input field.
    Text(Arc<str>),
    /// A list of file paths.
    Files(Arc<[String]>),
    /// Arbitrary binary data with an associated MIME type.
    Bytes {
        mime_type: Arc<str>,
        data: Arc<[u8]>,
    },
}

/// Drag payload initiated by the local application.
#[derive(Debug, Clone)]
pub struct DragSource {
    pub items: Arc<[DragData]>,
    pub action: DragAction,
}
