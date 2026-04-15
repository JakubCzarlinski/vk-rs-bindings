#[cfg(feature = "cursor")]
use crate::CursorEvent;
#[cfg(feature = "drag_drop")]
use crate::DragDropEvent;
#[cfg(feature = "gamepad")]
use crate::GamepadEvent;
#[cfg(feature = "ime")]
use crate::ImeEvent;
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
    /// Emitted for IME-specific platform notifications.
    #[cfg(feature = "ime")]
    Ime(ImeEvent),
    /// Emitted for cursor-specific platform notifications.
    #[cfg(feature = "cursor")]
    Cursor(CursorEvent),
    /// Emitted for drag-and-drop notifications.
    #[cfg(feature = "drag_drop")]
    DragDrop(DragDropEvent),
    /// Emitted for gamepad lifecycle and input notifications.
    #[cfg(feature = "gamepad")]
    Gamepad(GamepadEvent),
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
            Self::Ime(event) => Some(event.window_id()),
            #[cfg(feature = "cursor")]
            Self::Cursor(event) => Some(event.window_id()),
            #[cfg(feature = "drag_drop")]
            Self::DragDrop(event) => Some(event.window_id()),
            #[cfg(feature = "gamepad")]
            Self::Gamepad(_) => None,
            Self::Suspended | Self::Resumed => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn event_layout_is_compact() {
        use core::mem::size_of;

        assert!(size_of::<super::Event>() <= 56);

        #[cfg(feature = "ime")]
        assert!(size_of::<super::ImeEvent>() <= 48);

        #[cfg(feature = "drag_drop")]
        assert!(size_of::<super::DragDropEvent>() <= 48);
    }
}
