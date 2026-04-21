use crate::shared::device_select::{device_name, find_compute_queue_family, supports_queue_family};

pub(crate) struct DeviceGroupSelection<'inst> {
    pub(crate) leader: vk::PhysicalDevice<'inst>,
    pub(crate) raw_devices: Box<[vk::VkPhysicalDevice]>,
    pub(crate) queue_family_index: u32,
}

pub(crate) fn select_device_group<'inst>(
    instance: &'inst vk::Instance<'inst>,
) -> Result<DeviceGroupSelection<'inst>, String> {
    let mut physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|err| format!("vkEnumeratePhysicalDevices failed: {err:?}"))?;
    let mut group_count = 0;
    instance
        .vkEnumeratePhysicalDeviceGroups(&raw mut group_count, std::ptr::null_mut())
        .map_err(|err| format!("vkEnumeratePhysicalDeviceGroups failed: {err:?}"))?;
    let mut groups = vec![vk::VkPhysicalDeviceGroupProperties::DEFAULT; group_count as usize];
    instance
        .vkEnumeratePhysicalDeviceGroups(&raw mut group_count, groups.as_mut_ptr())
        .map_err(|err| format!("vkEnumeratePhysicalDeviceGroups fetch failed: {err:?}"))?;

    for group in groups.into_iter().take(group_count as usize) {
        if group.physicalDeviceCount == 0 {
            continue;
        }
        let raw_devices = &group.physicalDevices[..group.physicalDeviceCount as usize];
        let Some(leader_index) = physical_devices
            .iter()
            .position(|physical_device| physical_device.raw() == raw_devices[0])
        else {
            continue;
        };
        let leader_ref = &physical_devices[leader_index];
        let Some(queue_family_index) = find_compute_queue_family(leader_ref) else {
            continue;
        };
        let all_support_queue = raw_devices.iter().all(|raw_device| {
            physical_devices
                .iter()
                .find(|physical_device| physical_device.raw() == *raw_device)
                .is_some_and(|physical_device| {
                    supports_queue_family(
                        physical_device,
                        queue_family_index,
                        vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT.0,
                    )
                })
        });
        if !all_support_queue {
            continue;
        }

        let leader_name = device_name(leader_ref);
        println!(
            "Using device group leader: {leader_name} ({} physical devices)",
            raw_devices.len()
        );
        let leader = physical_devices.swap_remove(leader_index);
        return Ok(DeviceGroupSelection {
            leader,
            raw_devices: raw_devices.to_vec().into_boxed_slice(),
            queue_family_index,
        });
    }

    Err("no compatible physical device group with a shared compute queue family was found".into())
}
