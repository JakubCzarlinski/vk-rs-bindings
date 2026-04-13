use windsurf_core::{LogicalRect, WindowId};

/// Intended text-input use case for an active IME session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImePurpose {
    Normal,
    Password,
    Number,
    Email,
    Url,
    Phone,
    Terminal,
}

/// Requested IME state for a window.
#[derive(Debug, Clone, PartialEq)]
pub struct ImeState {
    /// Whether text composition should be active.
    pub enabled: bool,
    /// Input field purpose hint for platform keyboards and candidate UIs.
    pub purpose: ImePurpose,
    /// Optional logical rectangle describing the current caret or selection.
    pub cursor_area: Option<LogicalRect>,
}

impl Default for ImeState {
    fn default() -> Self {
        Self {
            enabled: false,
            purpose: ImePurpose::Normal,
            cursor_area: None,
        }
    }
}

/// IME-related events emitted by a backend.
#[derive(Debug, Clone, PartialEq)]
pub enum ImeEvent {
    Enabled {
        id: WindowId,
    },
    Disabled {
        id: WindowId,
    },
    Preedit {
        id: WindowId,
        text: String,
        selection: Option<(usize, usize)>,
    },
    Commit {
        id: WindowId,
        text: String,
    },
}
