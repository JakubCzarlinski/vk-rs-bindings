use alloc::vec::Vec;

use crate::group_allocator::GroupBindMode;
use crate::pool::Pool;

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
pub enum ResourceClass {
    Linear,
    Optimal,
    DedicatedOnly,
}

#[derive(Clone)]
pub struct AllocatorCreateInfo<'vk> {
    pub physical_device: &'vk vk::PhysicalDevice<'vk>,
    pub device: &'vk vk::Device<'vk>,
    pub default_pool: crate::pool::PoolCreateInfo,
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
            default_pool: crate::pool::PoolCreateInfo::new(),
            max_metadata_slots: 1 << 20,
        }
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
}

impl Default for SparseAllocationCreateInfo {
    fn default() -> Self {
        Self::DEFAULT
    }
}
