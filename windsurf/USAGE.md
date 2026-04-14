# windsurf Usage Guide

This guide describes how the current `windsurf` workspace is intended to be used.

## Crate layout

- `windsurf-core`: minimal shared types for window lifecycle, input, geometry, and a poll-friendly queue.
- `windsurf-extra`: optional higher-level abstractions for IME, drag-and-drop, cursors, and gamepads.
- `windsurf-macos`: macOS backend implementing the shared `Display` / `Window` surface.
- `windsurf-wayland`: Wayland + XDG-shell backend implementing `Display` and `Window`.
- `windsurf-examples`: runnable examples that depend on backend crates without living inside them.
- `windsurf`: facade crate that re-exports the shared and backend crates.

## When to use each crate

Use `windsurf-core` when you want the smallest possible API surface and you are
building or consuming a backend that only needs windows plus basic input.

Use `windsurf-extra` when you want richer interactions without forcing them into
every backend or every application.

Use `windsurf` when you want a single import path and are happy with the facade
re-export strategy.

If you want backend types through the facade, enable the relevant backend feature:

```toml
[dependencies]
windsurf = { path = "../windsurf", features = ["wayland"] }
```

Use the `macos` feature instead on macOS when you want the AppKit backend
re-exported from the facade.

## Core workflow

The intended control flow stays polling-oriented:

1. A backend pumps OS messages.
2. The backend translates them into `windsurf_core::Event`.
3. The backend pushes those events into `EventQueue`.
4. The application drains the queue whenever it chooses.

Minimal example:

```rust
use windsurf::{Event, EventQueue, WindowAttributes, WindowId};

let attrs = WindowAttributes::default();
assert!(attrs.decorations);

let mut events = EventQueue::new();
events.push(Event::WindowCreated { id: WindowId::new(1) });
events.push(Event::RedrawRequested { id: WindowId::new(1) });

for event in events.drain() {
    match event {
        Event::RedrawRequested { id } => {
            assert_eq!(id.raw(), 1);
        }
        _ => {}
    }
}
```

## Extra feature workflow

Optional richer events live beside the core queue instead of inside it. That
lets a backend expose these capabilities without bloating the minimal event type.

Typical backend shape:

1. Advertise optional support via `FeatureSet`.
2. Implement `ExtraFeatures`.
3. Emit `ExtraEvent` values into `ExtraEventQueue`.
4. Provide a polling hook like `Display::pump_extras(&mut ExtraEventQueue)`.

Example:

```rust
use windsurf::{
    CursorIcon, CursorSource, ExtraEvent, ExtraEventQueue, FeatureSet, ImeEvent, WindowId,
};

let supported = FeatureSet::IME.with(FeatureSet::CURSOR);
assert!(supported.contains(FeatureSet::CURSOR));

let mut extras = ExtraEventQueue::new();
extras.push(ExtraEvent::Ime(ImeEvent::Enabled { id: WindowId::new(3) }));

let source = CursorSource::Icon(CursorIcon::Pointer);
assert!(matches!(source, CursorSource::Icon(CursorIcon::Pointer)));
assert_eq!(extras.drain().count(), 1);
```

## Backend author guidance

The workspace now includes a Wayland backend in `windsurf-wayland` plus a macOS
backend in `windsurf-macos`. If you are
implementing the next backend:

1. Keep the producer side platform-specific.
2. Translate into `windsurf-core` and `windsurf-extra` types at the boundary.
3. Use `UnsupportedFeature` for partial extra support instead of inventing a new error type for this layer.
4. Keep raw platform handles available elsewhere; these crates are the portable API surface, not the entire backend.

## Application author guidance

Treat `windsurf-core` as the required contract and `windsurf-extra` as optional.
That gives you a clean fallback path when a platform or backend cannot support a
specific richer feature.

Good pattern:

- run your app from the core queue
- branch on `FeatureSet` for optional behavior
- keep rendering and platform escape hatches out of the event types

## Example

The runnable cross-platform Vulkan example lives at:

- `windsurf-examples/examples/basic_window.rs`

You can build or run it with:

```bash
cargo run -p windsurf-examples --example basic_window
```

## Status

As of April 13, 2026, this workspace contains:

- shared API crates
- a macOS backend
- a working Wayland backend
- example binaries and rustdoc examples

iOS and Android backends are still future work.
