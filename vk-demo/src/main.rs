#![allow(deprecated, unused_variables, unused_mut, unused_imports, dead_code)]

use core::ffi::CStr;
use std::ffi::c_void;
use vk::{
    CommandBuffer, Device, Entry, Instance, PhysicalDevice, Queue, VkDeviceCreateInfo,
    VkInstanceCreateInfo, VulkanLib,
};

// Minimal SPIR-V compute shader: result = a + b
const COMPUTE_SHADER_SPV: &[u8] = include_bytes!("shader.spv");

const APP_INFO: vk::VkApplicationInfo =
    vk::VkApplicationInfo::DEFAULT.with_apiVersion(vk::VK_API_VERSION_1_3);
// const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
// const INSTANCE_EXTENSIONS: [*const i8; 1] = [vk::VK_EXT_METAL_SURFACE_EXTENSION_NAME.as_ptr()];
// const LAYER_NAMES: [*const i8; 0] = []; // [VALIDATION_LAYER.as_ptr()];
const INSTANCE_CREATE_INFO: VkInstanceCreateInfo = VkInstanceCreateInfo::DEFAULT
    // .with_enabledLayerCount(LAYER_NAMES.len() as u32)
    .with_pApplicationInfo(&APP_INFO);
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
    let mut queue_family_index = None;
    for (i, family) in queue_families.iter().enumerate() {
        let flags = family.queueFamilyProperties.queueFlags;
        println!(
            "Queue family {}: flags = {:?} count = {}",
            i, flags, family.queueFamilyProperties.queueCount
        );
        if vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT & flags == vk::VkQueueFlagBits::EMPTY {
            continue;
        }

        queue_family_index = Some(i as u32);
        break;
    }

    let queue_family_index = match queue_family_index {
        Some(index) => {
            println!("Selected queue family index: {}", index);
            index
        }
        None => {
            eprintln!("No suitable queue family found");
            return;
        }
    };

    const QUEUE_PRIORITIES: [f32; 1] = [1.0f32];
    let queue_create_info = vk::VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_queueCount(QUEUE_PRIORITIES.len() as u32)
        .with_pQueuePriorities(QUEUE_PRIORITIES.as_ptr());

    // Create a logical device with the selected queue family
    // let extensions: [*const i8; 1] = [vk::VK_KHR_SWAPCHAIN_EXTENSION_NAME.as_ptr()];
    let logical_device: Device = match device.vkCreateDevice(
        &DEVICE_CREATE_INFO
            .with_queueCreateInfoCount(1)
            .with_pQueueCreateInfos(&queue_create_info),
        // .with_enabledExtensionCount(extensions.len() as u32)
        // .with_ppEnabledExtensionNames(extensions.as_ptr()),
        vk::null(),
    ) {
        Ok(dev) => {
            println!("Logical device created successfully");
            dev
        }
        Err(err) => {
            eprintln!("Failed to create logical device: {:?}", err);
            return;
        }
    };
    let compute_shader_u32: &[u32] = unsafe {
        std::slice::from_raw_parts(
            COMPUTE_SHADER_SPV.as_ptr() as *const u32,
            COMPUTE_SHADER_SPV.len() / 4,
        )
    };

    let shader_module_create_info = vk::VkShaderModuleCreateInfo::DEFAULT
        .with_codeSize(COMPUTE_SHADER_SPV.len())
        .with_pCode(compute_shader_u32.as_ptr());

    let shader_module =
        match logical_device.vkCreateShaderModule(&shader_module_create_info, vk::null()) {
            Ok(sm) => {
                println!("Shader module created successfully");
                sm
            }
            Err(err) => {
                eprintln!("Failed to create shader module: {:?}", err);
                return;
            }
        };

    // 2. Create descriptor set layout
    let bindings = [
        // Input buffer binding
        vk::VkDescriptorSetLayoutBinding::DEFAULT
            .with_binding(0)
            .with_descriptorType(vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_stageFlags(vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT.0),
        // Output buffer binding
        vk::VkDescriptorSetLayoutBinding::DEFAULT
            .with_binding(1)
            .with_descriptorType(vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_stageFlags(vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT.0),
    ];

    let descriptor_set_layout_create_info = vk::VkDescriptorSetLayoutCreateInfo::DEFAULT
        .with_bindingCount(bindings.len() as u32)
        .with_pBindings(bindings.as_ptr());

    let descriptor_set_layout = match logical_device
        .vkCreateDescriptorSetLayout(&descriptor_set_layout_create_info, vk::null())
    {
        Ok(dsl) => {
            println!("Descriptor set layout created successfully");
            dsl
        }
        Err(err) => {
            eprintln!("Failed to create descriptor set layout: {:?}", err);
            return;
        }
    };

    // 3. Create pipeline layout
    let pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo::DEFAULT
        .with_setLayoutCount(1)
        .with_pSetLayouts(&descriptor_set_layout);

    let pipeline_layout =
        match logical_device.vkCreatePipelineLayout(&pipeline_layout_create_info, vk::null()) {
            Ok(pl) => {
                println!("Pipeline layout created successfully");
                pl
            }
            Err(err) => {
                eprintln!("Failed to create pipeline layout: {:?}", err);
                return;
            }
        };

    // 4. Create compute pipeline
    let pipeline_shader_stage_create_info = vk::VkPipelineShaderStageCreateInfo::DEFAULT
        .with_stage(vk::VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT)
        .with_module(shader_module)
        .with_pName(c"main".as_ptr());

    let compute_pipeline_create_info = vk::VkComputePipelineCreateInfo::DEFAULT
        .with_stage(pipeline_shader_stage_create_info)
        .with_layout(pipeline_layout);

    let pipeline = match logical_device.vkCreateComputePipelines(
        vk::VkPipelineCache::NULL,
        1,
        &compute_pipeline_create_info,
        vk::null(),
    ) {
        Ok(pipelines) => {
            println!("Compute pipeline created successfully");
            pipelines
        }
        Err(err) => {
            eprintln!("Failed to create compute pipeline: {:?}", err);
            return;
        }
    };

    // ============== BUFFER CREATION ==============

    // 5. Create input buffer with values [3, 2]
    let buffer_size = 2 * std::mem::size_of::<u32>() as u64;

    let input_buffer_create_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(buffer_size)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);

    let input_buffer = match logical_device.vkCreateBuffer(&input_buffer_create_info, vk::null()) {
        Ok(buf) => {
            println!("Input buffer created successfully");
            buf
        }
        Err(err) => {
            eprintln!("Failed to create input buffer: {:?}", err);
            return;
        }
    };

    // 6. Create output buffer
    let output_buffer_create_info = vk::VkBufferCreateInfo::DEFAULT
        .with_size(std::mem::size_of::<u32>() as u64)
        .with_usage(vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0)
        .with_sharingMode(vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE);

    let output_buffer = match logical_device.vkCreateBuffer(&output_buffer_create_info, vk::null())
    {
        Ok(buf) => {
            println!("Output buffer created successfully");
            buf
        }
        Err(err) => {
            eprintln!("Failed to create output buffer: {:?}", err);
            return;
        }
    };

    // 7. Allocate device memory and bind to buffers
    // Get memory requirements
    let mut input_mem_reqs = vk::VkMemoryRequirements::DEFAULT;
    logical_device.vkGetBufferMemoryRequirements(input_buffer, &mut input_mem_reqs);

    let mut output_mem_reqs = vk::VkMemoryRequirements::DEFAULT;
    logical_device.vkGetBufferMemoryRequirements(output_buffer, &mut output_mem_reqs);

    // Find suitable memory type (host visible for easy upload)
    let mut mem_properties = vk::VkPhysicalDeviceMemoryProperties2::DEFAULT;
    device.vkGetPhysicalDeviceMemoryProperties2(&mut mem_properties);

    let memory_type_index = find_memory_type(
        &mem_properties,
        input_mem_reqs.memoryTypeBits,
        vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
            | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
    )
    .expect("Failed to find suitable memory type");

    // Allocate memory for input buffer
    let input_memory_alloc_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(input_mem_reqs.size)
        .with_memoryTypeIndex(memory_type_index);

    let input_memory = match logical_device.vkAllocateMemory(&input_memory_alloc_info, vk::null()) {
        Ok(mem) => {
            println!("Input memory allocated successfully");
            mem
        }
        Err(err) => {
            eprintln!("Failed to allocate input memory: {:?}", err);
            return;
        }
    };

    match logical_device.vkBindBufferMemory(input_buffer, input_memory, 0) {
        Ok(_) => {
            println!("Input buffer bound to memory successfully");
        }
        Err(err) => {
            eprintln!("Failed to bind input buffer to memory: {:?}", err);
            return;
        }
    }

    // Allocate memory for output buffer
    let output_memory_alloc_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(output_mem_reqs.size)
        .with_memoryTypeIndex(memory_type_index);

    let output_memory = match logical_device.vkAllocateMemory(&output_memory_alloc_info, vk::null())
    {
        Ok(mem) => {
            println!("Output memory allocated successfully");
            mem
        }
        Err(err) => {
            eprintln!("Failed to allocate output memory: {:?}", err);
            return;
        }
    };

    match logical_device.vkBindBufferMemory(output_buffer, output_memory, 0) {
        Ok(_) => {
            println!("Output buffer bound to memory successfully");
        }
        Err(err) => {
            eprintln!("Failed to bind output buffer to memory: {:?}", err);
            return;
        }
    }

    // 8. Upload data to input buffer: [3, 2]
    let input_data: [u32; 2] = [3, 2];
    let mut data_ptr: *mut c_void = std::ptr::null_mut();
    match logical_device.vkMapMemory(input_memory, 0, buffer_size, 0, &mut data_ptr) {
        Ok(_) => {
            unsafe { std::ptr::copy_nonoverlapping(input_data.as_ptr(), data_ptr as *mut u32, 2) };
            logical_device.vkUnmapMemory(input_memory);
            println!("Uploaded input data: [3, 2]");
        }
        Err(err) => {
            eprintln!("Failed to map input memory: {:?}", err);
            return;
        }
    }

    // ============== DESCRIPTOR SET ==============

    // 9. Create descriptor pool
    let pool_sizes = [vk::VkDescriptorPoolSize::DEFAULT
        .with_type(vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(2)];

    let descriptor_pool_create_info = vk::VkDescriptorPoolCreateInfo::DEFAULT
        .with_maxSets(1)
        .with_poolSizeCount(pool_sizes.len() as u32)
        .with_pPoolSizes(pool_sizes.as_ptr());

    let descriptor_pool =
        match logical_device.vkCreateDescriptorPool(&descriptor_pool_create_info, vk::null()) {
            Ok(pool) => {
                println!("Descriptor pool created successfully");
                pool
            }
            Err(err) => {
                eprintln!("Failed to create descriptor pool: {:?}", err);
                return;
            }
        };

    // 10. Allocate descriptor set
    let descriptor_set_alloc_info = vk::VkDescriptorSetAllocateInfo::DEFAULT
        .with_descriptorPool(descriptor_pool)
        .with_descriptorSetCount(1)
        .with_pSetLayouts(&descriptor_set_layout);

    let descriptor_sets: vk::VkDescriptorSet =
        match logical_device.vkAllocateDescriptorSets(&descriptor_set_alloc_info) {
            Ok(sets) => {
                println!("Descriptor set allocated successfully");
                sets
            }
            Err(err) => {
                eprintln!("Failed to allocate descriptor sets: {:?}", err);
                return;
            }
        };
    // let descriptor_set = descriptor_sets[0];

    // 11. Update descriptor set with buffer info
    let buffer_infos = [
        vk::VkDescriptorBufferInfo::DEFAULT
            .with_buffer(input_buffer)
            .with_offset(0)
            .with_range(buffer_size),
        vk::VkDescriptorBufferInfo::DEFAULT
            .with_buffer(output_buffer)
            .with_offset(0)
            .with_range(std::mem::size_of::<u32>() as u64),
    ];

    let write_descriptor_sets = [
        vk::VkWriteDescriptorSet::DEFAULT
            .with_dstSet(descriptor_sets)
            .with_dstBinding(0)
            .with_dstArrayElement(0)
            .with_descriptorType(vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_pBufferInfo(&buffer_infos[0]),
        vk::VkWriteDescriptorSet::DEFAULT
            .with_dstSet(descriptor_sets)
            .with_dstBinding(1)
            .with_dstArrayElement(0)
            .with_descriptorType(vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_pBufferInfo(&buffer_infos[1]),
    ];

    logical_device.vkUpdateDescriptorSets(
        write_descriptor_sets.len() as u32,
        write_descriptor_sets.as_ptr(),
        0,
        std::ptr::null(),
    );
    println!("Descriptor set updated");

    // ============== COMMAND RECORDING & EXECUTION ==============

    // 12. Create command pool and buffer (you already have this, but ensure it's for compute)
    let command_pool_create_info = vk::VkCommandPoolCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_flags(
            vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT.0,
        );
    let command_pool =
        match logical_device.vkCreateCommandPool(&command_pool_create_info, vk::null()) {
            Ok(pool) => {
                println!("Command pool created successfully");
                pool
            }
            Err(err) => {
                eprintln!("Failed to create command pool: {:?}", err);
                return;
            }
        };

    let command_buffer_allocate_info = vk::VkCommandBufferAllocateInfo::DEFAULT
        .with_level(vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .with_commandBufferCount(1)
        .with_commandPool(command_pool.raw());

    let command_buffers: Vec<CommandBuffer> =
        match command_pool.vkAllocateCommandBuffers(&command_buffer_allocate_info) {
            Ok(buffers) => {
                println!("Command buffer allocated successfully");
                buffers
            }
            Err(err) => {
                eprintln!("Failed to allocate command buffer: {:?}", err);
                return;
            }
        };
    let command_buffer = &command_buffers[0];

    // 13. Record commands
    let command_buffer_begin_info = vk::VkCommandBufferBeginInfo::DEFAULT;
    match command_buffer.vkBeginCommandBuffer(&command_buffer_begin_info) {
        Ok(_) => println!("Began recording command buffer"),
        Err(err) => {
            eprintln!("Failed to begin command buffer: {:?}", err);
            return;
        }
    };

    command_buffer.vkCmdBindPipeline(
        vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        pipeline,
    );

    command_buffer.vkCmdBindDescriptorSets(
        vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        pipeline_layout,
        0,
        1,
        &descriptor_sets,
        0,
        std::ptr::null(),
    );
    command_buffer.vkCmdDispatch(1, 1, 1);
    match command_buffer.vkEndCommandBuffer() {
        Ok(_) => println!("Finished recording command buffer"),
        Err(err) => {
            eprintln!("Failed to end command buffer: {:?}", err);
            return;
        }
    };

    // 14. Submit command buffer and wait for completion
    let queue: Queue = logical_device.vkGetDeviceQueue(queue_family_index, 0);
    let submit_info = vk::VkSubmitInfo::DEFAULT
        .with_commandBufferCount(1)
        .with_pCommandBuffers(&command_buffer.raw());
    match queue.vkQueueSubmit(1, &submit_info, vk::VkFence::NULL) {
        Ok(_) => println!("Command buffer submitted successfully"),
        Err(err) => {
            eprintln!("Failed to submit command buffer: {:?}", err);
            return;
        }
    };
    match queue.vkQueueWaitIdle() {
        Ok(_) => println!("Queue is idle, compute shader execution complete"),
        Err(err) => {
            eprintln!("Failed to wait for queue idle: {:?}", err);
            return;
        }
    };

    // 15. Read back result from output buffer
    let mut output_data: u32;
    let mut output_data_ptr: *mut c_void = std::ptr::null_mut();
    match logical_device.vkMapMemory(
        output_memory,
        0,
        std::mem::size_of::<u32>() as u64,
        0,
        &mut output_data_ptr,
    ) {
        Ok(_) => {
            output_data = unsafe { *(output_data_ptr as *const u32) };
            logical_device.vkUnmapMemory(output_memory);
            println!("Output data read back: {}", output_data);
        }
        Err(err) => {
            eprintln!("Failed to map output memory: {:?}", err);
            return;
        }
    }

    // 16. Verify result
    if output_data == 5 {
        println!(
            "Success! The compute shader correctly computed 3 + 2 = {}",
            output_data
        );
    } else {
        eprintln!("Error: Expected 5 but got {}", output_data);
    }

    // Cleanup happens automatically via Drop implementations
    println!("Compute shader execution complete!");
}

// Helper function to find memory type
fn find_memory_type(
    mem_properties: &vk::VkPhysicalDeviceMemoryProperties2,
    type_filter: u32,
    properties: u32,
) -> Option<u32> {
    (0..mem_properties.memoryProperties.memoryTypeCount).find(|&i| {
        (type_filter & (1 << i)) != 0
            && (mem_properties.memoryProperties.memoryTypes[i as usize].propertyFlags & properties)
                == properties
    })
}
fn cstr_to_string(cstr: &[i8]) -> String {
    let bytes: Vec<u8> = cstr
        .iter()
        .map(|&c| c as u8)
        .take_while(|&c| c != 0)
        .collect();
    String::from_utf8_lossy(&bytes).to_string()
}
