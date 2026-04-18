//! Core types for the `windsurf` windowing stack.
//!
//! `windsurf-core` is intentionally small and backend-agnostic. It gives
//! backend crates a common vocabulary for:
//!
//! - opaque window handles
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
//! use windsurf_core::{Event, EventQueue, WindowHandleAllocator};
//!
//! let mut queue = EventQueue::new();
//! let mut handles = WindowHandleAllocator::new();
//! let handle = handles.allocate().unwrap();
//! queue.push(Some(handle), Event::WindowCreated).unwrap();
//! queue.push(Some(handle), Event::CloseRequested).unwrap();
//!
//! let events: Vec<_> = queue.drain().collect();
//! assert_eq!(events.len(), 2);
//! ```
#![no_std]
mod backend;
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

pub use crate::backend::LoopBackend;
pub use crate::cursor::{CursorIcon, CursorMode, CursorSource};
pub use crate::drag_drop::{DragAction, DragData, DragPosition, DragSource};
pub use crate::event::{Event, ScopeKind};
pub use crate::feature::{FeatureKind, FeatureSet, Features, UnsupportedFeature};
pub use crate::gamepad::{GamepadAxis, GamepadButton, GamepadId};
pub use crate::geometry::{LogicalPosition, LogicalRect, LogicalSize};
pub use crate::ime::{ImePurpose, ImeState};
pub use crate::input::{ButtonState, KeyCode, KeyState, PointerButton, TextInput};
pub use crate::queue::{EventQueue, QueueOverflow, ScopedEvent};
pub use crate::window::{WindowAttributes, WindowHandle, WindowHandleAllocator};
