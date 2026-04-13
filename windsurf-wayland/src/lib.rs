#![doc = include_str!("../README.md")]
#![doc = r"
## Wayland, XKB, and `raw-window-handle`

This backend sits on top of three separate layers that solve different
problems:

- Wayland is the transport and object model. It gives us the connection,
  registry, compositor, surfaces, seats, and XDG shell objects that are used to
  create windows and receive input.
- XKB sits on top of `wl_keyboard`. Wayland key events are just hardware-ish
  keycodes plus a compositor-provided keymap. `xkbcommon` turns that keymap into
  a state machine that can translate those keycodes into logical keys and text.
- `raw-window-handle` does not participate in event handling at all. It is only
  the escape hatch that lets renderer crates borrow the native `wl_display*` and
  `wl_surface*` pointers that already exist underneath the Wayland client types.

In practice the flow is:

1. `Display::pump` drains the Wayland socket and dispatches protocol events.
2. Protocol handlers translate those events into `windsurf_core::Event`.
3. Keyboard handlers feed Wayland keymap/modifier/key events through XKB.
4. Renderers use `raw-window-handle` to create Vulkan/EGL/other surfaces against
   the same Wayland objects.

## Performance Notes

The backend keeps the hot path simple:

- the event pump does not allocate while dispatching and forwarding events
- keyboard text extraction uses a stack buffer first and only falls back to a
  heap allocation for unusually long composed text
- compositor keymaps are memory-mapped during load instead of copied into a
  temporary `Vec`

The remaining unavoidable allocations come from the public API itself, most
notably `Key::Character(String)`.
"]

mod display;
mod error;
mod input;
mod raw;
mod shell;
mod state;
mod util;
mod window;
mod xkb;

pub use crate::display::{Display, RawDisplay};
pub use crate::error::{ConnectError, PumpError, WindowError};
pub use crate::window::{RawWindow, Window};
