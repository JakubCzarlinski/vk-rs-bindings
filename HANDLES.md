# Design Document: Automated Vulkan Handle Wrappers and Dispatch

## 1. Overview

The goal of this refactor is to automate the generation of smart wrappers for all Vulkan objects (handles) with a strict lifetime hierarchy (e.g., `Device<'inst>`, `CommandPool<'dev>`, `Fence<'dev>`). Currently, codegen manually specifies a fixed list of `Tier`s (Entry, Instance, PhysicalDevice, Device, Queue, CommandPool, CommandBuffer). We want to dynamically process all handles that establish a parent-child relationship to generate these wrappers, chunked dispatch tables, and appropriate `Drop` implementations automatically.

## 2. Changes to Intermediate Representation (IR)

### 2.1 Update `TypedefKind`
The `vk.xml` file specifies parenthood of handles. E.g.:
`<type category="handle" parent="VkDevice" objtypeenum="...">`

`ir.rs` must be updated to store this information:
```rust
pub enum TypedefKind {
    // ...
    Handle {
        dispatchable: bool,
        parent: Option<String>,
        objtypeenum: Option<String>,
    },
    // ...
}
```

## 3. Dispatch Table Generation & Tier Resolution

### 3.1 Multi-Level Tier Resolution
Rather than looking only at `param[0]`, the generator must inspect the parameter list to determine the most specific dispatch table. 

- **Device Tier:** If `param[0]` is `VkDevice`, search for a handle in `param[1]` that has a `parent="VkDevice"`. 
  - Example: `vkCreateCommandPool(device, pCreateInfo, ...)` -> Grouped under **Device** (it creates the pool).
  - Example: `vkResetCommandPool(device, commandPool, ...)` -> Grouped under **CommandPool** (it operates on the pool).
- **Instance Tier:** `param[0]` is `VkInstance` or `VkPhysicalDevice`.

### 3.2 Dispatch Table Structs
The `Device` should contain specialized, small dispatch tables chunked by the objects they operate on:
- `DeviceDispatchTable` (Device-wide commands like `vkCreate...` or `vkWaitForFences`)
- `QueueDispatchTable` (Commands where `param[0]` is `VkQueue`)
- `CommandPoolDispatchTable` (Commands where `param[1]` is `VkCommandPool` or `param[0]` is a buffer from that pool)
- `DescriptorPoolDispatchTable` (Commands where `param[1]` is `VkDescriptorPool`)

## 4. Return Type Classification (Arrays vs. Single Structs)

To automate wrapper methods, we classify the return based on Vulkan XML attributes (specifically `len` and `optional` on `p*` output parameters):

1. **Single Struct / Handle:**
   - Criteria: An output parameter `pHandle` with no `len` attribute and `optional="false"`.
   - Rust Mapping: `-> Result<HandleWrapper<'a>, VkResult>` (or just `HandleWrapper` if no result code).

2. **Fixed/Dynamic Arrays:**
   - Criteria: An output parameter has a `len` attribute pointing to another parameter (e.g., `descriptorSetCount`).
   - Examples: `vkAllocateDescriptorSets`, `vkAllocateCommandBuffers`.
   - Rust Mapping: `-> Result<Vec<HandleWrapper<'a>>, VkResult>`.

3. **Two-Call Enumeration:**
   - Criteria: A command with a `count` pointer and an array pointer where the array is `optional="true"`.
   - Rust Mapping: Automatically perform the two-call dance and return `Result<Vec<T>, VkResult>`.

## 5. Specific Cases: Descriptors and Command Buffers

### 5.1 Descriptor Pools and Sets
- **DescriptorPool:** A child of `Device`. It manages a `DescriptorPoolDispatchTable` which contains `vkFreeDescriptorSets`, `vkUpdateDescriptorSets`, etc.
- **DescriptorSet:** A child of `DescriptorPool`. 
  - Lifetime: `DescriptorSet<'pool, 'dev>`. 
  - Drop: If the pool was created without `VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT`, the sets are not individualy freed. The generator must check for this or provide a scoped 'Free' method. Usually, we'll allow `vkFreeDescriptorSets` if the dispatch table entry is present.

### 5.2 Command Buffers
- **Parent:** `VkCommandPool`.
- **Lifetime:** `CommandBuffer<'pool, 'dev>`.
- **Drop:** Usually not freed individually unless specifically requested. `vkDestroyCommandPool` handles bulk cleanup.

## 6. Automated Drop Implementations

Codegen will match `HandleName` to its destructor:
1. Find `vkDestroy<Name>`: Typically takes `(Parent, Handle, Allocator)`. 
2. Find `vkFree<Name>s`: Typically takes `(Parent, Count, pHandles)`. Used for Buffers/Sets.

```rust
impl<'pool, 'dev> Drop for DescriptorSet<'pool, 'dev> {
    fn drop(&mut self) {
        // Only if the user hasn't manually freed or if we support auto-free
    }
}
```