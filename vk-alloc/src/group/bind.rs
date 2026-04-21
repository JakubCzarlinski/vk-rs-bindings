use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::group::device_mask::partition_device_mask;
use crate::group_allocator::GroupBindMode;
use alloc::boxed::Box;

fn instance0_indices() -> Box<[u32]> {
    Box::new([0])
}

pub(crate) fn bind_buffer<'vk>(
    device: &vk::Device<'vk>,
    buffer: &vk::Buffer<'vk>,
    allocation: &Allocation,
    mode: GroupBindMode,
    device_mask: u32,
) -> Result<(), AllocatorError> {
    let device_indices = match mode {
        GroupBindMode::Instance0 => instance0_indices(),
        GroupBindMode::PerDeviceInstance => partition_device_mask(device_mask),
        GroupBindMode::SplitInstanceRegions => {
            return Err(AllocatorError::GroupModeUnsupported);
        }
    };
    let group_info = vk::VkBindBufferMemoryDeviceGroupInfo::DEFAULT
        .with_deviceIndexCount(device_indices.len() as u32)
        .with_pDeviceIndices(device_indices.as_ptr());
    let bind = vk::VkBindBufferMemoryInfo::DEFAULT
        .with_pNext((&raw const group_info).cast())
        .with_buffer(buffer.raw())
        .with_memory(allocation.memory())
        .with_memoryOffset(allocation.offset());
    device
        .vkBindBufferMemory2(1, &raw const bind)
        .map_err(AllocatorError::Vulkan)?;
    Ok(())
}

pub(crate) fn bind_image<'vk>(
    device: &vk::Device<'vk>,
    image: &vk::Image<'vk>,
    allocation: &Allocation,
    mode: GroupBindMode,
    device_mask: u32,
    split_regions: &[vk::VkRect2D],
) -> Result<(), AllocatorError> {
    let device_indices = match mode {
        GroupBindMode::Instance0 => instance0_indices(),
        GroupBindMode::PerDeviceInstance | GroupBindMode::SplitInstanceRegions => {
            partition_device_mask(device_mask)
        }
    };
    let group_info = vk::VkBindImageMemoryDeviceGroupInfo::DEFAULT
        .with_deviceIndexCount(device_indices.len() as u32)
        .with_pDeviceIndices(device_indices.as_ptr())
        .with_splitInstanceBindRegionCount(split_regions.len() as u32)
        .with_pSplitInstanceBindRegions(split_regions.as_ptr());
    let bind = vk::VkBindImageMemoryInfo::DEFAULT
        .with_pNext((&raw const group_info).cast())
        .with_image(image.raw())
        .with_memory(allocation.memory())
        .with_memoryOffset(allocation.offset());
    device
        .vkBindImageMemory2(1, &raw const bind)
        .map_err(AllocatorError::Vulkan)?;
    Ok(())
}
