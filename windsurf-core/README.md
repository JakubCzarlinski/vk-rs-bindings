# windsurf-core

Minimal poll-driven windowing primitives for the `windsurf` workspace.

This crate intentionally only defines the shared API surface:

- opaque `WindowHandle`
- `WindowHandleAllocator` (`u8` handle space with reuse)
- logical geometry types
- `WindowAttributes`
- flat ID-free `Event`
- fixed-capacity `EventQueue` transporting `(Option<WindowHandle>, Event)`
- optional feature types (`FeatureSet`, `Features`, cursor/IME/drag/gamepad)
- backend trait contract (`LoopBackend`) for static-dispatch frontends

Keyboard input is split into:

- `Event::Key` for logical key transitions (`KeyCode`)
- `Event::TextInput` for UTF-8 text payloads

Optional event families are feature-gated in `Event`:

- `ime`
- `cursor`
- `drag_drop`
- `gamepad`

It does not create windows or connect to any platform backend by itself.

## Example

```rust
use windsurf_core::{Event, EventQueue, WindowHandleAllocator};

let mut handles = WindowHandleAllocator::new();
let window = handles.allocate().unwrap();

let mut queue = EventQueue::new();
queue.push(Some(window), Event::WindowCreated).unwrap();
queue.push(Some(window), Event::RedrawRequested).unwrap();

for (scope, event) in queue.drain() {
    match (scope, event) {
        (Some(handle), Event::RedrawRequested) => {
            assert_eq!(handle, window);
        }
        _ => {}
    }
}
```
