use crate::allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, LargeBuffer, LargeBufferCreateInfo,
};
use crate::error::AllocatorError;
use crate::group::bind::{bind_buffer, bind_image};
use crate::group::device_mask::validate_device_mask;
use crate::group::mode::validate_group_mode;
use crate::memory::arena::ArenaRegistry;
use crate::memory::select::choose_memory_type;
use crate::memory::{AllocationContext, AllocationRequest, ArenaPartition, BindBehavior};
use crate::pool::{Pool, PoolConfig, PoolCreateInfo};
use crate::resource::{
    AllocationCreateInfo, AllocatorCreateInfo, ResourceClass, SparseAllocationCreateInfo,
};
use crate::sparse::{SparseBufferAllocation, SparseImageAllocation};
use crate::stats::{AllocatorStats, StatsState};
use crate::vulkan::limits::{DeviceLimits, device_limits, memory_properties};
use crate::vulkan::requirements::{
    RequirementInfo, buffer_requirements, image_requirements, recommended_buffer_chunk_size,
};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use parking_lot::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupBindMode {
    Instance0,
    PerDeviceInstance,
    SplitInstanceRegions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupFailurePolicy {
    StrictFail,
}

#[derive(Clone)]
pub struct GroupAllocatorCreateInfo<'vk> {
    pub base: AllocatorCreateInfo<'vk>,
    pub device_mask: u32,
    pub failure_policy: GroupFailurePolicy,
}

impl<'vk> GroupAllocatorCreateInfo<'vk> {
    pub const fn new(
        physical_device: &'vk vk::PhysicalDevice<'vk>,
        device: &'vk vk::Device<'vk>,
        device_mask: u32,
    ) -> Self {
        Self {
            base: AllocatorCreateInfo::new(physical_device, device),
            device_mask,
            failure_policy: GroupFailurePolicy::StrictFail,
        }
    }

    #[must_use]
    pub const fn with_base(mut self, base: AllocatorCreateInfo<'vk>) -> Self {
        self.base = base;
        self
    }

    #[must_use]
    pub const fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }

    #[must_use]
    pub const fn with_failure_policy(mut self, failure_policy: GroupFailurePolicy) -> Self {
        self.failure_policy = failure_policy;
        self
    }
}

pub struct GroupAllocator<'vk> {
    device: &'vk vk::Device<'vk>,
    memory_properties: vk::VkPhysicalDeviceMemoryProperties,
    limits: DeviceLimits,
    pools: RwLock<Vec<PoolConfig>>,
    arenas: RwLock<ArenaRegistry>,
    stats: Arc<StatsState>,
    device_mask: u32,
}

impl<'vk> GroupAllocator<'vk> {
    pub fn new(
        physical_device: &'vk vk::PhysicalDevice<'vk>,
        device: &'vk vk::Device<'vk>,
        device_mask: u32,
    ) -> Result<Self, AllocatorError> {
        Self::from_create_info(GroupAllocatorCreateInfo::new(
            physical_device,
            device,
            device_mask,
        ))
    }

    pub fn from_create_info(info: GroupAllocatorCreateInfo<'vk>) -> Result<Self, AllocatorError> {
        validate_device_mask(info.device_mask)?;
        Ok(Self {
            device: info.base.device,
            memory_properties: memory_properties(info.base.physical_device),
            limits: device_limits(info.base.physical_device),
            pools: RwLock::new(vec![PoolConfig::from_create_info(info.base.default_pool)]),
            arenas: RwLock::new(ArenaRegistry::new()),
            stats: Arc::new(StatsState::new()),
            device_mask: info.device_mask,
        })
    }

    pub fn stats(&self) -> AllocatorStats {
        self.stats.snapshot()
    }

    pub const fn device(&self) -> &'vk vk::Device<'vk> {
        self.device
    }

    pub const fn limits(&self) -> &DeviceLimits {
        &self.limits
    }

    pub fn create_pool(&self, info: PoolCreateInfo) -> Result<Pool, AllocatorError> {
        let mut pools = self.pools.write();
        if pools.len() >= u32::MAX as usize {
            return Err(AllocatorError::OutOfAllocatorMetadata);
        }
        pools.push(PoolConfig::from_create_info(info));
        Ok(Pool::new((pools.len() - 1) as u32))
    }

    pub fn allocate_for_buffer(
        &self,
        buffer: &vk::Buffer<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        self.allocate_for_buffer_with_mode(buffer, alloc_info)
    }

    pub fn allocate_for_buffer_with_mode(
        &self,
        buffer: &vk::Buffer<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let mode = alloc_info
            .group_bind_mode
            .unwrap_or(GroupBindMode::Instance0);
        let requirement = buffer_requirements(self.device, buffer);
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let heap_index = self.memory_properties.memoryTypes[memory_type_index as usize].heapIndex;
        let heap_flags = self.memory_properties.memoryHeaps[heap_index as usize].flags;
        validate_group_mode(mode, self.device_mask, heap_flags, false, 0)?;
        let allocation = self.allocation_context().allocate(
            memory_type_index,
            AllocationRequest {
                resource_class: ResourceClass::Linear,
                requirement,
                dedicated_buffer: Some(buffer.raw()),
                dedicated_image: None,
                alloc_info,
                partition: ArenaPartition::group(self.device_mask, mode),
                bind_behavior: BindBehavior::Group {
                    split_region_count: 0,
                },
            },
        )?;
        bind_buffer(self.device, buffer, &allocation, mode, self.device_mask)?;
        Ok(allocation)
    }

    pub fn allocate_for_image(
        &self,
        image: &vk::Image<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        self.allocate_for_image_with_mode(image, alloc_info, &[])
    }

    pub fn allocate_for_image_with_mode(
        &self,
        image: &vk::Image<'vk>,
        alloc_info: AllocationCreateInfo,
        split_regions: &[vk::VkRect2D],
    ) -> Result<Allocation, AllocatorError> {
        let mode = alloc_info
            .group_bind_mode
            .unwrap_or(GroupBindMode::Instance0);
        let requirement = image_requirements(self.device, image);
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let heap_index = self.memory_properties.memoryTypes[memory_type_index as usize].heapIndex;
        let heap_flags = self.memory_properties.memoryHeaps[heap_index as usize].flags;
        validate_group_mode(
            mode,
            self.device_mask,
            heap_flags,
            true,
            split_regions.len(),
        )?;
        let allocation = self.allocation_context().allocate(
            memory_type_index,
            AllocationRequest {
                resource_class: ResourceClass::Optimal,
                requirement,
                dedicated_buffer: None,
                dedicated_image: Some(image.raw()),
                alloc_info,
                partition: ArenaPartition::group(self.device_mask, mode),
                bind_behavior: BindBehavior::Group {
                    split_region_count: split_regions.len(),
                },
            },
        )?;
        bind_image(
            self.device,
            image,
            &allocation,
            mode,
            self.device_mask,
            split_regions,
        )?;
        Ok(allocation)
    }

    pub fn create_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedBuffer<'vk>, AllocatorError> {
        self.create_buffer_with_mode(buffer_info, alloc_info)
    }

    pub fn create_buffer_with_mode(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedBuffer<'vk>, AllocatorError> {
        let buffer = self
            .device
            .vkCreateBuffer(buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let allocation = self.allocate_for_buffer_with_mode(&buffer, alloc_info)?;
        Ok(AllocatedBuffer::new(buffer, allocation))
    }

    pub fn create_image(
        &self,
        image_info: &vk::VkImageCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedImage<'vk>, AllocatorError> {
        let image = self
            .device
            .vkCreateImage(image_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let allocation = self.allocate_for_image(&image, alloc_info)?;
        Ok(AllocatedImage::new(image, allocation))
    }

    pub fn create_sparse_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<SparseBufferAllocation<'vk>, AllocatorError> {
        SparseBufferAllocation::new_group(self.device, self, buffer_info, sparse_info)
    }

    pub fn create_sparse_image(
        &self,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<SparseImageAllocation<'vk>, AllocatorError> {
        SparseImageAllocation::new_group(self.device, self, image_info, sparse_info)
    }

    pub fn create_large_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
        large_info: LargeBufferCreateInfo,
    ) -> Result<LargeBuffer<'vk>, AllocatorError> {
        self.create_large_buffer_with_mode(buffer_info, alloc_info, large_info)
    }

    pub fn create_large_buffer_with_mode(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
        large_info: LargeBufferCreateInfo,
    ) -> Result<LargeBuffer<'vk>, AllocatorError> {
        let chunk_size = large_info.preferred_chunk_size.unwrap_or_else(|| {
            recommended_buffer_chunk_size(buffer_info.size, buffer_info.usage, self.limits)
        });
        let chunk_size = chunk_size.max(1);
        let mut segments = Vec::new();
        let mut remaining = buffer_info.size;
        while remaining != 0 {
            let segment_size = remaining.min(chunk_size);
            let segment_info = (*buffer_info).with_size(segment_size);
            segments.push(self.create_buffer_with_mode(&segment_info, alloc_info.clone())?);
            remaining -= segment_size;
        }
        Ok(LargeBuffer::new(buffer_info.size, chunk_size, segments))
    }

    pub(crate) fn allocate_page(
        &self,
        requirements: vk::VkMemoryRequirements,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let mode = alloc_info
            .group_bind_mode
            .unwrap_or(GroupBindMode::Instance0);
        let requirement = RequirementInfo {
            requirements,
            dedicated_required: false,
            dedicated_preferred: false,
        };
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        self.allocation_context().allocate(
            memory_type_index,
            AllocationRequest {
                resource_class: ResourceClass::Linear,
                requirement,
                dedicated_buffer: None,
                dedicated_image: None,
                alloc_info,
                partition: ArenaPartition::group(self.device_mask, mode),
                bind_behavior: BindBehavior::Group {
                    split_region_count: 0,
                },
            },
        )
    }

    fn allocation_context(&self) -> AllocationContext<'_, 'vk> {
        AllocationContext {
            device: self.device,
            memory_properties: &self.memory_properties,
            limits: &self.limits,
            pools: &self.pools,
            arenas: &self.arenas,
            stats: self.stats.clone(),
        }
    }
}
