use std::ffi::CStr;
use vk::{
    Entry, VkDeviceCreateInfo, VkInstanceCreateInfo, VkPhysicalDevice, VkStructureType, VulkanLib,
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
    let instance = match entry.vkCreateInstance(&INSTANCE_CREATE_INFO, vk::null()) {
        Ok(inst) => inst,
        Err(err) => {
            eprintln!("Failed to create Vulkan instance: {:?}", err);
            return;
        }
    };

    let mut num_gpus = 0;
    if let Err(err) = instance.vkEnumeratePhysicalDevices(&mut num_gpus, std::ptr::null_mut()) {
        eprintln!("Failed to enumerate physical devices: {:?}", err);
    } else if num_gpus == 0 {
        eprintln!("No physical devices found.");
    }
    println!("Number of physical devices: {}", num_gpus);

    let gpu_vec = vec![VkPhysicalDevice::default(); num_gpus as usize];
    let gpus: *mut VkPhysicalDevice = gpu_vec.as_ptr() as *mut VkPhysicalDevice;
    if let Err(err) = instance.vkEnumeratePhysicalDevices(&mut num_gpus, gpus) {
        eprintln!("Failed to enumerate physical devices: {:?}", err);
    }

    // Get the physical device properties of each GPU
    let mut props_outer = vk::VkPhysicalDeviceProperties2::DEFAULT;
    for gpu in gpu_vec.iter() {
        instance.vkGetPhysicalDeviceProperties2(*gpu, &mut props_outer);
        let props = props_outer.properties;

        let device_name = cstr_to_string(&props.deviceName);
        println!(
            "API Version: {}.{}.{}",
            vk::VK_VERSION_MAJOR(props.apiVersion),
            vk::VK_VERSION_MINOR(props.apiVersion),
            vk::VK_VERSION_PATCH(props.apiVersion)
        );
        println!(
            "Driver Version: {}.{}.{}",
            vk::VK_VERSION_MAJOR(props.driverVersion),
            vk::VK_VERSION_MINOR(props.driverVersion),
            vk::VK_VERSION_PATCH(props.driverVersion)
        );
        println!("Vendor ID: {:X}", props.vendorID);
        println!("Device ID: {:X}", props.deviceID);
        println!("Pipeline Cache UUID: {:02X?}", props.pipelineCacheUUID);
        println!("GPU {}", device_name);
    }

    // Create device
    let _device = match instance.vkCreateDevice(gpu_vec[0], &DEVICE_CREATE_INFO, vk::null()) {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to create Vulkan device: {:?}", err);
            return;
        }
    };
}

fn cstr_to_string(cstr: &[i8]) -> String {
    let bytes: Vec<u8> = cstr
        .iter()
        .map(|&c| c as u8)
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}
