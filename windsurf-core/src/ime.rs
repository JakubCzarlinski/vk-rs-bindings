use crate::LogicalRect;

extern crate alloc;

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
    pub enabled: bool,
    pub purpose: ImePurpose,
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
