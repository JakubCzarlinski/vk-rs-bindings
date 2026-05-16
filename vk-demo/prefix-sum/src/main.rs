#![allow(deprecated)]

use core::ffi::{CStr, c_char};
use core::iter;
use vk::*;
use vk_alloc::{AllocationCreateInfo, Allocator};

#[repr(C, align(4))]
struct AlignedSpv<const N: usize>([u8; N]);

macro_rules! include_spirv_words {
    ($path:expr) => {{
        static SPV: AlignedSpv<{ include_bytes!($path).len() }> =
            AlignedSpv(*include_bytes!($path));
        unsafe { core::slice::from_raw_parts(SPV.0.as_ptr().cast::<u32>(), SPV.0.len() / 4) }
    }};
}

const PREFIX_SUM_SPV: &[u32] = include_spirv_words!(concat!(env!("OUT_DIR"), "/prefix_sum.spv"));

const ITEM_COUNT: usize = 1 << 16;
const BENCHMARK_RUNS: usize = 100;
const BLOCK_SIZE: u32 = 1024;
const BLOCK_COUNT: u32 = ITEM_COUNT as u32 / BLOCK_SIZE;
const LOOKBACK_WORDS: usize = BLOCK_COUNT as usize * 3;
const DATA_SIZE: VkDeviceSize = (ITEM_COUNT * size_of::<u32>()) as VkDeviceSize;
const LOOKBACK_SIZE: VkDeviceSize = (LOOKBACK_WORDS * size_of::<u32>()) as VkDeviceSize;
const TIMESTAMP_QUERY_COUNT: u32 = (BENCHMARK_RUNS * 2) as u32;

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";
const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Work-Efficient Prefix Sum Demo".as_ptr());
const DEVICE_CREATE_INFO: VkDeviceCreateInfo = VkDeviceCreateInfo::DEFAULT;

const BINDINGS: [VkDescriptorSetLayoutBinding; 2] = [
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(0)
        .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT),
    VkDescriptorSetLayoutBinding::DEFAULT
        .with_binding(1)
        .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(1)
        .with_stageFlags(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT),
];
const DSL_INFO: VkDescriptorSetLayoutCreateInfo =
    VkDescriptorSetLayoutCreateInfo::DEFAULT.with_pBindings(&BINDINGS);

fn main() -> Result<(), String> {
    let library = VulkanLib::load().map_err(|e| e.to_string())?;
    let entry = Entry::new(&library);
    let layer_names = enabled_validation_layers(&entry)?;
    let instance_info = VkInstanceCreateInfo::DEFAULT
        .with_pApplicationInfo(&APP_INFO)
        .with_ppEnabledLayerNames(&layer_names);
    let instance = entry
        .vkCreateInstance(&instance_info, null())
        .map_err(|e| format!("vkCreateInstance failed: {e:?}"))?;

    let (device, physical_device, queue_family_index) = create_device(&instance)?;
    let allocator = Allocator::new(&physical_device, &device)
        .map_err(|e| format!("Allocator creation failed: {e:?}"))?;
    let queue = device.vkGetDeviceQueue(queue_family_index, 0);

    let data_buffer = {
        let mut data_buffer = create_storage_buffer(&allocator, DATA_SIZE)?;
        write_input(data_buffer.allocation_mut())?;
        data_buffer
    };
    let lookback_buffer = {
        let mut lookback_buffer = create_storage_buffer(&allocator, LOOKBACK_SIZE)?;
        clear_lookback(lookback_buffer.allocation_mut())?;
        lookback_buffer
    };

    let descriptor_pool = create_descriptor_pool(&device)?;
    let descriptor_set_layout = device
        .vkCreateDescriptorSetLayout(&DSL_INFO, null())
        .map_err(|e| format!("vkCreateDescriptorSetLayout failed: {e:?}"))?;
    let descriptor_sets = create_descriptor_set(
        &descriptor_pool,
        &descriptor_set_layout,
        data_buffer.buffer(),
        lookback_buffer.buffer(),
    )?;
    let descriptor_set = descriptor_sets
        .first()
        .ok_or("No descriptor sets allocated")?;

    let (pipeline_layout, pipelines) = create_compute_pipeline(&device, &descriptor_set_layout)?;
    let pipeline = pipelines.first().ok_or("No compute pipeline created")?;

    let mut properties = VkPhysicalDeviceProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceProperties2(&mut properties);
    let elapsed_ns = run_prefix_sum_benchmark(
        &device,
        &queue,
        queue_family_index,
        pipeline,
        &pipeline_layout,
        descriptor_set,
        data_buffer.buffer(),
        lookback_buffer.buffer(),
        properties.properties.limits.timestampPeriod,
    )?;

    verify_zero_prefix_sum(data_buffer.allocation())?;
    let total_us = elapsed_ns / 1_000.0;
    let average_us = total_us / BENCHMARK_RUNS as f64;
    println!("Success: exclusive prefix sum over 2^16 zero values verified");
    println!(
        "GPU timestamp benchmark: {BENCHMARK_RUNS} runs in {:.3} us ({average_us:.3} us/run)",
        total_us,
    );
    Ok(())
}

fn create_device<'inst>(
    instance: &'inst Instance<'inst>,
) -> Result<(Device<'inst>, PhysicalDevice<'inst>, u32), String> {
    let physical_device = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|e| format!("vkEnumeratePhysicalDevices failed: {e:?}"))?
        .into_iter()
        .next()
        .ok_or("No physical devices found")?;
    let queue_family_index =
        find_compute_queue_family(&physical_device).ok_or("No compute queue family found")?;
    println!("Selected queue family index: {queue_family_index}",);

    const PRIORITIES: [f32; 1] = [1.0];
    let queue_info = VkDeviceQueueCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_pQueuePriorities(&PRIORITIES);
    let queue_infos = [queue_info];
    const VULKAN13_FEATURES: VkPhysicalDeviceVulkan13Features<'_> =
        VkPhysicalDeviceVulkan13Features::DEFAULT.with_synchronization2(VK_TRUE);
    let device_info = DEVICE_CREATE_INFO
        .with_pQueueCreateInfos(&queue_infos)
        .with_pNext_VkPhysicalDeviceVulkan13Features(&VULKAN13_FEATURES);
    let device = physical_device
        .vkCreateDevice(&device_info, null())
        .map_err(|e| format!("vkCreateDevice failed: {e:?}"))?;

    Ok((device, physical_device, queue_family_index))
}

fn find_compute_queue_family(physical_device: &PhysicalDevice<'_>) -> Option<u32> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());
    let mut props: Vec<_> =
        iter::repeat_n(VkQueueFamilyProperties2::DEFAULT, count as usize).collect();
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, props.as_mut_ptr());

    props
        .iter()
        .position(|p| {
            p.queueFamilyProperties.queueCount > 0
                && p.queueFamilyProperties
                    .queueFlags
                    .intersects(VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT)
                && !p
                    .queueFamilyProperties
                    .queueFlags
                    .intersects(VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT)
        })
        .map(|index| index as u32)
}

fn create_compute_pipeline<'a>(
    device: &'a Device<'a>,
    descriptor_set_layout: &DescriptorSetLayout<'a>,
) -> Result<(PipelineLayout<'a>, Box<[Pipeline<'a>]>), String> {
    let layouts = [descriptor_set_layout.raw()];
    let pipeline_layout_info = VkPipelineLayoutCreateInfo::DEFAULT.with_pSetLayouts(&layouts);
    let pipeline_layout = device
        .vkCreatePipelineLayout(&pipeline_layout_info, null())
        .map_err(|e| format!("vkCreatePipelineLayout failed: {e:?}"))?;

    const SHADER_MODULE_INFO: VkShaderModuleCreateInfo<'_> =
        VkShaderModuleCreateInfo::DEFAULT.with_pCode(PREFIX_SUM_SPV);
    let shader_module = device
        .vkCreateShaderModule(&SHADER_MODULE_INFO, null())
        .map_err(|e| format!("vkCreateShaderModule failed: {e:?}"))?;
    let stage = VkPipelineShaderStageCreateInfo::DEFAULT
        .with_stage(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT)
        .with_module(shader_module.raw())
        .with_pName(c"main".as_ptr());
    let pipeline_info = VkComputePipelineCreateInfo::DEFAULT
        .with_stage(stage)
        .with_layout(pipeline_layout.raw());
    let pipelines = device
        .vkCreateComputePipelines(VkPipelineCache::NULL, &[pipeline_info], null())
        .map_err(|e| format!("vkCreateComputePipelines failed: {e:?}"))?;

    Ok((pipeline_layout, pipelines))
}

fn run_prefix_sum_benchmark(
    device: &Device<'_>,
    queue: &Queue<'_>,
    queue_family_index: u32,
    pipeline: &Pipeline<'_>,
    pipeline_layout: &PipelineLayout<'_>,
    descriptor_set: &DescriptorSet<'_>,
    data_buffer: &Buffer<'_>,
    lookback_buffer: &Buffer<'_>,
    timestamp_period_ns: f32,
) -> Result<f64, String> {
    let query_pool_info = VkQueryPoolCreateInfo::DEFAULT
        .with_queryType(VkQueryType::VK_QUERY_TYPE_TIMESTAMP)
        .with_queryCount(TIMESTAMP_QUERY_COUNT);
    let query_pool = device
        .vkCreateQueryPool(&query_pool_info, null())
        .map_err(|e| format!("vkCreateQueryPool failed: {e:?}"))?;

    let pool_info = VkCommandPoolCreateInfo::DEFAULT
        .with_queueFamilyIndex(queue_family_index)
        .with_flags(VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT);
    let command_pool = device
        .vkCreateCommandPool(&pool_info, null())
        .map_err(|e| format!("vkCreateCommandPool failed: {e:?}"))?;
    let alloc_info = VkCommandBufferAllocateInfo::DEFAULT
        .with_commandPool(command_pool.raw())
        .with_level(VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .with_commandBufferCount(1);
    let command_buffers = command_pool
        .vkAllocateCommandBuffers(&alloc_info)
        .map_err(|e| format!("vkAllocateCommandBuffers failed: {e:?}"))?;
    let command_buffer = &command_buffers[0];

    command_buffer
        .vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("vkBeginCommandBuffer failed: {e:?}"))?;
    command_buffer.vkCmdBindPipeline(
        VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_COMPUTE,
        pipeline.raw(),
    );
    let raw_sets = [descriptor_set.raw()];
    let bind_info = VkBindDescriptorSetsInfo::DEFAULT
        .with_stageFlags(VkShaderStageFlagBits::VK_SHADER_STAGE_COMPUTE_BIT)
        .with_layout(pipeline_layout.raw())
        .with_pDescriptorSets(&raw_sets);
    command_buffer.vkCmdBindDescriptorSets2(&bind_info);

    command_buffer.vkCmdResetQueryPool(query_pool.raw(), 0, TIMESTAMP_QUERY_COUNT);
    for run in 0..BENCHMARK_RUNS {
        command_buffer.vkCmdFillBuffer(data_buffer.raw(), 0, DATA_SIZE, 0);
        command_buffer.vkCmdFillBuffer(lookback_buffer.raw(), 0, LOOKBACK_SIZE, 0);
        transfer_to_shader_barrier(command_buffer);

        let start_query = (run * 2) as u32;
        command_buffer.vkCmdWriteTimestamp2(
            VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT,
            query_pool.raw(),
            start_query,
        );
        command_buffer.vkCmdDispatch(BLOCK_COUNT, 1, 1);
        command_buffer.vkCmdWriteTimestamp2(
            VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT,
            query_pool.raw(),
            start_query + 1,
        );

        shader_to_transfer_barrier(command_buffer);
    }
    shader_to_host_barrier(command_buffer);

    command_buffer
        .vkEndCommandBuffer()
        .map_err(|e| format!("vkEndCommandBuffer failed: {e:?}"))?;

    let command_infos =
        [VkCommandBufferSubmitInfo::DEFAULT.with_commandBuffer(command_buffer.raw())];
    let submit = VkSubmitInfo2::DEFAULT.with_pCommandBufferInfos(&command_infos);

    queue
        .vkQueueSubmit2(&[submit], VkFence::NULL)
        .map_err(|e| format!("vkQueueSubmit2 failed: {e:?}"))?;
    queue
        .vkQueueWaitIdle()
        .map_err(|e| format!("vkQueueWaitIdle failed: {e:?}"))?;

    let mut timestamp_bytes = vec![0u8; TIMESTAMP_QUERY_COUNT as usize * size_of::<u64>()];
    query_pool
        .vkGetQueryPoolResults(
            0,
            TIMESTAMP_QUERY_COUNT,
            &mut timestamp_bytes,
            size_of::<u64>() as VkDeviceSize,
            VkQueryResultFlagBits::VK_QUERY_RESULT_64_BIT
                | VkQueryResultFlagBits::VK_QUERY_RESULT_WAIT_BIT,
        )
        .map_err(|e| format!("vkGetQueryPoolResults failed: {e:?}"))?;
    let timestamps = unsafe {
        core::slice::from_raw_parts(
            timestamp_bytes.as_ptr().cast::<u64>(),
            TIMESTAMP_QUERY_COUNT as usize,
        )
    };

    let mut total_ticks = 0u64;
    for run in 0..BENCHMARK_RUNS {
        let start = timestamps[run * 2];
        let end = timestamps[run * 2 + 1];
        total_ticks += end.saturating_sub(start);
    }

    Ok(total_ticks as f64 * timestamp_period_ns as f64)
}

fn transfer_to_shader_barrier(command_buffer: &CommandBuffer<'_>) {
    const MEMORY_BARRIERS: [VkMemoryBarrier2<'_>; 1] = [VkMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_WRITE_BIT)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT)
        .with_dstAccessMask(VkAccessFlagBits2(
            VkAccessFlagBits2::VK_ACCESS_2_SHADER_READ_BIT.0
                | VkAccessFlagBits2::VK_ACCESS_2_SHADER_WRITE_BIT.0,
        ))];
    const DEPENDENCY: VkDependencyInfo<'_> =
        VkDependencyInfo::DEFAULT.with_pMemoryBarriers(&MEMORY_BARRIERS);
    command_buffer.vkCmdPipelineBarrier2(&DEPENDENCY);
}

fn shader_to_transfer_barrier(command_buffer: &CommandBuffer<'_>) {
    const MEMORY_BARRIERS: [VkMemoryBarrier2<'_>; 1] = [VkMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_SHADER_WRITE_BIT)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        .with_dstAccessMask(VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_WRITE_BIT)];
    const DEPENDENCY: VkDependencyInfo<'_> =
        VkDependencyInfo::DEFAULT.with_pMemoryBarriers(&MEMORY_BARRIERS);
    command_buffer.vkCmdPipelineBarrier2(&DEPENDENCY);
}

fn shader_to_host_barrier(command_buffer: &CommandBuffer<'_>) {
    const MEMORY_BARRIERS: [VkMemoryBarrier2<'_>; 1] = [VkMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_SHADER_WRITE_BIT)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_HOST_BIT)
        .with_dstAccessMask(VkAccessFlagBits2::VK_ACCESS_2_HOST_READ_BIT)];
    const DEPENDENCY: VkDependencyInfo<'_> =
        VkDependencyInfo::DEFAULT.with_pMemoryBarriers(&MEMORY_BARRIERS);
    command_buffer.vkCmdPipelineBarrier2(&DEPENDENCY);
}

fn create_storage_buffer<'a>(
    allocator: &'a Allocator<'a>,
    size: VkDeviceSize,
) -> Result<vk_alloc::AllocatedBuffer<'a>, String> {
    const USAGE: VkBufferUsageFlags2CreateInfo<'_> = VkBufferUsageFlags2CreateInfo::DEFAULT
        .with_usage(VkBufferUsageFlagBits2(
            VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0
                | VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_TRANSFER_DST_BIT.0,
        ));
    let buffer_info = VkBufferCreateInfo::DEFAULT
        .with_size(size)
        .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_pNext_VkBufferUsageFlags2CreateInfo(&USAGE);

    allocator
        .create_buffer(
            &buffer_info,
            AllocationCreateInfo::new().with_memory_type_policy(
                vk_alloc::MemoryTypePolicy::UPLOAD.with_required_flags(
                    VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                        | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
                ),
            ),
        )
        .map_err(|e| format!("Buffer allocation failed: {e:?}"))
}

fn write_input(allocation: &mut vk_alloc::Allocation) -> Result<(), String> {
    let values = allocation
        .mapped_slice_mut::<u32>(ITEM_COUNT)
        .ok_or("Data allocation is not host mapped")?;
    for (index, value) in values.iter_mut().enumerate() {
        *value = index as u32;
    }
    Ok(())
}

fn clear_lookback(allocation: &mut vk_alloc::Allocation) -> Result<(), String> {
    let values = allocation
        .mapped_slice_mut::<u32>(LOOKBACK_WORDS)
        .ok_or("Lookback allocation is not host mapped")?;
    values.fill(0);
    Ok(())
}

fn verify_zero_prefix_sum(allocation: &vk_alloc::Allocation) -> Result<(), String> {
    let ptr = allocation.mapped_ptr().cast::<u32>();
    if ptr.is_null() {
        return Err("Data allocation is not host mapped".into());
    }

    let values = unsafe { core::slice::from_raw_parts(ptr, ITEM_COUNT) };
    for (index, &value) in values.iter().enumerate() {
        if value != 0 {
            return Err(format!(
                "Mismatch at index {index}: expected 0, got {value}"
            ));
        }
    }
    Ok(())
}

fn create_descriptor_pool<'a>(device: &'a Device<'a>) -> Result<DescriptorPool<'a>, String> {
    const POOL_SIZES: [VkDescriptorPoolSize; 1] = [VkDescriptorPoolSize::DEFAULT
        .with_type(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
        .with_descriptorCount(2)];
    const POOL_INFO: VkDescriptorPoolCreateInfo<'_> = VkDescriptorPoolCreateInfo::DEFAULT
        .with_maxSets(1)
        .with_flags(
            VkDescriptorPoolCreateFlagBits::VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT,
        )
        .with_pPoolSizes(&POOL_SIZES);
    device
        .vkCreateDescriptorPool(&POOL_INFO, null())
        .map_err(|e| format!("vkCreateDescriptorPool failed: {e:?}"))
}

fn create_descriptor_set<'a>(
    descriptor_pool: &'a DescriptorPool<'a>,
    layout: &DescriptorSetLayout<'a>,
    data_buffer: &Buffer<'a>,
    lookback_buffer: &Buffer<'a>,
) -> Result<Box<[DescriptorSet<'a>]>, String> {
    let layouts = [layout.raw()];
    let alloc_info = VkDescriptorSetAllocateInfo::DEFAULT
        .with_descriptorPool(descriptor_pool.raw())
        .with_pSetLayouts(&layouts);
    let sets = descriptor_pool
        .vkAllocateDescriptorSets(&alloc_info)
        .map_err(|e| format!("vkAllocateDescriptorSets failed: {e:?}"))?;
    let set = sets.first().ok_or("No descriptor sets allocated")?;

    let buffer_infos = [
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(data_buffer.raw())
            .with_range(DATA_SIZE),
        VkDescriptorBufferInfo::DEFAULT
            .with_buffer(lookback_buffer.raw())
            .with_range(LOOKBACK_SIZE),
    ];
    let writes = [
        VkWriteDescriptorSet::DEFAULT
            .with_dstSet(set.raw())
            .with_dstBinding(0)
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_pBufferInfo(&buffer_infos[0..1]),
        VkWriteDescriptorSet::DEFAULT
            .with_dstSet(set.raw())
            .with_dstBinding(1)
            .with_descriptorType(VkDescriptorType::VK_DESCRIPTOR_TYPE_STORAGE_BUFFER)
            .with_pBufferInfo(&buffer_infos[1..2]),
    ];
    set.device().vkUpdateDescriptorSets(&writes, &[]);

    Ok(sets)
}

fn enabled_validation_layers(entry: &Entry<'_>) -> Result<Vec<*const c_char>, String> {
    let layers = enumerate_instance_layers(entry)?;
    let validation_available = layers.iter().any(|layer| {
        let name = unsafe { CStr::from_ptr(layer.layerName.as_ptr()) };
        name == VALIDATION_LAYER
    });

    if validation_available {
        println!(
            "Validation layer: enabled ({})",
            VALIDATION_LAYER.to_string_lossy()
        );
        Ok(vec![VALIDATION_LAYER.as_ptr()])
    } else {
        println!(
            "Validation layer: unavailable ({})",
            VALIDATION_LAYER.to_string_lossy()
        );
        Ok(Vec::new())
    }
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
