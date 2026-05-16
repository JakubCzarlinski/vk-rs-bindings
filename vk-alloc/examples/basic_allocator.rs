#[path = "shared/mod.rs"]
mod shared;

use shared::bootstrap::{create_device, create_instance};
use shared::device_select::{device_name, select_single_device};
use vk::{
    Entry, VkBufferCreateInfo, VkBufferUsageFlagBits2, VkBufferUsageFlags2CreateInfo,
    VkSharingMode, VulkanLib,
};
use vk_alloc::{
    AllocationCreateInfo, Allocator, AllocatorCreateInfo, MemoryTypePolicy, PoolCreateInfo,
};

fn main() -> Result<(), String> {
    let library = VulkanLib::load().map_err(|err| format!("failed to load Vulkan: {err:?}"))?;
    let entry = Entry::new(&library);
    let instance = create_instance(&entry, c"vk-alloc basic allocator example")?;
    {
        let (physical_device, queue_family_index) = select_single_device(&instance)?;
        let physical_device_name = device_name(&physical_device);
        let device = create_device(&physical_device, queue_family_index)?;

        println!("Selected device: {physical_device_name}");
        println!("Queue family index: {queue_family_index}");

        let allocator_info = AllocatorCreateInfo::new(&physical_device, &device).with_default_pool(
            PoolCreateInfo::new()
                .with_host_visible_block_size(4 * 1024 * 1024)
                .with_device_local_block_size(16 * 1024 * 1024)
                .with_dedicated_threshold(8 * 1024 * 1024),
        );
        let allocator =
            Allocator::from_create_info(allocator_info).map_err(|err| format!("{err:?}"))?;
        {
            const STAGING_USAGE: VkBufferUsageFlags2CreateInfo<'_> =
                VkBufferUsageFlags2CreateInfo::DEFAULT
                    .with_usage(VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT);
            const STAGING_INFO: VkBufferCreateInfo<'_> = VkBufferCreateInfo::DEFAULT
                .with_size(4096)
                .with_pNext_VkBufferUsageFlags2CreateInfo(&STAGING_USAGE)
                .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);
            const DEVICE_LOCAL_USAGE: VkBufferUsageFlags2CreateInfo<'_> =
                VkBufferUsageFlags2CreateInfo::DEFAULT
                    .with_usage(VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_TRANSFER_DST_BIT);
            const DEVICE_LOCAL_INFO: VkBufferCreateInfo<'_> = VkBufferCreateInfo::DEFAULT
                .with_size(4096)
                .with_pNext_VkBufferUsageFlags2CreateInfo(&DEVICE_LOCAL_USAGE)
                .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);

            let mut staging = allocator
                .create_buffer(
                    &STAGING_INFO,
                    AllocationCreateInfo::new()
                        .with_memory_type_policy(MemoryTypePolicy::UPLOAD)
                        .with_dedicated_threshold(1024 * 1024),
                )
                .map_err(|err| format!("failed to create staging buffer: {err:?}"))?;
            let slice = staging
                .allocation_mut()
                .mapped_slice_mut::<u32>(4)
                .ok_or("staging allocation was not host visible")?;
            slice.copy_from_slice(&[1, 2, 3, 4]);

            let device_local = allocator
                .create_buffer(
                    &DEVICE_LOCAL_INFO,
                    AllocationCreateInfo::new()
                        .with_memory_type_policy(MemoryTypePolicy::DEVICE_LOCAL),
                )
                .map_err(|err| format!("failed to create device-local buffer: {err:?}"))?;

            // Note the manual drop is usally not necessary, but we want to see the stats change as we drop the buffers.
            println!("Allocator stats: {:?}", allocator.stats());
            drop(staging);
            println!("Allocator stats: {:?}", allocator.stats());
            drop(device_local);
            println!("Allocator stats: {:?}", allocator.stats());
        }
    }
    Ok(())
}
