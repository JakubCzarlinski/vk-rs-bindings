extern crate alloc;

/// Stable identifier for a gamepad instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(pub u32);

/// Normalized controller button mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    // Face
    South,
    East,
    West,
    North,
    // Shoulders
    LeftShoulder,
    RightShoulder,
    // Triggers (digital buttons, distinct from analog axes)
    LeftTrigger,
    RightTrigger,
    // Center
    Select,
    Start,
    Mode,
    // Sticks
    LeftStick,
    RightStick,
    // D-Pad
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    // Modern extensions (PS5, Switch Pro, Xbox Elite)
    Touchpad, // PS4/PS5 touchpad click
    Mute,     // DualSense mute button
    Capture,  // Switch capture button
    Paddle1,  // Xbox Elite paddles (optional)
    Paddle2,
    Paddle3,
    Paddle4,
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
    TouchpadX,
    TouchpadY,
}
