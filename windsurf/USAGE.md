# windsurf Usage Guide

## Crate layout

- `windsurf-core`: shared ID-free event types, `WindowHandle`, fixed-capacity event queue, backend traits.
- `windsurf-wayland`: Wayland backend implementing `LoopBackend`.
- `windsurf`: facade exposing the default `EventLoop` + `Window` API.
- `windsurf-examples`: runnable sample apps.

## Enable backend

```toml
[dependencies]
windsurf = { path = "../windsurf", features = ["wayland"] }
```

## Core workflow

Single central loop, no callbacks:

1. Create `EventLoop`.
2. Create one or more `Window`s from it.
3. Poll or wait for events as `(Option<WindowHandle>, Event)`.
4. Route by `WindowHandle` when scope is `Some`.

```rust
use core::time::Duration;
use windsurf::{Event, EventLoop, Window, WindowAttributes};

let mut event_loop = EventLoop::connect()?;
let _window = Window::new(&event_loop, WindowAttributes::default())?;

loop {
    if let Some((scope, event)) = event_loop.wait_event(Some(Duration::from_millis(16)))? {
        if matches!((scope, event), (Some(_), Event::CloseRequested)) {
            break;
        }
    }
}
# Ok::<(), Box<dyn core::error::Error>>(())
```

## Optional features

Use `Features` (`set_ime_state`, `set_cursor`, `set_cursor_mode`, `start_drag`) on `EventLoop` and branch behavior based on `supported_features()`.
