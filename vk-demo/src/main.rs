use std::ffi::CStr;
use vk::{
    Device, Entry, Instance, PhysicalDevice, VkDeviceCreateInfo, VkInstanceCreateInfo,
    VkStructureType, VulkanLib,
};

const APP_INFO: vk::VkApplicationInfo = vk::VkApplicationInfo {
    sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
    pNext: vk::null(),
    pApplicationName: vk::null(),
    applicationVersion: 0,
    pEngineName: vk::null(),
    engineVersion: 0,
    apiVersion: vk::VK_API_VERSION_1_4,
};
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
const LAYER_NAMES: [*const i8; 1] = [VALIDATION_LAYER.as_ptr()];
const INSTANCE_CREATE_INFO: VkInstanceCreateInfo = VkInstanceCreateInfo {
    sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
    pNext: vk::null(),
    flags: 0,
    pApplicationInfo: &APP_INFO,
    enabledLayerCount: 1,
    ppEnabledLayerNames: LAYER_NAMES.as_ptr(),
    enabledExtensionCount: 0,
    ppEnabledExtensionNames: vk::null(),
};
const DEVICE_CREATE_INFO: VkDeviceCreateInfo = vk::VkDeviceCreateInfo::DEFAULT;

fn main() {
    let library = match VulkanLib::load() {
        Ok(lib) => lib,
        Err(err) => {
            eprintln!("Failed to load Vulkan library: {:?}", err);
            return;
        }
    };

    let entry = Entry::new(&library);
    let instance: Instance = match entry.vkCreateInstance(&INSTANCE_CREATE_INFO, vk::null()) {
        Ok(inst) => inst,
        Err(err) => {
            eprintln!("Failed to create Vulkan instance: {:?}", err);
            return;
        }
    };

    let physical_devices: Vec<PhysicalDevice> = match instance.vkEnumeratePhysicalDevices() {
        Ok(devs) => devs,
        Err(err) => {
            eprintln!("Failed to enumerate physical devices: {:?}", err);
            return;
        }
    };

    println!("Found {} physical devices:", physical_devices.len());
    for (i, device) in physical_devices.iter().enumerate() {
        let mut props = vk::VkPhysicalDeviceProperties2::DEFAULT;
        device.vkGetPhysicalDeviceProperties2(&mut props);
        let device_name = cstr_to_string(&props.properties.deviceName);
        println!("  {}: {}", i, device_name);
    }

    let device = match physical_devices.first() {
        Some(dev) => dev,
        None => {
            eprintln!("No physical devices found");
            return;
        }
    };

    // Get family properties of the physical device
    let mut queue_family_count = 0;
    device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut queue_family_count, vk::null_mut());
    if queue_family_count == 0 {
        eprintln!("No queue families found");
        return;
    }
    let mut queue_families =
        vec![vk::VkQueueFamilyProperties2::DEFAULT; queue_family_count as usize];
    device.vkGetPhysicalDeviceQueueFamilyProperties2(
        &mut queue_family_count,
        queue_families.as_mut_ptr(),
    );
    println!("Found {} queue families", queue_family_count);

    // Get a queue from the logical device
    let mut queue_count = 0;
    device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut queue_count, vk::null_mut());
    if queue_count == 0 {
        eprintln!("No queue families found");
        return;
    }
    let mut queue_families = vec![vk::VkQueueFamilyProperties2::DEFAULT; queue_count as usize];
    device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut queue_count, queue_families.as_mut_ptr());
    let mut queue_family_index = None;
    for (i, family) in queue_families.iter().enumerate() {
        let flags = family.queueFamilyProperties.queueFlags;
        if vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT & flags != vk::VkQueueFlagBits::EMPTY {
            queue_family_index = Some(i as u32);
            break;
        }
    }

    let queue_family_index = match queue_family_index {
        Some(index) => index,
        None => {
            eprintln!("No suitable queue family found");
            return;
        }
    };

    let logical_device: Device = match device.vkCreateDevice(&DEVICE_CREATE_INFO, vk::null()) {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to create logical device: {:?}", err);
            return;
        }
    };
    println!("Logical device created successfully");

    // Create a queue from the logical device
    let _queue = logical_device.vkGetDeviceQueue(queue_family_index, 0);
    println!("Queue obtained successfully");

    // Create a command pool
    let mut command_info = vk::VkCommandPoolCreateInfo::DEFAULT;
    command_info.queueFamilyIndex = queue_family_index;
    let _command_pool = match logical_device.vkCreateCommandPool(&command_info, vk::null()) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to create command pool: {:?}", err);
            return;
        }
    };
    println!("Command pool created successfully");
}

fn cstr_to_string(cstr: &[i8]) -> String {
    let bytes: Vec<u8> = cstr
        .iter()
        .map(|&c| c as u8)
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}
