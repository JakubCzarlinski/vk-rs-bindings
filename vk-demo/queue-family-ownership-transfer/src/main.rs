#![allow(deprecated)]

use core::ffi::{CStr, c_char};
use core::iter;
use vk::*;
use vk_alloc::{AllocationCreateInfo, Allocator, MemoryTypePolicy};

const BUFFER_WORDS: usize = 16;
const BUFFER_SIZE: VkDeviceSize = (BUFFER_WORDS * size_of::<u32>()) as VkDeviceSize;
const FILL_PATTERN: u32 = 0x1234_abcd;
const GRAPHICS_RELEASED_VALUE: u64 = 1;
const TRANSFER_COMPLETE_VALUE: u64 = 2;
const GRAPHICS_COMPLETE_VALUE: u64 = 3;

const APP_INFO: VkApplicationInfo = VkApplicationInfo::DEFAULT
    .with_apiVersion(VK_API_VERSION_1_4)
    .with_applicationVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_engineVersion(VK_MAKE_VERSION(0, 1, 0))
    .with_pEngineName(c"vk-demo".as_ptr())
    .with_pApplicationName(c"Queue Family Ownership Transfer Demo".as_ptr());

const DEVICE_CREATE_INFO: VkDeviceCreateInfo = VkDeviceCreateInfo::DEFAULT;
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

#[derive(Debug, Clone, Copy)]
struct QueueFamilySelection {
    graphics: u32,
    transfer: u32,
}

fn main() -> Result<(), String> {
    let library = VulkanLib::load().map_err(|e| e.to_string())?;
    let entry = Entry::new(&library);
    print_loader_info(&entry)?;

    let instance = {
        let layer_names = enabled_validation_layers(&entry)?;
        let instance_info = VkInstanceCreateInfo::DEFAULT
            .with_pApplicationInfo(&APP_INFO)
            .with_ppEnabledLayerNames(&layer_names);
        entry
            .vkCreateInstance(&instance_info, null())
            .map_err(|e| format!("vkCreateInstance failed: {e:?}"))
    }?;

    let physical_device = instance
        .vkEnumeratePhysicalDevices()
        .map_err(|e| format!("vkEnumeratePhysicalDevices failed: {e:?}"))?
        .into_iter()
        .next()
        .ok_or("No physical devices found")?;

    print_physical_device(&physical_device);

    let selection = {
        let queue_families = queue_family_properties(&physical_device);
        print_queue_families(&queue_families);
        select_queue_families(&queue_families)
    }
    .ok_or("No device queue family pair with graphics and distinct transfer support found")?;
    println!(
        "Selected queue families: graphics={} transfer={}",
        selection.graphics, selection.transfer
    );

    let device = create_device(&physical_device, selection)?;
    let allocator = Allocator::new(&physical_device, &device)
        .map_err(|e| format!("Allocator creation failed: {e:?}"))?;

    run_transfer_demo(&device, &allocator, selection)?;

    Ok(())
}

fn print_loader_info(entry: &Entry<'_>) -> Result<(), String> {
    let mut api_version = VK_API_VERSION_1_0;
    entry
        .vkEnumerateInstanceVersion(&mut api_version)
        .map_err(|e| format!("vkEnumerateInstanceVersion failed: {e:?}"))?;

    println!("Vulkan loader");
    println!("  API version: {}", format_version(api_version));
    Ok(())
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

fn print_physical_device(physical_device: &PhysicalDevice<'_>) {
    let mut properties = VkPhysicalDeviceProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceProperties2(&mut properties);
    println!(
        "Physical device: {}",
        c_char_array_to_string(&properties.properties.deviceName)
    );
    println!(
        "  API version: {}",
        format_version(properties.properties.apiVersion)
    );
}

fn print_queue_families(queue_families: &[VkQueueFamilyProperties2<'_>]) {
    println!("Queue families: {}", queue_families.len());
    for (index, family) in queue_families.iter().enumerate() {
        let props = family.queueFamilyProperties;
        println!(
            "  [{index}] count={} flags={} timestamp_bits={} granularity={}x{}x{}",
            props.queueCount,
            props.queueFlags,
            props.timestampValidBits,
            props.minImageTransferGranularity.width,
            props.minImageTransferGranularity.height,
            props.minImageTransferGranularity.depth,
        );
    }
}

fn select_queue_families(
    queue_families: &[VkQueueFamilyProperties2<'_>],
) -> Option<QueueFamilySelection> {
    let graphics = queue_families.iter().position(|family| {
        let props = family.queueFamilyProperties;
        props.queueCount > 0
            && props
                .queueFlags
                .intersects(VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT)
    })? as u32;

    let dedicated_transfer = queue_families
        .iter()
        .enumerate()
        .filter(|(index, family)| {
            *index as u32 != graphics && family.queueFamilyProperties.queueCount > 0
        })
        .filter(|(_, family)| {
            let flags = family.queueFamilyProperties.queueFlags;
            flags.intersects(VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT)
                && !flags.intersects(VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT)
                && !flags.intersects(VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT)
        })
        .map(|(index, _)| index as u32)
        .next();

    let transfer = dedicated_transfer.or_else(|| {
        queue_families
            .iter()
            .enumerate()
            .find(|(index, family)| {
                *index as u32 != graphics
                    && family.queueFamilyProperties.queueCount > 0
                    && family
                        .queueFamilyProperties
                        .queueFlags
                        .intersects(VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT)
            })
            .map(|(index, _)| index as u32)
    })?;

    Some(QueueFamilySelection { graphics, transfer })
}

fn create_device<'inst>(
    physical_device: &'inst PhysicalDevice<'inst>,
    selection: QueueFamilySelection,
) -> Result<Device<'inst>, String> {
    const PRIORITIES: [f32; 1] = [1.0];

    let queue_infos = [
        VkDeviceQueueCreateInfo::DEFAULT
            .with_queueFamilyIndex(selection.graphics)
            .with_pQueuePriorities(&PRIORITIES),
        VkDeviceQueueCreateInfo::DEFAULT
            .with_queueFamilyIndex(selection.transfer)
            .with_pQueuePriorities(&PRIORITIES),
    ];
    let mut vulkan13_features =
        VkPhysicalDeviceVulkan13Features::DEFAULT.with_synchronization2(VK_TRUE);
    let vulkan12_features = VkPhysicalDeviceVulkan12Features::DEFAULT
        .with_timelineSemaphore(VK_TRUE)
        .with_pNext_chain_VkDeviceCreateInfo(&mut vulkan13_features);
    let device_info = DEVICE_CREATE_INFO
        .with_pQueueCreateInfos(&queue_infos)
        .with_pNext_VkPhysicalDeviceVulkan12Features(&vulkan12_features);

    physical_device
        .vkCreateDevice(&device_info, null())
        .map_err(|e| format!("vkCreateDevice failed: {e:?}"))
}

fn run_transfer_demo(
    device: &Device<'_>,
    allocator: &Allocator<'_>,
    selection: QueueFamilySelection,
) -> Result<(), String> {
    let graphics_queue = device.vkGetDeviceQueue(selection.graphics, 0);
    let transfer_queue = device.vkGetDeviceQueue(selection.transfer, 0);

    let source = create_transfer_buffer(allocator, BUFFER_SIZE)?;
    let destination = create_transfer_buffer(allocator, BUFFER_SIZE)?;
    let readback = create_transfer_buffer(allocator, BUFFER_SIZE)?;

    let graphics_pool = create_command_pool(device, selection.graphics)?;
    let transfer_pool = create_command_pool(device, selection.transfer)?;

    let graphics_release = allocate_command_buffer(&graphics_pool)?;
    let transfer_copy = allocate_command_buffer(&transfer_pool)?;
    let graphics_readback = allocate_command_buffer(&graphics_pool)?;

    record_graphics_release(
        &graphics_release,
        source.buffer(),
        destination.buffer(),
        selection,
    )?;
    record_transfer_copy(
        &transfer_copy,
        source.buffer(),
        destination.buffer(),
        selection,
    )?;
    record_graphics_readback(
        &graphics_readback,
        destination.buffer(),
        readback.buffer(),
        selection,
    )?;

    let timeline = create_timeline_semaphore(device)?;

    submit(
        &graphics_queue,
        &graphics_release,
        &[],
        &[(timeline.raw(), GRAPHICS_RELEASED_VALUE)],
        VkFence::NULL,
    )?;
    println!(
        "graphics: filled buffers, released ownership, signaled timeline value {GRAPHICS_RELEASED_VALUE}"
    );

    submit(
        &transfer_queue,
        &transfer_copy,
        &[(timeline.raw(), GRAPHICS_RELEASED_VALUE)],
        &[(timeline.raw(), TRANSFER_COMPLETE_VALUE)],
        VkFence::NULL,
    )?;
    println!(
        "transfer: waited value {GRAPHICS_RELEASED_VALUE}, copied, released destination, signaled value {TRANSFER_COMPLETE_VALUE}"
    );

    submit(
        &graphics_queue,
        &graphics_readback,
        &[(timeline.raw(), TRANSFER_COMPLETE_VALUE)],
        &[(timeline.raw(), GRAPHICS_COMPLETE_VALUE)],
        VkFence::NULL,
    )?;
    wait_for_timeline(device, timeline.raw(), GRAPHICS_COMPLETE_VALUE)?;
    println!(
        "graphics: waited value {TRANSFER_COMPLETE_VALUE}, acquired destination, copied to readback, signaled value {GRAPHICS_COMPLETE_VALUE}"
    );

    verify_readback(readback.allocation())?;
    Ok(())
}

fn record_graphics_release(
    cmd: &CommandBuffer<'_>,
    source: &Buffer<'_>,
    destination: &Buffer<'_>,
    selection: QueueFamilySelection,
) -> Result<(), String> {
    cmd.vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("vkBeginCommandBuffer graphics release failed: {e:?}"))?;

    cmd.vkCmdFillBuffer(source.raw(), 0, BUFFER_SIZE, FILL_PATTERN);
    cmd.vkCmdFillBuffer(destination.raw(), 0, BUFFER_SIZE, 0);

    let release_barriers = [
        queue_release_barrier(source.raw(), selection.graphics, selection.transfer),
        queue_release_barrier(destination.raw(), selection.graphics, selection.transfer),
    ];
    let dependency = VkDependencyInfo::DEFAULT.with_pBufferMemoryBarriers(&release_barriers);
    cmd.vkCmdPipelineBarrier2(&dependency);

    cmd.vkEndCommandBuffer()
        .map_err(|e| format!("vkEndCommandBuffer graphics release failed: {e:?}"))?;
    Ok(())
}

fn record_transfer_copy(
    cmd: &CommandBuffer<'_>,
    source: &Buffer<'_>,
    destination: &Buffer<'_>,
    selection: QueueFamilySelection,
) -> Result<(), String> {
    cmd.vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("vkBeginCommandBuffer transfer failed: {e:?}"))?;

    let acquire_barriers = [
        queue_acquire_barrier(
            source.raw(),
            selection.graphics,
            selection.transfer,
            VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_READ_BIT,
        ),
        queue_acquire_barrier(
            destination.raw(),
            selection.graphics,
            selection.transfer,
            VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_WRITE_BIT,
        ),
    ];
    let acquire_dependency =
        VkDependencyInfo::DEFAULT.with_pBufferMemoryBarriers(&acquire_barriers);
    cmd.vkCmdPipelineBarrier2(&acquire_dependency);

    let copy_region = VkBufferCopy2::DEFAULT.with_size(BUFFER_SIZE);
    let copy_regions = [copy_region];
    let copy_info = VkCopyBufferInfo2::DEFAULT
        .with_srcBuffer(source.raw())
        .with_dstBuffer(destination.raw())
        .with_pRegions(&copy_regions);
    cmd.vkCmdCopyBuffer2(&copy_info);

    let release_barriers = [queue_release_barrier(
        destination.raw(),
        selection.transfer,
        selection.graphics,
    )];
    let release_dependency =
        VkDependencyInfo::DEFAULT.with_pBufferMemoryBarriers(&release_barriers);
    cmd.vkCmdPipelineBarrier2(&release_dependency);

    cmd.vkEndCommandBuffer()
        .map_err(|e| format!("vkEndCommandBuffer transfer failed: {e:?}"))?;
    Ok(())
}

fn record_graphics_readback(
    cmd: &CommandBuffer<'_>,
    destination: &Buffer<'_>,
    readback: &Buffer<'_>,
    selection: QueueFamilySelection,
) -> Result<(), String> {
    cmd.vkBeginCommandBuffer(&VkCommandBufferBeginInfo::DEFAULT)
        .map_err(|e| format!("vkBeginCommandBuffer graphics readback failed: {e:?}"))?;

    let acquire_barriers = [queue_acquire_barrier(
        destination.raw(),
        selection.transfer,
        selection.graphics,
        VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_READ_BIT,
    )];
    let acquire_dependency =
        VkDependencyInfo::DEFAULT.with_pBufferMemoryBarriers(&acquire_barriers);
    cmd.vkCmdPipelineBarrier2(&acquire_dependency);

    let copy_region = VkBufferCopy2::DEFAULT.with_size(BUFFER_SIZE);
    let copy_regions = [copy_region];
    let copy_info = VkCopyBufferInfo2::DEFAULT
        .with_srcBuffer(destination.raw())
        .with_dstBuffer(readback.raw())
        .with_pRegions(&copy_regions);
    cmd.vkCmdCopyBuffer2(&copy_info);

    let host_barriers = [VkBufferMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_WRITE_BIT)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_HOST_BIT)
        .with_dstAccessMask(VkAccessFlagBits2::VK_ACCESS_2_HOST_READ_BIT)
        .with_srcQueueFamilyIndex(selection.graphics)
        .with_dstQueueFamilyIndex(selection.graphics)
        .with_buffer(readback.raw())
        .with_size(BUFFER_SIZE)];
    let host_dependency = VkDependencyInfo::DEFAULT.with_pBufferMemoryBarriers(&host_barriers);
    cmd.vkCmdPipelineBarrier2(&host_dependency);

    cmd.vkEndCommandBuffer()
        .map_err(|e| format!("vkEndCommandBuffer graphics readback failed: {e:?}"))?;
    Ok(())
}

fn queue_release_barrier(
    buffer: VkBuffer,
    src_family: u32,
    dst_family: u32,
) -> VkBufferMemoryBarrier2<'static> {
    VkBufferMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_TRANSFER_WRITE_BIT)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_NONE)
        .with_dstAccessMask(VkAccessFlagBits2::VK_ACCESS_2_NONE)
        .with_srcQueueFamilyIndex(src_family)
        .with_dstQueueFamilyIndex(dst_family)
        .with_buffer(buffer)
        .with_size(BUFFER_SIZE)
}

fn queue_acquire_barrier(
    buffer: VkBuffer,
    src_family: u32,
    dst_family: u32,
    dst_access: VkAccessFlags2,
) -> VkBufferMemoryBarrier2<'static> {
    VkBufferMemoryBarrier2::DEFAULT
        .with_srcStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_NONE)
        .with_srcAccessMask(VkAccessFlagBits2::VK_ACCESS_2_NONE)
        .with_dstStageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        .with_dstAccessMask(dst_access)
        .with_srcQueueFamilyIndex(src_family)
        .with_dstQueueFamilyIndex(dst_family)
        .with_buffer(buffer)
        .with_size(BUFFER_SIZE)
}

fn create_transfer_buffer<'a>(
    allocator: &'a Allocator<'a>,
    size: VkDeviceSize,
) -> Result<vk_alloc::AllocatedBuffer<'a>, String> {
    let usage = VkBufferUsageFlags2CreateInfo::DEFAULT.with_usage(
        VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT
            | VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_TRANSFER_DST_BIT,
    );
    let buffer_info = VkBufferCreateInfo::DEFAULT
        .with_sharingMode(VkSharingMode::VK_SHARING_MODE_EXCLUSIVE)
        .with_pNext_VkBufferUsageFlags2CreateInfo(&usage)
        .with_size(size);

    allocator
        .create_buffer(
            &buffer_info,
            AllocationCreateInfo::new().with_memory_type_policy(
                MemoryTypePolicy::UPLOAD.with_required_flags(
                    VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT
                        | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
                ),
            ),
        )
        .map_err(|e| format!("Buffer allocation failed: {e:?}"))
}

fn create_command_pool<'a>(
    device: &'a Device<'a>,
    queue_family: u32,
) -> Result<CommandPool<'a>, String> {
    let pool_info = VkCommandPoolCreateInfo::DEFAULT
        .with_flags(VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT)
        .with_queueFamilyIndex(queue_family);

    device
        .vkCreateCommandPool(&pool_info, null())
        .map_err(|e| format!("vkCreateCommandPool failed: {e:?}"))
}

fn allocate_command_buffer<'a>(pool: &'a CommandPool<'a>) -> Result<CommandBuffer<'a>, String> {
    let info = VkCommandBufferAllocateInfo::DEFAULT
        .with_level(VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY)
        .with_commandBufferCount(1)
        .with_commandPool(pool.raw());
    let buffers = pool
        .vkAllocateCommandBuffers(&info)
        .map_err(|e| format!("vkAllocateCommandBuffers failed: {e:?}"))?;

    buffers
        .into_vec()
        .pop()
        .ok_or("vkAllocateCommandBuffers returned no command buffers".into())
}

fn create_timeline_semaphore<'a>(device: &'a Device<'a>) -> Result<Semaphore<'a>, String> {
    let timeline_info = VkSemaphoreTypeCreateInfo::DEFAULT
        .with_semaphoreType(VkSemaphoreType::VK_SEMAPHORE_TYPE_TIMELINE)
        .with_initialValue(0);
    let semaphore_info =
        VkSemaphoreCreateInfo::DEFAULT.with_pNext_VkSemaphoreTypeCreateInfo(&timeline_info);

    device
        .vkCreateSemaphore(&semaphore_info, null())
        .map_err(|e| format!("vkCreateSemaphore timeline failed: {e:?}"))
}

fn submit(
    queue: &Queue<'_>,
    command_buffer: &CommandBuffer<'_>,
    waits: &[(VkSemaphore, u64)],
    signals: &[(VkSemaphore, u64)],
    fence: VkFence,
) -> Result<(), String> {
    let wait_infos: Vec<_> = waits
        .iter()
        .map(|&(semaphore, value)| {
            VkSemaphoreSubmitInfo::DEFAULT
                .with_semaphore(semaphore)
                .with_value(value)
                .with_stageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        })
        .collect();
    let signal_infos: Vec<_> = signals
        .iter()
        .map(|&(semaphore, value)| {
            VkSemaphoreSubmitInfo::DEFAULT
                .with_semaphore(semaphore)
                .with_value(value)
                .with_stageMask(VkPipelineStageFlagBits2::VK_PIPELINE_STAGE_2_TRANSFER_BIT)
        })
        .collect();
    let command_infos =
        [VkCommandBufferSubmitInfo::DEFAULT.with_commandBuffer(command_buffer.raw())];
    let submit = VkSubmitInfo2::DEFAULT
        .with_pWaitSemaphoreInfos(&wait_infos)
        .with_pCommandBufferInfos(&command_infos)
        .with_pSignalSemaphoreInfos(&signal_infos);

    queue
        .vkQueueSubmit2(&[submit], fence)
        .map_err(|e| format!("vkQueueSubmit2 failed: {e:?}"))?;

    Ok(())
}

fn wait_for_timeline(
    device: &Device<'_>,
    semaphore: VkSemaphore,
    value: u64,
) -> Result<(), String> {
    let semaphores = [semaphore];
    let values = [value];
    let wait_info = VkSemaphoreWaitInfo::DEFAULT.with_semaphoreCount_slices(&semaphores, &values);

    device
        .vkWaitSemaphores(&wait_info, u64::MAX)
        .map_err(|e| format!("vkWaitSemaphores failed: {e:?}"))?;
    Ok(())
}

fn verify_readback(allocation: &vk_alloc::Allocation) -> Result<(), String> {
    let ptr = allocation.mapped_ptr().cast::<u32>();
    if ptr.is_null() {
        return Err("Readback allocation is not host mapped".into());
    }

    let values = unsafe { core::slice::from_raw_parts(ptr, BUFFER_WORDS) };
    if values.iter().copied().all(|value| value == FILL_PATTERN) {
        println!("Success: readback contains {BUFFER_WORDS} copies of {FILL_PATTERN:#010x}");
        Ok(())
    } else {
        Err(format!(
            "Readback mismatch: expected all values to be {FILL_PATTERN:#010x}, got {values:#010x?}"
        ))
    }
}

fn queue_family_properties(
    physical_device: &PhysicalDevice<'_>,
) -> Vec<VkQueueFamilyProperties2<'static>> {
    let mut count = 0;
    physical_device.vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, null_mut());

    let mut properties: Vec<_> =
        iter::repeat_n(VkQueueFamilyProperties2::DEFAULT, count as usize).collect();
    if count != 0 {
        physical_device
            .vkGetPhysicalDeviceQueueFamilyProperties2(&mut count, properties.as_mut_ptr());
        properties.truncate(count as usize);
    }

    properties
}

fn c_char_array_to_string(chars: &[c_char]) -> String {
    let bytes: Vec<u8> = chars
        .iter()
        .take_while(|&&c| c != 0)
        .map(|&c| c as u8)
        .collect();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn format_version(version: u32) -> String {
    format!(
        "{}.{}.{}",
        VK_API_VERSION_MAJOR(version),
        VK_API_VERSION_MINOR(version),
        VK_API_VERSION_PATCH(version),
    )
}
