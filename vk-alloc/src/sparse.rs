use crate::allocation::Allocation;
use crate::allocator::Allocator;
use crate::error::AllocatorError;
use crate::group_allocator::{GroupAllocator, GroupBindMode};
use crate::resource::{AllocationCreateInfo, SparseAllocationCreateInfo};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct PageTable<K, V> {
    pages: BTreeMap<K, V>,
}

impl<K, V> PageTable<K, V> {
    pub(crate) const fn new() -> Self {
        Self {
            pages: BTreeMap::new(),
        }
    }
}

impl<K, V> Default for PageTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> PageTable<K, V>
where
    K: Ord,
{
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.pages.insert(key, value)
    }

    pub(crate) fn remove(&mut self, key: &K) -> Option<V> {
        self.pages.remove(key)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn get(&self, key: &K) -> Option<&V> {
        self.pages.get(key)
    }

    pub(crate) fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        for (key, value) in &self.pages {
            f(key, value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SparseBufferBindList {
    binds: Box<[vk::VkSparseMemoryBind]>,
}

impl SparseBufferBindList {
    pub fn binds(&self) -> &[vk::VkSparseMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct SparseImageBindList {
    binds: Box<[vk::VkSparseImageMemoryBind]>,
}

impl SparseImageBindList {
    pub fn binds(&self) -> &[vk::VkSparseImageMemoryBind] {
        &self.binds
    }
}

#[derive(Debug, Clone)]
pub struct PreparedBindSparseInfo {
    buffer_binds: Box<[vk::VkSparseMemoryBind]>,
    buffer_infos: Box<[vk::VkSparseBufferMemoryBindInfo]>,
    image_binds: Box<[vk::VkSparseImageMemoryBind]>,
    image_infos: Box<[vk::VkSparseImageMemoryBindInfo]>,
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

type SparsePageTable = Arc<RwLock<PageTable<SparsePageKey, Allocation>>>;

struct SparseBase {
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

fn empty_box<T>() -> Box<[T]> {
    Vec::new().into_boxed_slice()
}

fn sparse_buffer_base<'vk>(
    device: &'vk vk::Device<'vk>,
    buffer_info: &vk::VkBufferCreateInfo,
    sparse_info: SparseAllocationCreateInfo,
    group_allocator: bool,
) -> Result<(vk::Buffer<'vk>, SparseBase), AllocatorError> {
    if group_allocator && sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions) {
        return Err(AllocatorError::GroupModeUnsupported);
    }
    if buffer_info.flags & vk::VkBufferCreateFlagBits::VK_BUFFER_CREATE_SPARSE_BINDING_BIT.0 == 0 {
        return Err(AllocatorError::SparseBindingUnsupported);
    }
    let buffer = device
        .vkCreateBuffer(buffer_info, vk::null())
        .map_err(AllocatorError::Vulkan)?;
    let mut requirements = vk::VkMemoryRequirements::DEFAULT;
    buffer.vkGetBufferMemoryRequirements(&raw mut requirements);
    Ok((
        buffer,
        SparseBase {
            page_size: sparse_info
                .page_size
                .unwrap_or(requirements.alignment.max(1)),
            pages: Arc::new(RwLock::new(PageTable::new())),
            base_alloc_info: sparse_info.into_allocation_info(),
        },
    ))
}

fn sparse_image_base<'vk>(
    device: &'vk vk::Device<'vk>,
    image_info: &vk::VkImageCreateInfo,
    sparse_info: SparseAllocationCreateInfo,
    group_allocator: bool,
) -> Result<(vk::Image<'vk>, SparseBase), AllocatorError> {
    if group_allocator
        && sparse_info.group_bind_mode == Some(GroupBindMode::SplitInstanceRegions)
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
    let page_size = if let Some(page_size) = sparse_info.page_size {
        page_size
    } else if !group_allocator {
        let mut count = 0;
        image.vkGetImageSparseMemoryRequirements(&raw mut count, core::ptr::null_mut());
        64 * 1024
    } else {
        64 * 1024
    };
    Ok((
        image,
        SparseBase {
            page_size,
            pages: Arc::new(RwLock::new(PageTable::default())),
            base_alloc_info: sparse_info.into_allocation_info(),
        },
    ))
}

pub struct SparseBufferAllocation<'vk> {
    buffer: vk::Buffer<'vk>,
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseBufferAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk vk::Device<'vk>,
        _allocator: &Allocator<'vk>,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (buffer, base) = sparse_buffer_base(device, buffer_info, sparse_info, false)?;
        Ok(Self {
            buffer,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub(crate) fn new_group(
        device: &'vk vk::Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        buffer_info: &vk::VkBufferCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (buffer, base) = sparse_buffer_base(device, buffer_info, sparse_info, true)?;
        Ok(Self {
            buffer,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
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
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn build_bind_list(&self) -> SparseBufferBindList {
        let mut binds = Vec::new();
        self.pages.read().for_each(|page, allocation| {
            binds.push(
                vk::VkSparseMemoryBind::DEFAULT
                    .with_resourceOffset(page.0 * self.page_size)
                    .with_size(self.page_size)
                    .with_memory(allocation.memory())
                    .with_memoryOffset(allocation.offset()),
            );
        });
        SparseBufferBindList {
            binds: binds.into_boxed_slice(),
        }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let buffer_binds = self.build_bind_list().binds;
        let buffer_infos = vec![
            vk::VkSparseBufferMemoryBindInfo::DEFAULT
                .with_buffer(self.buffer.raw())
                .with_bindCount(buffer_binds.len() as u32)
                .with_pBinds(buffer_binds.as_ptr()),
        ]
        .into_boxed_slice();
        let info = vk::VkBindSparseInfo::DEFAULT
            .with_bufferBindCount(buffer_infos.len() as u32)
            .with_pBufferBinds(buffer_infos.as_ptr());
        PreparedBindSparseInfo {
            buffer_binds,
            buffer_infos,
            image_binds: empty_box(),
            image_infos: empty_box(),
            info,
        }
    }
}

pub struct SparseImageAllocation<'vk> {
    image: vk::Image<'vk>,
    page_size: u64,
    pages: SparsePageTable,
    base_alloc_info: AllocationCreateInfo,
}

impl<'vk> SparseImageAllocation<'vk> {
    pub(crate) fn new(
        device: &'vk vk::Device<'vk>,
        _allocator: &Allocator<'vk>,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (image, base) = sparse_image_base(device, image_info, sparse_info, false)?;
        Ok(Self {
            image,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
        })
    }

    pub(crate) fn new_group(
        device: &'vk vk::Device<'vk>,
        _allocator: &GroupAllocator<'vk>,
        image_info: &vk::VkImageCreateInfo,
        sparse_info: SparseAllocationCreateInfo,
    ) -> Result<Self, AllocatorError> {
        let (image, base) = sparse_image_base(device, image_info, sparse_info, true)?;
        Ok(Self {
            image,
            page_size: base.page_size,
            pages: base.pages,
            base_alloc_info: base.base_alloc_info,
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
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn update_page_group(
        &self,
        allocator: &GroupAllocator<'vk>,
        page_index: u64,
        resident: bool,
    ) -> Result<(), AllocatorError> {
        update_sparse_page(
            &self.pages,
            self.page_size,
            self.base_alloc_info.clone(),
            page_index,
            resident,
            |requirements, alloc_info| allocator.allocate_page(requirements, alloc_info),
        )
    }

    pub fn build_bind_list(&self) -> SparseImageBindList {
        let mut binds = Vec::new();
        self.pages.read().for_each(|page, allocation| {
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
        SparseImageBindList {
            binds: binds.into_boxed_slice(),
        }
    }

    pub fn prepare_bind_info(&self) -> PreparedBindSparseInfo {
        let image_binds = self.build_bind_list().binds;
        let image_infos = vec![
            vk::VkSparseImageMemoryBindInfo::DEFAULT
                .with_image(self.image.raw())
                .with_bindCount(image_binds.len() as u32)
                .with_pBinds(image_binds.as_ptr()),
        ]
        .into_boxed_slice();
        let info = vk::VkBindSparseInfo::DEFAULT
            .with_imageBindCount(image_infos.len() as u32)
            .with_pImageBinds(image_infos.as_ptr());
        PreparedBindSparseInfo {
            buffer_binds: empty_box(),
            buffer_infos: empty_box(),
            image_binds,
            image_infos,
            info,
        }
    }
}

fn update_sparse_page(
    pages: &SparsePageTable,
    page_size: u64,
    base_alloc_info: AllocationCreateInfo,
    page_index: u64,
    resident: bool,
    allocate: impl Fn(
        vk::VkMemoryRequirements,
        AllocationCreateInfo,
    ) -> Result<Allocation, AllocatorError>,
) -> Result<(), AllocatorError> {
    let key = SparsePageKey(page_index);
    if resident {
        let requirements = vk::VkMemoryRequirements::DEFAULT
            .with_size(page_size)
            .with_alignment(page_size)
            .with_memoryTypeBits(u32::MAX);
        let allocation = allocate(requirements, base_alloc_info)?;
        pages.write().insert(key, allocation);
    } else {
        pages.write().remove(&key);
    }
    Ok(())
}
