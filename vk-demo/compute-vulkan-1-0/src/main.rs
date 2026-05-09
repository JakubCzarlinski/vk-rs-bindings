use alloc::boxed::Box;
use core::ffi::CStr;
use core::mem;
use core::ptr;
use vk::*;

extern crate alloc;

// Minimal SPIR-V compute shader: result = a + b
const COMPUTE_SHADER_SPV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/shader.spv"));

const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_0)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Vulkan 1.0 Compute Demo".as_ptr());
// We are on MacOS, where validation layers are not available, so skip enabling them
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
const LAYER_NAMES: [*const i8; 1] = [VALIDATION_LAYER.as_ptr()];
const INSTANCE_CREATE_INFO: VkInstanceCreateInfo = VkInstanceCreateInfo::DEFAULT
    .with_enabledLayerCount(LAYER_NAMES.len() as u32)
    .with_ppEnabledLayerNames(LAYER_NAMES.as_ptr())
    .with_pApplicationInfo(&APP_INFO);

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

const DSL_INFO: VkDescriptorSetLayoutCreateInfo =
    VkDescriptorSetLayoutCreateInfo::DEFAULT.with_pBindings(&BINDINGS);

const SHADER_MODULE_INFO: VkShaderModuleCreateInfo = VkShaderModuleCreateInfo::DEFAULT
    .with_codeSize(COMPUTE_SHADER_SPV.len())
    .with_pCode(COMPUTE_SHADER_SPV.as_ptr().cast::<u32>());

fn main() {
    let library = VulkanLib::load().expect("Failed to load Vulkan library");
    let instance: Instance = create_instance(&library);

    let (device, physical_device, queue_family_index) = create_device(&instance);

    let memory_type_index = find_host_visible_memory_type(
        &physical_device,
        u32::MAX,
        VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
            | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
    )
    .expect("Failed to find suitable memory type");

    {
        const INPUT_BUFFER_SIZE: u64 = 2 * mem::size_of::<u32>() as u64;
        let input_buffer = HostVisibleBuffer::new(&device, INPUT_BUFFER_SIZE, memory_type_index)
            .expect("Failed to create input buffer");
        const OUTPUT_BUFFER_SIZE: u64 = mem::size_of::<u32>() as u64;
        let output_buffer = HostVisibleBuffer::new(&device, OUTPUT_BUFFER_SIZE, memory_type_index)
            .expect("Failed to create output buffer");
        let queue = &device.vkGetDeviceQueue(queue_family_index, 0);

        let descriptor_set_layout = device
            .vkCreateDescriptorSetLayout(&DSL_INFO, null())
            .expect("Failed to create descriptor set layout");
        let (pipeline_layout, pipeline) = create_compute_pipeline(&device, &descriptor_set_layout)
            .expect("Failed to create pipeline");

        write_to_buffer(&input_buffer.memory, &[3u32, 2u32]).expect("Failed to upload data");

        let descriptor_pool =
            create_descriptor_pool(&device).expect("Failed to create descriptor pool");

        let descriptor_set = create_descriptor_set(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
            &input_buffer.buffer,
            &output_buffer.buffer,
            INPUT_BUFFER_SIZE,
            OUTPUT_BUFFER_SIZE,
        )
        .expect("Failed to setup descriptor set");

        let first_pipeline = pipeline.first().expect("No pipelines created");
        let first_descriptor_set = descriptor_set
            .first()
            .expect("No descriptor sets allocated");

        run_compute(
            &device,
            queue,
            queue_family_index,
            first_pipeline,
            &pipeline_layout,
            first_descriptor_set,
        )
        .expect("Failed to run compute");

        let result = read_from_buffer(&output_buffer.memory).expect("Failed to read result");

        println!("3 + 2 = {result}");
    }
}

fn create_instance(library: &'_ VulkanLib) -> Instance<'_> {
    let entry: Entry<'_> = Entry::new(library);
    entry
        .vkCreateInstance(&INSTANCE_CREATE_INFO, null())
        .expect("Failed to create instance")
}

#[allow(deprecated)]
fn find_queue_family(physical_device: &PhysicalDevice) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties(&raw mut count, null_mut());
    if count == 0 {
        return None;
    }

    let mut props = Box::<[VkQueueFamilyProperties]>::new_uninit_slice(count as usize);
    physical_device
        .vkGetPhysicalDeviceQueueFamilyProperties(&raw mut count, props.as_mut_ptr().cast());

    let props = unsafe { props.assume_init() };

    props.iter().enumerate().find_map(|(i, p)| {
        if (p.queueFlags & VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT.0) != 0 {
            Some(i as u32)
        } else {
            None
        }
    })
}

fn create_device<'inst>(instance: &'inst Instance) -> (Device<'inst>, PhysicalDevice<'inst>, u32) {
    let physical_device = instance
        .vkEnumeratePhysicalDevices()
        .expect("Failed to enumerate physical devices")
        .into_iter()
        .next()
        .expect("No physical devices found");

    let queue_family_index =
        find_queue_family(&physical_device).expect("No suitable queue family found");

    let device = physical_device
        .vkCreateDevice(
            &VkDeviceCreateInfo::DEFAULT.with_pQueueCreateInfos(&[
                VkDeviceQueueCreateInfo::DEFAULT
                    .with_pQueuePriorities(&[1.0f32])
                    .with_queueFamilyIndex(queue_family_index),
            ]),
            null(),
        )
        .expect("Failed to create logical device");

    (device, physical_device, queue_family_index)
}

fn create_compute_pipeline<'a>(
    device: &'a Device<'a>,
    descriptor_set_layout: &DescriptorSetLayout<'a>,
) -> Result<(PipelineLayout<'a>, Box<[Pipeline<'a>]>), VkResult> {
    let pipeline_layout = {
        let layouts = [descriptor_set_layout.raw()];
        let pll_info = VkPipelineLayoutCreateInfo::DEFAULT
            .with_setLayoutCount(1)
            .with_pSetLayouts(layouts.as_ptr());
        device.vkCreatePipelineLayout(&pll_info, null())
    }?;
    let shader_module = device.vkCreateShaderModule(&SHADER_MODULE_INFO, null())?;

    let stage = VkPipelineShaderStageCreateInfo::DEFAULT
        .with_stage(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT)
        .with_pName(c"main".as_ptr())
        .with_module(shader_module.raw());
    let pipe_info = VkComputePipelineCreateInfo::DEFAULT
        .with_stage(stage)
        .with_layout(pipeline_layout.raw());
    let pipelines =
        device.vkCreateComputePipelines(VkPipelineCache::NULL, 1, &raw const pipe_info, null())?;

    Ok((pipeline_layout, pipelines))
}

struct HostVisibleBuffer<'a> {
    memory: DeviceMemory<'a>,
    buffer: Buffer<'a>,
}

impl HostVisibleBuffer<'_> {
    fn new<'a>(
        device: &'a Device<'a>,
        size: u64,
        memory_type_index: u32,
    ) -> Result<HostVisibleBuffer<'a>, VkResult> {
        let buffer: Buffer<'_> = {
            #[allow(deprecated)]
            let buffer_info = VkBufferCreateInfo::DEFAULT
                .with_usage(VkBufferUsageFlagBits::VK_BUFFER_USAGE_STORAGE_BUFFER_BIT.0)
                .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
                .with_size(size);
            device.vkCreateBuffer(&buffer_info, null())
        }?;

        let memory: DeviceMemory<'_> = {
            let mut requirements = VkMemoryRequirements::DEFAULT;
            buffer.vkGetBufferMemoryRequirements(&mut requirements);

            let allocate_info = VkMemoryAllocateInfo::DEFAULT
                .with_allocationSize(requirements.size)
                .with_memoryTypeIndex(memory_type_index);

            device.vkAllocateMemory(&allocate_info, null())
        }?;
        buffer.vkBindBufferMemory(memory.raw(), 0)?;

        Ok(HostVisibleBuffer { memory, buffer })
    }
}

#[allow(deprecated)]
fn find_host_visible_memory_type(
    physical_device: &PhysicalDevice<'_>,
    memory_type_bits: u32,
    required_flags: u32,
) -> Result<u32, ()> {
    let mut properties = VkPhysicalDeviceMemoryProperties::DEFAULT;
    physical_device.vkGetPhysicalDeviceMemoryProperties(&mut properties);

    for index in 0..properties.memoryTypeCount {
        let mask = 1u32 << index;
        if (memory_type_bits & mask) == 0 {
            continue;
        }

        let flags = properties.memoryTypes[index as usize].propertyFlags;
        if (flags & required_flags) == required_flags {
            return Ok(index);
        }
    }

    Err(())
}

fn write_to_buffer(memory: &DeviceMemory<'_>, data: &[u32]) -> Result<(), VkResult> {
    {
        let mut mapped = null_mut();
        let bytes = mem::size_of_val(data);
        memory.vkMapMemory(0, bytes as u64, 0, &raw mut mapped)?;
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr().cast::<u8>(), mapped.cast::<u8>(), bytes);
        }
    }
    memory.vkUnmapMemory();
    Ok(())
}

fn read_from_buffer(memory: &DeviceMemory<'_>) -> Result<u32, VkResult> {
    let mut mapped = null_mut();
    {
        const BYTES: u64 = mem::size_of::<u32>() as u64;
        memory.vkMapMemory(0, BYTES, 0, &raw mut mapped)?;
    }
    let value = unsafe { mapped.cast::<u32>().read() };
    memory.vkUnmapMemory();
    Ok(value)
}

fn create_descriptor_pool<'a>(device: &'a Device<'a>) -> Result<DescriptorPool<'a>, VkResult> {
    const POOL_INFO: VkDescriptorPoolCreateInfo = VkDescriptorPoolCreateInfo::DEFAULT
        .with_maxSets(1)
        .with_flags(
            vk::VkDescriptorPoolCreateFlagBits::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT.0,
        )
        .with_pPoolSizes(&[VkDescriptorPoolSize::DEFAULT
            .with_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_descriptorCount(2)]);
    device.vkCreateDescriptorPool(&POOL_INFO, null())
}

fn create_descriptor_set<'a>(
    device: &'a Device<'a>,
    descriptor_pool: &'a DescriptorPool<'a>,
    layout: &DescriptorSetLayout<'a>,
    input: &Buffer<'a>,
    output: &Buffer<'a>,
    input_size: u64,
    output_size: u64,
) -> Result<Box<[DescriptorSet<'a>]>, VkResult> {
    let ds: Box<[DescriptorSet<'_>]> = {
        let layouts = [layout.raw()];
        let alloc_info = VkDescriptorSetAllocateInfo::DEFAULT
            .with_descriptorPool(descriptor_pool.raw())
            .with_pSetLayouts(&layouts);
        descriptor_pool.vkAllocateDescriptorSets(&alloc_info)
    }?;
    let first_ds = &ds.first().expect("No descriptor sets allocated");

    let b_infos = [
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(input.raw())
            .with_offset(0)
            .with_range(input_size),
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(output.raw())
            .with_offset(0)
            .with_range(output_size),
    ];
    let writes = [
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_dstBinding(0)
            .with_pBufferInfo(&b_infos[0..1])
            .with_dstSet(first_ds.raw()),
        VkWriteDescriptorSet::DEFAULT
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_dstBinding(1)
            .with_pBufferInfo(&b_infos[1..2])
            .with_dstSet(first_ds.raw()),
    ];
    device.vkUpdateDescriptorSets(&writes, &[]);

    Ok(ds)
}

#[allow(deprecated)]
fn run_compute<'a>(
    device: &Device<'a>,
    queue: &Queue<'a>,
    queue_familiy_index: u32,
    pipeline: &Pipeline<'a>,
    layout: &PipelineLayout<'a>,
    descriptor_set: &DescriptorSet<'a>,
) -> Result<(), VkResult> {
    let cmd_pool = {
        let pool_info = VkCommandPoolCreateInfo::DEFAULT
            .with_flags(
                VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT.0,
            )
            .with_queueFamilyIndex(queue_familiy_index);
        device.vkCreateCommandPool(&pool_info, null())
    }?;
    let cmd_buffers = {
        let cmd_buffer_info = VkCommandBufferAllocateInfo::DEFAULT
            .with_level(VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
            .with_commandBufferCount(1)
            .with_commandPool(cmd_pool.raw());
        cmd_pool.vkAllocateCommandBuffers(&cmd_buffer_info)
    }?;
    let cmd_buffer = &cmd_buffers[0];

    cmd_buffer.vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)?;
    cmd_buffer.vkCmdBindPipeline(
        VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        pipeline.raw(),
    );
    {
        let raw_ds = [descriptor_set.raw()];
        cmd_buffer.vkCmdBindDescriptorSets(
            VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
            layout.raw(),
            0,
            1,
            raw_ds.as_ptr(),
            &[],
        );
    }
    cmd_buffer.vkCmdDispatch(1, 1, 1);
    cmd_buffer.vkEndCommandBuffer()?;
    {
        let command_buffers = [cmd_buffer.raw()];
        let submit = VkSubmitInfo::DEFAULT.with_pCommandBuffers(&command_buffers);
        queue.vkQueueSubmit(&[submit], VkFence::NULL)?;
    }
    queue.vkQueueWaitIdle()?;

    Ok(())
}
