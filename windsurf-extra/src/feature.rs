use crate::{CursorMode, CursorSource, DragSource, ImeState};
use alloc::fmt;
use core::error::Error;
use windsurf_core::WindowId;

extern crate alloc;

/// Named optional capability that a backend may expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureKind {
    Ime,
    /// Initiating drags from the local application.
    DragDropSource,
    /// Receiving drag payloads from external applications.
    DragDropDestination,
    Cursor,
    Gamepad,
}

/// Compact feature bitset for optional `windsurf-extra` support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FeatureSet(u32);

impl FeatureSet {
    pub const IME: Self = Self(1 << 0);
    pub const DRAG_DROP_SOURCE: Self = Self(1 << 1);
    pub const DRAG_DROP_DESTINATION: Self = Self(1 << 2);
    pub const CURSOR: Self = Self(1 << 3);
    pub const GAMEPAD: Self = Self(1 << 4);
    pub const DRAG_DROP: Self = Self(Self::DRAG_DROP_SOURCE.0 | Self::DRAG_DROP_DESTINATION.0);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(
            Self::IME.0
                | Self::DRAG_DROP_SOURCE.0
                | Self::DRAG_DROP_DESTINATION.0
                | Self::CURSOR.0
                | Self::GAMEPAD.0,
        )
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Returned when an application asks a backend for an unsupported extra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnsupportedFeature {
    pub feature: FeatureKind,
}

impl UnsupportedFeature {
    pub const fn new(feature: FeatureKind) -> Self {
        Self { feature }
    }
}

impl fmt::Display for UnsupportedFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not supported by this backend", self.feature)
    }
}

impl Error for UnsupportedFeature {}

/// Backend-facing trait for optional richer interactions.
///
/// Applications should treat this as an escape hatch layered on top of the
/// minimal core API. A backend may support any subset of these features.
pub trait ExtraFeatures {
    /// Report the optional features implemented by this backend.
    fn supported_features(&self) -> FeatureSet;

    /// Enable, disable, or retarget the IME for a specific window.
    fn set_ime_state(&self, window: WindowId, state: &ImeState) -> Result<(), UnsupportedFeature>;

    /// Change the cursor image shown for a window.
    fn set_cursor(&self, window: WindowId, source: &CursorSource)
    -> Result<(), UnsupportedFeature>;

    /// Change the cursor mode for a window.
    fn set_cursor_mode(&self, window: WindowId, mode: CursorMode)
    -> Result<(), UnsupportedFeature>;

    /// Start a drag originating from the given window.
    ///
    /// This requires [`FeatureSet::DRAG_DROP_SOURCE`].
    fn start_drag(&self, window: WindowId, source: DragSource) -> Result<(), UnsupportedFeature>;
}

#[cfg(test)]
mod tests {
    use super::FeatureSet;

    #[test]
    fn feature_set_contains_bits() {
        let features = FeatureSet::IME.with(FeatureSet::CURSOR);
        assert!(features.contains(FeatureSet::IME));
        assert!(features.contains(FeatureSet::CURSOR));
        assert!(!features.contains(FeatureSet::GAMEPAD));
        assert!(!features.contains(FeatureSet::DRAG_DROP_SOURCE));
        assert!(!features.contains(FeatureSet::DRAG_DROP_DESTINATION));
    }
}
