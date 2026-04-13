# windsurf-extra

Optional higher-level abstractions for the `windsurf` workspace.

`windsurf-core` stays intentionally small. This crate holds the broader,
more opinionated surface area that application authors often still want:

- IME state and text composition events
- drag-and-drop payloads and events
- cursor icons, custom cursor images, and cursor modes
- gamepad connection and input events

Backends can implement `ExtraFeatures` to expose these capabilities while
keeping the minimal core crate independent from them.

## Example

```rust
use windsurf_core::{LogicalRect, WindowId};
use windsurf_extra::{ExtraEvent, ExtraEventQueue, ImeEvent, ImePurpose, ImeState};

let mut queue = ExtraEventQueue::new();
let ime = ImeState {
    enabled: true,
    purpose: ImePurpose::Email,
    cursor_area: Some(LogicalRect::new(16.0, 32.0, 12.0, 18.0)),
};

assert!(ime.enabled);

queue.push(ExtraEvent::Ime(ImeEvent::Enabled { id: WindowId::new(5) }));
assert_eq!(queue.drain().count(), 1);
```
