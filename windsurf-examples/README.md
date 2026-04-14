# windsurf-examples

Runnable examples for the `windsurf` workspace.

This crate keeps large example binaries, shader assets, and build-time tooling
out of the backend crates themselves.

## Included examples

- `basic_window`: cross-platform Vulkan triangle using the `windsurf` facade
  and selecting Wayland or `AppKit` from the target platform

Run it with:

```bash
cargo run -p windsurf-examples --example basic_window
```
