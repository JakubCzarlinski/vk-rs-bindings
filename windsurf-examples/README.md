# windsurf-examples

Runnable examples for the `windsurf` workspace.

This crate keeps large example binaries, shader assets, and build-time tooling
out of the backend crates themselves.

## Included examples

- `basic_window`: cross-platform Vulkan triangle using the `windsurf` facade
  and selecting Wayland or `AppKit` from the target platform. It now also
  showcases `windsurf-extra` usage (`supported_features`, `set_cursor`,
  `set_ime_state`, and `pump_extras`).

The launcher entrypoint is `examples/basic_window.rs`.
Event/extras demo logic is in `examples/basic_window_app.rs`.
Most Vulkan/render code is in `examples/basic_window_vulkan.rs`.

Run it with:

```bash
cargo run -p windsurf-examples --example basic_window
```
