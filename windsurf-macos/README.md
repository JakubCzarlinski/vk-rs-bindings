# windsurf-macos

macOS backend for the `windsurf` windowing API family.

This crate currently implements:

- `Display::connect()`
- non-blocking `pump()`
- `Window::new()` on top of `AppKit`
- `raw-window-handle` `AppKit` integration
- a `CAMetalLayer`-backed content view suitable for Vulkan `VK_EXT_metal_surface`

The backend is intentionally minimal. It focuses on window lifecycle, resize,
scale factor, redraw requests, and native handle access needed by renderer
crates.
