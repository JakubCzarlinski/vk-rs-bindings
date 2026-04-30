use std::ffi::CStr;

pub(crate) fn queue_family_properties(
    physical_device: &vk::PhysicalDevice<'_>,
) -> Vec<vk::VkQueueFamilyProperties2> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&raw mut count, std::ptr::null_mut());
    let mut properties = vec![vk::VkQueueFamilyProperties2::DEFAULT; count as usize];
    physical_device
        .vkGetPhysicalDeviceQueueFamilyProperties2(&raw mut count, properties.as_mut_ptr());
    properties.truncate(count as usize);
    properties
}

pub(crate) fn supports_queue_family(
    physical_device: &vk::PhysicalDevice<'_>,
    queue_family_index: u32,
    required_flags: u32,
) -> bool {
    queue_family_properties(physical_device)
        .get(queue_family_index as usize)
        .is_some_and(|properties| {
            properties.queueFamilyProperties.queueFlags & required_flags == required_flags
        })
}

pub(crate) fn find_compute_queue_family(physical_device: &vk::PhysicalDevice<'_>) -> Option<u32> {
    queue_family_properties(physical_device)
        .iter()
        .enumerate()
        .find_map(|(index, properties)| {
            if properties.queueFamilyProperties.queueFlags
                & vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT.0
                != 0
            {
                Some(index as u32)
            } else {
                None
            }
        })
}

pub(crate) fn device_name(physical_device: &vk::PhysicalDevice<'_>) -> String {
    let mut properties = vk::VkPhysicalDeviceProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceProperties2(&mut properties);
    let name = unsafe { CStr::from_ptr(properties.properties.deviceName.as_ptr()) };
    name.to_string_lossy().into_owned()
}

pub(crate) fn select_single_device<'inst>(
    instance: &'inst vk::Instance<'inst>,
) -> Result<(vk::PhysicalDevice<'inst>, u32), String> {
    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|err| format!("vkEnumeratePhysicalDevices failed: {err:?}"))?;
    for physical_device in physical_devices {
        if let Some(queue_family_index) = find_compute_queue_family(&physical_device) {
            return Ok((physical_device, queue_family_index));
        }
    }
    Err("no physical device with a compute-capable queue family was found".into())
}
