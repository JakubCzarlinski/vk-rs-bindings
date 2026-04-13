# windsurf-core

Minimal poll-driven windowing primitives for the `windsurf` workspace.

This crate intentionally only defines the shared API surface:

- `WindowId`
- logical geometry types
- `WindowAttributes`
- flat `Event`
- drainable `EventQueue`

It does not create windows or connect to any platform backend by itself. That is
left to backend crates such as Wayland/AppKit/UIKit/Android integrations.

## Example

```rust
use windsurf_core::{Event, EventQueue, WindowId};

let mut queue = EventQueue::new();
queue.push(Event::WindowCreated { id: WindowId::new(1) });
queue.push(Event::RedrawRequested { id: WindowId::new(1) });

for event in queue.drain() {
    match event {
        Event::RedrawRequested { id } => {
            assert_eq!(id.raw(), 1);
        }
        _ => {}
    }
}
```
