use crate::group::device_mask::partition_device_mask;
use crate::group::mode::validate_group_mode;
use crate::group_allocator::GroupBindMode;
use crate::memory::range_allocator::RangeAllocator;
use crate::memory::select::{score_memory_type, should_dedicate};
use crate::resource::{AllocationCreateInfo, MemoryTypePolicy};
use crate::sparse::PageTable;
use crate::vulkan::requirements::RequirementInfo;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn memory_type_scoring_prefers_expected_flags() {
    let host_visible = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
        | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0;
    let device_local = vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0;
    let gpu_only = score_memory_type(MemoryTypePolicy::DEVICE_LOCAL, device_local).unwrap();
    let cpu_only = score_memory_type(MemoryTypePolicy::HOST_VISIBLE, host_visible).unwrap();
    let cpu_to_gpu =
        score_memory_type(MemoryTypePolicy::UPLOAD, host_visible | device_local).unwrap();

    assert!(gpu_only > score_memory_type(MemoryTypePolicy::DEVICE_LOCAL, host_visible).unwrap());
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
    let alloc = AllocationCreateInfo::new().with_dedicated_threshold(1024);
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
fn strict_fail_group_mode_validation_rejects_unsupported_modes() {
    let err = validate_group_mode(GroupBindMode::PerDeviceInstance, 0b11, 0, false, 0)
        .expect_err("mode should fail without multi-instance heap");
    assert_eq!(err, crate::AllocatorError::GroupModeUnsupported);
}

#[test]
fn per_device_instance_allows_single_device_without_multi_instance_heap() {
    validate_group_mode(GroupBindMode::PerDeviceInstance, 0b1, 0, false, 0)
        .expect("single-device mask should be valid without multi-instance heaps");
}

#[test]
fn device_mask_partitioning_respects_mask_bits() {
    assert_eq!(&*partition_device_mask(0b1011), &vec![0, 1, 3]);
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
