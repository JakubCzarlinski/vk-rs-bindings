use crate::error::AllocatorError;
use crate::group::device_mask::{partition_device_mask, validate_device_mask};
use crate::group_allocator::GroupBindMode;

pub(crate) fn validate_group_mode(
    mode: GroupBindMode,
    device_mask: u32,
    heap_flags: u32,
    is_image: bool,
    split_regions: usize,
) -> Result<(), AllocatorError> {
    validate_device_mask(device_mask)?;
    match mode {
        GroupBindMode::Instance0 => Ok(()),
        GroupBindMode::PerDeviceInstance => {
            let active_device_count = device_mask.count_ones();
            if active_device_count > 1
                && heap_flags & vk::VkMemoryHeapFlagBits::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT.0 == 0
            {
                return Err(AllocatorError::GroupModeUnsupported);
            }
            let _ = partition_device_mask(device_mask);
            Ok(())
        }
        GroupBindMode::SplitInstanceRegions => {
            if !is_image || split_regions == 0 {
                return Err(AllocatorError::InvalidSparseRegion);
            }
            let _ = partition_device_mask(device_mask);
            Ok(())
        }
    }
}
