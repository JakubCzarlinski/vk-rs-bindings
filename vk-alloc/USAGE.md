# `vk-alloc` Usage Guide

`vk-alloc` is a handwritten allocator for `vk-rs-bindings`.

This crate is intended for explicit Vulkan applications that want:

- a fast single-device allocator path
- an explicit device-group allocator path
- sparse binding support without mixing sparse bookkeeping into the normal hot path
- suballocation and dedicated allocation policy in one place

## When To Use Each Top-Level Type

### `Allocator<'vk>`

Use this for the common case: one logical Vulkan device, no device-group binding semantics.

Choose `Allocator` when:

- you have one logical device
- you do not need `VkBind*Memory*DeviceGroupInfo`
- you want the smallest and simplest allocator path

Do not use `Allocator` for:

- explicit device-group instance binding
- split-instance image binding

### `GroupAllocator<'vk>`

Use this only when the application is intentionally using Vulkan device groups.

Choose `GroupAllocator` when:

- the device was created for device-group usage
- placement must be controlled with `GroupBindMode`
- unsupported group modes must fail explicitly

Do not use `GroupAllocator` as a generic “future proof” wrapper. It is the explicit multi-device path.

### `Allocation`

This is the memory binding result for a normal non-sparse allocation.

It gives you:

- the `VkDeviceMemory` handle
- the bound offset
- the allocation size
- an optional mapped pointer view
- allocator-owned lifetime tracking for the backing block

Use it when:

- you already created a Vulkan buffer or image yourself
- you want to call `allocate_for_buffer` or `allocate_for_image`

### `AllocatedBuffer<'vk>` and `AllocatedImage<'vk>`

These wrap both the Vulkan resource handle and the allocation.

Use them when:

- you want the allocator to create and bind the resource for you
- you want destruction order to stay coupled: resource first, then allocator-owned memory bookkeeping

### `Pool` and `PoolCreateInfo`

Pools partition arenas. They are not Vulkan `VkCommandPool`-style objects.

Use a custom pool when:

- you want a separate fragmentation domain
- you want different block sizes
- you want to isolate resource classes or allocator behavior for a subsystem

`Pool` is an opaque handle returned by `create_pool`; it is consumed through
`AllocationCreateInfo::with_pool`.

```rust,no_run
use vk_alloc::{AllocationCreateInfo, Allocator, PoolCreateInfo};

fn pool_example<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<(), vk_alloc::AllocatorError> {
    let allocator = Allocator::new(physical_device, device)?;
    let streaming_pool = allocator.create_pool(
        PoolCreateInfo::new().with_host_visible_block_size(4 * 1024 * 1024),
    )?;

    let alloc_info = AllocationCreateInfo::new().with_pool(streaming_pool);
    let _ = alloc_info;
    Ok(())
}
```

### `SparseBufferAllocation` and `SparseImageAllocation`

Use these only for sparse Vulkan resources.

They manage:

- sparse page residency bookkeeping
- backing page allocations
- bind-list generation

They do not manage:

- copy scheduling
- migration planning
- scatter/gather execution

### `ImportedHostBuffer<'vk>` and `HostImportBufferCreateInfo`

Use these when Vulkan should bind to memory you already own on the host.

Choose this path when:

- data already lives in pinned or externally managed host memory
- zero-copy or low-copy integration matters more than device-local placement
- the platform exposes `VK_EXT_external_memory_host`

This is buffer-only in the current implementation.

### `LargeBuffer<'vk>` and `LargeBufferCreateInfo`

Use this when one logical tensor or dataset should be represented as a single allocator-facing object, but one normal Vulkan allocation would be too large or undesirable.

Choose this path when:

- one logical buffer should be chunked automatically
- you want the allocator to respect device allocation limits
- you want storage-buffer-aware chunk sizing without manually managing segments

This is a logical abstraction over multiple normal `AllocatedBuffer<'vk>` segments.

### `AllocatorStats`

Use this for coarse allocator counters and debugging.

It is useful for:

- allocation count inspection
- reserved bytes tracking
- dedicated allocation counts

### `AllocatorError`

This reports allocator policy and Vulkan failures.

Important variants include:

- `Vulkan(VkResult)`
- `AllocationTooLarge`
- `NoCompatibleMemoryType`
- `DedicatedAllocationRequired`
- `GroupModeUnsupported`
- `SparseBindingUnsupported`

## Memory Type Selection

This crate does not hide Vulkan memory property bits behind a usage enum.

Instead, it uses `MemoryTypePolicy`:

- `required_flags`: must be present
- `preferred_flags`: improve score
- `avoid_flags`: reduce score

Built-in presets:

- `MemoryTypePolicy::DEVICE_LOCAL`
- `MemoryTypePolicy::HOST_VISIBLE`
- `MemoryTypePolicy::UPLOAD`
- `MemoryTypePolicy::READBACK`
- `MemoryTypePolicy::UNIFIED`

### When To Use Each Policy

`DEVICE_LOCAL`

- Use for GPU-only resources where CPU mapping is not needed.

`HOST_VISIBLE`

- Use when CPU access is mandatory but you do not want to bias toward a specific upload or readback pattern.

`UPLOAD`

- Use for staging or CPU-written buffers.
- This requires `HOST_VISIBLE` and prefers `HOST_COHERENT` and `DEVICE_LOCAL`.

`READBACK`

- Use for CPU-read results.
- This requires `HOST_VISIBLE` and prefers `HOST_CACHED` and `HOST_COHERENT`.

`UNIFIED`

- Use when you want to prefer a memory type that is both CPU-visible and GPU-local when the platform exposes one.
- On UMA systems this usually finds the shared CPU/GPU memory type.
- On discrete systems it degrades to the best host-visible candidate because Vulkan memory types still have to match the requested property bits.

## Single-Device API

### Construction

```rust,no_run
use vk_alloc::Allocator;

fn make_allocator<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<Allocator<'vk>, vk_alloc::AllocatorError> {
    Allocator::new(physical_device, device)
}
```

### Allocator-owned resource creation

```rust,no_run
use vk_alloc::{AllocationCreateInfo, Allocator, MemoryTypePolicy};

fn create_buffer<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<vk_alloc::AllocatedBuffer<'vk>, vk_alloc::AllocatorError> {
    let allocator = Allocator::new(physical_device, device)?;

    let buffer_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(4096)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0);

    allocator.create_buffer(
        &buffer_info,
        AllocationCreateInfo::new().with_memory_type_policy(MemoryTypePolicy::UPLOAD),
    )
}
```

### Allocation for an existing resource

Use this when resource creation is handled elsewhere but you still want allocator-backed memory:

- `allocate_for_buffer`
- `allocate_for_image`

### Mapping

```rust,no_run
fn write_words(allocation: &mut vk_alloc::Allocation, values: &[u32]) {
    let slice = allocation.mapped_slice_mut::<u32>(values.len()).unwrap();
    slice.copy_from_slice(values);
}
```

## Group API

### `GroupBindMode`

`Instance0`

- Use when you want the simplest explicit device-group binding behavior.

`PerDeviceInstance`

- Use when the heap supports multi-instance allocation and you want per-device memory instances.

`SplitInstanceRegions`

- Use only for image cases where Vulkan allows split-instance regions and the caller can supply the regions explicitly.

### Example

```rust,no_run
use vk_alloc::{
    AllocationCreateInfo, GroupAllocator, GroupBindMode,
};

fn create_group_buffer<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<vk_alloc::AllocatedBuffer<'vk>, vk_alloc::AllocatorError> {
    let allocator = GroupAllocator::new(physical_device, device, 0b11)?;

    let buffer_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(4096)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0);

    allocator.create_buffer_with_mode(
        &buffer_info,
        AllocationCreateInfo::new().with_group_bind_mode(GroupBindMode::Instance0),
    )
}
```

## Sparse API

Sparse resources are intentionally separate from the normal allocator path.

Use sparse APIs when:

- the logical object is larger than one practical backing allocation
- residency must be updated page-by-page
- sparse binding is part of the application model

Do not use sparse APIs just to imitate ordinary suballocation.

## Imported Host Memory API

This path uses `VK_EXT_external_memory_host`.

What the allocator does:

- validates imported host pointer alignment
- queries `vkGetMemoryHostPointerPropertiesEXT`
- selects a compatible Vulkan memory type
- allocates imported `VkDeviceMemory`
- binds the buffer

What the allocator does not do:

- allocate the host memory for you
- free the host allocation on drop
- guarantee device-local performance

### Example

```rust,no_run
use vk_alloc::{
    AllocationCreateInfo, Allocator, HostImportBufferCreateInfo, MemoryTypePolicy,
};

fn import_buffer<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
    host_ptr: *mut u8,
    size: u64,
) -> Result<vk_alloc::ImportedHostBuffer<'vk>, vk_alloc::AllocatorError> {
    let allocator = Allocator::new(physical_device, device)?;
    let buffer_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(size)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0);

    allocator.import_host_buffer(
        &buffer_info,
        HostImportBufferCreateInfo::new(host_ptr, size),
        AllocationCreateInfo::new().with_memory_type_policy(MemoryTypePolicy::HOST_VISIBLE),
    )
}
```

## Large Buffer API

This path automatically chunks one logical buffer into multiple normal Vulkan buffers.

What the allocator considers:

- `maxMemoryAllocationSize`
- `maxStorageBufferRange` for storage buffers
- `maxUniformBufferRange` for uniform buffers
- an optional caller-provided preferred chunk size

### Example

```rust,no_run
use vk_alloc::{
    AllocationCreateInfo, Allocator, LargeBufferCreateInfo, MemoryTypePolicy,
};

fn create_large_buffer<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<vk_alloc::LargeBuffer<'vk>, vk_alloc::AllocatorError> {
    let allocator = Allocator::new(physical_device, device)?;
    let total_size = 128 * 1024 * 1024u64;
    let buffer_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(total_size)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0);

    allocator.create_large_buffer(
        &buffer_info,
        AllocationCreateInfo::new().with_memory_type_policy(MemoryTypePolicy::DEVICE_LOCAL),
        LargeBufferCreateInfo::new(),
    )
}
```

Useful methods on `LargeBuffer`:

- `total_size()`
- `chunk_size()`
- `segment_count()`
- `segments()`
- `segment_for_offset(offset)`
- `write_bytes(offset, bytes)` for host-visible segment sets

### Sparse example

```rust,no_run
use vk_alloc::{Allocator, SparseAllocationCreateInfo};

fn create_sparse<'vk>(
    physical_device: &'vk vk::PhysicalDevice<'vk>,
    device: &'vk vk::Device<'vk>,
) -> Result<vk_alloc::PreparedBindSparseInfo, vk_alloc::AllocatorError> {
    let allocator = Allocator::new(physical_device, device)?;
    let image_info = vk::VkImageCreateInfo::DEFAULT
        .with_flags(vk::VkImageCreateFlagBits::VK_IMAGE_CREATE_SPARSE_BINDING_BIT.0)
        .with_imageType(vk::VkImageType::VK_IMAGE_TYPE_2D)
        .with_extent(vk::VkExtent3D::DEFAULT.with_width(64).with_height(64).with_depth(1))
        .with_mipLevels(1)
        .with_arrayLayers(1)
        .with_format(vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM)
        .with_tiling(vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL)
        .with_initialLayout(vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED)
        .with_usage(vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT.0)
        .with_samples(vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);

    let sparse = allocator.create_sparse_image(&image_info, SparseAllocationCreateInfo::new())?;
    Ok(sparse.prepare_bind_info())
}
```

## Large Allocations

Normal Vulkan non-sparse resources bind to one `VkDeviceMemory` object.

That means:

- if a resource’s memory requirement exceeds `maxMemoryAllocationSize`
- the normal allocator path must fail

In `vk-alloc` that failure is reported as `AllocatorError::AllocationTooLarge`.

For larger logical tensors or datasets, use one of these approaches:

- chunk the logical object across multiple buffers
- use sparse buffers or sparse images
- build an indirection layer in your runtime or shader interface

## Pools And Default Sizes

Default block sizes:

- host-visible: `16 MiB`
- device-local: `64 MiB`
- mixed/other: `32 MiB`

Default dedicated behavior:

- dedicate when Vulkan requires it
- dedicate when the caller forces it
- otherwise dedicate once the request exceeds `50%` of the arena block size, unless overridden

## What This Allocator Does Not Do

- It does not control Vulkan driver internal allocations.
- It does not replace `VkAllocationCallbacks` for host-side driver bookkeeping.
- It does not schedule copy or migration work.
- It does not automatically replicate data across a device group.

## Vulkan Driver Internals

You cannot use `vk-alloc` for the driver’s own internal GPU memory management.

Vulkan only exposes limited hooks for internal allocations:

- `VkAllocationCallbacks` for some host-side driver allocations

Those callbacks are not a replacement for explicit `VkDeviceMemory` management and do not let you suballocate driver-owned GPU memory.

## Validation

Current automated validation:

- `cargo test -p vk-alloc`
- `cargo check -p vk-demo --bin compute`

Criterion:

- `cargo bench -p vk-alloc --no-run`
- `cargo bench -p vk-alloc --no-run`
