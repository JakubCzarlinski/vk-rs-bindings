use alloc::sync::Arc;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::error::AllocatorError;
use crate::resource::{AllocationCreateInfo, AllocationStrategy, MemoryTypePolicy};

#[derive(Debug)]
pub struct DeviceFns {
    pub device: vk::VkDevice,
    pub free_memory: vk::PFN_vkFreeMemory,
    pub map_memory: vk::PFN_vkMapMemory,
    pub unmap_memory: vk::PFN_vkUnmapMemory,
}

#[derive(Debug)]
pub struct OwnedMemory {
    raw: vk::VkDeviceMemory,
    size: u64,
    mapped: AtomicPtr<u8>,
    host_visible: bool,
    device_fns: Arc<DeviceFns>,
}

unsafe impl Send for OwnedMemory {}
unsafe impl Sync for OwnedMemory {}

impl OwnedMemory {
    pub fn new(
        raw: vk::VkDeviceMemory,
        size: u64,
        host_visible: bool,
        device_fns: Arc<DeviceFns>,
    ) -> Self {
        Self {
            raw,
            size,
            mapped: AtomicPtr::new(null_mut()),
            host_visible,
            device_fns,
        }
    }

    pub fn raw(&self) -> vk::VkDeviceMemory {
        self.raw
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn mapped_ptr(&self) -> *mut u8 {
        if !self.host_visible {
            return null_mut();
        }
        let cached = self.mapped.load(Ordering::Acquire);
        if !cached.is_null() {
            return cached;
        }
        let mut out: *mut c_void = null_mut();
        let result = unsafe {
            (self.device_fns.map_memory)(
                self.device_fns.device,
                self.raw,
                0,
                self.size,
                0,
                &raw mut out,
            )
        };
        if result >= vk::VkResult::VK_SUCCESS {
            let out = out.cast::<u8>();
            self.mapped.store(out, Ordering::Release);
            out
        } else {
            null_mut()
        }
    }
}

impl Drop for OwnedMemory {
    fn drop(&mut self) {
        let mapped = self.mapped.swap(null_mut(), Ordering::AcqRel);
        if !mapped.is_null() {
            unsafe {
                (self.device_fns.unmap_memory)(self.device_fns.device, self.raw);
            }
        }
        unsafe {
            (self.device_fns.free_memory)(self.device_fns.device, self.raw, null());
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequirementInfo {
    pub requirements: vk::VkMemoryRequirements,
    pub dedicated_required: bool,
    pub dedicated_preferred: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceLimits {
    pub max_memory_allocation_size: u64,
    pub max_storage_buffer_range: u32,
    pub max_uniform_buffer_range: u32,
    pub min_imported_host_pointer_alignment: u64,
}

pub fn memory_properties(
    physical_device: &vk::PhysicalDevice<'_>,
) -> vk::VkPhysicalDeviceMemoryProperties {
    let mut props = vk::VkPhysicalDeviceMemoryProperties2::DEFAULT;
    physical_device.vkGetPhysicalDeviceMemoryProperties2(&raw mut props);
    props.memoryProperties
}

pub fn device_limits(physical_device: &vk::PhysicalDevice<'_>) -> DeviceLimits {
    let mut host = vk::VkPhysicalDeviceExternalMemoryHostPropertiesEXT::DEFAULT;
    let mut maintenance3 = vk::VkPhysicalDeviceMaintenance3Properties::DEFAULT;
    maintenance3 = maintenance3.with_pNext((&raw mut host).cast::<c_void>());
    let mut props = vk::VkPhysicalDeviceProperties2::DEFAULT
        .with_pNext((&raw mut maintenance3).cast::<c_void>());
    physical_device.vkGetPhysicalDeviceProperties2(&raw mut props);
    DeviceLimits {
        max_memory_allocation_size: maintenance3.maxMemoryAllocationSize,
        max_storage_buffer_range: props.properties.limits.maxStorageBufferRange,
        max_uniform_buffer_range: props.properties.limits.maxUniformBufferRange,
        min_imported_host_pointer_alignment: host.minImportedHostPointerAlignment,
    }
}

pub fn buffer_requirements(device: &vk::Device<'_>, buffer: &vk::Buffer<'_>) -> RequirementInfo {
    let info = vk::VkBufferMemoryRequirementsInfo2::DEFAULT.with_buffer(buffer.raw());
    let mut dedicated = vk::VkMemoryDedicatedRequirements::DEFAULT;
    let mut req =
        vk::VkMemoryRequirements2::DEFAULT.with_pNext((&raw mut dedicated).cast::<c_void>());
    device.vkGetBufferMemoryRequirements2(&raw const info, &raw mut req);
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation != 0,
        dedicated_preferred: dedicated.prefersDedicatedAllocation != 0,
    }
}

pub fn image_requirements(device: &vk::Device<'_>, image: &vk::Image<'_>) -> RequirementInfo {
    let info = vk::VkImageMemoryRequirementsInfo2::DEFAULT.with_image(image.raw());
    let mut dedicated = vk::VkMemoryDedicatedRequirements::DEFAULT;
    let mut req =
        vk::VkMemoryRequirements2::DEFAULT.with_pNext((&raw mut dedicated).cast::<c_void>());
    device.vkGetImageMemoryRequirements2(&raw const info, &raw mut req);
    RequirementInfo {
        requirements: req.memoryRequirements,
        dedicated_required: dedicated.requiresDedicatedAllocation != 0,
        dedicated_preferred: dedicated.prefersDedicatedAllocation != 0,
    }
}

pub fn score_memory_type(policy: MemoryTypePolicy, property_flags: u32) -> Option<i32> {
    if (property_flags & policy.required_flags) != policy.required_flags {
        return None;
    }
    let mut score = 0;
    score += ((property_flags & policy.preferred_flags).count_ones() as i32) * 16;
    score -= ((property_flags & policy.avoid_flags).count_ones() as i32) * 8;
    score += (property_flags.count_ones() as i32) * 2;
    Some(score)
}

pub fn choose_memory_type(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    alloc_info: &AllocationCreateInfo,
) -> Result<u32, AllocatorError> {
    let mut best = None;
    for index in 0..properties.memoryTypeCount {
        let mask = 1u32 << index;
        if memory_type_bits & mask == 0 {
            continue;
        }
        let property_flags = properties.memoryTypes[index as usize].propertyFlags;
        if let Some(score) = score_memory_type(alloc_info.memory_type_policy, property_flags) {
            match best {
                Some((_, best_score)) if best_score >= score => {}
                _ => best = Some((index, score)),
            }
        }
    }
    best.map(|(index, _)| index)
        .ok_or(AllocatorError::NoCompatibleMemoryType)
}

pub fn is_host_visible(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> bool {
    properties.memoryTypes[memory_type_index as usize].propertyFlags
        & vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0
        != 0
}

pub(crate) fn is_device_local(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
) -> bool {
    properties.memoryTypes[memory_type_index as usize].propertyFlags
        & vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0
        != 0
}

pub(crate) fn block_size_for(
    properties: &vk::VkPhysicalDeviceMemoryProperties,
    memory_type_index: u32,
    pool: &crate::pool::PoolConfig,
) -> u64 {
    let host_visible = is_host_visible(properties, memory_type_index);
    let device_local = is_device_local(properties, memory_type_index);
    if host_visible {
        pool.host_visible_block_size
    } else if device_local {
        pool.device_local_block_size
    } else {
        pool.mixed_block_size
    }
}

pub fn should_dedicate(
    alloc_info: &AllocationCreateInfo,
    requirements: RequirementInfo,
    arena_block_size: u64,
) -> Result<bool, AllocatorError> {
    if requirements.dedicated_required {
        return match alloc_info.strategy {
            AllocationStrategy::NeverDedicated => Err(AllocatorError::DedicatedAllocationRequired),
            _ => Ok(true),
        };
    }
    match alloc_info.strategy {
        AllocationStrategy::AlwaysDedicated => Ok(true),
        AllocationStrategy::NeverDedicated => Ok(false),
        AllocationStrategy::Auto => {
            let threshold = alloc_info
                .dedicated_threshold
                .unwrap_or(arena_block_size / 2);
            Ok(requirements.requirements.size >= threshold || requirements.dedicated_preferred)
        }
    }
}

pub fn allocate_owned_memory(
    device: &vk::Device<'_>,
    requirements: &vk::VkMemoryRequirements,
    memory_type_index: u32,
    dedicated_buffer: Option<vk::VkBuffer>,
    dedicated_image: Option<vk::VkImage>,
    device_mask: Option<u32>,
    host_visible: bool,
) -> Result<OwnedMemory, AllocatorError> {
    let mut dedicated_info = vk::VkMemoryDedicatedAllocateInfo::DEFAULT;
    let mut flags_info = vk::VkMemoryAllocateFlagsInfo::DEFAULT;
    let mut allocate_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(requirements.size)
        .with_memoryTypeIndex(memory_type_index);

    let mut next = null::<c_void>();
    if dedicated_buffer.is_some() || dedicated_image.is_some() {
        dedicated_info = dedicated_info
            .with_buffer(dedicated_buffer.unwrap_or(vk::VkBuffer::NULL))
            .with_image(dedicated_image.unwrap_or(vk::VkImage::NULL));
        next = (&raw const dedicated_info).cast::<c_void>();
    }
    if let Some(mask) = device_mask {
        flags_info = flags_info
            .with_flags(vk::VkMemoryAllocateFlagBits::VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT.0)
            .with_deviceMask(mask)
            .with_pNext(next);
        next = (&raw const flags_info).cast::<c_void>();
    }
    if !next.is_null() {
        allocate_info = allocate_info.with_pNext(next);
    }

    let memory = device
        .vkAllocateMemory(&raw const allocate_info, null())
        .map_err(AllocatorError::Vulkan)?;
    let free_memory = memory.table().vkFreeMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let map_memory = memory.table().vkMapMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let unmap_memory = memory.table().vkUnmapMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let device_fns = Arc::new(DeviceFns {
        device: memory.parent().raw(),
        free_memory,
        map_memory,
        unmap_memory,
    });
    let raw = memory.raw();
    core::mem::forget(memory);
    Ok(OwnedMemory::new(
        raw,
        requirements.size,
        host_visible,
        device_fns,
    ))
}

pub fn validate_allocation_size(
    requested_size: u64,
    max_memory_allocation_size: u64,
) -> Result<(), AllocatorError> {
    if max_memory_allocation_size != 0 && requested_size > max_memory_allocation_size {
        Err(AllocatorError::AllocationTooLarge)
    } else {
        Ok(())
    }
}

pub fn validate_host_pointer_alignment(
    host_ptr: *mut u8,
    required_alignment: u64,
) -> Result<(), AllocatorError> {
    if required_alignment <= 1 {
        return Ok(());
    }
    if (host_ptr as usize as u64) % required_alignment == 0 {
        Ok(())
    } else {
        Err(AllocatorError::InvalidHostPointerAlignment)
    }
}

pub fn recommended_buffer_chunk_size(
    total_size: u64,
    usage_flags: u32,
    limits: DeviceLimits,
) -> u64 {
    let mut chunk = if limits.max_memory_allocation_size != 0 {
        limits.max_memory_allocation_size
    } else {
        total_size
    };
    if usage_flags & (vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT.0 as u32)
        != 0
        && limits.max_storage_buffer_range != 0
    {
        chunk = chunk.min(limits.max_storage_buffer_range as u64);
    }
    if usage_flags & (vk::VkBufferUsageFlagBits2::VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT.0 as u32)
        != 0
        && limits.max_uniform_buffer_range != 0
    {
        chunk = chunk.min(limits.max_uniform_buffer_range as u64);
    }
    chunk.max(1)
}
