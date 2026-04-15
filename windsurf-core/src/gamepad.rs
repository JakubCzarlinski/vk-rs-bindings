use crate::ButtonState;
use alloc::string::String;

extern crate alloc;

/// Stable identifier for a gamepad instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(pub u64);

/// Normalized controller button mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    South,
    East,
    West,
    North,
    LeftShoulder,
    RightShoulder,
    LeftTrigger,
    RightTrigger,
    Select,
    Start,
    Mode,
    LeftStick,
    RightStick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Other(u16),
}

/// Normalized controller axis mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
    Other(u16),
}

/// Gamepad lifecycle and input events.
#[derive(Debug, Clone, PartialEq)]
pub enum GamepadEvent {
    Connected {
        gamepad: GamepadId,
        name: String,
    },
    Disconnected {
        gamepad: GamepadId,
    },
    Button {
        gamepad: GamepadId,
        button: GamepadButton,
        state: ButtonState,
    },
    Axis {
        gamepad: GamepadId,
        axis: GamepadAxis,
        value: f32,
    },
}
