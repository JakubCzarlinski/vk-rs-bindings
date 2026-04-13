/// Logical pixel size used throughout the public API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalSize {
    pub width: u32,
    pub height: u32,
}

impl LogicalSize {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for LogicalSize {
    fn default() -> Self {
        Self::new(1280, 720)
    }
}

/// Logical window-space position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalPosition {
    pub x: f64,
    pub y: f64,
}

impl LogicalPosition {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Logical rectangle, primarily for cursor and text-input integration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl LogicalRect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
