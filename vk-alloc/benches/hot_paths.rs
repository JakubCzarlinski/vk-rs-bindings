use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use parking_lot::Mutex;
use std::collections::BTreeMap;
use vk_alloc::{
    AllocationCreateInfo, GroupBindMode, MemoryTypePolicy, Pool, SparseAllocationCreateInfo,
};

#[derive(Default)]
struct BenchPageTable<K, V> {
    pages: BTreeMap<K, V>,
}

impl<K, V> BenchPageTable<K, V>
where
    K: Ord,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.pages.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.pages.remove(key)
    }
}

struct BenchRangeAllocator {
    free_ranges: BTreeMap<u64, u64>,
}

impl BenchRangeAllocator {
    fn new(size: u64) -> Self {
        let mut free_ranges = BTreeMap::new();
        free_ranges.insert(0, size);
        Self { free_ranges }
    }

    fn allocate(&mut self, size: u64, alignment: u64) -> Option<u64> {
        let alignment = alignment.max(1);
        let candidate = self.free_ranges.iter().find_map(|(&offset, &len)| {
            let aligned = align_up(offset, alignment);
            let padding = aligned - offset;
            if len >= size + padding {
                Some((offset, len, aligned))
            } else {
                None
            }
        })?;

        self.free_ranges.remove(&candidate.0);
        if candidate.2 > candidate.0 {
            self.free_ranges
                .insert(candidate.0, candidate.2 - candidate.0);
        }
        let end = candidate.2 + size;
        let candidate_end = candidate.0 + candidate.1;
        if end < candidate_end {
            self.free_ranges.insert(end, candidate_end - end);
        }
        Some(candidate.2)
    }

    fn free(&mut self, offset: u64, size: u64) {
        let mut start = offset;
        let mut len = size;

        if let Some((&prev_start, &prev_len)) = self.free_ranges.range(..=offset).next_back()
            && prev_start + prev_len == offset
        {
            start = prev_start;
            len += prev_len;
            self.free_ranges.remove(&prev_start);
        }
        if let Some((&next_start, &next_len)) = self.free_ranges.range(offset..).next()
            && offset + size == next_start
        {
            len += next_len;
            self.free_ranges.remove(&next_start);
        }
        self.free_ranges.insert(start, len);
    }
}

const fn align_up(value: u64, alignment: u64) -> u64 {
    if alignment <= 1 {
        value
    } else {
        let rem = value % alignment;
        if rem == 0 {
            value
        } else {
            value + (alignment - rem)
        }
    }
}

fn score_memory_type(policy: MemoryTypePolicy, property_flags: u32) -> Option<i32> {
    if (property_flags & policy.required_flags) != policy.required_flags {
        return None;
    }
    let mut score = 0;
    score += ((property_flags & policy.preferred_flags).count_ones() as i32) * 16;
    score -= ((property_flags & policy.avoid_flags).count_ones() as i32) * 8;
    score += (property_flags.count_ones() as i32) * 2;
    Some(score)
}

fn validate_group_mode(
    mode: GroupBindMode,
    device_mask: u32,
    heap_flags: u32,
    is_image: bool,
    split_region_count: usize,
) -> bool {
    if device_mask == 0 {
        return false;
    }
    match mode {
        GroupBindMode::Instance0 => true,
        GroupBindMode::PerDeviceInstance => {
            heap_flags & vk::VkMemoryHeapFlagBits::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT.0 != 0
        }
        GroupBindMode::SplitInstanceRegions => is_image && split_region_count != 0,
    }
}

fn bench_range_allocator(c: &mut Criterion) {
    let mut group = c.benchmark_group("range_allocator");
    for &size in &[64u64, 256, 1024, 4096] {
        group.bench_with_input(BenchmarkId::new("alloc_free", size), &size, |b, &size| {
            b.iter(|| {
                let mut alloc = BenchRangeAllocator::new(1 << 20);
                let offset = alloc.allocate(size, 64).unwrap();
                alloc.free(offset, size);
                criterion::black_box(offset);
            });
        });
    }
    group.finish();
}

fn bench_range_allocator_contended(c: &mut Criterion) {
    c.bench_function("range_allocator_contended_mutex", |b| {
        let alloc = Mutex::new(BenchRangeAllocator::new(1 << 20));
        b.iter(|| {
            let offset = alloc.lock().allocate(256, 64).unwrap();
            alloc.lock().free(offset, 256);
            criterion::black_box(offset);
        });
    });
}

fn bench_memory_type_scoring(c: &mut Criterion) {
    let host_visible = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
        | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0;
    let device_local = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0;
    c.bench_function("memory_type_scoring", |b| {
        b.iter(|| {
            criterion::black_box(score_memory_type(
                MemoryTypePolicy::UPLOAD,
                host_visible | device_local,
            ))
        });
    });
}

fn bench_page_table(c: &mut Criterion) {
    c.bench_function("page_table_insert_remove", |b| {
        b.iter(|| {
            let mut table = BenchPageTable::<u32, u32>::default();
            for i in 0..256 {
                table.insert(i, i);
            }
            for i in 0..256 {
                criterion::black_box(table.remove(&i));
            }
        });
    });
}

fn bench_group_validation(c: &mut Criterion) {
    c.bench_function("group_mode_validation", |b| {
        b.iter(|| {
            criterion::black_box(validate_group_mode(
                GroupBindMode::PerDeviceInstance,
                0b1111,
                vk::VkMemoryHeapFlagBits::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT.0,
                false,
                0,
            ))
        });
    });
}

fn bench_builder_defaults(c: &mut Criterion) {
    c.bench_function("allocation_create_info_default", |b| {
        b.iter(|| criterion::black_box(AllocationCreateInfo::new()));
    });
    c.bench_function("sparse_allocation_create_info_builder", |b| {
        b.iter(|| {
            criterion::black_box(
                SparseAllocationCreateInfo::new()
                    .with_memory_type_policy(MemoryTypePolicy::UPLOAD)
                    .with_page_size(64 * 1024)
                    .with_pool(Pool::DEFAULT)
                    .with_group_bind_mode(GroupBindMode::PerDeviceInstance),
            )
        });
    });
}

criterion_group!(
    benches,
    bench_range_allocator,
    bench_range_allocator_contended,
    bench_memory_type_scoring,
    bench_page_table,
    bench_group_validation,
    bench_builder_defaults
);
criterion_main!(benches);
