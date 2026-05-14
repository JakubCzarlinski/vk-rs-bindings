use crate::error::AllocatorError;

#[derive(Debug, Clone, Copy)]
pub struct DeviceLimits {
    pub max_memory_allocation_size: u64,
    pub max_storage_buffer_range: u32,
    pub max_uniform_buffer_range: u32,
    pub min_imported_host_pointer_alignment: u64,
}

pub(crate) fn memory_properties(
    physical_device: &vk::PhysicalDevice<'_>,
) -> vk::VkPhysicalDeviceMemoryProperties {
    let mut props = vk::VkPhysicalDeviceMemoryProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceMemoryProperties2(&mut props);
    props.memoryProperties
}

pub(crate) fn device_limits(physical_device: &vk::PhysicalDevice<'_>) -> DeviceLimits {
    let mut host = vk::VkPhysicalDeviceExternalMemoryHostPropertiesEXT::DEFAULT;
    let mut maintenance3 = vk::VkPhysicalDeviceMaintenance3Properties::DEFAULT
        .with_pNext_chain_VkPhysicalDeviceProperties2(&mut host);
    let mut props = vk::VkPhysicalDeviceProperties2::DEFAULT
        .with_pNext_VkPhysicalDeviceMaintenance3Properties(&mut maintenance3);
    physical_device.vkGetPhysicalDeviceProperties2(&mut props);
    DeviceLimits {
        max_memory_allocation_size: maintenance3.maxMemoryAllocationSize,
        max_storage_buffer_range: props.properties.limits.maxStorageBufferRange,
        max_uniform_buffer_range: props.properties.limits.maxUniformBufferRange,
        min_imported_host_pointer_alignment: host.minImportedHostPointerAlignment,
    }
}

pub(crate) fn validate_allocation_size(
    requested_size: u64,
    max_memory_allocation_size: u64,
) -> Result<(), AllocatorError> {
    if max_memory_allocation_size != 0 && requested_size > max_memory_allocation_size {
        Err(AllocatorError::AllocationTooLarge)
    } else {
        Ok(())
    }
}

pub(crate) fn validate_host_pointer_alignment(
    host_ptr: *mut u8,
    required_alignment: u64,
) -> Result<(), AllocatorError> {
    if required_alignment <= 1 {
        return Ok(());
    }
    if (host_ptr as usize as u64).is_multiple_of(required_alignment) {
        Ok(())
    } else {
        Err(AllocatorError::InvalidHostPointerAlignment)
    }
}
