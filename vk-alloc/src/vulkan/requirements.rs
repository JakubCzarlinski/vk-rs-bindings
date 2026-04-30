use crate::vulkan::limits::DeviceLimits;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RequirementInfo {
    pub(crate) requirements: vk::VkMemoryRequirements,
    pub(crate) dedicated_required: bool,
    pub(crate) dedicated_preferred: bool,
}

pub(crate) fn buffer_requirements(
    device: &vk::Device<'_>,
    buffer: &vk::Buffer<'_>,
) -> RequirementInfo {
    let info = vk::VkBufferMemoryRequirementsInfo2::DEFAULT.with_buffer(buffer.raw());
    let mut dedicated = vk::VkMemoryDedicatedRequirements::DEFAULT;
    let mut req = vk::VkMemoryRequirements2::DEFAULT
        .with_pNext((&raw mut dedicated).cast::<core::ffi::c_void>());
    device.vkGetBufferMemoryRequirements2(&info, &mut req);
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation == vk::VK_TRUE,
        dedicated_preferred: dedicated.prefersDedicatedAllocation == vk::VK_TRUE,
    }
}

pub(crate) fn image_requirements(
    device: &vk::Device<'_>,
    image: &vk::Image<'_>,
) -> RequirementInfo {
    let info = vk::VkImageMemoryRequirementsInfo2::DEFAULT.with_image(image.raw());
    let mut dedicated = vk::VkMemoryDedicatedRequirements::DEFAULT;
    let mut req = vk::VkMemoryRequirements2::DEFAULT
        .with_pNext((&raw mut dedicated).cast::<core::ffi::c_void>());
    device.vkGetImageMemoryRequirements2(&info, &mut req);
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation == vk::VK_TRUE,
        dedicated_preferred: dedicated.prefersDedicatedAllocation == vk::VK_TRUE,
    }
}

pub(crate) fn recommended_buffer_chunk_size(
    total_size: u64,
    usage_flags: u32,
    limits: DeviceLimits,
) -> u64 {
    let mut chunk = if limits.max_memory_allocation_size != 0 {
        limits.max_memory_allocation_size
    } else {
        total_size
    };
    if usage_flags & (vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0 as u32)
        != 0
        && limits.max_storage_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_storage_buffer_range));
    }
    if usage_flags & (vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT.0 as u32)
        != 0
        && limits.max_uniform_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_uniform_buffer_range));
    }
    chunk.max(1)
}
