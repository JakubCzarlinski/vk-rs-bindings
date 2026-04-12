use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use parking_lot::Mutex;

use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::internal::block::{
    BlockMemory, DedicatedMemory, GroupBlockMetadata, SharedSource, SingleBlockMetadata,
    mapped_with_offset,
};
use crate::internal::vk_ops::OwnedMemory;
use crate::stats::StatsState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArenaKey {
    pub memory_type_index: u32,
    pub resource_class: crate::resource::ResourceClass,
    pub pool_id: u32,
    pub partition_mask: u32,
}

#[derive(Debug)]
pub struct ArenaState<BlockMeta> {
    pub next_block_id: u32,
    pub blocks: Vec<Arc<BlockMemory<BlockMeta>>>,
}

impl<BlockMeta> ArenaState<BlockMeta>
where
    BlockMeta: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            next_block_id: 1,
            blocks: Vec::new(),
        }
    }

    pub(crate) fn allocate_from_existing(
        &self,
        request_size: u64,
        alignment: u64,
        stats: Arc<StatsState>,
    ) -> Option<Allocation> {
        self.blocks.iter().find_map(|block| {
            let offset = block.allocate(request_size, alignment)?;
            let source: SharedSource = block.clone();
            Some(Allocation::new(
                block.id,
                block.arena_id,
                offset,
                request_size,
                mapped_with_offset(&source, offset),
                source,
                Some(stats.clone()),
            ))
        })
    }

    pub fn push_block(&mut self, block: Arc<BlockMemory<BlockMeta>>) {
        self.blocks.push(block);
        self.next_block_id += 1;
    }
}

pub type SharedSingleArena = Arc<Mutex<ArenaState<SingleBlockMetadata>>>;
pub type SharedGroupArena = Arc<Mutex<ArenaState<GroupBlockMetadata>>>;
pub type SingleArenaRegistry = BTreeMap<ArenaKey, SharedSingleArena>;
pub type GroupArenaRegistry = BTreeMap<ArenaKey, SharedGroupArena>;

pub(crate) fn make_block_allocation<M>(
    block: Arc<BlockMemory<M>>,
    request_size: u64,
    alignment: u64,
    stats: Arc<StatsState>,
) -> Result<Allocation, AllocatorError>
where
    M: Send + Sync + 'static,
{
    let offset = block
        .allocate(request_size, alignment)
        .ok_or(AllocatorError::OutOfAllocatorMetadata)?;
    let source: SharedSource = block.clone();
    Ok(Allocation::new(
        block.id,
        block.arena_id,
        offset,
        request_size,
        mapped_with_offset(&source, offset),
        source,
        Some(stats),
    ))
}

pub(crate) fn make_dedicated_allocation(
    block_id: u32,
    arena_id: u32,
    size: u64,
    memory: OwnedMemory,
    stats: Arc<StatsState>,
) -> Allocation {
    stats.on_dedicated();
    let dedicated = Arc::new(DedicatedMemory {
        block_id,
        arena_id,
        memory,
    });
    let source: SharedSource = dedicated;
    Allocation::new(
        block_id,
        arena_id,
        0,
        size,
        mapped_with_offset(&source, 0),
        source,
        Some(stats),
    )
}

pub fn single_block_meta() -> SingleBlockMetadata {
    SingleBlockMetadata
}

pub fn group_block_meta(device_mask: u32) -> GroupBlockMetadata {
    GroupBlockMetadata { device_mask }
}
