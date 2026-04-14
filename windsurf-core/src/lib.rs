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
mod event;
mod geometry;
mod input;
mod queue;
mod window;

pub use crate::event::Event;
pub use crate::geometry::{LogicalPosition, LogicalRect, LogicalSize};
pub use crate::input::{ButtonState, KeyCode, KeyState, PointerButton, TextInput};
pub use crate::queue::EventQueue;
pub use crate::window::{WindowAttributes, WindowId};
