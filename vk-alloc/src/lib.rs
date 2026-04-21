#![no_std]

mod allocation;
mod allocator;
mod error;
mod group;
mod group_allocator;
mod memory;
mod pool;
mod resource;
mod sparse;
mod stats;
mod vulkan;
extern crate alloc;

pub use allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, HostImportBufferCreateInfo, ImportedHostBuffer,
    LargeBuffer, LargeBufferCreateInfo, LargeBufferSegment,
};
pub use allocator::Allocator;
pub use error::AllocatorError;
pub use group_allocator::{
    GroupAllocator, GroupAllocatorCreateInfo, GroupBindMode, GroupFailurePolicy,
};
pub use pool::{Pool, PoolCreateInfo};
pub use resource::{
    AllocationCreateInfo, AllocationStrategy, AllocatorCreateInfo, MemoryTypePolicy,
    SparseAllocationCreateInfo,
};
pub use sparse::{
    PreparedBindSparseInfo, SparseBufferAllocation, SparseBufferBindList, SparseImageAllocation,
    SparseImageBindList,
};
pub use stats::AllocatorStats;
pub use vulkan::limits::DeviceLimits;

#[cfg(test)]
#[path = "../tests/unit/mod.rs"]
mod tests;
