use crate::vulkan::limits::{DeviceLimits, validate_host_pointer_alignment};
use crate::vulkan::requirements::recommended_buffer_chunk_size;

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
            vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0 as u32,
            limits,
        ),
        1024
    );
    assert_eq!(
        recommended_buffer_chunk_size(
            1 << 20,
            vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT.0 as u32,
            limits,
        ),
        256
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
