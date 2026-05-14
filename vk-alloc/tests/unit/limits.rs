use crate::vulkan::limits::{DeviceLimits, validate_host_pointer_alignment};
use crate::vulkan::requirements::{buffer_usage_flags2, recommended_buffer_chunk_size};

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
            vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0,
            limits,
        ),
        1024
    );
    assert_eq!(
        recommended_buffer_chunk_size(
            1 << 20,
            vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT.0,
            limits,
        ),
        256
    );
}

#[test]
fn buffer_usage_flags2_prefers_pnext_usage() {
    let usage2 = vk::VkBufferUsageFlags2CreateInfo::DEFAULT
        .with_usage(vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0);
    let buffer_info =
        vk::VkBufferCreateInfo::DEFAULT.with_pNext_VkBufferUsageFlags2CreateInfo(&usage2);

    assert_eq!(
        buffer_usage_flags2(&buffer_info),
        vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0
    );
}

#[test]
#[allow(deprecated)]
fn buffer_usage_flags2_falls_back_to_legacy_usage() {
    let buffer_info = vk::VkBufferCreateInfo::DEFAULT
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT);

    assert_eq!(
        buffer_usage_flags2(&buffer_info),
        vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT
            .0
            .into()
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
