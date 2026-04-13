//! Optional higher-level abstractions layered on top of `windsurf-core`.
//!
//! The core crate intentionally stops at window lifecycle and basic input.
//! `windsurf-extra` collects the broader feature surface that many applications
//! still want, but that should not be mandatory for every backend or binary:
//!
//! - IME enablement and composition events
//! - drag-and-drop payloads and notifications
//! - cursor state and custom cursor data
//! - gamepad events
//!
//! # Design intent
//!
//! Backends can implement [`ExtraFeatures`] and emit [`ExtraEvent`] values while
//! applications keep a clear distinction between:
//!
//! - the minimal required event stream from `windsurf-core`
//! - the optional richer stream from this crate
//!
//! # Example
//!
//! ```rust
//! use windsurf_core::WindowId;
//! use windsurf_extra::{CursorIcon, CursorSource, ExtraEvent, ExtraEventQueue, ImeEvent};
//!
//! let mut queue = ExtraEventQueue::new();
//! queue.push(ExtraEvent::Ime(ImeEvent::Enabled { id: WindowId::new(1) }));
//!
//! let cursor = CursorSource::Icon(CursorIcon::Pointer);
//! let drained: Vec<_> = queue.drain().collect();
//!
//! assert_eq!(drained.len(), 1);
//! assert!(matches!(cursor, CursorSource::Icon(CursorIcon::Pointer)));
//! ```

mod cursor;
mod drag_drop;
mod event;
mod feature;
mod gamepad;
mod ime;
mod queue;

pub use crate::cursor::{CursorEvent, CursorIcon, CursorMode, CursorSource};
pub use crate::drag_drop::{DragAction, DragData, DragDropEvent, DragSource};
pub use crate::event::ExtraEvent;
pub use crate::feature::{ExtraFeatures, FeatureKind, FeatureSet, UnsupportedFeature};
pub use crate::gamepad::{GamepadAxis, GamepadButton, GamepadEvent, GamepadId};
pub use crate::ime::{ImeEvent, ImePurpose, ImeState};
pub use crate::queue::ExtraEventQueue;
