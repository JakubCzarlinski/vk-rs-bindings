use crate::LogicalSize;
use alloc::string::String;

extern crate alloc;

/// Stable identifier for a window managed by a backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(u64);

impl WindowId {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Parameters shared by backend window constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowAttributes {
    /// Initial window title.
    pub title: String,
    /// Initial logical inner size.
    pub size: LogicalSize,
    /// Optional logical minimum size.
    pub min_size: Option<LogicalSize>,
    /// Optional logical maximum size.
    pub max_size: Option<LogicalSize>,
    /// Whether the platform should show system decorations when available.
    pub decorations: bool,
    /// Whether the compositor should allow transparency when supported.
    pub transparent: bool,
}

impl Default for WindowAttributes {
    fn default() -> Self {
        Self {
            title: String::from("windsurf"),
            size: LogicalSize::default(),
            min_size: None,
            max_size: None,
            decorations: true,
            transparent: false,
        }
    }
}
