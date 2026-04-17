//! Core types for the `windsurf` windowing stack.
//!
//! `windsurf-core` is intentionally small and backend-agnostic. It gives
//! backend crates a common vocabulary for:
//!
//! - window identifiers
//! - logical geometry
//! - window attributes
//! - flat input and lifecycle events
//! - a poll-friendly drainable event queue
//!
//! The crate does not own a run loop and does not define any thread policy.
//! Platform crates are expected to push translated OS events into
//! [`EventQueue`], while application code decides when and how to drain it.
//!
//! # Example
//!
//! ```rust
//! use windsurf_core::{Event, EventQueue, WindowId};
//!
//! let mut queue = EventQueue::new();
//! queue.push(Event::WindowCreated { id: WindowId::new(7) });
//! queue.push(Event::CloseRequested { id: WindowId::new(7) });
//!
//! let events: Vec<_> = queue.drain().collect();
//! assert_eq!(events.len(), 2);
//! ```
#![no_std]
mod cursor;
mod drag_drop;
mod event;
mod feature;
mod gamepad;
mod geometry;
mod ime;
mod input;
mod queue;
mod window;

pub use crate::cursor::{CursorEvent, CursorIcon, CursorMode, CursorSource};
pub use crate::drag_drop::{DragAction, DragData, DragPosition, DragSource};
pub use crate::event::Event;
pub use crate::feature::{FeatureKind, FeatureSet, Features, UnsupportedFeature};
pub use crate::gamepad::{GamepadAxis, GamepadButton, GamepadId};
pub use crate::geometry::{LogicalPosition, LogicalRect, LogicalSize};
pub use crate::ime::{ImePurpose, ImeState};
pub use crate::input::{ButtonState, KeyCode, KeyState, PointerButton, TextInput};
pub use crate::queue::EventQueue;
pub use crate::window::{WindowAttributes, WindowId};
