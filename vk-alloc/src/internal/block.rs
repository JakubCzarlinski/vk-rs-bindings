use alloc::sync::Arc;
use core::ptr::null_mut;

use parking_lot::Mutex;

use crate::internal::buddy::RangeAllocator;
use crate::internal::vk_ops::OwnedMemory;

pub trait AllocationSource: Send + Sync {
    fn raw_memory(&self) -> vk::VkDeviceMemory;
    fn mapped_ptr(&self) -> *mut u8;
    fn free_range(&self, offset: u64, size: u64);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingleBlockMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupBlockMetadata {
    pub device_mask: u32,
}

#[derive(Debug)]
pub struct BlockMemory<M> {
    pub id: u32,
    pub arena_id: u32,
    pub size: u64,
    metadata: M,
    pub memory: OwnedMemory,
    pub ranges: Mutex<RangeAllocator>,
}

impl<M> BlockMemory<M> {
    pub fn new(id: u32, arena_id: u32, size: u64, metadata: M, memory: OwnedMemory) -> Self {
        Self {
            id,
            arena_id,
            size,
            metadata,
            memory,
            ranges: Mutex::new(RangeAllocator::new(size)),
        }
    }

    pub fn allocate(&self, size: u64, alignment: u64) -> Option<u64> {
        self.ranges.lock().allocate(size, alignment)
    }
}

impl<M> AllocationSource for BlockMemory<M>
where
    M: Send + Sync + 'static,
{
    fn raw_memory(&self) -> vk::VkDeviceMemory {
        self.memory.raw()
    }

    fn mapped_ptr(&self) -> *mut u8 {
        self.memory.mapped_ptr()
    }

    fn free_range(&self, offset: u64, size: u64) {
        self.ranges.lock().free(offset, size);
    }
}

#[derive(Debug)]
pub struct DedicatedMemory {
    pub(crate) block_id: u32,
    pub(crate) arena_id: u32,
    pub memory: OwnedMemory,
}

impl AllocationSource for DedicatedMemory {
    fn raw_memory(&self) -> vk::VkDeviceMemory {
        self.memory.raw()
    }

    fn mapped_ptr(&self) -> *mut u8 {
        self.memory.mapped_ptr()
    }

    fn free_range(&self, _offset: u64, _size: u64) {}
}

pub type SharedSource = Arc<dyn AllocationSource>;

pub fn mapped_with_offset(source: &SharedSource, offset: u64) -> *mut u8 {
    let base = source.mapped_ptr();
    if base.is_null() {
        null_mut()
    } else {
        unsafe { base.add(offset as usize) }
    }
}
