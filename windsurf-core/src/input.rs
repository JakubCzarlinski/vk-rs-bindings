use alloc::string::String;
extern crate alloc;

/// Pressed or released state for pointer and gamepad buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

/// Pointer button normalized across supported backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

/// State transition for keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
    Repeated,
}

/// Minimal cross-platform key representation.
///
/// This is intentionally small. A backend can still provide richer information
/// via raw platform handles or side-channel APIs later.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Character(String),
    Escape,
    Enter,
    Tab,
    Backspace,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Shift,
    Control,
    Alt,
    Meta,
    CapsLock,
    NumLock,
    ScrollLock,
    Pause,
    PrintScreen,
    ContextMenu,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Unknown,
}
