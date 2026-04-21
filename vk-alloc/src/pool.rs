/// Opaque pool handle used in allocation requests.
///
/// Pool `0` is always the allocator's default pool; additional pools come from
/// `Allocator::create_pool` / `GroupAllocator::create_pool` and are selected by
/// `AllocationCreateInfo::with_pool`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pool(pub(crate) u32);

impl Pool {
    pub const DEFAULT: Self = Self(0);
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    pub const fn id(self) -> u32 {
        self.0
    }
}

/// User-facing pool configuration used when creating a pool.
///
/// Optional fields fall back to allocator defaults.
#[derive(Debug, Clone, Copy)]
pub struct PoolCreateInfo {
    pub host_visible_block_size: Option<u64>,
    pub device_local_block_size: Option<u64>,
    pub mixed_block_size: Option<u64>,
    pub dedicated_threshold: Option<u64>,
}

impl PoolCreateInfo {
    pub const DEFAULT: Self = Self {
        host_visible_block_size: None,
        device_local_block_size: None,
        mixed_block_size: None,
        dedicated_threshold: None,
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_host_visible_block_size(mut self, host_visible_block_size: u64) -> Self {
        self.host_visible_block_size = Some(host_visible_block_size);
        self
    }

    #[must_use]
    pub const fn with_device_local_block_size(mut self, device_local_block_size: u64) -> Self {
        self.device_local_block_size = Some(device_local_block_size);
        self
    }

    #[must_use]
    pub const fn with_mixed_block_size(mut self, mixed_block_size: u64) -> Self {
        self.mixed_block_size = Some(mixed_block_size);
        self
    }

    #[must_use]
    pub const fn with_dedicated_threshold(mut self, dedicated_threshold: u64) -> Self {
        self.dedicated_threshold = Some(dedicated_threshold);
        self
    }
}

impl Default for PoolCreateInfo {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PoolConfig {
    pub host_visible_block_size: u64,
    pub device_local_block_size: u64,
    pub mixed_block_size: u64,
    pub dedicated_threshold: Option<u64>,
}

impl PoolConfig {
    pub(crate) const DEFAULT: Self = Self {
        host_visible_block_size: 16 * 1024 * 1024,
        device_local_block_size: 64 * 1024 * 1024,
        mixed_block_size: 32 * 1024 * 1024,
        dedicated_threshold: None,
    };
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PoolConfig {
    pub(crate) fn from_create_info(info: PoolCreateInfo) -> Self {
        Self {
            host_visible_block_size: info
                .host_visible_block_size
                .unwrap_or(Self::DEFAULT.host_visible_block_size),
            device_local_block_size: info
                .device_local_block_size
                .unwrap_or(Self::DEFAULT.device_local_block_size),
            mixed_block_size: info
                .mixed_block_size
                .unwrap_or(Self::DEFAULT.mixed_block_size),
            dedicated_threshold: info.dedicated_threshold,
        }
    }
}
