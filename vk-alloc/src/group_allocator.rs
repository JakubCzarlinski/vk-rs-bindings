use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use parking_lot::{Mutex, RwLock};

use crate::allocation::{
    AllocatedBuffer, AllocatedImage, Allocation, LargeBuffer, LargeBufferCreateInfo,
};
use crate::error::AllocatorError;
use crate::internal::arena::{
    ArenaKey, ArenaState, GroupArenaRegistry, SharedGroupArena, group_block_meta,
    make_block_allocation, make_dedicated_allocation,
};
use crate::internal::block::BlockMemory;
use crate::internal::device_group::{partition_device_mask, validate_group_mode};
use crate::internal::vk_ops::{
    DeviceLimits, allocate_owned_memory, block_size_for, buffer_requirements, choose_memory_type,
    device_limits, image_requirements, is_host_visible, memory_properties,
    recommended_buffer_chunk_size, should_dedicate, validate_allocation_size,
};
use crate::pool::{Pool, PoolConfig, PoolCreateInfo};
use crate::resource::{
    AllocationCreateInfo, AllocatorCreateInfo, MemoryTypePolicy, ResourceClass,
    SparseAllocationCreateInfo,
};
use crate::sparse::{SparseBufferAllocation, SparseImageAllocation};
use crate::stats::{AllocatorStats, StatsState};

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
}

pub struct GroupAllocator<'vk> {
    device: &'vk vk::Device<'vk>,
    memory_properties: vk::VkPhysicalDeviceMemoryProperties,
    limits: DeviceLimits,
    default_pool: PoolConfig,
    pools: Mutex<Vec<PoolConfig>>,
    arenas: RwLock<GroupArenaRegistry>,
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
        crate::internal::device_group::validate_device_mask(info.device_mask)?;
        Ok(Self {
            device: info.base.device,
            memory_properties: memory_properties(info.base.physical_device),
            limits: device_limits(info.base.physical_device),
            default_pool: PoolConfig::from_create_info(info.base.default_pool),
            pools: Mutex::new(vec![PoolConfig::from_create_info(info.base.default_pool)]),
            arenas: RwLock::new(GroupArenaRegistry::new()),
            stats: Arc::new(StatsState::new()),
            device_mask: info.device_mask,
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

    pub fn allocate_for_buffer(
        &self,
        buffer: &vk::Buffer<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        self.allocate_for_buffer_with_mode(
            buffer,
            alloc_info
                .group_bind_mode
                .unwrap_or(GroupBindMode::Instance0),
            alloc_info,
        )
    }

    pub fn allocate_for_buffer_with_mode(
        &self,
        buffer: &vk::Buffer<'vk>,
        mode: GroupBindMode,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        let requirement = buffer_requirements(self.device, buffer);
        let memory_type_index = choose_memory_type(
            &self.memory_properties,
            requirement.requirements.memoryTypeBits,
            &alloc_info,
        )?;
        let heap_index = self.memory_properties.memoryTypes[memory_type_index as usize].heapIndex;
        let heap_flags = self.memory_properties.memoryHeaps[heap_index as usize].flags;
        validate_group_mode(mode, self.device_mask, heap_flags, false, 0)?;
        let allocation = self.allocate_common(
            memory_type_index,
            ResourceClass::Linear,
            alloc_info.pool,
            requirement,
            Some(buffer.raw()),
            None,
            mode,
            0,
        )?;
        self.bind_buffer(buffer, &allocation, mode)?;
        Ok(allocation)
    }

    pub fn allocate_for_image(
        &self,
        image: &vk::Image<'vk>,
        alloc_info: AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError> {
        self.allocate_for_image_with_mode(
            image,
            alloc_info
                .group_bind_mode
                .unwrap_or(GroupBindMode::Instance0),
            alloc_info,
            &[],
        )
    }

    pub fn allocate_for_image_with_mode(
        &self,
        image: &vk::Image<'vk>,
        mode: GroupBindMode,
        alloc_info: AllocationCreateInfo,
        split_regions: &[vk::VkRect2D],
    ) -> Result<Allocation, AllocatorError> {
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
        let allocation = self.allocate_common(
            memory_type_index,
            ResourceClass::Optimal,
            alloc_info.pool,
            requirement,
            None,
            Some(image.raw()),
            mode,
            split_regions.len(),
        )?;
        self.bind_image(image, &allocation, mode, split_regions)?;
        Ok(allocation)
    }

    pub fn create_buffer(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedBuffer<'vk>, AllocatorError> {
        self.create_buffer_with_mode(
            buffer_info,
            alloc_info
                .group_bind_mode
                .unwrap_or(GroupBindMode::Instance0),
            alloc_info,
        )
    }

    pub fn create_buffer_with_mode(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        mode: GroupBindMode,
        alloc_info: AllocationCreateInfo,
    ) -> Result<AllocatedBuffer<'vk>, AllocatorError> {
        let buffer = self
            .device
            .vkCreateBuffer(buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let allocation = self.allocate_for_buffer_with_mode(&buffer, mode, alloc_info)?;
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
        self.create_large_buffer_with_mode(
            buffer_info,
            alloc_info
                .group_bind_mode
                .unwrap_or(GroupBindMode::Instance0),
            alloc_info,
            large_info,
        )
    }

    pub fn create_large_buffer_with_mode(
        &self,
        buffer_info: &vk::VkBufferCreateInfo,
        mode: GroupBindMode,
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
            segments.push(self.create_buffer_with_mode(&segment_info, mode, alloc_info.clone())?);
            remaining -= segment_size;
        }
        Ok(LargeBuffer::new(buffer_info.size, chunk_size, segments))
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
            crate::internal::vk_ops::RequirementInfo {
                requirements,
                dedicated_required: false,
                dedicated_preferred: false,
            },
            None,
            None,
            alloc_info
                .group_bind_mode
                .unwrap_or(GroupBindMode::Instance0),
            0,
        )
    }

    fn pool_config(&self, pool: Pool) -> PoolConfig {
        self.pools
            .lock()
            .get(pool.id() as usize)
            .copied()
            .unwrap_or(self.default_pool)
    }

    fn arena_for(&self, key: ArenaKey) -> SharedGroupArena {
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

    #[allow(clippy::too_many_lines)]
    fn allocate_common(
        &self,
        memory_type_index: u32,
        resource_class: ResourceClass,
        pool: Pool,
        requirement: crate::internal::vk_ops::RequirementInfo,
        dedicated_buffer: Option<vk::VkBuffer>,
        dedicated_image: Option<vk::VkImage>,
        mode: GroupBindMode,
        split_region_count: usize,
    ) -> Result<Allocation, AllocatorError> {
        let key = ArenaKey {
            memory_type_index,
            resource_class,
            pool_id: pool.id(),
            partition_mask: match mode {
                GroupBindMode::PerDeviceInstance | GroupBindMode::SplitInstanceRegions => {
                    self.device_mask
                }
                GroupBindMode::Instance0 => 1,
            },
        };
        validate_allocation_size(
            requirement.requirements.size,
            self.limits.max_memory_allocation_size,
        )?;
        let arena = self.arena_for(key);
        let config = self.pool_config(pool);
        let arena_block_size = block_size_for(&self.memory_properties, memory_type_index, &config);
        let alloc_info = AllocationCreateInfo {
            memory_type_policy: MemoryTypePolicy::new(),
            strategy: crate::resource::AllocationStrategy::Auto,
            pool,
            dedicated_threshold: config.dedicated_threshold,
            group_bind_mode: Some(mode),
        };
        if should_dedicate(&alloc_info, requirement, arena_block_size)? {
            let memory = allocate_owned_memory(
                self.device,
                &requirement.requirements,
                memory_type_index,
                dedicated_buffer,
                dedicated_image,
                Some(self.device_mask),
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
        let memory = allocate_owned_memory(
            self.device,
            &vk::VkMemoryRequirements::DEFAULT
                .with_size(block_size)
                .with_alignment(requirement.requirements.alignment)
                .with_memoryTypeBits(requirement.requirements.memoryTypeBits),
            memory_type_index,
            None,
            None,
            Some(self.device_mask),
            is_host_visible(&self.memory_properties, memory_type_index),
        )?;
        let mut arena = arena.lock();
        let block_id = arena.next_block_id;
        let arena_id = ((pool.id() + 1) << 16) | memory_type_index;
        let block = Arc::new(BlockMemory::new(
            block_id,
            arena_id,
            memory.size(),
            group_block_meta(key.partition_mask),
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
        .map_err(|_| {
            if split_region_count != 0 {
                AllocatorError::InvalidSparseRegion
            } else {
                AllocatorError::OutOfAllocatorMetadata
            }
        })
    }

    fn bind_buffer(
        &self,
        buffer: &vk::Buffer<'vk>,
        allocation: &Allocation,
        mode: GroupBindMode,
    ) -> Result<(), AllocatorError> {
        let device_indices = match mode {
            GroupBindMode::Instance0 => vec![0],
            GroupBindMode::PerDeviceInstance => partition_device_mask(self.device_mask),
            GroupBindMode::SplitInstanceRegions => {
                return Err(AllocatorError::GroupModeUnsupported);
            }
        };
        let group_info = vk::VkBindBufferMemoryDeviceGroupInfo::DEFAULT
            .with_deviceIndexCount(device_indices.len() as u32)
            .with_pDeviceIndices(device_indices.as_ptr());
        let bind = vk::VkBindBufferMemoryInfo::DEFAULT
            .with_pNext((&raw const group_info).cast())
            .with_buffer(buffer.raw())
            .with_memory(allocation.memory())
            .with_memoryOffset(allocation.offset());
        self.device
            .vkBindBufferMemory2(1, &raw const bind)
            .map_err(AllocatorError::Vulkan)?;
        Ok(())
    }

    fn bind_image(
        &self,
        image: &vk::Image<'vk>,
        allocation: &Allocation,
        mode: GroupBindMode,
        split_regions: &[vk::VkRect2D],
    ) -> Result<(), AllocatorError> {
        let device_indices = match mode {
            GroupBindMode::Instance0 => vec![0],
            GroupBindMode::PerDeviceInstance | GroupBindMode::SplitInstanceRegions => {
                partition_device_mask(self.device_mask)
            }
        };
        let group_info = vk::VkBindImageMemoryDeviceGroupInfo::DEFAULT
            .with_deviceIndexCount(device_indices.len() as u32)
            .with_pDeviceIndices(device_indices.as_ptr())
            .with_splitInstanceBindRegionCount(split_regions.len() as u32)
            .with_pSplitInstanceBindRegions(split_regions.as_ptr());
        let bind = vk::VkBindImageMemoryInfo::DEFAULT
            .with_pNext((&raw const group_info).cast())
            .with_image(image.raw())
            .with_memory(allocation.memory())
            .with_memoryOffset(allocation.offset());
        self.device
            .vkBindImageMemory2(1, &raw const bind)
            .map_err(AllocatorError::Vulkan)?;
        Ok(())
    }
}
