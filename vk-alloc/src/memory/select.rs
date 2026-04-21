use crate::error::AllocatorError;
use crate::pool::PoolConfig;
use crate::resource::{AllocationCreateInfo, AllocationStrategy, MemoryTypePolicy};
use crate::vulkan::requirements::RequirementInfo;

pub(crate) fn score_memory_type(policy: MemoryTypePolicy, property_flags: u32) -> Option<i32> {
    if (property_flags & policy.required_flags) != policy.required_flags {
        return None;
    }
    let mut score = 0;
    score += ((property_flags & policy.preferred_flags).count_ones() as i32) * 16;
    score -= ((property_flags & policy.avoid_flags).count_ones() as i32) * 8;
    score += (property_flags.count_ones() as i32) * 2;
    Some(score)
}

pub(crate) fn choose_memory_type(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    alloc_info: &AllocationCreateInfo,
) -> Result<u32, AllocatorError> {
    let mut best = None;
    for index in 0..properties.memoryTypeCount {
        let mask = 1u32 << index;
        if memory_type_bits & mask == 0 {
            continue;
        }
        let property_flags = properties.memoryTypes[index as usize].propertyFlags;
        if let Some(score) = score_memory_type(alloc_info.memory_type_policy, property_flags) {
            match best {
                Some((_, best_score)) if best_score >= score => {}
                _ => best = Some((index, score)),
            }
        }
    }
    best.map(|(index, _)| index)
        .ok_or(AllocatorError::NoCompatibleMemoryType)
}

pub(crate) fn is_host_visible(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> bool {
    properties.memoryTypes[memory_type_index as usize].propertyFlags
        & vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
        != 0
}

fn is_device_local(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> bool {
    properties.memoryTypes[memory_type_index as usize].propertyFlags
        & vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0
        != 0
}

pub(crate) fn block_size_for(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
    pool: &PoolConfig,
) -> u64 {
    let host_visible = is_host_visible(properties, memory_type_index);
    let device_local = is_device_local(properties, memory_type_index);
    if host_visible {
        pool.host_visible_block_size
    } else if device_local {
        pool.device_local_block_size
    } else {
        pool.mixed_block_size
    }
}

pub(crate) fn should_dedicate(
    alloc_info: &AllocationCreateInfo,
    requirements: RequirementInfo,
    arena_block_size: u64,
) -> Result<bool, AllocatorError> {
    if requirements.dedicated_required {
        return match alloc_info.strategy {
            AllocationStrategy::NeverDedicated => Err(AllocatorError::DedicatedAllocationRequired),
            _ => Ok(true),
        };
    }
    match alloc_info.strategy {
        AllocationStrategy::AlwaysDedicated => Ok(true),
        AllocationStrategy::NeverDedicated => Ok(false),
        AllocationStrategy::Auto => {
            let threshold = alloc_info
                .dedicated_threshold
                .unwrap_or(arena_block_size / 2);
            Ok(requirements.requirements.size >= threshold || requirements.dedicated_preferred)
        }
    }
}
