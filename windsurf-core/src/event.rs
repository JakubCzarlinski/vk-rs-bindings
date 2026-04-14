use crate::{ButtonState, KeyCode, KeyState, PointerButton, TextInput, WindowId};

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
        scancode: u32,
        state: KeyState,
    },
    /// Emitted for textual keyboard/IME output.
    ///
    /// A backend may emit this independently from [`Self::Key`], especially for
    /// compose/IME flows where text does not map 1:1 to key transitions.
    TextInput { id: WindowId, text: TextInput },
    /// Emitted when a touch contact starts.
    TouchStart {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    /// Emitted when a touch contact moves.
    TouchMove {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    /// Emitted when a touch contact ends.
    TouchEnd {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    /// Emitted when a touch contact is cancelled by the platform.
    TouchCancel { id: WindowId, finger: u64 },
    /// Emitted when the application is backgrounded/suspended.
    Suspended,
    /// Emitted when the application resumes from suspension.
    Resumed,
}
