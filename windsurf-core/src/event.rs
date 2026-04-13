use crate::{ButtonState, Key, KeyState, PointerButton, WindowId};

/// Flat event enum shared by all `windsurf` backends.
///
/// The design avoids nested, callback-oriented event trees. Backends translate
/// OS messages into this enum and push them into an [`crate::EventQueue`].
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    WindowCreated {
        id: WindowId,
    },
    WindowResized {
        id: WindowId,
        width: u32,
        height: u32,
    },
    ScaleFactorChanged {
        id: WindowId,
        factor: f64,
    },
    CloseRequested {
        id: WindowId,
    },
    WindowDestroyed {
        id: WindowId,
    },
    RedrawRequested {
        id: WindowId,
    },
    PointerEntered {
        id: WindowId,
    },
    PointerLeft {
        id: WindowId,
    },
    PointerMoved {
        id: WindowId,
        x: f64,
        y: f64,
    },
    PointerButton {
        id: WindowId,
        button: PointerButton,
        state: ButtonState,
    },
    PointerScroll {
        id: WindowId,
        dx: f64,
        dy: f64,
    },
    KeyboardFocusIn {
        id: WindowId,
    },
    KeyboardFocusOut {
        id: WindowId,
    },
    Key {
        id: WindowId,
        key: Key,
        scancode: u32,
        state: KeyState,
    },
    TouchStart {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    TouchMove {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    TouchEnd {
        id: WindowId,
        finger: u64,
        x: f64,
        y: f64,
    },
    TouchCancel {
        id: WindowId,
        finger: u64,
    },
    Suspended,
    Resumed,
}
