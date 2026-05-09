# vk-rs-bindings

Rust Vulkan bindings generated from Khronos registry XML, plus generator and
demos.

## Versioning

I plan to move versioning to be pinned to Vulkan. This will happen when I am
happy with the API in this repo. As of May 1st 2026, bindings are generated from
Vulkan 1.4.350.

## Installation

It is not currently possible to publish this crate to crates.io due to the
number of features exposed.

```bash
cargo add vk-rs-bindings \
  --git https://github.com/JakubCzarlinski/vk-rs-bindings \
  --rename vk \
  --tag v0.1.3
```

See [https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html)
for more details.

## Typical Usage

Add the crate and only the Vulkan features/extensions you need. We recommend
only enabling the extension you actually use - this keeps the namespace cleaner
but also reduces the size of dispatch tables and the function pointers loaded at
runtime. Note that static linking is not planned.

```toml
[dependencies.vk]
package = "vk-rs-bindings"
features = [
  "VK_VERSION_1_4",
  "VK_KHR_surface",
  "VK_KHR_swapchain",
]
```

Minimal startup pattern:

```rust
use vk::*;

let lib = VulkanLib::load().expect("load Vulkan loader");
let entry = Entry::new(&lib);

let app = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_pApplicationName(c"my-app".as_ptr())
    .with_pEngineName(c"my-engine".as_ptr());

let inst_info = VkInstanceCreateInfo::DEFAULT.with_pApplicationInfo(&app);
let instance = entry.vkCreateInstance(&inst_info, null()).expect("vkCreateInstance");
// Instance dropped here, vkDestroyInstance called automatically.
// Manual call to vkDestroyInstance is permitted, but not required.
// All descendants of instance are also dropped automatically due to lifetime
// tracking.
```

## Design Choices

### Safety

These bindings do not prevent you from calling Vulkan functions in an unsafe
way. For example, attempting to call functions that are not supported by the
current instance/device will result in a panic at runtime. The implementation of
these bindings stores Vulkan function pointers in tables as `Option` types.

In some applicaitons, the cost of a branch on every Vulkan call may be
unacceptable, we use `unwrap_unchecked` to call the function pointers. Some
applications may run with extensions conditionally enabled, in which case,
making the `Option` type useful. It is up to the user to decide how
wish to handle unsupported functions (`None` values in tables). Some strategies
include:

- Group code per supported extension, so that unsupported functions are never
  checked or called.
  - One branch per group, rather than one branch per call.
  - Might introduce some code duplication.
  - More planning is required to group functions by extension, although the docs
    generated should aid in this.

- Check for `None` for each call, such that unsupported functions are never
  called.
  - One branch per call, so potentially more overhead.
  - Less code duplication, but more verbose.
  - Easier to implement, as less consideration is required for enabled
    extensions.

TODO(czarlinski): discuss optionals and arrays.

### Naming

The naming in these bindings is designed to be as close to the original C API as
possible. This is largely motivated by the fact that cross-referencing the
Vulkan specification is a common part of Vulkan development, and having the same
names in Rust makes this marginally simpler as you can copy-paste. It may be
more idiomatic to use Rust naming conventions, but to me, the benefits of
web-search and grepping outweighs aesthetics.

### Feature Gating

TODO(czarlinski): document this.

### Hierarchy of Vulkan Objects and Lifetimes

TODO(czarlinski): document this.

## Regenerating bindings

From the workspace root:

```bash
./generate.sh
cargo fmt
```

`vk-codegen` emits all generated sources into `vk-rs-bindings/`.

## Demos

- `cargo run -p vk-demo-compute`
- `cargo run -p vk-demo-compute-vulkan-1-0`
- `cargo run -p vk-demo-spinning-triangle`

## Workspace layout

- `vk-codegen/`: parses `vk.xml` + `video.xml` into IR and generates Rust bindings.
- `vk-rs-bindings/`: generated low-level Vulkan FFI crate (raw handles, structs, commands, enums, consts).
- `vk-alloc/`: allocator utilities built on top of `vk-rs-bindings`. This is in early stages of development.
- `vk-demo/`: example applications showing end-to-end Vulkan usage. Each demo is a separate crate.
