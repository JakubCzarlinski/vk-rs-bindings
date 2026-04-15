use crate::{LogicalRect, WindowId};
use alloc::string::String;

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
        selection: Option<(u32, u32)>,
    },
    Commit {
        id: WindowId,
        text: String,
    },
}

impl ImeEvent {
    pub const fn window_id(&self) -> WindowId {
        match self {
            Self::Enabled { id }
            | Self::Disabled { id }
            | Self::Preedit { id, .. }
            | Self::Commit { id, .. } => *id,
        }
    }
}
