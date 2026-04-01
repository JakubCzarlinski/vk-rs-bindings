#![allow(deprecated)]

use core::ffi::CStr;
use std::ffi::c_void;
use vk::*;

// Minimal SPIR-V compute shader: result = a + b
const COMPUTE_SHADER_SPV: &[u8] = include_bytes!("shader.spv");

const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Vulkan Compute Demo".as_ptr());
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
const LAYER_NAMES: [*const i8; 1] = [VALIDATION_LAYER.as_ptr()];
const INSTANCE_CREATE_INFO: VkInstanceCreateInfo = VkInstanceCreateInfo::DEFAULT
    .with_enabledLayerCount(LAYER_NAMES.len() as u32)
    .with_ppEnabledLayerNames(LAYER_NAMES.as_ptr())
    .with_pApplicationInfo(&APP_INFO);
const DEVICE_CREATE_INFO: VkDeviceCreateInfo = VkDeviceCreateInfo::DEFAULT;

const BINDINGS: [VkDescriptorSetLayoutBinding; 2] = [
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(0)
        .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT.0),
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(1)
        .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT.0),
];

const DSL_INFO: VkDescriptorSetLayoutCreateInfo = VkDescriptorSetLayoutCreateInfo::DEFAULT
    .with_bindingCount(2)
    .with_pBindings(BINDINGS.as_ptr());

const SHADER_MODULE_INFO: VkShaderModuleCreateInfo = VkShaderModuleCreateInfo::DEFAULT
    .with_codeSize(COMPUTE_SHADER_SPV.len())
    .with_pCode(COMPUTE_SHADER_SPV.as_ptr() as *const u32);

fn main() {
    let library = VulkanLib::load().expect("Failed to load Vulkan library");
    let instance: Instance = create_instance(&library);

    let (device, memory_properties, queue_family_index) = create_device(&instance);

    let queue = device.vkGetDeviceQueue(queue_family_index, 0);

    let (descriptor_set_layout, pipeline_layout, pipeline) =
        create_compute_pipeline(&device).expect("Failed to create pipeline");

    let buffer_size = 2 * std::mem::size_of::<u32>() as u64;
    let (input_buffer, input_memory) =
        create_storage_buffer(&device, memory_properties, buffer_size)
            .expect("Failed to create input buffer");
    let (output_buffer, output_memory) = create_storage_buffer(&device, memory_properties, 4)
        .expect("Failed to create output buffer");

    write_to_buffer(&input_memory, &[3u32, 2u32]).expect("Failed to upload data");

    let descriptor_pool =
        create_descriptor_pool(&device).expect("Failed to create descriptor pool");

    let descriptor_set = create_descriptor_set(
        &device,
        &descriptor_pool,
        &descriptor_set_layout,
        &input_buffer,
        &output_buffer,
        buffer_size,
    )
    .expect("Failed to setup descriptor set");

    let first_pipeline = pipeline.first().expect("No pipelines created");
    let first_descriptor_set = descriptor_set
        .first()
        .expect("No descriptor sets allocated");

    run_compute(
        &device,
        &queue,
        queue_family_index,
        first_pipeline,
        &pipeline_layout,
        first_descriptor_set,
    )
    .expect("Failed to run compute");

    let result = read_from_buffer::<u32>(&output_memory).expect("Failed to read result");

    if result == 5 {
        println!("Success! 3 + 2 = {}", result);
    } else {
        println!("Error: expected 5, got {}", result);
    }

    println!("Compute shader execution complete!");
}

fn create_instance(library: &'_ VulkanLib) -> Instance<'_> {
    let entry: Entry<'_> = Entry::new(library);
    entry
        .vkCreateInstance(&INSTANCE_CREATE_INFO, null())
        .expect("Failed to create instance")
}

fn find_queue_family(physical_device: &PhysicalDevice) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());
    let mut props = vec![VkQueueFamilyProperties2::DEFAULT; count as usize];
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, props.as_mut_ptr());

    props.iter().enumerate().find_map(|(i, p)| {
        if (p.queueFamilyProperties.queueFlags & VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT.0) != 0 {
            Some(i as u32)
        } else {
            None
        }
    })
}

fn create_device<'inst>(
    instance: &'inst Instance,
) -> (Device<'inst>, VkPhysicalDeviceMemoryProperties, u32) {
    let physical_devices = instance
        .vkEnumeratePhysicalDevices()
        .expect("Failed to enumerate physical devices");
    let physical_device = &physical_devices.first().expect("No physical devices found");

    let mut props = VkPhysicalDeviceMemoryProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceMemoryProperties2(&mut props);
    let memory_properties = props.memoryProperties;

    let queue_family_index =
        find_queue_family(physical_device).expect("No suitable queue family found");

    const PRIORITIES: [f32; 1] = [1.0f32];
    let queue_info = VkDeviceQueueCreateInfo::DEFAULT
        .with_queueCount(1)
        .with_pQueuePriorities(PRIORITIES.as_ptr())
        .with_queueFamilyIndex(queue_family_index);

    let device = physical_device
        .vkCreateDevice(
            &DEVICE_CREATE_INFO
                .with_queueCreateInfoCount(1)
                .with_pQueueCreateInfos(&queue_info),
            null(),
        )
        .expect("Failed to create logical device");

    (device, memory_properties, queue_family_index)
}

fn create_compute_pipeline<'a>(
    device: &'a Device<'a>,
) -> Result<
    (
        DescriptorSetLayout<'a>,
        PipelineLayout<'a>,
        Vec<Pipeline<'a>>,
    ),
    String,
> {
    let ds_layout = device
        .vkCreateDescriptorSetLayout(&DSL_INFO, null())
        .map_err(|e| format!("DSLayout: {:?}", e))?;

    let layouts = [ds_layout.raw()];
    let pll_info = VkPipelineLayoutCreateInfo::DEFAULT
        .with_setLayoutCount(1)
        .with_pSetLayouts(layouts.as_ptr());
    let pipeline_layout = device
        .vkCreatePipelineLayout(&pll_info, null())
        .map_err(|e| format!("PLLayout: {:?}", e))?;

    let shader_module = device
        .vkCreateShaderModule(&SHADER_MODULE_INFO, null())
        .map_err(|e| format!("ShaderModule: {:?}", e))?;
    let stage = VkPipelineShaderStageCreateInfo::DEFAULT
        .with_stage(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT)
        .with_pName(c"main".as_ptr())
        .with_module(shader_module.raw());
    let pipe_info = VkComputePipelineCreateInfo::DEFAULT
        .with_stage(stage)
        .with_layout(pipeline_layout.raw());

    let pipelines = device
        .vkCreateComputePipelines(VkPipelineCache::NULL, 1, &pipe_info, null())
        .map_err(|e| format!("Pipeline: {:?}", e))?;

    Ok((ds_layout, pipeline_layout, pipelines))
}

fn create_storage_buffer<'a>(
    device: &'a Device<'a>,
    memory_properties: VkPhysicalDeviceMemoryProperties,
    size: u64,
) -> Result<(Buffer<'a>, DeviceMemory<'a>), String> {
    let buffer_info = VkBufferCreateInfo::DEFAULT
        .with_usage(VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0)
        .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_size(size);
    let buffer = device
        .vkCreateBuffer(&buffer_info, null())
        .map_err(|e| format!("Buffer: {:?}", e))?;

    let mut reqs = VkMemoryRequirements::DEFAULT;
    buffer.vkGetBufferMemoryRequirements(&mut reqs);

    const FLAGS: u32 = VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
        | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0;
    let mem_type = (0..memory_properties.memoryTypeCount)
        .find(|&i| {
            (reqs.memoryTypeBits & (1 << i)) != 0
                && (memory_properties.memoryTypes[i as usize].propertyFlags & FLAGS) == FLAGS
        })
        .ok_or("Memory type not found")?;

    let a_info = VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(reqs.size)
        .with_memoryTypeIndex(mem_type);
    let memory = device
        .vkAllocateMemory(&a_info, null())
        .map_err(|e| format!("Allocate: {:?}", e))?;
    buffer
        .vkBindBufferMemory(memory.raw(), 0)
        .map_err(|e| format!("Bind: {:?}", e))?;

    Ok((buffer, memory))
}

fn write_to_buffer<T>(memory: &DeviceMemory, data: &[T]) -> Result<(), String> {
    let size = std::mem::size_of_val(data) as u64;
    let mut ptr: *mut c_void = null_mut();
    memory
        .vkMapMemory(0, size, 0, &mut ptr)
        .map_err(|e| format!("Map: {:?}", e))?;
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut T, data.len());
    }
    memory.vkUnmapMemory();
    Ok(())
}

fn read_from_buffer<T: Copy>(memory: &DeviceMemory) -> Result<T, String> {
    let mut ptr: *mut c_void = null_mut();
    memory
        .vkMapMemory(0, std::mem::size_of::<T>() as u64, 0, &mut ptr)
        .map_err(|e| format!("MapRead: {:?}", e))?;
    let val = unsafe { *(ptr as *const T) };
    memory.vkUnmapMemory();
    Ok(val)
}

fn create_descriptor_pool<'a>(device: &'a Device<'a>) -> Result<DescriptorPool<'a>, String> {
    const POOL_SIZE: VkDescriptorPoolSize = VkDescriptorPoolSize::DEFAULT
        .with_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(2);
    const POOL_INFO: VkDescriptorPoolCreateInfo = VkDescriptorPoolCreateInfo::DEFAULT
        .with_maxSets(1)
        .with_flags(
            vk::VkDescriptorPoolCreateFlagBits::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT.0,
        )
        .with_poolSizeCount(1)
        .with_pPoolSizes(&POOL_SIZE);
    device
        .vkCreateDescriptorPool(&POOL_INFO, null())
        .map_err(|e| format!("Pool: {:?}", e))
}

fn create_descriptor_set<'a>(
    device: &'a Device<'a>,
    descriptor_pool: &'a DescriptorPool<'a>,
    layout: &DescriptorSetLayout<'a>,
    input: &Buffer<'a>,
    output: &Buffer<'a>,
    input_size: u64,
) -> Result<Vec<DescriptorSet<'a>>, String> {
    let layouts = [layout.raw()];
    let alloc_info = VkDescriptorSetAllocateInfo::DEFAULT
        .with_descriptorSetCount(1)
        .with_descriptorPool(descriptor_pool.raw())
        .with_pSetLayouts(layouts.as_ptr());
    let ds = descriptor_pool
        .vkAllocateDescriptorSets(&alloc_info)
        .map_err(|e| format!("DSAlloc: {:?}", e))?;

    let first_ds = &ds.first().ok_or("No descriptor sets allocated")?;

    let b_infos = [
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(input.raw())
            .with_offset(0)
            .with_range(input_size),
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(output.raw())
            .with_offset(0)
            .with_range(4),
    ];
    let writes = [
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_dstBinding(0)
            .with_pBufferInfo(&b_infos[0])
            .with_dstSet(first_ds.raw()),
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(1)
            .with_dstBinding(1)
            .with_pBufferInfo(&b_infos[1])
            .with_dstSet(first_ds.raw()),
    ];
    device.vkUpdateDescriptorSets(2, writes.as_ptr(), 0, null());

    Ok(ds)
}

fn run_compute<'a>(
    device: &Device<'a>,
    queue: &Queue<'a>,
    queue_familiy_index: u32,
    pipeline: &Pipeline<'a>,
    layout: &PipelineLayout<'a>,
    descriptor_set: &DescriptorSet<'a>,
) -> Result<(), String> {
    let pool_info = VkCommandPoolCreateInfo::DEFAULT
        .with_flags(VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT.0)
        .with_queueFamilyIndex(queue_familiy_index);
    let cmd_pool: CommandPool = device
        .vkCreateCommandPool(&pool_info, null())
        .map_err(|e| format!("CommandPool: {:?}", e))?;

    let cmd_buffer_info = VkCommandBufferAllocateInfo::DEFAULT
        .with_level(VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .with_commandBufferCount(1)
        .with_commandPool(cmd_pool.raw());
    let cmd_buffers: Vec<CommandBuffer<'_>> = cmd_pool
        .vkAllocateCommandBuffers(&cmd_buffer_info)
        .map_err(|e| format!("CBAlloc: {:?}", e))?;
    let raw_cmd_buffers = [cmd_buffers[0].raw()];
    let cmd_buffer: &CommandBuffer<'_> = &cmd_buffers[0];

    cmd_buffer
        .vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("BeginCB: {:?}", e))?;
    cmd_buffer.vkCmdBindPipeline(
        VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        pipeline.raw(),
    );
    let raw_ds = [descriptor_set.raw()];
    cmd_buffer.vkCmdBindDescriptorSets(
        VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        layout.raw(),
        0,
        1,
        raw_ds.as_ptr(),
        0,
        null(),
    );
    cmd_buffer.vkCmdDispatch(1, 1, 1);
    cmd_buffer
        .vkEndCommandBuffer()
        .map_err(|e| format!("EndCB: {:?}", e))?;

    let submit = VkSubmitInfo::DEFAULT
        .with_commandBufferCount(1)
        .with_pCommandBuffers(raw_cmd_buffers.as_ptr());
    println!("Submitting compute command buffer...");
    queue
        .vkQueueSubmit(1, &submit, VkFence::NULL)
        .map_err(|e| format!("Submit: {:?}", e))?;
    queue
        .vkQueueWaitIdle()
        .map_err(|e| format!("WaitIdle: {:?}", e))?;

    Ok(())
}
