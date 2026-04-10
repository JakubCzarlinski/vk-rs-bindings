# vk-codegen

**vk-codegen** is a Vulkan Rust FFI generator. It processes the Khronos Vulkan Registry XML files (`vk.xml` and `video.xml`) to produce a complete, feature-gated `vk-rs-bindings` Rust crate.

## Overview

Writing and maintaining manual Rust bindings for Vulkan is tedious and prone to errors. `vk-codegen` automates this by systematically reading the official Vulkan registry XML files and outputting raw FFI bindings tailored for Rust.

## Usage

```bash
cargo run -- [--vk <vk.xml>] [--video <video.xml>] [--out <dir>]
```

Alternatively, if you have built the binary:

```bash
vk-codegen --vk path/to/vk.xml --video path/to/video.xml --out out_dir/
```

### Arguments

- `--vk <path>`: Path to the main `vk.xml` registry file. (Default: `vk.xml`)
- `--video <path>`: Path to the `video.xml` registry file. (Default: `video.xml`)
- `--out <dir>`: Output directory for the generated `vk-rs-bindings` crate. (Default: `out`)

## Design & Directory Structure

The project executes a transformation pipeline, moving from raw XML to strongly-typed Rust FFI bindings:

- **`src/parser/`**: Parses XML files (`vk.xml`, `video.xml`) into IR.
- **`src/ir.rs` (Intermediate Representation)**: Defines the central `Registry` struct and associated domain entities (`Struct`, `Enum`, `Command`). The parsed XML data is populated here, decoupling parsing logic from code generation.
- **`src/cfggen.rs`**: Handlers complex extension dependencies, converting Vulkan's boolean logic expressions into valid Rust `#[cfg(...)]` attributes for granular feature-gating.
- **`src/codegen/`**: Consumes the `Registry` (IR) to generate the final Rust source code. It produces internal modules (`enums_rs`, `commands_rs`, `types_rs`) and dynamically writes the resultant crate's `Cargo.toml`.
- **`src/main.rs`**: The entry point which handles CLI arguments, orchestrates the entire pipeline, and flushes output bundles to disk.
