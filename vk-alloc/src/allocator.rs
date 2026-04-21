use crate::allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, HostImportBufferCreateInfo, ImportedHostBuffer,
    LargeBuffer, LargeBufferCreateInfo,
};
use crate::error::AllocatorError;
use crate::memory::arena::ArenaRegistry;
use crate::memory::select::choose_memory_type;
use crate::memory::{AllocationContext, AllocationRequest, ArenaPartition, BindBehavior};
use crate::pool::{Pool, PoolConfig, PoolCreateInfo};
use crate::resource::{
    AllocationCreateInfo, AllocatorCreateInfo, ResourceClass, SparseAllocationCreateInfo,
};
use crate::sparse::{SparseBufferAllocation, SparseImageAllocation};
use crate::stats::{AllocatorStats, StatsState};
use crate::vulkan::import::import_host_buffer;
use crate::vulkan::limits::{DeviceLimits, device_limits, memory_properties};
use crate::vulkan::requirements::{
    RequirementInfo, buffer_requirements, image_requirements, recommended_buffer_chunk_size,
};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use parking_lot::RwLock;

pub struct Allocator<'vk> {
    device: &'vk vk::Device<'vk>,
    memory_properties: vk::VkPhysicalDeviceMemoryProperties,
    limits: DeviceLimits,
    pools: RwLock<Vec<PoolConfig>>,
    arenas: RwLock<ArenaRegistry>,
    stats: Arc<StatsState>,
}

impl<'vk> Allocator<'vk> {
    pub fn new(
        physical_device: &'vk vk::PhysicalDevice<'vk>,
        device: &'vk vk::Device<'vk>,
    ) -> Result<Self, AllocatorError> {
        Self::from_create_info(AllocatorCreateInfo::new(physical_device, device))
    }

    pub fn from_create_info(info: AllocatorCreateInfo<'vk>) -> Result<Self, AllocatorError> {
        Ok(Self {
            device: info.device,
            memory_properties: memory_properties(info.physical_device),
            limits: device_limits(info.physical_device),
            pools: RwLock::new(vec![PoolConfig::from_create_info(info.default_pool)]),
            arenas: RwLock::new(ArenaRegistry::new()),
            stats: Arc::new(StatsState::new()),
        })
    }

    pub fn stats(&self) -> AllocatorStats {
        self.stats.snapshot()
    }

    pub fn device(&self) -> &'vk vk::Device<'vk> {
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

    pub fn create_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedBuffer<'vk>, AllocatorError> {
        let buffer = self
            .device
            .vkCreateBuffer(buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let allocation = self.allocate_for_buffer(&buffer, alloc_info)?;
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

    pub fn allocate_for_buffer(
        &self,
        buffer: &vk::Buffer<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let requirement = buffer_requirements(self.device, buffer);
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let allocation = self.allocation_context().allocate(
            memory_type_index,
            AllocationRequest {
                resource_class: ResourceClass::Linear,
                requirement,
                dedicated_buffer: Some(buffer.raw()),
                dedicated_image: None,
                alloc_info,
                partition: ArenaPartition::single(),
                bind_behavior: BindBehavior::Direct,
            },
        )?;
        buffer
            .vkBindBufferMemory(allocation.memory(), allocation.offset())
            .map_err(AllocatorError::Vulkan)?;
        Ok(allocation)
    }

    pub fn allocate_for_image(
        &self,
        image: &vk::Image<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let requirement = image_requirements(self.device, image);
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let allocation = self.allocation_context().allocate(
            memory_type_index,
            AllocationRequest {
                resource_class: ResourceClass::Optimal,
                requirement,
                dedicated_buffer: None,
                dedicated_image: Some(image.raw()),
                alloc_info,
                partition: ArenaPartition::single(),
                bind_behavior: BindBehavior::Direct,
            },
        )?;
        image
            .vkBindImageMemory(allocation.memory(), allocation.offset())
            .map_err(AllocatorError::Vulkan)?;
        Ok(allocation)
    }

    pub fn create_sparse_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<SparseBufferAllocation<'vk>, AllocatorError> {
        SparseBufferAllocation::new(self.device, self, buffer_info, sparse_info)
    }

    pub fn create_sparse_image(
        &self,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<SparseImageAllocation<'vk>, AllocatorError> {
        SparseImageAllocation::new(self.device, self, image_info, sparse_info)
    }

    pub fn create_large_buffer(
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
            segments.push(self.create_buffer(&segment_info, alloc_info.clone())?);
            remaining -= segment_size;
        }
        Ok(LargeBuffer::new(buffer_info.size, chunk_size, segments))
    }

    pub fn import_host_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        import_info: HostImportBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<ImportedHostBuffer<'vk>, AllocatorError> {
        import_host_buffer(
            self.device,
            &self.memory_properties,
            &self.limits,
            buffer_info,
            import_info,
            alloc_info,
        )
    }

    pub(crate) fn allocate_page(
        &self,
        requirements: vk::VkMemoryRequirements,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
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
                partition: ArenaPartition::single(),
                bind_behavior: BindBehavior::Direct,
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
