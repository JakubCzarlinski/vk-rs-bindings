use crate::group_allocator::GroupBindMode;
use crate::pool::{Pool, PoolCreateInfo};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryTypePolicy {
    pub required_flags: u32,
    pub preferred_flags: u32,
    pub avoid_flags: u32,
}

impl Default for MemoryTypePolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTypePolicy {
    pub const DEFAULT: Self = Self {
        required_flags: 0,
        preferred_flags: 0,
        avoid_flags: 0,
    };

    pub const DEVICE_LOCAL: Self = Self {
        required_flags: 0,
        preferred_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0,
        avoid_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0,
    };

    pub const HOST_VISIBLE: Self = Self {
        required_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0,
        preferred_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
        avoid_flags: 0,
    };

    pub const UPLOAD: Self = Self {
        required_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0,
        preferred_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0
            | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0,
        avoid_flags: 0,
    };

    pub const READBACK: Self = Self {
        required_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0,
        preferred_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_CACHED_BIT.0
            | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
        avoid_flags: 0,
    };

    pub const UNIFIED: Self = Self {
        required_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT.0,
        preferred_flags: vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.0
            | vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT.0,
        avoid_flags: 0,
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_required_flags(mut self, flags: u32) -> Self {
        self.required_flags = flags;
        self
    }

    #[must_use]
    pub const fn with_preferred_flags(mut self, flags: u32) -> Self {
        self.preferred_flags = flags;
        self
    }

    #[must_use]
    pub const fn with_avoid_flags(mut self, flags: u32) -> Self {
        self.avoid_flags = flags;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    Auto,
    NeverDedicated,
    AlwaysDedicated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ResourceClass {
    Linear,
    Optimal,
}

#[derive(Clone)]
pub struct AllocatorCreateInfo<'vk> {
    pub physical_device: &'vk vk::PhysicalDevice<'vk>,
    pub device: &'vk vk::Device<'vk>,
    pub default_pool: PoolCreateInfo,
    pub max_metadata_slots: u32,
}

impl<'vk> AllocatorCreateInfo<'vk> {
    pub const fn new(
        physical_device: &'vk vk::PhysicalDevice<'vk>,
        device: &'vk vk::Device<'vk>,
    ) -> Self {
        Self {
            physical_device,
            device,
            default_pool: PoolCreateInfo::new(),
            max_metadata_slots: 1 << 20,
        }
    }

    #[must_use]
    pub const fn with_default_pool(mut self, default_pool: PoolCreateInfo) -> Self {
        self.default_pool = default_pool;
        self
    }

    #[must_use]
    pub const fn with_max_metadata_slots(mut self, max_metadata_slots: u32) -> Self {
        self.max_metadata_slots = max_metadata_slots;
        self
    }
}

#[derive(Debug, Clone)]
pub struct AllocationCreateInfo {
    pub memory_type_policy: MemoryTypePolicy,
    pub strategy: AllocationStrategy,
    pub pool: Pool,
    pub dedicated_threshold: Option<u64>,
    pub group_bind_mode: Option<GroupBindMode>,
}

impl AllocationCreateInfo {
    pub const DEFAULT: Self = Self {
        memory_type_policy: MemoryTypePolicy::DEFAULT,
        strategy: AllocationStrategy::Auto,
        pool: Pool::DEFAULT,
        dedicated_threshold: None,
        group_bind_mode: None,
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_memory_type_policy(mut self, memory_type_policy: MemoryTypePolicy) -> Self {
        self.memory_type_policy = memory_type_policy;
        self
    }

    #[must_use]
    pub const fn with_strategy(mut self, strategy: AllocationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    #[must_use]
    pub const fn with_pool(mut self, pool: Pool) -> Self {
        self.pool = pool;
        self
    }

    #[must_use]
    pub const fn with_dedicated_threshold(mut self, dedicated_threshold: u64) -> Self {
        self.dedicated_threshold = Some(dedicated_threshold);
        self
    }

    #[must_use]
    pub const fn without_dedicated_threshold(mut self) -> Self {
        self.dedicated_threshold = None;
        self
    }

    #[must_use]
    pub const fn with_group_bind_mode(mut self, group_bind_mode: GroupBindMode) -> Self {
        self.group_bind_mode = Some(group_bind_mode);
        self
    }

    #[must_use]
    pub const fn without_group_bind_mode(mut self) -> Self {
        self.group_bind_mode = None;
        self
    }
}

impl Default for AllocationCreateInfo {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Debug, Clone)]
pub struct SparseAllocationCreateInfo {
    pub memory_type_policy: MemoryTypePolicy,
    pub page_size: Option<u64>,
    pub queue_family_index: Option<u32>,
    pub pool: Pool,
    pub group_bind_mode: Option<GroupBindMode>,
    pub split_instance_regions: Vec<vk::VkRect2D>,
}

impl SparseAllocationCreateInfo {
    pub const DEFAULT: Self = Self {
        memory_type_policy: MemoryTypePolicy::DEFAULT,
        page_size: None,
        queue_family_index: None,
        pool: Pool::DEFAULT,
        group_bind_mode: None,
        split_instance_regions: Vec::new(),
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_memory_type_policy(mut self, memory_type_policy: MemoryTypePolicy) -> Self {
        self.memory_type_policy = memory_type_policy;
        self
    }

    #[must_use]
    pub const fn with_page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    #[must_use]
    pub const fn without_page_size(mut self) -> Self {
        self.page_size = None;
        self
    }

    #[must_use]
    pub const fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = Some(queue_family_index);
        self
    }

    #[must_use]
    pub const fn without_queue_family_index(mut self) -> Self {
        self.queue_family_index = None;
        self
    }

    #[must_use]
    pub const fn with_pool(mut self, pool: Pool) -> Self {
        self.pool = pool;
        self
    }

    #[must_use]
    pub const fn with_group_bind_mode(mut self, group_bind_mode: GroupBindMode) -> Self {
        self.group_bind_mode = Some(group_bind_mode);
        self
    }

    #[must_use]
    pub const fn without_group_bind_mode(mut self) -> Self {
        self.group_bind_mode = None;
        self
    }

    #[must_use]
    pub fn with_split_instance_regions(
        mut self,
        split_instance_regions: Vec<vk::VkRect2D>,
    ) -> Self {
        self.split_instance_regions = split_instance_regions;
        self
    }

    pub(crate) fn into_allocation_info(self) -> AllocationCreateInfo {
        AllocationCreateInfo::new()
            .with_memory_type_policy(self.memory_type_policy)
            .with_pool(self.pool)
            .with_strategy(AllocationStrategy::Auto)
            .with_group_bind_mode_option(self.group_bind_mode)
    }
}

impl AllocationCreateInfo {
    pub(crate) const fn with_group_bind_mode_option(
        mut self,
        group_bind_mode: Option<GroupBindMode>,
    ) -> Self {
        self.group_bind_mode = group_bind_mode;
        self
    }
}

impl Default for SparseAllocationCreateInfo {
    fn default() -> Self {
        Self::DEFAULT
    }
}
