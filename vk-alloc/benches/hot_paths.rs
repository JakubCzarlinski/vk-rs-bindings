use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use parking_lot::Mutex;
use vk_alloc::group_allocator::GroupBindMode;
use vk_alloc::{
    AllocationCreateInfo, BenchPageTable, BenchRangeAllocator, MemoryTypePolicy,
    bench_score_memory_type, bench_validate_group_mode,
};

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
            criterion::black_box(bench_score_memory_type(
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
            bench_validate_group_mode(
                GroupBindMode::PerDeviceInstance,
                0b1111,
                vk::VkMemoryHeapFlagBits::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT.0,
                false,
                0,
            )
            .unwrap();
            criterion::black_box(());
        });
    });
}

fn bench_allocation_create_info_default(c: &mut Criterion) {
    c.bench_function("allocation_create_info_const_default", |b| {
        b.iter(|| criterion::black_box(AllocationCreateInfo::new()));
    });
}

criterion_group!(
    benches,
    bench_range_allocator,
    bench_range_allocator_contended,
    bench_memory_type_scoring,
    bench_page_table,
    bench_group_validation,
    bench_allocation_create_info_default
);
criterion_main!(benches);
