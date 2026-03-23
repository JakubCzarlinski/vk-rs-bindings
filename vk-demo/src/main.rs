use vk::{Device, Entry, VkPhysicalDevice, VkStructureType, VulkanLib};

fn main() {
    let library = VulkanLib::load();

    if library.is_err() {
        eprintln!("Failed to load Vulkan library: {:?}", library.err());
        return;
    }
    let library = library.unwrap();

    let entry = Entry::new(&library);

    let app_info = vk::VkApplicationInfo {
        sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: vk::null(),
        pApplicationName: vk::null(),
        applicationVersion: 0,
        pEngineName: vk::null(),
        engineVersion: 0,
        apiVersion: vk::VK_API_VERSION_1_4,
    };

    let layer_name = c"VK_LAYER_KHRONOS_validation";
    let layer_names = [layer_name.as_ptr()];
    let create_info = vk::VkInstanceCreateInfo {
        sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: vk::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 1,
        ppEnabledLayerNames: layer_names.as_ptr(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: vk::null(),
    };

    let instance = match entry.vkCreateInstance(&create_info, vk::null()) {
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
    let mut properties = vk::VkPhysicalDeviceProperties::DEFAULT;
    for gpu in gpu_vec.iter() {
        instance.vkGetPhysicalDeviceProperties(*gpu, &mut properties);

        let device_name = cstr_to_string(&properties.deviceName);
        println!(
            "API Version: {}.{}.{}",
            vk::VK_VERSION_MAJOR(properties.apiVersion),
            vk::VK_VERSION_MINOR(properties.apiVersion),
            vk::VK_VERSION_PATCH(properties.apiVersion)
        );
        println!(
            "Driver Version: {}.{}.{}",
            vk::VK_VERSION_MAJOR(properties.driverVersion),
            vk::VK_VERSION_MINOR(properties.driverVersion),
            vk::VK_VERSION_PATCH(properties.driverVersion)
        );
        println!("Vendor ID: {:X}", properties.vendorID);
        println!("Device ID: {:X}", properties.deviceID);
        println!("Pipeline Cache UUID: {:02X?}", properties.pipelineCacheUUID);
        println!("GPU {}", device_name);
    }

    // Create device
    let create_info = vk::VkDeviceCreateInfo::DEFAULT;
    let device: Device = match instance.vkCreateDevice(gpu_vec[0], &create_info, vk::null()) {
        Ok(dev) => dev,
        Err(err) => {
            eprintln!("Failed to create Vulkan device: {:?}", err);
            return;
        }
    };
    // pub const NULL: *const std::ffi::c_void = core::ptr::null();

    // let a: VkAllocationCallbacks = NULL;

    // Cleanup
    device.vkDestroyDevice(vk::null());
    instance.vkDestroyInstance(vk::null());
}

fn cstr_to_string(cstr: &[i8]) -> String {
    let bytes: Vec<u8> = cstr
        .iter()
        .map(|&c| c as u8)
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}
