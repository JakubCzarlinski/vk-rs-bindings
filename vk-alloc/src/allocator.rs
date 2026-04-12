use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use parking_lot::{Mutex, RwLock};

use crate::allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, HostImportBufferCreateInfo, ImportedHostBuffer,
    LargeBuffer, LargeBufferCreateInfo,
};
use crate::error::AllocatorError;
use crate::internal::arena::{
    ArenaKey, ArenaState, SharedSingleArena, SingleArenaRegistry, make_block_allocation,
    make_dedicated_allocation, single_block_meta,
};
use crate::internal::block::BlockMemory;
use crate::internal::vk_ops::{
    DeviceLimits, allocate_owned_memory, block_size_for, buffer_requirements, choose_memory_type,
    device_limits, image_requirements, is_host_visible, memory_properties,
    recommended_buffer_chunk_size, should_dedicate, validate_allocation_size,
    validate_host_pointer_alignment,
};
use crate::pool::{Pool, PoolConfig, PoolCreateInfo};
use crate::resource::{
    AllocationCreateInfo, AllocatorCreateInfo, ResourceClass, SparseAllocationCreateInfo,
};
use crate::sparse::{SparseBufferAllocation, SparseImageAllocation};
use crate::stats::{AllocatorStats, StatsState};

pub struct Allocator<'vk> {
    device: &'vk vk::Device<'vk>,
    memory_properties: vk::VkPhysicalDeviceMemoryProperties,
    limits: DeviceLimits,
    default_pool: PoolConfig,
    pools: Mutex<Vec<PoolConfig>>,
    arenas: RwLock<SingleArenaRegistry>,
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
            default_pool: PoolConfig::from_create_info(info.default_pool),
            pools: Mutex::new(vec![PoolConfig::from_create_info(info.default_pool)]),
            arenas: RwLock::new(SingleArenaRegistry::new()),
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
        let mut pools = self.pools.lock();
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
        let allocation = self.allocate_common(
            memory_type_index,
            ResourceClass::Linear,
            alloc_info.pool,
            alloc_info.clone(),
            requirement,
            Some(buffer.raw()),
            None,
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
        let allocation = self.allocate_common(
            memory_type_index,
            ResourceClass::Optimal,
            alloc_info.pool,
            alloc_info,
            requirement,
            None,
            Some(image.raw()),
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
        if import_info.host_ptr.is_null() || import_info.size == 0 {
            return Err(AllocatorError::OutOfBounds);
        }
        if buffer_info.size != import_info.size {
            return Err(AllocatorError::InvalidHostImport);
        }
        validate_host_pointer_alignment(
            import_info.host_ptr,
            self.limits.min_imported_host_pointer_alignment,
        )?;
        validate_allocation_size(import_info.size, self.limits.max_memory_allocation_size)?;
        let external_memory_info = vk::VkExternalMemoryBufferCreateInfo::DEFAULT
            .with_handleTypes(import_info.handle_type.0);
        let buffer_info = (*buffer_info).with_pNext((&raw const external_memory_info).cast());
        let buffer = self
            .device
            .vkCreateBuffer(&raw const buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let requirement = buffer_requirements(self.device, &buffer);
        let mut host_props = vk::VkMemoryHostPointerPropertiesEXT::DEFAULT;
        self.device
            .vkGetMemoryHostPointerPropertiesEXT(
                import_info.handle_type,
                import_info.host_ptr.cast(),
                &raw mut host_props,
            )
            .map_err(AllocatorError::Vulkan)?;
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            host_props.memoryTypeBits & requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let import = vk::VkImportMemoryHostPointerInfoEXT::DEFAULT
            .with_handleType(import_info.handle_type)
            .with_pHostPointer(import_info.host_ptr.cast());
        let allocate_info = vk::VkMemoryAllocateInfo::DEFAULT
            .with_pNext((&raw const import).cast())
            .with_allocationSize(requirement.requirements.size)
            .with_memoryTypeIndex(memory_type_index);
        let memory = self
            .device
            .vkAllocateMemory(&raw const allocate_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        buffer
            .vkBindBufferMemory(memory.raw(), 0)
            .map_err(AllocatorError::Vulkan)?;
        Ok(ImportedHostBuffer::new(
            buffer,
            memory,
            import_info.host_ptr,
            import_info.size,
        ))
    }

    pub(crate) fn allocate_page(
        &self,
        requirements: vk::VkMemoryRequirements,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirements.memoryTypeBits,
            &alloc_info,
        )?;
        self.allocate_common(
            memory_type_index,
            ResourceClass::Linear,
            alloc_info.pool,
            alloc_info,
            crate::internal::vk_ops::RequirementInfo {
                requirements,
                dedicated_required: false,
                dedicated_preferred: false,
            },
            None,
            None,
        )
    }

    fn pool_config(&self, pool: Pool) -> PoolConfig {
        self.pools
            .lock()
            .get(pool.id() as usize)
            .copied()
            .unwrap_or(self.default_pool)
    }

    fn arena_for(&self, key: ArenaKey) -> SharedSingleArena {
        if let Some(existing) = self.arenas.read().get(&key).cloned() {
            return existing;
        }

        let mut arenas = self.arenas.write();
        if let Some(existing) = arenas.get(&key).cloned() {
            return existing;
        }

        let arena = Arc::new(Mutex::new(ArenaState::new()));
        arenas.insert(key, arena.clone());
        arena
    }

    fn allocate_common(
        &self,
        memory_type_index: u32,
        resource_class: ResourceClass,
        pool: Pool,
        alloc_info: AllocationCreateInfo,
        requirement: crate::internal::vk_ops::RequirementInfo,
        dedicated_buffer: Option<vk::VkBuffer>,
        dedicated_image: Option<vk::VkImage>,
    ) -> Result<Allocation, AllocatorError> {
        let key = ArenaKey {
            memory_type_index,
            resource_class,
            pool_id: pool.id(),
            partition_mask: 0,
        };
        validate_allocation_size(
            requirement.requirements.size,
            self.limits.max_memory_allocation_size,
        )?;
        let arena = self.arena_for(key);
        let config = self.pool_config(pool);
        let arena_block_size = block_size_for(&self.memory_properties, memory_type_index, &config);
        if should_dedicate(&alloc_info, requirement, arena_block_size)? {
            let memory = allocate_owned_memory(
                self.device,
                &requirement.requirements,
                memory_type_index,
                dedicated_buffer,
                dedicated_image,
                None,
                is_host_visible(&self.memory_properties, memory_type_index),
            )?;
            self.stats.on_allocate(requirement.requirements.size);
            return Ok(make_dedicated_allocation(
                0,
                ((pool.id() + 1) << 16) | memory_type_index,
                requirement.requirements.size,
                memory,
                self.stats.clone(),
            ));
        }

        let existing = {
            let arena = arena.lock();
            arena.allocate_from_existing(
                requirement.requirements.size,
                requirement.requirements.alignment,
                self.stats.clone(),
            )
        };
        if let Some(allocation) = existing {
            self.stats.on_allocate(requirement.requirements.size);
            return Ok(allocation);
        }

        let block_size = if self.limits.max_memory_allocation_size == 0 {
            arena_block_size.max(requirement.requirements.size)
        } else {
            arena_block_size
                .max(requirement.requirements.size)
                .min(self.limits.max_memory_allocation_size)
        };
        let block_req = vk::VkMemoryRequirements::DEFAULT
            .with_size(block_size)
            .with_alignment(requirement.requirements.alignment)
            .with_memoryTypeBits(requirement.requirements.memoryTypeBits);
        let memory = allocate_owned_memory(
            self.device,
            &block_req,
            memory_type_index,
            None,
            None,
            None,
            is_host_visible(&self.memory_properties, memory_type_index),
        )?;
        let mut arena = arena.lock();
        let block_id = arena.next_block_id;
        let arena_id = ((pool.id() + 1) << 16) | memory_type_index;
        let block = Arc::new(BlockMemory::new(
            block_id,
            arena_id,
            memory.size(),
            single_block_meta(),
            memory,
        ));
        self.stats.on_block_allocated(block.size);
        arena.push_block(block.clone());
        drop(arena);
        self.stats.on_allocate(requirement.requirements.size);
        make_block_allocation(
            block,
            requirement.requirements.size,
            requirement.requirements.alignment,
            self.stats.clone(),
        )
    }
}
