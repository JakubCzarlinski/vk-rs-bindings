pub(crate) mod arena;
pub(crate) mod arena_key;
pub(crate) mod block;
mod owned;
pub(crate) mod range_allocator;
pub(crate) mod select;

use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::group_allocator::GroupBindMode;
use crate::memory::arena::{
    ArenaRegistry, ArenaState, SharedArena, make_block_allocation, make_dedicated_allocation,
};
use crate::memory::arena_key::ArenaKey;
use crate::memory::block::{BlockMemory, BlockMetadata};
use crate::memory::owned::allocate_owned_memory;
use crate::memory::select::{block_size_for, is_host_visible, should_dedicate};
use crate::pool::{Pool, PoolConfig};
use crate::resource::{AllocationCreateInfo, ResourceClass};
use crate::stats::StatsState;
use crate::vulkan::limits::{DeviceLimits, validate_allocation_size};
use crate::vulkan::requirements::RequirementInfo;
use alloc::sync::Arc;
use alloc::vec::Vec;
use parking_lot::RwLock;

#[derive(Debug, Clone, Copy)]
pub(crate) enum AllocationMode {
    Single,
    Group { device_mask: u32 },
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BindBehavior {
    Direct,
    Group { split_region_count: usize },
}

impl BindBehavior {
    const fn split_region_count(self) -> usize {
        match self {
            Self::Direct => 0,
            Self::Group {
                split_region_count, ..
            } => split_region_count,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArenaPartition {
    mode: AllocationMode,
    partition_mask: u32,
}

impl ArenaPartition {
    pub(crate) const fn single() -> Self {
        Self {
            mode: AllocationMode::Single,
            partition_mask: 0,
        }
    }

    pub(crate) const fn group(device_mask: u32, mode: GroupBindMode) -> Self {
        Self {
            mode: AllocationMode::Group { device_mask },
            partition_mask: match mode {
                GroupBindMode::Instance0 => 1,
                GroupBindMode::PerDeviceInstance | GroupBindMode::SplitInstanceRegions => {
                    device_mask
                }
            },
        }
    }

    const fn key_mask(self) -> u32 {
        self.partition_mask
    }

    const fn allocation_device_mask(self) -> Option<u32> {
        match self.mode {
            AllocationMode::Single => None,
            AllocationMode::Group { device_mask } => Some(device_mask),
        }
    }

    const fn block_metadata(self) -> BlockMetadata {
        match self.mode {
            AllocationMode::Single => BlockMetadata::Single,
            AllocationMode::Group { .. } => BlockMetadata::Group {
                device_mask: self.partition_mask,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AllocationRequest {
    pub(crate) resource_class: ResourceClass,
    pub(crate) requirement: RequirementInfo,
    pub(crate) dedicated_buffer: Option<vk::VkBuffer>,
    pub(crate) dedicated_image: Option<vk::VkImage>,
    pub(crate) alloc_info: AllocationCreateInfo,
    pub(crate) partition: ArenaPartition,
    pub(crate) bind_behavior: BindBehavior,
}

pub(crate) struct AllocationContext<'a, 'vk> {
    pub(crate) device: &'vk vk::Device<'vk>,
    pub(crate) memory_properties: &'a vk::VkPhysicalDeviceMemoryProperties,
    pub(crate) limits: &'a DeviceLimits,
    pub(crate) pools: &'a RwLock<Vec<PoolConfig>>,
    pub(crate) arenas: &'a RwLock<ArenaRegistry>,
    pub(crate) stats: Arc<StatsState>,
}

impl AllocationContext<'_, '_> {
    fn pool_config(&self, pool: Pool) -> Result<PoolConfig, AllocatorError> {
        self.pools
            .read()
            .get(pool.id() as usize)
            .copied()
            .ok_or(AllocatorError::InvalidPool)
    }

    fn arena_for(&self, key: ArenaKey) -> SharedArena {
        if let Some(existing) = self.arenas.read().get(&key).cloned() {
            return existing;
        }

        let mut arenas = self.arenas.write();
        if let Some(existing) = arenas.get(&key).cloned() {
            return existing;
        }

        let arena = Arc::new(RwLock::new(ArenaState::new()));
        arenas.insert(key, arena.clone());
        arena
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn allocate(
        &self,
        memory_type_index: u32,
        request: AllocationRequest,
    ) -> Result<Allocation, AllocatorError> {
        let pool = request.alloc_info.pool;
        let key = ArenaKey {
            memory_type_index,
            resource_class: request.resource_class,
            pool_id: pool.id(),
            partition_mask: request.partition.key_mask(),
        };
        validate_allocation_size(
            request.requirement.requirements.size,
            self.limits.max_memory_allocation_size,
        )?;
        let arena = self.arena_for(key);
        let config = self.pool_config(pool)?;
        let arena_block_size = block_size_for(self.memory_properties, memory_type_index, &config);

        let mut alloc_info = request.alloc_info;
        if alloc_info.dedicated_threshold.is_none() {
            alloc_info.dedicated_threshold = config.dedicated_threshold;
        }

        if should_dedicate(&alloc_info, request.requirement, arena_block_size)? {
            let memory = allocate_owned_memory(
                self.device,
                &request.requirement.requirements,
                memory_type_index,
                request.dedicated_buffer,
                request.dedicated_image,
                request.partition.allocation_device_mask(),
                is_host_visible(self.memory_properties, memory_type_index),
            )?;
            self.stats
                .on_allocate(request.requirement.requirements.size);
            return Ok(make_dedicated_allocation(
                0,
                ((pool.id() + 1) << 16) | memory_type_index,
                request.requirement.requirements.size,
                memory,
                self.stats.clone(),
            ));
        }

        let existing = {
            let arena = arena.read();
            arena.allocate_from_existing(
                request.requirement.requirements.size,
                request.requirement.requirements.alignment,
                self.stats.clone(),
            )
        };
        if let Some(allocation) = existing {
            self.stats
                .on_allocate(request.requirement.requirements.size);
            return Ok(allocation);
        }

        let mut arena = arena.write();
        if let Some(allocation) = arena.allocate_from_existing(
            request.requirement.requirements.size,
            request.requirement.requirements.alignment,
            self.stats.clone(),
        ) {
            self.stats
                .on_allocate(request.requirement.requirements.size);
            return Ok(allocation);
        }

        let block_size = if self.limits.max_memory_allocation_size == 0 {
            arena_block_size.max(request.requirement.requirements.size)
        } else {
            arena_block_size
                .max(request.requirement.requirements.size)
                .min(self.limits.max_memory_allocation_size)
        };
        let block_req = vk::VkMemoryRequirements::DEFAULT
            .with_size(block_size)
            .with_alignment(request.requirement.requirements.alignment)
            .with_memoryTypeBits(request.requirement.requirements.memoryTypeBits);
        let memory = allocate_owned_memory(
            self.device,
            &block_req,
            memory_type_index,
            None,
            None,
            request.partition.allocation_device_mask(),
            is_host_visible(self.memory_properties, memory_type_index),
        )?;
        let block_id = arena.next_block_id;
        let arena_id = ((pool.id() + 1) << 16) | memory_type_index;
        let block = Arc::new(BlockMemory::new(
            block_id,
            arena_id,
            memory.size(),
            request.partition.block_metadata(),
            memory,
        ));
        self.stats.on_block_allocated(block.size);
        arena.push_block(block.clone());
        drop(arena);
        self.stats
            .on_allocate(request.requirement.requirements.size);
        make_block_allocation(
            block,
            request.requirement.requirements.size,
            request.requirement.requirements.alignment,
            self.stats.clone(),
        )
        .map_err(|_| {
            if request.bind_behavior.split_region_count() != 0 {
                AllocatorError::InvalidSparseRegion
            } else {
                AllocatorError::OutOfAllocatorMetadata
            }
        })
    }
}
