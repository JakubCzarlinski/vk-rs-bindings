# windsurf-wayland

Wayland backend for the `windsurf` event-loop API.

This crate exposes `WaylandBackend`, which implements
`windsurf_core::LoopBackend` and powers the facade `windsurf::EventLoop` /
`windsurf::Window` types on Linux when the `wayland` feature is enabled.

Implemented in this milestone:

- non-callback, poll-driven event delivery (`poll_event` / `wait_event`)
- tuple-scoped transport `(Option<WindowHandle>, Event)`
- ID-free event payloads
- fixed-capacity queue in the hot path (no per-event queue allocation)
- `epoll`-based timed waiting on the Wayland socket
- `raw-window-handle` interop for renderer setup (not event identity)

`WindowHandle` is the stable routing identity for application logic.
Raw handles are rendering escape hatches only.
