use crate::{CursorEvent, DragDropEvent, GamepadEvent, ImeEvent};

/// Optional event stream layered alongside `windsurf_core::Event`.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtraEvent {
    Ime(ImeEvent),
    Cursor(CursorEvent),
    DragDrop(DragDropEvent),
    Gamepad(GamepadEvent),
}
