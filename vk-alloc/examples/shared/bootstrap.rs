use crate::shared::validation::{VALIDATION_LAYER, require_validation_layer};
use std::ffi::CStr;
use std::ptr::null;

pub(crate) fn create_instance<'lib>(
    entry: &'lib vk::Entry<'lib>,
    app_name: &CStr,
) -> Result<vk::Instance<'lib>, String> {
    require_validation_layer(entry)?;
    let app_info = vk::VkApplicationInfo::DEFAULT
        .with_apiVersion(vk::VK_API_VERSION_1_4)
        .with_applicationVersion(vk::VK_MAKE_VERSION(0, 1, 0))
        .with_engineVersion(vk::VK_MAKE_VERSION(0, 1, 0))
        .with_pEngineName(c"vk-alloc".as_ptr())
        .with_pApplicationName(app_name.as_ptr());
    let layer_names = [VALIDATION_LAYER.as_ptr()];
    let create_info = vk::VkInstanceCreateInfo::DEFAULT
        .with_pApplicationInfo(&raw const app_info)
        .with_enabledLayerCount(layer_names.len() as u32)
        .with_ppEnabledLayerNames(layer_names.as_ptr());
    entry
        .vkCreateInstance(&create_info, null())
        .map_err(|err| format!("vkCreateInstance failed: {err:?}"))
}

pub(crate) fn create_device<'inst>(
    physical_device: &vk::PhysicalDevice<'inst>,
    queue_family_index: u32,
) -> Result<vk::Device<'inst>, String> {
    let priorities = [1.0f32];
    let queue_info = vk::VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_queueCount(1)
        .with_pQueuePriorities(priorities.as_ptr());
    let create_info = vk::VkDeviceCreateInfo::DEFAULT
        .with_queueCreateInfoCount(1)
        .with_pQueueCreateInfos(&raw const queue_info);
    physical_device
        .vkCreateDevice(&create_info, null())
        .map_err(|err| format!("vkCreateDevice failed: {err:?}"))
}

pub(crate) fn create_group_device<'inst>(
    physical_device: &vk::PhysicalDevice<'inst>,
    raw_devices: &[vk::VkPhysicalDevice],
    queue_family_index: u32,
) -> Result<vk::Device<'inst>, String> {
    let priorities = [1.0f32];
    let queue_info = vk::VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_queueCount(1)
        .with_pQueuePriorities(priorities.as_ptr());
    let group_info = vk::VkDeviceGroupDeviceCreateInfo::DEFAULT
        .with_physicalDeviceCount(raw_devices.len() as u32)
        .with_pPhysicalDevices(raw_devices.as_ptr());
    let create_info = vk::VkDeviceCreateInfo::DEFAULT
        .with_pNext((&raw const group_info).cast())
        .with_queueCreateInfoCount(1)
        .with_pQueueCreateInfos(&raw const queue_info);
    physical_device
        .vkCreateDevice(&create_info, null())
        .map_err(|err| format!("vkCreateDevice for device group failed: {err:?}"))
}
