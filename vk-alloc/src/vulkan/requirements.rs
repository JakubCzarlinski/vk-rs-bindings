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

pub(crate) fn buffer_usage_flags2(buffer_info: &vk::VkBufferCreateInfo) -> vk::VkBufferUsageFlags2 {
    let mut next = buffer_info.pNext.cast::<vk::VkBaseInStructure>();
    while !next.is_null() {
        let base = unsafe { &*next };
        if base.sType == vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO {
            let usage_info = unsafe { &*next.cast::<vk::VkBufferUsageFlags2CreateInfo>() };
            return usage_info.usage;
        }
        next = base.pNext;
    }
    legacy_buffer_usage_flags2(buffer_info.usage)
}

#[allow(deprecated)]
fn legacy_buffer_usage_flags2(usage: vk::VkBufferUsageFlags) -> vk::VkBufferUsageFlags2 {
    vk::VkBufferUsageFlagBits2(usage.0 as u64)
}

pub(crate) fn recommended_buffer_chunk_size(
    total_size: u64,
    usage_flags: vk::VkBufferUsageFlags2,
    limits: DeviceLimits,
) -> u64 {
    let mut chunk = if limits.max_memory_allocation_size != 0 {
        limits.max_memory_allocation_size
    } else {
        total_size
    };
    if usage_flags.intersects(vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT)
        && limits.max_storage_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_storage_buffer_range));
    }
    if usage_flags.intersects(vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT)
        && limits.max_uniform_buffer_range != 0
    {
        chunk = chunk.min(u64::from(limits.max_uniform_buffer_range));
    }
    chunk.max(1)
}
