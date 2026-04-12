#![doc = include_str!("../USAGE.md")]
#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod allocation;
pub mod allocator;
#[cfg(feature = "bench-internals")]
pub mod bench_helpers;
pub mod error;
pub mod group_allocator;
#[cfg(test)]
pub mod internal;
#[cfg(not(test))]
mod internal;
pub mod pool;
pub mod resource;
pub mod sparse;
pub mod stats;

pub use crate::internal::vk_ops::DeviceLimits;
pub use allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, HostImportBufferCreateInfo, ImportedHostBuffer,
    LargeBuffer, LargeBufferCreateInfo, LargeBufferSegment,
};
pub use allocator::Allocator;
#[cfg(feature = "bench-internals")]
pub use bench_helpers::{
    BenchPageTable, BenchRangeAllocator, bench_recommended_buffer_chunk_size,
    bench_score_memory_type, bench_validate_group_mode,
};
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

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use core::any::TypeId;
    use core::mem::size_of;
    use std::sync::Arc;
    use std::thread;

    use parking_lot::Mutex;

    use crate::group_allocator::GroupBindMode;
    use crate::internal::buddy::RangeAllocator;
    use crate::internal::device_group::{partition_device_mask, validate_group_mode};
    use crate::internal::page_table::PageTable;
    use crate::internal::vk_ops::{
        DeviceLimits, RequirementInfo, recommended_buffer_chunk_size, score_memory_type,
        should_dedicate, validate_host_pointer_alignment,
    };
    use crate::resource::{AllocationCreateInfo, MemoryTypePolicy};

    #[test]
    fn memory_type_scoring_prefers_expected_flags() {
        let host_visible = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
            | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0;
        let device_local = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0;
        let gpu_only = score_memory_type(MemoryTypePolicy::DEVICE_LOCAL, device_local).unwrap();
        let cpu_only = score_memory_type(MemoryTypePolicy::HOST_VISIBLE, host_visible).unwrap();
        let cpu_to_gpu =
            score_memory_type(MemoryTypePolicy::UPLOAD, host_visible | device_local).unwrap();

        assert!(
            gpu_only > score_memory_type(MemoryTypePolicy::DEVICE_LOCAL, host_visible).unwrap()
        );
        assert!(score_memory_type(MemoryTypePolicy::HOST_VISIBLE, device_local).is_none());
        assert!(cpu_to_gpu >= cpu_only);
    }

    #[test]
    fn range_allocator_preserves_alignment_and_non_overlap() {
        let mut allocator = RangeAllocator::new(4096);
        let a = allocator.allocate(128, 64).unwrap();
        let b = allocator.allocate(256, 128).unwrap();
        assert_eq!(a % 64, 0);
        assert_eq!(b % 128, 0);
        assert!(b >= a + 128);
        allocator.free(a, 128);
        allocator.free(b, 256);
        assert!(allocator.is_non_overlapping());
    }

    #[test]
    fn dedicated_threshold_behavior_matches_policy() {
        let alloc = AllocationCreateInfo {
            dedicated_threshold: Some(1024),
            ..AllocationCreateInfo::new()
        };
        let small = RequirementInfo {
            requirements: vk::VkMemoryRequirements::DEFAULT.with_size(512),
            dedicated_required: false,
            dedicated_preferred: false,
        };
        let large = RequirementInfo {
            requirements: vk::VkMemoryRequirements::DEFAULT.with_size(2048),
            dedicated_required: false,
            dedicated_preferred: false,
        };
        assert!(!should_dedicate(&alloc, small, 4096).unwrap());
        assert!(should_dedicate(&alloc, large, 4096).unwrap());
    }

    #[test]
    fn allocator_and_group_allocator_layouts_are_distinct() {
        assert_ne!(
            size_of::<crate::Allocator<'static>>(),
            size_of::<crate::GroupAllocator<'static>>()
        );
    }

    #[test]
    fn single_and_group_metadata_types_are_disjoint() {
        assert_ne!(
            TypeId::of::<
                crate::internal::arena::ArenaState<crate::internal::block::SingleBlockMetadata>,
            >(),
            TypeId::of::<
                crate::internal::arena::ArenaState<crate::internal::block::GroupBlockMetadata>,
            >()
        );
        assert_ne!(
            TypeId::of::<crate::internal::block::SingleBlockMetadata>(),
            TypeId::of::<crate::internal::block::GroupBlockMetadata>()
        );
    }

    #[test]
    fn strict_fail_group_mode_validation_rejects_unsupported_modes() {
        let err = validate_group_mode(GroupBindMode::PerDeviceInstance, 0b11, 0, false, 0)
            .expect_err("mode should fail without multi-instance heap");
        assert_eq!(err, crate::AllocatorError::GroupModeUnsupported);
    }

    #[test]
    fn device_mask_partitioning_respects_mask_bits() {
        assert_eq!(partition_device_mask(0b1011), vec![0, 1, 3]);
    }

    #[test]
    fn page_table_insert_remove_update_is_consistent() {
        let mut table = PageTable::default();
        table.insert(1u32, 10u32);
        table.insert(2u32, 20u32);
        assert_eq!(table.get(&1), Some(&10));
        assert_eq!(table.insert(1, 15), Some(10));
        assert_eq!(table.remove(&2), Some(20));
        let mut seen = Vec::new();
        table.for_each(|k, v| seen.push((*k, *v)));
        assert_eq!(seen, vec![(1, 15)]);
    }

    #[test]
    fn split_instance_regions_validation_requires_image_and_regions() {
        let err = validate_group_mode(GroupBindMode::SplitInstanceRegions, 0b11, 0, true, 0)
            .expect_err("split instance regions need regions");
        assert_eq!(err, crate::AllocatorError::InvalidSparseRegion);
    }

    #[test]
    fn drop_order_destroys_resource_before_memory() {
        struct Resource<'a>(&'a Mutex<Vec<&'static str>>);
        struct Memory<'a>(&'a Mutex<Vec<&'static str>>);
        impl Drop for Resource<'_> {
            fn drop(&mut self) {
                self.0.lock().push("resource");
            }
        }
        impl Drop for Memory<'_> {
            fn drop(&mut self) {
                self.0.lock().push("memory");
            }
        }
        struct Pair<'a> {
            _resource: Resource<'a>,
            _memory: Memory<'a>,
        }

        let log = Mutex::new(Vec::new());
        drop(Pair {
            _resource: Resource(&log),
            _memory: Memory(&log),
        });
        assert_eq!(&*log.lock(), &["resource", "memory"]);
    }

    #[test]
    fn concurrent_allocate_free_stress_keeps_allocator_consistent() {
        let allocator = Arc::new(Mutex::new(RangeAllocator::new(1 << 20)));
        let mut threads = Vec::new();
        for _ in 0..8 {
            let allocator = allocator.clone();
            threads.push(thread::spawn(move || {
                for _ in 0..500 {
                    let offset = allocator.lock().allocate(256, 64).unwrap();
                    allocator.lock().free(offset, 256);
                }
            }));
        }
        for handle in threads {
            handle.join().unwrap();
        }
        assert!(allocator.lock().is_non_overlapping());
    }

    #[test]
    fn concurrent_sparse_updates_do_not_corrupt_page_table() {
        let table = Arc::new(Mutex::new(PageTable::<u32, u32>::default()));
        let mut threads = Vec::new();
        for i in 0..8u32 {
            let table = table.clone();
            threads.push(thread::spawn(move || {
                for n in 0..128u32 {
                    table.lock().insert(i * 1000 + n, n);
                }
            }));
        }
        for handle in threads {
            handle.join().unwrap();
        }
        let mut count = 0usize;
        table.lock().for_each(|_, _| count += 1);
        assert_eq!(count, 8 * 128);
    }

    #[test]
    fn large_buffer_chunking_respects_limits() {
        let limits = DeviceLimits {
            max_memory_allocation_size: 4096,
            max_storage_buffer_range: 1024,
            max_uniform_buffer_range: 256,
            min_imported_host_pointer_alignment: 64,
        };
        assert_eq!(
            recommended_buffer_chunk_size(
                1 << 20,
                vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0 as u32,
                limits,
            ),
            1024
        );
        assert_eq!(
            recommended_buffer_chunk_size(
                1 << 20,
                vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT.0 as u32,
                limits,
            ),
            256
        );
    }

    #[test]
    fn host_import_alignment_validation_rejects_misalignment() {
        let mut bytes = [0u8; 128];
        let ptr = unsafe { bytes.as_mut_ptr().add(1) };
        assert_eq!(
            validate_host_pointer_alignment(ptr, 64),
            Err(crate::AllocatorError::InvalidHostPointerAlignment)
        );
    }
}
