use core::ffi::{CStr, c_char};
use vk::*;

const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Vulkan Query Info Demo".as_ptr());

const INSTANCE_CREATE_INFO: VkInstanceCreateInfo =
    VkInstanceCreateInfo::DEFAULT.with_pApplicationInfo(&APP_INFO);

fn main() -> Result<(), String> {
    let library = VulkanLib::load().map_err(|e| e.to_string())?;
    let entry = Entry::new(&library);

    print_instance_info(&entry)?;

    let instance = entry
        .vkCreateInstance(&INSTANCE_CREATE_INFO, null())
        .map_err(|e| format!("vkCreateInstance failed: {e:?}"))?;

    print_device_groups(&instance)?;

    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|e| format!("vkEnumeratePhysicalDevices failed: {e:?}"))?;
    println!("Physical devices: {}", physical_devices.len());

    for (index, physical_device) in physical_devices.iter().enumerate() {
        print_physical_device(index, physical_device)?;
    }

    Ok(())
}

fn print_instance_info(entry: &Entry<'_>) -> Result<(), String> {
    let mut api_version = VK_API_VERSION_1_0;
    entry
        .vkEnumerateInstanceVersion(&mut api_version)
        .map_err(|e| format!("vkEnumerateInstanceVersion failed: {e:?}"))?;

    println!("Vulkan loader");
    println!("  API version: {}", format_version(api_version));
    println!();

    let layers = enumerate_instance_layers(entry)?;
    println!("Instance layers: {}", layers.len());
    for layer in &layers {
        println!(
            "  {} spec={} impl={} - {}",
            c_char_array_to_string(&layer.layerName),
            format_version(layer.specVersion),
            layer.implementationVersion,
            c_char_array_to_string(&layer.description),
        );

        let extensions = enumerate_instance_extensions(entry, layer.layerName.as_ptr())?;
        print_extensions("    layer extensions", &extensions);
    }

    let extensions = enumerate_instance_extensions(entry, null())?;
    print_extensions("Instance extensions", &extensions);
    println!();

    Ok(())
}

fn print_device_groups(instance: &Instance<'_>) -> Result<(), String> {
    let mut count = 0;
    instance
        .vkEnumeratePhysicalDeviceGroups(&mut count, null_mut())
        .map_err(|e| format!("vkEnumeratePhysicalDeviceGroups count failed: {e:?}"))?;

    let mut groups = vec![VkPhysicalDeviceGroupProperties::DEFAULT; count as usize];
    if count != 0 {
        instance
            .vkEnumeratePhysicalDeviceGroups(&mut count, groups.as_mut_ptr())
            .map_err(|e| format!("vkEnumeratePhysicalDeviceGroups failed: {e:?}"))?;
        groups.truncate(count as usize);
    }

    println!("Physical device groups: {}", groups.len());
    for (group_index, group) in groups.iter().enumerate() {
        println!(
            "  Group {group_index}: devices={}, subset_allocation={}",
            group.physicalDeviceCount,
            bool32(group.subsetAllocation),
        );
        for device_index in 0..group.physicalDeviceCount as usize {
            println!(
                "    [{device_index}] {:?}",
                group.physicalDevices[device_index],
            );
        }
    }
    println!();

    Ok(())
}

fn print_physical_device(index: usize, physical_device: &PhysicalDevice<'_>) -> Result<(), String> {
    let mut properties = VkPhysicalDeviceProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceProperties2(&mut properties);

    let mut features = VkPhysicalDeviceFeatures2::DEFAULT;
    physical_device.vkGetPhysicalDeviceFeatures2(&mut features);

    let mut memory = VkPhysicalDeviceMemoryProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceMemoryProperties2(&mut memory);

    let queue_families = queue_family_properties(physical_device);
    let extensions = enumerate_device_extensions(physical_device, null())?;
    let layers = enumerate_device_layers(physical_device)?;
    let tools = physical_device_tools(physical_device)?;

    println!(
        "Device {index}: {}",
        c_char_array_to_string(&properties.properties.deviceName)
    );
    println!("  Raw handle: {:?}", physical_device.raw());
    println!("  Type: {}", properties.properties.deviceType);
    println!(
        "  API version: {}",
        format_version(properties.properties.apiVersion)
    );
    println!("  Driver version: {}", properties.properties.driverVersion);
    println!(
        "  Vendor/device ID: {:#06x}/{:#06x}",
        properties.properties.vendorID, properties.properties.deviceID,
    );
    println!(
        "  Pipeline cache UUID: {}",
        format_uuid(&properties.properties.pipelineCacheUUID),
    );
    print_selected_limits(&properties.properties.limits);
    print_sparse_properties(&properties.properties.sparseProperties);
    print_memory_properties(&memory.memoryProperties);
    print_queue_families(&queue_families);
    print_extensions("  Device extensions", &extensions);

    println!("  Device layers: {}", layers.len());
    for layer in &layers {
        println!(
            "    {} spec={} impl={} - {}",
            c_char_array_to_string(&layer.layerName),
            format_version(layer.specVersion),
            layer.implementationVersion,
            c_char_array_to_string(&layer.description),
        );
    }

    print_tools(&tools);
    print_selected_features(&features.features);
    print_logical_device_probe(physical_device, &queue_families);
    println!();

    Ok(())
}

fn print_selected_limits(limits: &VkPhysicalDeviceLimits) {
    println!("  Selected limits");
    println!(
        "    max image dimensions: 1D={} 2D={} 3D={} cube={} array_layers={}",
        limits.maxImageDimension1D,
        limits.maxImageDimension2D,
        limits.maxImageDimension3D,
        limits.maxImageDimensionCube,
        limits.maxImageArrayLayers,
    );
    println!(
        "    max buffer ranges: uniform={} storage={}",
        limits.maxUniformBufferRange, limits.maxStorageBufferRange,
    );
    println!(
        "    max descriptors: bound_sets={} per_stage_resources={} set_sampled_images={} set_storage_buffers={}",
        limits.maxBoundDescriptorSets,
        limits.maxPerStageResources,
        limits.maxDescriptorSetSampledImages,
        limits.maxDescriptorSetStorageBuffers,
    );
    println!(
        "    max compute: workgroups={:?} workgroup_size={:?} invocations={} shared_memory={}",
        limits.maxComputeWorkGroupCount,
        limits.maxComputeWorkGroupSize,
        limits.maxComputeWorkGroupInvocations,
        limits.maxComputeSharedMemorySize,
    );
    println!(
        "    alignments: min_uniform={} min_storage={} non_coherent_atom={} buffer_image_granularity={}",
        limits.minUniformBufferOffsetAlignment,
        limits.minStorageBufferOffsetAlignment,
        limits.nonCoherentAtomSize,
        limits.bufferImageGranularity,
    );
    println!(
        "    timestamps: valid_compute_graphics={} period={} ns",
        bool32(limits.timestampComputeAndGraphics),
        limits.timestampPeriod,
    );
}

fn print_sparse_properties(sparse: &VkPhysicalDeviceSparseProperties) {
    println!("  Sparse properties");
    println!(
        "    2D={} 2D_MSAA={} 3D={} aligned_mip={} non_resident_strict={}",
        bool32(sparse.residencyStandard2DBlockShape),
        bool32(sparse.residencyStandard2DMultisampleBlockShape),
        bool32(sparse.residencyStandard3DBlockShape),
        bool32(sparse.residencyAlignedMipSize),
        bool32(sparse.residencyNonResidentStrict),
    );
}

fn print_memory_properties(memory: &VkPhysicalDeviceMemoryProperties) {
    println!("  Memory heaps: {}", memory.memoryHeapCount);
    for index in 0..memory.memoryHeapCount as usize {
        let heap = memory.memoryHeaps[index];
        println!(
            "    [{index}] size={} MiB flags={}",
            heap.size / 1024 / 1024,
            heap.flags,
        );
    }

    println!("  Memory types: {}", memory.memoryTypeCount);
    for index in 0..memory.memoryTypeCount as usize {
        let memory_type = memory.memoryTypes[index];
        println!(
            "    [{index}] heap={} flags={}",
            memory_type.heapIndex, memory_type.propertyFlags,
        );
    }
}

fn print_queue_families(queue_families: &[VkQueueFamilyProperties2<'_>]) {
    println!("  Queue families: {}", queue_families.len());
    for (index, family) in queue_families.iter().enumerate() {
        let props = family.queueFamilyProperties;
        println!(
            "    [{index}] count={} flags={} timestamp_bits={} granularity={}x{}x{}",
            props.queueCount,
            props.queueFlags,
            props.timestampValidBits,
            props.minImageTransferGranularity.width,
            props.minImageTransferGranularity.height,
            props.minImageTransferGranularity.depth,
        );
    }
}

fn print_tools(tools: &[VkPhysicalDeviceToolProperties<'_>]) {
    println!("  Active tools: {}", tools.len());
    for tool in tools {
        println!(
            "    {} {} purposes={} layer={} - {}",
            c_char_array_to_string(&tool.name),
            c_char_array_to_string(&tool.version),
            tool.purposes,
            c_char_array_to_string(&tool.layer),
            c_char_array_to_string(&tool.description),
        );
    }
}

fn print_selected_features(features: &VkPhysicalDeviceFeatures) {
    println!("  Selected features");
    println!(
        "    robust_buffer_access={} geometry_shader={} tessellation_shader={} sampler_anisotropy={}",
        bool32(features.robustBufferAccess),
        bool32(features.geometryShader),
        bool32(features.tessellationShader),
        bool32(features.samplerAnisotropy),
    );
    println!(
        "    shader_int64={} shader_float64={} sparse_binding={} multi_draw_indirect={}",
        bool32(features.shaderInt64),
        bool32(features.shaderFloat64),
        bool32(features.sparseBinding),
        bool32(features.multiDrawIndirect),
    );
}

fn print_logical_device_probe(
    physical_device: &PhysicalDevice<'_>,
    queue_families: &[VkQueueFamilyProperties2<'_>],
) {
    let Some(queue_family_index) = queue_families
        .iter()
        .position(|family| family.queueFamilyProperties.queueCount > 0)
        .map(|index| index as u32)
    else {
        println!("  Logical device probe: skipped, no queue families with queues");
        return;
    };

    const PRIORITIES: [f32; 1] = [1.0];
    let queue_info = VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_pQueuePriorities(&PRIORITIES);
    let queue_infos = [queue_info];
    let device_info = VkDeviceCreateInfo::DEFAULT.with_pQueueCreateInfos(&queue_infos);

    match physical_device.vkCreateDevice(&device_info, null()) {
        Ok(device) => {
            println!("  Logical device probe");
            println!("    Created device from queue family {queue_family_index}");
            println!(
                "    Queue 0 handle: {:?}",
                device.vkGetDeviceQueue(queue_family_index, 0).raw()
            );
        }
        Err(e) => println!("  Logical device probe: vkCreateDevice failed: {e:?}"),
    }
}

fn queue_family_properties(
    physical_device: &PhysicalDevice<'_>,
) -> Vec<VkQueueFamilyProperties2<'static>> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());

    let mut properties = vec![VkQueueFamilyProperties2::DEFAULT; count as usize];
    if count != 0 {
        physical_device
            .vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, properties.as_mut_ptr());
        properties.truncate(count as usize);
    }

    properties
}

fn enumerate_instance_layers(entry: &Entry<'_>) -> Result<Vec<VkLayerProperties>, String> {
    let mut count = 0;
    entry
        .vkEnumerateInstanceLayerProperties(&mut count, null_mut())
        .map_err(|e| format!("vkEnumerateInstanceLayerProperties count failed: {e:?}"))?;

    let mut layers = vec![VkLayerProperties::DEFAULT; count as usize];
    if count != 0 {
        entry
            .vkEnumerateInstanceLayerProperties(&mut count, layers.as_mut_ptr())
            .map_err(|e| format!("vkEnumerateInstanceLayerProperties failed: {e:?}"))?;
        layers.truncate(count as usize);
    }

    Ok(layers)
}

fn enumerate_instance_extensions(
    entry: &Entry<'_>,
    layer_name: *const c_char,
) -> Result<Vec<VkExtensionProperties>, String> {
    let mut count = 0;
    entry
        .vkEnumerateInstanceExtensionProperties(layer_name, &mut count, null_mut())
        .map_err(|e| format!("vkEnumerateInstanceExtensionProperties count failed: {e:?}"))?;

    let mut extensions = vec![VkExtensionProperties::DEFAULT; count as usize];
    if count != 0 {
        entry
            .vkEnumerateInstanceExtensionProperties(layer_name, &mut count, extensions.as_mut_ptr())
            .map_err(|e| format!("vkEnumerateInstanceExtensionProperties failed: {e:?}"))?;
        extensions.truncate(count as usize);
    }

    Ok(extensions)
}

fn enumerate_device_extensions(
    physical_device: &PhysicalDevice<'_>,
    layer_name: *const c_char,
) -> Result<Vec<VkExtensionProperties>, String> {
    let mut count = 0;
    physical_device
        .vkEnumerateDeviceExtensionProperties(layer_name, &mut count, null_mut())
        .map_err(|e| format!("vkEnumerateDeviceExtensionProperties count failed: {e:?}"))?;

    let mut extensions = vec![VkExtensionProperties::DEFAULT; count as usize];
    if count != 0 {
        physical_device
            .vkEnumerateDeviceExtensionProperties(layer_name, &mut count, extensions.as_mut_ptr())
            .map_err(|e| format!("vkEnumerateDeviceExtensionProperties failed: {e:?}"))?;
        extensions.truncate(count as usize);
    }

    Ok(extensions)
}

fn enumerate_device_layers(
    physical_device: &PhysicalDevice<'_>,
) -> Result<Vec<VkLayerProperties>, String> {
    let mut count = 0;
    physical_device
        .vkEnumerateDeviceLayerProperties(&mut count, null_mut())
        .map_err(|e| format!("vkEnumerateDeviceLayerProperties count failed: {e:?}"))?;

    let mut layers = vec![VkLayerProperties::DEFAULT; count as usize];
    if count != 0 {
        physical_device
            .vkEnumerateDeviceLayerProperties(&mut count, layers.as_mut_ptr())
            .map_err(|e| format!("vkEnumerateDeviceLayerProperties failed: {e:?}"))?;
        layers.truncate(count as usize);
    }

    Ok(layers)
}

fn physical_device_tools(
    physical_device: &PhysicalDevice<'_>,
) -> Result<Vec<VkPhysicalDeviceToolProperties<'static>>, String> {
    let mut count = 0;
    physical_device
        .vkGetPhysicalDeviceToolProperties(&mut count, null_mut())
        .map_err(|e| format!("vkGetPhysicalDeviceToolProperties count failed: {e:?}"))?;

    let mut tools = vec![VkPhysicalDeviceToolProperties::DEFAULT; count as usize];
    if count != 0 {
        physical_device
            .vkGetPhysicalDeviceToolProperties(&mut count, tools.as_mut_ptr())
            .map_err(|e| format!("vkGetPhysicalDeviceToolProperties failed: {e:?}"))?;
        tools.truncate(count as usize);
    }

    Ok(tools)
}

fn print_extensions(label: &str, extensions: &[VkExtensionProperties]) {
    println!("{label}: {}", extensions.len());
    for extension in extensions {
        println!(
            "  {} spec={}",
            c_char_array_to_string(&extension.extensionName),
            extension.specVersion,
        );
    }
}

fn c_char_array_to_string<const N: usize>(chars: &[c_char; N]) -> String {
    unsafe { CStr::from_ptr(chars.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

fn format_version(version: u32) -> String {
    format!(
        "{}.{}.{}",
        VK_VERSION_MAJOR(version),
        VK_VERSION_MINOR(version),
        VK_VERSION_PATCH(version),
    )
}

fn format_uuid(uuid: &[u8; VK_UUID_SIZE as usize]) -> String {
    uuid.iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

fn bool32(value: VkBool32) -> bool {
    value != VK_FALSE
}
