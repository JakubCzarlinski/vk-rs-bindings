use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AllocatorStats {
    pub allocation_count: u64,
    pub free_count: u64,
    pub bytes_requested: u64,
    pub bytes_reserved: u64,
    pub dedicated_allocations: u64,
    pub block_count: u64,
}

impl AllocatorStats {
    pub const ZERO: Self = Self {
        allocation_count: 0,
        free_count: 0,
        bytes_requested: 0,
        bytes_reserved: 0,
        dedicated_allocations: 0,
        block_count: 0,
    };

    pub const fn new() -> Self {
        Self::ZERO
    }
}

#[derive(Debug, Default)]
pub(crate) struct StatsState {
    allocation_count: AtomicU64,
    free_count: AtomicU64,
    bytes_requested: AtomicU64,
    bytes_reserved: AtomicU64,
    dedicated_allocations: AtomicU64,
    block_count: AtomicU64,
}

impl StatsState {
    pub(crate) const fn new() -> Self {
        Self {
            allocation_count: AtomicU64::new(0),
            free_count: AtomicU64::new(0),
            bytes_requested: AtomicU64::new(0),
            bytes_reserved: AtomicU64::new(0),
            dedicated_allocations: AtomicU64::new(0),
            block_count: AtomicU64::new(0),
        }
    }

    pub(crate) fn on_allocate(&self, requested: u64) {
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
        self.bytes_requested.fetch_add(requested, Ordering::Relaxed);
    }

    pub(crate) fn on_free(&self) {
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn on_block_allocated(&self, reserved: u64) {
        self.block_count.fetch_add(1, Ordering::Relaxed);
        self.bytes_reserved.fetch_add(reserved, Ordering::Relaxed);
    }

    pub(crate) fn on_dedicated(&self) {
        self.dedicated_allocations.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn snapshot(&self) -> AllocatorStats {
        AllocatorStats {
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            free_count: self.free_count.load(Ordering::Relaxed),
            bytes_requested: self.bytes_requested.load(Ordering::Relaxed),
            bytes_reserved: self.bytes_reserved.load(Ordering::Relaxed),
            dedicated_allocations: self.dedicated_allocations.load(Ordering::Relaxed),
            block_count: self.block_count.load(Ordering::Relaxed),
        }
    }
}
