use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use parking_lot::Mutex;

use crate::allocation::Allocation;
use crate::allocator::Allocator;
use crate::error::AllocatorError;
use crate::group_allocator::{GroupAllocator, GroupBindMode};
use crate::internal::page_table::PageTable;
use crate::resource::{AllocationCreateInfo, SparseAllocationCreateInfo};

#[derive(Debug, Clone)]
pub struct SparseBufferBindList {
    binds: Vec<vk::VkSparseMemoryBind>,
}

impl SparseBufferBindList {
    pub fn binds(&self) -> &[vk::VkSparseMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct SparseImageBindList {
    binds: Vec<vk::VkSparseImageMemoryBind>,
}

impl SparseImageBindList {
    pub fn binds(&self) -> &[vk::VkSparseImageMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct PreparedBindSparseInfo {
    buffer_binds: Vec<vk::VkSparseMemoryBind>,
    buffer_infos: Vec<vk::VkSparseBufferMemoryBindInfo>,
    image_binds: Vec<vk::VkSparseImageMemoryBind>,
    image_infos: Vec<vk::VkSparseImageMemoryBindInfo>,
    info: vk::VkBindSparseInfo,
}

impl PreparedBindSparseInfo {
    pub fn vk_info(&self) -> &vk::VkBindSparseInfo {
        &self.info
    }

    pub fn buffer_binds(&self) -> &[vk::VkSparseMemoryBind] {
        &self.buffer_binds
    }

    pub fn buffer_infos(&self) -> &[vk::VkSparseBufferMemoryBindInfo] {
        &self.buffer_infos
    }

    pub fn image_binds(&self) -> &[vk::VkSparseImageMemoryBind] {
        &self.image_binds
    }

    pub fn image_infos(&self) -> &[vk::VkSparseImageMemoryBindInfo] {
        &self.image_infos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SparsePageKey(u64);

pub struct SparseBufferAllocation<'vk> {
    buffer: vk::Buffer<'vk>,
    page_size: u64,
    pages: Arc<Mutex<PageTable<SparsePageKey, Allocation>>>,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseBufferAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk vk::Device<'vk>,
        _allocator: &Allocator<'vk>,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        if buffer_info.flags & vk::VkBufferCreateFlagBits::VK_BUFFER_CREATE_SPARSE_BINDING_BIT.0
            == 0
        {
            return Err(AllocatorError::SparseBindingUnsupported);
        }
        let buffer = device
            .vkCreateBuffer(buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let mut requirements = vk::VkMemoryRequirements::DEFAULT;
        buffer.vkGetBufferMemoryRequirements(&raw mut requirements);
        Ok(Self {
            buffer,
            page_size: sparse_info
                .page_size
                .unwrap_or(requirements.alignment.max(1)),
            pages: Arc::new(Mutex::new(PageTable::default())),
            base_alloc_info: AllocationCreateInfo {
                memory_type_policy: sparse_info.memory_type_policy,
                strategy: crate::resource::AllocationStrategy::Auto,
                pool: sparse_info.pool,
                dedicated_threshold: None,
                group_bind_mode: sparse_info.group_bind_mode,
            },
        })
    }

    pub(crate) fn new_group(
        device: &'vk vk::Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        if sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions) {
            return Err(AllocatorError::GroupModeUnsupported);
        }
        if buffer_info.flags & vk::VkBufferCreateFlagBits::VK_BUFFER_CREATE_SPARSE_BINDING_BIT.0
            == 0
        {
            return Err(AllocatorError::SparseBindingUnsupported);
        }
        let buffer = device
            .vkCreateBuffer(buffer_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let mut requirements = vk::VkMemoryRequirements::DEFAULT;
        buffer.vkGetBufferMemoryRequirements(&raw mut requirements);
        Ok(Self {
            buffer,
            page_size: sparse_info
                .page_size
                .unwrap_or(requirements.alignment.max(1)),
            pages: Arc::new(Mutex::new(PageTable::default())),
            base_alloc_info: AllocationCreateInfo {
                memory_type_policy: sparse_info.memory_type_policy,
                strategy: crate::resource::AllocationStrategy::Auto,
                pool: sparse_info.pool,
                dedicated_threshold: None,
                group_bind_mode: sparse_info.group_bind_mode,
            },
        })
    }

    pub fn buffer(&self) -> &vk::Buffer<'vk> {
        &self.buffer
    }

    pub fn update_page(
        &self,
        allocator: &Allocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        let key = SparsePageKey(page_index);
        if resident {
            let requirements = vk::VkMemoryRequirements::DEFAULT
                .with_size(self.page_size)
                .with_alignment(self.page_size)
                .with_memoryTypeBits(u32::MAX);
            let allocation = allocator.allocate_page(requirements, self.base_alloc_info.clone())?;
            self.pages.lock().insert(key, allocation);
        } else {
            self.pages.lock().remove(&key);
        }
        Ok(())
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        let key = SparsePageKey(page_index);
        if resident {
            let requirements = vk::VkMemoryRequirements::DEFAULT
                .with_size(self.page_size)
                .with_alignment(self.page_size)
                .with_memoryTypeBits(u32::MAX);
            let allocation = allocator.allocate_page(requirements, self.base_alloc_info.clone())?;
            self.pages.lock().insert(key, allocation);
        } else {
            self.pages.lock().remove(&key);
        }
        Ok(())
    }

    pub fn build_bind_list(&self) -> SparseBufferBindList {
        let mut binds = Vec::new();
        self.pages.lock().for_each(|page, allocation| {
            binds.push(
                vk::VkSparseMemoryBind::DEFAULT
                    .with_resourceOffset(page.0 * self.page_size)
                    .with_size(self.page_size)
                    .with_memory(allocation.memory())
                    .with_memoryOffset(allocation.offset()),
            );
        });
        SparseBufferBindList { binds }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let buffer_binds = self.build_bind_list().binds;
        let buffer_infos = vec![
            vk::VkSparseBufferMemoryBindInfo::DEFAULT
                .with_buffer(self.buffer.raw())
                .with_bindCount(buffer_binds.len() as u32)
                .with_pBinds(buffer_binds.as_ptr()),
        ];
        let info = vk::VkBindSparseInfo::DEFAULT
            .with_bufferBindCount(buffer_infos.len() as u32)
            .with_pBufferBinds(buffer_infos.as_ptr());
        PreparedBindSparseInfo {
            buffer_binds,
            buffer_infos,
            image_binds: Vec::new(),
            image_infos: Vec::new(),
            info,
        }
    }
}

pub struct SparseImageAllocation<'vk> {
    image: vk::Image<'vk>,
    page_size: u64,
    pages: Arc<Mutex<PageTable<SparsePageKey, Allocation>>>,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseImageAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk vk::Device<'vk>,
        _: &Allocator<'vk>,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        if image_info.flags & vk::VkImageCreateFlagBits::VK_IMAGE_CREATE_SPARSE_BINDING_BIT.0 == 0 {
            return Err(AllocatorError::SparseBindingUnsupported);
        }
        let image = device
            .vkCreateImage(image_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        let mut count = 0;
        image.vkGetImageSparseMemoryRequirements(&raw mut count, core::ptr::null_mut());
        let page_size = sparse_info.page_size.unwrap_or(64 * 1024);
        Ok(Self {
            image,
            page_size,
            pages: Arc::new(Mutex::new(PageTable::default())),
            base_alloc_info: AllocationCreateInfo {
                memory_type_policy: sparse_info.memory_type_policy,
                strategy: crate::resource::AllocationStrategy::Auto,
                pool: sparse_info.pool,
                dedicated_threshold: None,
                group_bind_mode: sparse_info.group_bind_mode,
            },
        })
    }

    pub(crate) fn new_group(
        device: &'vk vk::Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        if sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions)
            && sparse_info.split_instance_regions.is_empty()
        {
            return Err(AllocatorError::InvalidSparseRegion);
        }
        if image_info.flags & vk::VkImageCreateFlagBits::VK_IMAGE_CREATE_SPARSE_BINDING_BIT.0 == 0 {
            return Err(AllocatorError::SparseBindingUnsupported);
        }
        let image = device
            .vkCreateImage(image_info, vk::null())
            .map_err(AllocatorError::Vulkan)?;
        Ok(Self {
            image,
            page_size: sparse_info.page_size.unwrap_or(64 * 1024),
            pages: Arc::new(Mutex::new(PageTable::default())),
            base_alloc_info: AllocationCreateInfo {
                memory_type_policy: sparse_info.memory_type_policy,
                strategy: crate::resource::AllocationStrategy::Auto,
                pool: sparse_info.pool,
                dedicated_threshold: None,
                group_bind_mode: sparse_info.group_bind_mode,
            },
        })
    }

    pub fn image(&self) -> &vk::Image<'vk> {
        &self.image
    }

    pub fn update_page(
        &self,
        allocator: &Allocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        let key = SparsePageKey(page_index);
        if resident {
            let requirements = vk::VkMemoryRequirements::DEFAULT
                .with_size(self.page_size)
                .with_alignment(self.page_size)
                .with_memoryTypeBits(u32::MAX);
            let allocation = allocator.allocate_page(requirements, self.base_alloc_info.clone())?;
            self.pages.lock().insert(key, allocation);
        } else {
            self.pages.lock().remove(&key);
        }
        Ok(())
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        let key = SparsePageKey(page_index);
        if resident {
            let requirements = vk::VkMemoryRequirements::DEFAULT
                .with_size(self.page_size)
                .with_alignment(self.page_size)
                .with_memoryTypeBits(u32::MAX);
            let allocation = allocator.allocate_page(requirements, self.base_alloc_info.clone())?;
            self.pages.lock().insert(key, allocation);
        } else {
            self.pages.lock().remove(&key);
        }
        Ok(())
    }

    pub fn build_bind_list(&self) -> SparseImageBindList {
        let mut binds = Vec::new();
        self.pages.lock().for_each(|page, allocation| {
            binds.push(
                vk::VkSparseImageMemoryBind::DEFAULT
                    .with_offset(vk::VkOffset3D::DEFAULT.with_x((page.0 * self.page_size) as i32))
                    .with_extent(
                        vk::VkExtent3D::DEFAULT
                            .with_width(self.page_size as u32)
                            .with_height(1)
                            .with_depth(1),
                    )
                    .with_memory(allocation.memory())
                    .with_memoryOffset(allocation.offset()),
            );
        });
        SparseImageBindList { binds }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let image_binds = self.build_bind_list().binds;
        let image_infos = vec![
            vk::VkSparseImageMemoryBindInfo::DEFAULT
                .with_image(self.image.raw())
                .with_bindCount(image_binds.len() as u32)
                .with_pBinds(image_binds.as_ptr()),
        ];
        let info = vk::VkBindSparseInfo::DEFAULT
            .with_imageBindCount(image_infos.len() as u32)
            .with_pImageBinds(image_infos.as_ptr());
        PreparedBindSparseInfo {
            buffer_binds: Vec::new(),
            buffer_infos: Vec::new(),
            image_binds,
            image_infos,
            info,
        }
    }
}
