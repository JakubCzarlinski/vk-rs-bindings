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

    pub(crate) const fn new() -> Self {
        Self::DEFAULT
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PoolConfig {
    pub(crate) fn from_create_info(info: PoolCreateInfo) -> Self {
        let mut config = Self::new();
        if let Some(value) = info.host_visible_block_size {
            config.host_visible_block_size = value;
        }
        if let Some(value) = info.device_local_block_size {
            config.device_local_block_size = value;
        }
        if let Some(value) = info.mixed_block_size {
            config.mixed_block_size = value;
        }
        config.dedicated_threshold = info.dedicated_threshold;
        config
    }
}
