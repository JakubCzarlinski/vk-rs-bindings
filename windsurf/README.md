# windsurf

Facade crate for the `windsurf` windowing API family.

`windsurf` now exposes a single-loop, poll-driven API:

- `EventLoop::new()` / `EventLoop::connect()`
- `EventLoop::poll_event()`
- `EventLoop::wait_event(timeout)`
- `Window::new(&EventLoop, WindowAttributes)`

Event transport is tuple-scoped:

- `Some(WindowHandle)` for window-scoped events
- `None` for global events

`Event` payloads are ID-free; window identity is always external.

## Linux Wayland

Enable the backend via feature flag:

```toml
[dependencies]
windsurf = { path = "../windsurf", features = ["wayland"] }
```

## Quick start

```no_run
use std::time::Duration;
use windsurf::{Event, EventLoop, Window, WindowAttributes};

let mut event_loop = EventLoop::connect()?;
let _window = Window::new(&event_loop, WindowAttributes::default())?;

loop {
    if let Some((window, event)) = event_loop.wait_event(Some(Duration::from_millis(16)))? {
        if matches!((window, event), (Some(_), Event::CloseRequested)) {
            break;
        }
    }
}
# Ok::<(), Box<dyn core::error::Error>>(())
```
