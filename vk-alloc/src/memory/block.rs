use crate::memory::owned::OwnedMemory;
use crate::memory::range_allocator::RangeAllocator;
use alloc::sync::Arc;
use core::ptr::null_mut;
use parking_lot::Mutex;

pub(crate) trait AllocationSource: Send + Sync {
    fn raw_memory(&self) -> vk::VkDeviceMemory;
    fn mapped_ptr(&self) -> *mut u8;
    fn free_range(&self, offset: u64, size: u64);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockMetadata {
    Single,
    Group { device_mask: u32 },
}

#[derive(Debug)]
pub(crate) struct BlockMemory {
    pub(crate) id: u32,
    pub(crate) arena_id: u32,
    pub(crate) size: u64,
    _metadata: BlockMetadata,
    pub(crate) memory: OwnedMemory,
    ranges: Mutex<RangeAllocator>,
}

impl BlockMemory {
    pub(crate) fn new(
        id: u32,
        arena_id: u32,
        size: u64,
        metadata: BlockMetadata,
        memory: OwnedMemory,
    ) -> Self {
        Self {
            id,
            arena_id,
            size,
            _metadata: metadata,
            memory,
            ranges: Mutex::new(RangeAllocator::new(size)),
        }
    }

    pub(crate) fn allocate(&self, size: u64, alignment: u64) -> Option<u64> {
        self.ranges.lock().allocate(size, alignment)
    }
}

impl AllocationSource for BlockMemory {
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
pub(crate) struct DedicatedMemory {
    pub(crate) memory: OwnedMemory,
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

pub(crate) type SharedSource = Arc<dyn AllocationSource>;

pub(crate) fn mapped_with_offset(source: &SharedSource, offset: u64) -> *mut u8 {
    let base = source.mapped_ptr();
    if base.is_null() {
        null_mut()
    } else {
        unsafe { base.add(offset as usize) }
    }
}
