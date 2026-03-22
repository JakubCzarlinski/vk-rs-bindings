use vk_rs_bindings::{
    InstanceDispatchTable, VK_API_VERSION_1_4, VkApplicationInfo, VkInstance, VkResult, VulkanLib,
};

fn main() {
    let vk_lib = VulkanLib::load();

    if vk_lib.is_err() {
        eprintln!("Failed to load Vulkan library: {:?}", vk_lib.err());
        return;
    }
    let vk_lib = vk_lib.unwrap();

    let entry_table = vk_lib.entry_table();

    // Create Vulkan instance
    let app_info = VkApplicationInfo {
        sType: vk_rs_bindings::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: std::ptr::null(),
        pApplicationName: std::ptr::null(),
        applicationVersion: 0,
        pEngineName: std::ptr::null(),
        engineVersion: 0,
        apiVersion: VK_API_VERSION_1_4,
    };
    let create_info = vk_rs_bindings::VkInstanceCreateInfo {
        sType: vk_rs_bindings::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: std::ptr::null(),
    };

    let instance_result = entry_table.vkCreateInstance_safe(None, &create_info);
    if let Err(err) = instance_result {
        eprintln!("Failed to create Vulkan instance: {:?}", err);
        return;
    }
    let instance = instance_result.unwrap();
}
