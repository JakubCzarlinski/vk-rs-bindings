use crate::error::AllocatorError;
use crate::group_allocator::GroupBindMode;
use crate::internal::buddy::RangeAllocator;
use crate::internal::device_group::validate_group_mode;
use crate::internal::page_table::PageTable;
use crate::internal::vk_ops::{DeviceLimits, recommended_buffer_chunk_size, score_memory_type};
use crate::resource::MemoryTypePolicy;

pub struct BenchRangeAllocator {
    inner: RangeAllocator,
}

impl BenchRangeAllocator {
    pub fn new(size: u64) -> Self {
        Self {
            inner: RangeAllocator::new(size),
        }
    }

    pub fn allocate(&mut self, size: u64, alignment: u64) -> Option<u64> {
        self.inner.allocate(size, alignment)
    }

    pub fn free(&mut self, offset: u64, size: u64) {
        self.inner.free(offset, size);
    }
}

pub fn bench_score_memory_type(policy: MemoryTypePolicy, property_flags: u32) -> Option<i32> {
    score_memory_type(policy, property_flags)
}

pub fn bench_validate_group_mode(
    mode: GroupBindMode,
    device_mask: u32,
    heap_flags: u32,
    is_image: bool,
    split_region_count: usize,
) -> Result<(), AllocatorError> {
    validate_group_mode(mode, device_mask, heap_flags, is_image, split_region_count).map(|_| ())
}

pub struct BenchPageTable<K, V> {
    inner: PageTable<K, V>,
}

impl<K, V> Default for BenchPageTable<K, V>
where
    K: Ord,
{
    fn default() -> Self {
        Self {
            inner: PageTable::default(),
        }
    }
}

impl<K, V> BenchPageTable<K, V>
where
    K: Ord,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }
}

pub fn bench_recommended_buffer_chunk_size(
    total_size: u64,
    usage: u32,
    limits: DeviceLimits,
) -> u64 {
    recommended_buffer_chunk_size(total_size, usage, limits)
}
