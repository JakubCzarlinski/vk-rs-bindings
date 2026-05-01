# vk-rs-bindings

Rust Vulkan bindings generated from Khronos registry XML, plus generator and demos.

## Versioning

I plan to move versioning to be pinned to Vulkan. This will happen when I am happy with the API in this repo. As of May 1st 2026, bindings are generated from Vulkan 1.4.350.

## Installation

It is not currently possible to publish this crate to crates.io due to the number of features exposed.

`cargo add vk-rs-bindings --git https://github.com/JakubCzarlinski/vk-rs-bindings --rename vk --tag v0.1.3`

See [https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html](https://blog.rust-lang.org/2023/10/26/broken-badges-and-23k-keywords.html) for more details.

## Typical vk-rs-bindings usage

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
// All descendants of instance are also dropped automatically.
```

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
- `vk-demo/`: example applications showing end-to-end Vulkan usage.
  `vk-demo/compute/`, `vk-demo/compute-vulkan-1-0/`, and `vk-demo/spinning-triangle/` are separate crates so they can keep independent dependencies and Vulkan feature sets.
