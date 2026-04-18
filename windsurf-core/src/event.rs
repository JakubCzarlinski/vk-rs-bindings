#[cfg(feature = "cursor")]
use crate::CursorMode;
use crate::{ButtonState, KeyCode, KeyState, PointerButton, TextInput};
#[cfg(feature = "drag_drop")]
use crate::{DragAction, DragData, DragPosition};
#[cfg(feature = "gamepad")]
use crate::{GamepadAxis, GamepadButton, GamepadId};
#[cfg(feature = "drag_drop")]
use alloc::string::String;
#[cfg(any(feature = "ime", feature = "drag_drop"))]
use alloc::sync::Arc;

extern crate alloc;

/// Flat event enum shared by all `windsurf` backends.
///
/// Window identity is intentionally carried out-of-band by the surrounding
/// transport tuple `(Option<WindowHandle>, Event)`.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Emitted immediately after a backend creates a new window.
    WindowCreated,
    /// Emitted when a window's logical size changes.
    WindowResized { width: u32, height: u32 },
    /// Emitted when a window's backing scale factor changes.
    ScaleFactorChanged { factor: f64 },
    /// Emitted once per close request gesture from the platform.
    CloseRequested,
    /// Emitted when a backend tears down a window.
    WindowDestroyed,
    /// Emitted when the application should redraw a window.
    RedrawRequested,
    /// Emitted when the pointer enters a window's content area.
    PointerEntered,
    /// Emitted when the pointer leaves a window's content area.
    PointerLeft,
    /// Emitted for pointer motion in logical coordinates.
    PointerMoved { x: f64, y: f64 },
    /// Emitted for normalized pointer button transitions.
    PointerButton {
        button: PointerButton,
        state: ButtonState,
    },
    /// Emitted for pointer wheel/axis deltas.
    PointerScroll { dx: f64, dy: f64 },
    /// Emitted when a window receives keyboard focus.
    KeyboardFocusIn,
    /// Emitted when a window loses keyboard focus.
    KeyboardFocusOut,
    /// Emitted for key transitions.
    Key {
        key: KeyCode,
        scancode: u16,
        state: KeyState,
    },
    /// Emitted for textual keyboard/IME output.
    TextInput { text: TextInput },
    /// Emitted for IME-specific platform notifications.
    #[cfg(feature = "ime")]
    ImeEnabled,
    #[cfg(feature = "ime")]
    ImeDisabled,
    #[cfg(feature = "ime")]
    ImePreedit {
        text: Arc<str>,
        selection: Option<(u32, u32)>,
    },
    #[cfg(feature = "ime")]
    ImeCommit { text: Arc<str> },
    /// Emitted for cursor-specific policy/state notifications.
    #[cfg(feature = "cursor")]
    CursorModeChanged { mode: CursorMode },
    #[cfg(feature = "cursor")]
    CursorVisibilityChanged { visible: bool },
    /// Emitted for drag-and-drop notifications.
    #[cfg(feature = "drag_drop")]
    DragDropEntered {
        position: DragPosition,
        /// List of MIME types offered by the drag source, in descending preference order.
        offered: Arc<[String]>,
    },
    #[cfg(feature = "drag_drop")]
    DragDropMoved { position: DragPosition },
    #[cfg(feature = "drag_drop")]
    DragDropLeft,
    #[cfg(feature = "drag_drop")]
    DragDropDropped {
        position: DragPosition,
        data: Arc<[DragData]>,
        action: DragAction,
    },
    /// Emitted for gamepad lifecycle and input notifications.
    #[cfg(feature = "gamepad")]
    GamepadConnected { gamepad: GamepadId },
    #[cfg(feature = "gamepad")]
    GamepadDisconnected { gamepad: GamepadId },
    #[cfg(feature = "gamepad")]
    GamepadButton {
        gamepad: GamepadId,
        button: GamepadButton,
        state: ButtonState,
    },
    #[cfg(feature = "gamepad")]
    GamepadAxis {
        gamepad: GamepadId,
        axis: GamepadAxis,
        value: f32,
    },
    /// Emitted when a touch contact starts.
    TouchStart { touch: TouchPoint },
    /// Emitted when a touch contact moves.
    TouchMove { touch: TouchPoint },
    /// Emitted when a touch contact ends.
    TouchEnd { finger: u8 },
    /// Emitted when a touch contact is cancelled by the platform.
    TouchCancel { finger: u8 },
    /// Emitted when the application is backgrounded/suspended.
    Suspended,
    /// Emitted when the application resumes from suspension.
    Resumed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TouchPoint {
    pub finger: u8,
    pub x: f64,
    pub y: f64,
    pub force: Option<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Window,
    Global,
}

impl Event {
    /// Expected transport scope for this event variant.
    pub const fn scope_kind(&self) -> ScopeKind {
        match self {
            #[cfg(feature = "gamepad")]
            Self::GamepadConnected { .. }
            | Self::GamepadDisconnected { .. }
            | Self::GamepadButton { .. }
            | Self::GamepadAxis { .. } => ScopeKind::Global,
            Self::Suspended | Self::Resumed => ScopeKind::Global,
            _ => ScopeKind::Window,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, ScopeKind};

    #[test]
    fn scope_kind_marks_global_events() {
        assert_eq!(Event::Suspended.scope_kind(), ScopeKind::Global);
        assert_eq!(Event::Resumed.scope_kind(), ScopeKind::Global);
    }

    #[test]
    fn scope_kind_marks_window_events() {
        assert_eq!(Event::WindowCreated.scope_kind(), ScopeKind::Window);
        assert_eq!(
            Event::PointerMoved { x: 1.0, y: 2.0 }.scope_kind(),
            ScopeKind::Window
        );
    }
}
