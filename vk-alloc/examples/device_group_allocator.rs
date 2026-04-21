#[path = "shared/mod.rs"]
mod shared;

use shared::bootstrap::{create_group_device, create_instance};
use shared::device_select::device_name;
use shared::group_select::select_device_group;
use std::ptr::null;
use vk_alloc::{
    AllocationCreateInfo, GroupAllocator, GroupAllocatorCreateInfo, GroupBindMode,
    MemoryTypePolicy, PoolCreateInfo,
};

fn main() -> Result<(), String> {
    let library = vk::VulkanLib::load().map_err(|err| format!("failed to load Vulkan: {err:?}"))?;
    let entry = vk::Entry::new(&library);
    let mut instance = create_instance(&entry, c"vk-alloc device-group allocator example")?;
    {
        let group = select_device_group(&instance)?;
        let leader_name = device_name(&group.leader);
        let mut device =
            create_group_device(&group.leader, &group.raw_devices, group.queue_family_index)?;
        let device_mask = if group.raw_devices.len() >= u32::BITS as usize {
            u32::MAX
        } else {
            (1u32 << group.raw_devices.len()) - 1
        };

        println!(
            "Selected device group leader: {leader_name} ({} physical devices)",
            group.raw_devices.len()
        );
        println!("Queue family index: {}", group.queue_family_index);
        println!("Device mask: {device_mask:#b}");

        {
            let allocator_info = GroupAllocatorCreateInfo::new(&group.leader, &device, device_mask)
                .with_base(
                    vk_alloc::AllocatorCreateInfo::new(&group.leader, &device).with_default_pool(
                        PoolCreateInfo::new()
                            .with_device_local_block_size(32 * 1024 * 1024)
                            .with_dedicated_threshold(16 * 1024 * 1024),
                    ),
                );
            let allocator = GroupAllocator::from_create_info(allocator_info)
                .map_err(|err| format!("{err:?}"))?;
            println!("Group allocator stats: {:?}", allocator.stats());

            let usage = vk::VkBufferUsageFlags2CreateInfo::DEFAULT
                .with_usage(vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0);
            let buffer_info = vk::VkBufferCreateInfo::DEFAULT
                .with_size(1024 * 16) // 8 KB
                .with_pNext((&raw const usage).cast())
                .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);
            let buffer = allocator
                .create_buffer_with_mode(
                    &buffer_info,
                    AllocationCreateInfo::new()
                        .with_memory_type_policy(MemoryTypePolicy::DEVICE_LOCAL)
                        .with_group_bind_mode(GroupBindMode::PerDeviceInstance),
                )
                .map_err(|err| format!("failed to create device-group buffer: {err:?}"))?;

            println!("Group allocator stats: {:?}", allocator.stats());
            std::thread::sleep(std::time::Duration::from_secs(5));
            drop(buffer);
            println!("Group allocator stats: {:?}", allocator.stats());

            drop(allocator);
        }

        device.vkDestroyDevice(null());
    }
    instance.vkDestroyInstance(null());
    Ok(())
}
