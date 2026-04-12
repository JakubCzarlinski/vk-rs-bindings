use vk::VkResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocatorError {
    Vulkan(VkResult),
    AllocationTooLarge,
    InvalidHostPointerAlignment,
    InvalidHostImport,
    HostVisibleRequired,
    OutOfBounds,
    NoCompatibleMemoryType,
    DedicatedAllocationRequired,
    GroupModeUnsupported,
    InvalidDeviceMask,
    SubsetAllocationUnsupported,
    PeerMemoryUnsupported,
    SparseBindingUnsupported,
    SparseQueueRequired,
    OutOfAllocatorMetadata,
    InvalidSparseRegion,
}

impl From<VkResult> for AllocatorError {
    fn from(value: VkResult) -> Self {
        Self::Vulkan(value)
    }
}
