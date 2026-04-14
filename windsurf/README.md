# windsurf

Facade crate for the `windsurf` windowing API family.

`windsurf` re-exports:

- `windsurf-core` unconditionally
- `windsurf-extra` when the default `extras` feature is enabled
- target-selected `Display` / `Window` types when a backend feature is enabled
- backend modules such as `windsurf::macos` and `windsurf::wayland`

This keeps the minimal surface available while still giving applications a
single crate import when they want the larger abstraction set.

## Current scope

The workspace currently defines the shared API and documentation surface:

- core events and queueing
- optional IME / drag-and-drop / cursor / gamepad abstractions
- a macOS backend in `windsurf-macos`
- a Wayland backend in `windsurf-wayland`
- facade re-exports

The facade re-exports backend crates when their feature is enabled.

```toml
[dependencies]
windsurf = { path = "../windsurf", features = ["wayland"] }
```

Use the `macos` feature on macOS or the `wayland` feature on Linux to get
root-level `windsurf::Display` and `windsurf::Window` aliases for the active
backend.

## Quick start

```rust
use windsurf::{Event, EventQueue, WindowAttributes, WindowId};

let attrs = WindowAttributes::default();
assert_eq!(attrs.title, "windsurf");

let mut queue = EventQueue::new();
queue.push(Event::WindowCreated { id: WindowId::new(42) });
assert_eq!(queue.drain().count(), 1);
```

See [USAGE.md](USAGE.md) for the detailed guide.

## Extras in backends

Current `windsurf-macos` and `windsurf-wayland` backends expose:

- core events via `Display::pump(&mut EventQueue)`
- optional richer events via `Display::pump_extras(&mut ExtraEventQueue)`
- `ExtraFeatures` support for IME and cursor APIs
