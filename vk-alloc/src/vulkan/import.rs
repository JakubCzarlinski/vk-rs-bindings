use crate::allocation::{HostImportBufferCreateInfo, ImportedHostBuffer};
use crate::error::AllocatorError;
use crate::memory::select::choose_memory_type;
use crate::resource::AllocationCreateInfo;
use crate::vulkan::limits::{
    DeviceLimits, validate_allocation_size, validate_host_pointer_alignment,
};
use crate::vulkan::requirements::buffer_requirements;

pub(crate) fn import_host_buffer<'vk>(
    device: &'vk vk::Device<'vk>,
    memory_properties: &vk::VkPhysicalDeviceMemoryProperties,
    limits: &DeviceLimits,
    buffer_info: &vk::VkBufferCreateInfo,
    import_info: HostImportBufferCreateInfo,
    alloc_info: AllocationCreateInfo,
) -> Result<ImportedHostBuffer<'vk>, AllocatorError> {
    if import_info.host_ptr.is_null() || import_info.size == 0 {
        return Err(AllocatorError::OutOfBounds);
    }
    if buffer_info.size != import_info.size {
        return Err(AllocatorError::InvalidHostImport);
    }
    validate_host_pointer_alignment(
        import_info.host_ptr,
        limits.min_imported_host_pointer_alignment,
    )?;
    validate_allocation_size(import_info.size, limits.max_memory_allocation_size)?;

    let external_memory_info =
        vk::VkExternalMemoryBufferCreateInfo::DEFAULT.with_handleTypes(import_info.handle_type.0);
    let buffer_info = (*buffer_info).with_pNext((&raw const external_memory_info).cast());
    let buffer = device
        .vkCreateBuffer(&buffer_info, vk::null())
        .map_err(AllocatorError::Vulkan)?;
    let requirement = buffer_requirements(device, &buffer);
    let mut host_props = vk::VkMemoryHostPointerPropertiesEXT::DEFAULT;
    device
        .vkGetMemoryHostPointerPropertiesEXT(
            import_info.handle_type,
            import_info.host_ptr.cast(),
            &mut host_props,
        )
        .map_err(AllocatorError::Vulkan)?;
    let memory_type_index = choose_memory_type(
        memory_properties,
        host_props.memoryTypeBits & requirement.requirements.memoryTypeBits,
        &alloc_info,
    )?;
    let import = vk::VkImportMemoryHostPointerInfoEXT::DEFAULT
        .with_handleType(import_info.handle_type)
        .with_pHostPointer(import_info.host_ptr.cast());
    let allocate_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_pNext((&raw const import).cast())
        .with_allocationSize(requirement.requirements.size)
        .with_memoryTypeIndex(memory_type_index);
    let memory = device
        .vkAllocateMemory(&allocate_info, vk::null())
        .map_err(AllocatorError::Vulkan)?;
    buffer
        .vkBindBufferMemory(memory.raw(), 0)
        .map_err(AllocatorError::Vulkan)?;
    Ok(ImportedHostBuffer::new(
        buffer,
        memory,
        import_info.host_ptr,
        import_info.size,
    ))
}
