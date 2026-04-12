use alloc::vec::Vec;

use crate::error::AllocatorError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupValidation {
    pub device_mask: u32,
    pub device_indices_len: u32,
}

pub fn validate_device_mask(device_mask: u32) -> Result<(), AllocatorError> {
    if device_mask == 0 {
        Err(AllocatorError::InvalidDeviceMask)
    } else {
        Ok(())
    }
}

pub fn partition_device_mask(device_mask: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for bit in 0..32 {
        let mask = 1u32 << bit;
        if device_mask & mask != 0 {
            out.push(bit);
        }
    }
    out
}

pub fn validate_group_mode(
    mode: crate::group_allocator::GroupBindMode,
    device_mask: u32,
    heap_flags: u32,
    is_image: bool,
    split_regions: usize,
) -> Result<GroupValidation, AllocatorError> {
    validate_device_mask(device_mask)?;
    match mode {
        crate::group_allocator::GroupBindMode::Instance0 => Ok(GroupValidation {
            device_mask,
            device_indices_len: 1,
        }),
        crate::group_allocator::GroupBindMode::PerDeviceInstance => {
            if heap_flags & vk::VkMemoryHeapFlagBits::VK_MEMORY_HEAP_MULTI_INSTANCE_BIT.0 == 0 {
                return Err(AllocatorError::GroupModeUnsupported);
            }
            Ok(GroupValidation {
                device_mask,
                device_indices_len: partition_device_mask(device_mask).len() as u32,
            })
        }
        crate::group_allocator::GroupBindMode::SplitInstanceRegions => {
            if !is_image || split_regions == 0 {
                return Err(AllocatorError::InvalidSparseRegion);
            }
            Ok(GroupValidation {
                device_mask,
                device_indices_len: partition_device_mask(device_mask).len() as u32,
            })
        }
    }
}
