use crate::{
    ButtonState, CursorEvent, DragAction, DragData, DragPosition, GamepadAxis, GamepadButton,
    GamepadId, KeyCode, KeyState, PointerButton, TextInput, WindowId,
};
use alloc::string::String;
use alloc::sync::Arc;
extern crate alloc;

/// Flat event enum shared by all `windsurf` backends.
///
/// The design avoids nested, callback-oriented event trees. Backends translate
/// OS messages into this enum and push them into an [`crate::EventQueue`].
///
/// # Ordering
///
/// Backends should preserve platform delivery order as closely as possible
/// while still normalizing events.
///
/// # Keyboard Semantics
///
/// [`Self::Key`] carries logical key transitions, while [`Self::TextInput`]
/// carries textual output (including IME composition commits). Consumers should
/// not assume a strict 1:1 relationship between the two.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Emitted immediately after a backend creates a new window.
    WindowCreated { id: WindowId },
    /// Emitted when a window's logical size changes.
    WindowResized {
        id: WindowId,
        width: u32,
        height: u32,
    },
    /// Emitted when a window's backing scale factor changes.
    ScaleFactorChanged { id: WindowId, factor: f64 },
    /// Emitted once per close request gesture from the platform.
    CloseRequested { id: WindowId },
    /// Emitted when a backend tears down a window.
    WindowDestroyed { id: WindowId },
    /// Emitted when the application should redraw a window.
    RedrawRequested { id: WindowId },
    /// Emitted when the pointer enters a window's content area.
    PointerEntered { id: WindowId },
    /// Emitted when the pointer leaves a window's content area.
    PointerLeft { id: WindowId },
    /// Emitted for pointer motion in logical coordinates.
    PointerMoved { id: WindowId, x: f64, y: f64 },
    /// Emitted for normalized pointer button transitions.
    PointerButton {
        id: WindowId,
        button: PointerButton,
        state: ButtonState,
    },
    /// Emitted for pointer wheel/axis deltas.
    PointerScroll { id: WindowId, dx: f64, dy: f64 },
    /// Emitted when a window receives keyboard focus.
    KeyboardFocusIn { id: WindowId },
    /// Emitted when a window loses keyboard focus.
    KeyboardFocusOut { id: WindowId },
    /// Emitted for key transitions.
    Key {
        id: WindowId,
        key: KeyCode,
        scancode: u16,
        state: KeyState,
    },
    /// Emitted for textual keyboard/IME output.
    ///
    /// A backend may emit this independently from [`Self::Key`], especially for
    /// compose/IME flows where text does not map 1:1 to key transitions.
    TextInput { id: WindowId, text: TextInput },
    /// Emitted for IME-specific platform notifications.
    #[cfg(feature = "ime")]
    ImeEnabled { id: WindowId },
    #[cfg(feature = "ime")]
    ImeDisabled { id: WindowId },
    #[cfg(feature = "ime")]
    ImePreedit {
        id: WindowId,
        text: Arc<str>,
        selection: Option<(u32, u32)>,
    },
    #[cfg(feature = "ime")]
    ImeCommit { id: WindowId, text: Arc<str> },
    /// Emitted for cursor-specific platform notifications.
    #[cfg(feature = "cursor")]
    Cursor(CursorEvent),
    /// Emitted for drag-and-drop notifications.
    #[cfg(feature = "drag_drop")]
    /// Emitted when a drag enters a window's bounds.
    DragDropEntered {
        id: WindowId,
        position: DragPosition,
        /// List of MIME types offered by the drag source, in descending preference order.
        offered: Arc<[String]>,
    },
    #[cfg(feature = "drag_drop")]
    DragDropMoved {
        id: WindowId,
        position: DragPosition,
    },
    #[cfg(feature = "drag_drop")]
    DragDropLeft { id: WindowId },
    #[cfg(feature = "drag_drop")]
    DragDropDropped {
        id: WindowId,
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
    TouchStart { id: WindowId, touch: TouchPoint },
    /// Emitted when a touch contact moves.
    TouchMove { id: WindowId, touch: TouchPoint },
    /// Emitted when a touch contact ends.
    TouchEnd { id: WindowId, finger: u8 },
    /// Emitted when a touch contact is cancelled by the platform.
    TouchCancel { id: WindowId, finger: u8 },
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

impl Event {
    /// Return the window targeted by this event when applicable.
    ///
    /// Global lifecycle events (for example [`Self::Suspended`]) return
    /// `None`.
    pub const fn window_id(&self) -> Option<WindowId> {
        match self {
            Self::WindowCreated { id }
            | Self::WindowResized { id, .. }
            | Self::ScaleFactorChanged { id, .. }
            | Self::CloseRequested { id }
            | Self::WindowDestroyed { id }
            | Self::RedrawRequested { id }
            | Self::PointerEntered { id }
            | Self::PointerLeft { id }
            | Self::PointerMoved { id, .. }
            | Self::PointerButton { id, .. }
            | Self::PointerScroll { id, .. }
            | Self::KeyboardFocusIn { id }
            | Self::KeyboardFocusOut { id }
            | Self::Key { id, .. }
            | Self::TextInput { id, .. }
            | Self::TouchStart { id, .. }
            | Self::TouchMove { id, .. }
            | Self::TouchEnd { id, .. }
            | Self::TouchCancel { id, .. } => Some(*id),
            #[cfg(feature = "ime")]
            Self::ImeEnabled { id }
            | Self::ImeDisabled { id }
            | Self::ImePreedit { id, .. }
            | Self::ImeCommit { id, .. } => Some(*id),
            #[cfg(feature = "cursor")]
            Self::Cursor(event) => Some(event.window_id()),
            #[cfg(feature = "drag_drop")]
            Self::DragDropEntered { id, .. }
            | Self::DragDropMoved { id, .. }
            | Self::DragDropLeft { id }
            | Self::DragDropDropped { id, .. } => Some(*id),
            #[cfg(feature = "gamepad")]
            Self::GamepadConnected { .. }
            | Self::GamepadDisconnected { .. }
            | Self::GamepadButton { .. }
            | Self::GamepadAxis { .. } => None,
            Self::Suspended | Self::Resumed => None,
        }
    }
}
