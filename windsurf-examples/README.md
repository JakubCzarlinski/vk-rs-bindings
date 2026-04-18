# windsurf-examples

Runnable examples for the `windsurf` workspace.

## Included examples

- `basic_window`: minimal single-loop app using `EventLoop` + `Window` and
  tuple-scoped event routing (`examples/basic_window_app.rs`).
- `multi_window_shared_state`: two-window loop showing routing by
  `WindowHandle` and per-window worker dispatch.

Run them with:

```bash
cargo run -p windsurf-examples --example basic_window
cargo run -p windsurf-examples --example multi_window_shared_state
```
