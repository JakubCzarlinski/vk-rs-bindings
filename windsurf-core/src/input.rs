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

/// Layout-agnostic logical key code.
///
/// Text is intentionally not represented in this enum. Backends should emit
/// text composition through [`crate::Event::TextInput`] so key state and text
/// input remain independently consumable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
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

/// UTF-8 text produced by a keyboard/IME action.
///
/// This alias keeps APIs explicit about whether they carry logical key codes or
/// text payloads.
pub type TextInput = String;
