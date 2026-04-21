use crate::allocation::Allocation;
use crate::error::AllocatorError;
use crate::memory::arena_key::ArenaKey;
use crate::memory::block::{BlockMemory, DedicatedMemory, SharedSource, mapped_with_offset};
use crate::memory::owned::OwnedMemory;
use crate::stats::StatsState;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use parking_lot::RwLock;

#[derive(Debug)]
pub(crate) struct ArenaState {
    pub(crate) next_block_id: u32,
    blocks: Vec<Arc<BlockMemory>>,
}

impl ArenaState {
    pub(crate) const fn new() -> Self {
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

    pub(crate) fn push_block(&mut self, block: Arc<BlockMemory>) {
        self.blocks.push(block);
        self.next_block_id += 1;
    }
}

pub(crate) type SharedArena = Arc<RwLock<ArenaState>>;
pub(crate) type ArenaRegistry = BTreeMap<ArenaKey, SharedArena>;

pub(crate) fn make_block_allocation(
    block: Arc<BlockMemory>,
    request_size: u64,
    alignment: u64,
    stats: Arc<StatsState>,
) -> Result<Allocation, AllocatorError> {
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
    let dedicated = Arc::new(DedicatedMemory { memory });
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
