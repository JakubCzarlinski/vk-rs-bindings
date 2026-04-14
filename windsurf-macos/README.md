# windsurf-macos

macOS backend for the `windsurf` windowing API family.

This crate currently implements:

- `Display::connect()`
- non-blocking `pump()`
- non-blocking `pump_extras()`
- `Window::new()` on top of `AppKit`
- `raw-window-handle` `AppKit` integration
- a `CAMetalLayer`-backed content view suitable for Vulkan `VK_EXT_metal_surface`
- `windsurf_extra::ExtraFeatures` on `Display`
  - supported: `FeatureSet::IME`, `FeatureSet::CURSOR`, `FeatureSet::DRAG_DROP_DESTINATION`
  - unsupported: `FeatureSet::DRAG_DROP_SOURCE` and gamepad

The backend is intentionally minimal. It focuses on window lifecycle, resize,
scale factor, redraw requests, and native handle access needed by renderer
crates.
