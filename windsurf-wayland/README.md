# windsurf-wayland

Wayland backend for the `windsurf` windowing API family.

This crate implements:

- `Display::connect()`
- pollable `raw_fd()`
- non-blocking `pump()`
- XDG-shell `Window::new()`
- raw-window-handle integration for Wayland
- translation of core lifecycle, pointer, and keyboard events into `windsurf-core::Event`

`WindowAttributes::decorations` is mapped to the optional XDG decoration
protocol when the compositor exposes it. `WindowAttributes::transparent`
controls whether the backend advertises an opaque region for the surface.

## Scope

The backend currently targets Wayland + XDG shell and focuses on the minimal
windowing contract. It does not try to own rendering. Use the raw handles for
Vulkan, EGL, or your own Wayland code.

## Architecture

Three layers matter here, and they do different jobs:

- Wayland: connection, registry, surfaces, XDG shell, seats, pointer events, and
  raw keycode delivery
- XKB: translation of compositor-provided keyboard state into logical keys and
  UTF-8 text
- `raw-window-handle`: escape hatch for renderer crates that need the underlying
  `wl_display*` and `wl_surface*`

The important relationship is that `raw-window-handle` is not another protocol
layer. It simply borrows the native pointers that already back the Wayland
client objects this crate owns.

For keyboard input, Wayland alone is not enough. The compositor sends a keymap
fd via `wl_keyboard::keymap`; `xkbcommon` parses that keymap and later uses it
with modifier updates plus key events to produce `windsurf_core::Key` values.

## Performance

The backend is written to stay cheap in the common loop:

- `Display::pump()` only flushes, reads, dispatches, and forwards events
- key text extraction uses a stack buffer first to avoid heap churn on normal
  character input
- compositor keymaps are memory-mapped during load instead of copied into a
  temporary `Vec`

The runnable Vulkan triangle example now lives in the workspace-level
`windsurf-examples` crate so this backend crate stays focused on the backend
implementation itself. Run it with `cargo run -p windsurf-examples --example
basic_window`.

## Example

```no_run
use std::time::Duration;

use windsurf_core::{Event, EventQueue, WindowAttributes};
use windsurf_wayland::{Display, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::connect()?;
    let _window = Window::new(&display, WindowAttributes::default())?;
    let mut events = EventQueue::new();

    loop {
        display.pump(&mut events)?;

        for event in events.drain() {
            if let Event::CloseRequested { .. } = event {
                return Ok(());
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
```
