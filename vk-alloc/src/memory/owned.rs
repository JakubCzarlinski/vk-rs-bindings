use crate::error::AllocatorError;
use alloc::sync::Arc;
use core::ffi::c_void;
use core::ptr::{null, null_mut};
use core::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
pub(crate) struct DeviceFns {
    pub(crate) device: vk::VkDevice,
    pub(crate) free_memory: vk::PFN_vkFreeMemory,
    pub(crate) map_memory: vk::PFN_vkMapMemory,
    pub(crate) unmap_memory: vk::PFN_vkUnmapMemory,
}

unsafe impl Send for DeviceFns {}
unsafe impl Sync for DeviceFns {}

#[derive(Debug)]
pub(crate) struct OwnedMemory {
    raw: vk::VkDeviceMemory,
    size: u64,
    mapped: AtomicPtr<u8>,
    host_visible: bool,
    device_fns: Arc<DeviceFns>,
}

unsafe impl Send for OwnedMemory {}
unsafe impl Sync for OwnedMemory {}

impl OwnedMemory {
    pub(crate) fn new(
        raw: vk::VkDeviceMemory,
        size: u64,
        host_visible: bool,
        device_fns: Arc<DeviceFns>,
    ) -> Self {
        Self {
            raw,
            size,
            mapped: AtomicPtr::new(null_mut()),
            host_visible,
            device_fns,
        }
    }

    pub(crate) const fn raw(&self) -> vk::VkDeviceMemory {
        self.raw
    }

    pub(crate) const fn size(&self) -> u64 {
        self.size
    }

    pub(crate) fn mapped_ptr(&self) -> *mut u8 {
        if !self.host_visible {
            return null_mut();
        }
        let cached = self.mapped.load(Ordering::Acquire);
        if !cached.is_null() {
            return cached;
        }
        let mut out: *mut c_void = null_mut();
        let result = unsafe {
            (self.device_fns.map_memory)(
                self.device_fns.device,
                self.raw,
                0,
                self.size,
                0,
                &raw mut out,
            )
        };
        if result >= vk::VkResult::VK_SUCCESS {
            let out = out.cast::<u8>();
            self.mapped.store(out, Ordering::Release);
            out
        } else {
            null_mut()
        }
    }
}

impl Drop for OwnedMemory {
    fn drop(&mut self) {
        let mapped = self.mapped.swap(null_mut(), Ordering::AcqRel);
        if !mapped.is_null() {
            unsafe {
                (self.device_fns.unmap_memory)(self.device_fns.device, self.raw);
            }
        }
        unsafe {
            (self.device_fns.free_memory)(self.device_fns.device, self.raw, null());
        }
    }
}

pub(crate) fn allocate_owned_memory(
    device: &vk::Device<'_>,
    requirements: &vk::VkMemoryRequirements,
    memory_type_index: u32,
    dedicated_buffer: Option<vk::VkBuffer>,
    dedicated_image: Option<vk::VkImage>,
    device_mask: Option<u32>,
    host_visible: bool,
) -> Result<OwnedMemory, AllocatorError> {
    let mut dedicated_info = vk::VkMemoryDedicatedAllocateInfo::DEFAULT;
    let mut flags_info = vk::VkMemoryAllocateFlagsInfo::DEFAULT;
    let mut allocate_info = vk::VkMemoryAllocateInfo::DEFAULT
        .with_allocationSize(requirements.size)
        .with_memoryTypeIndex(memory_type_index);

    let mut next = null::<c_void>();
    if dedicated_buffer.is_some() || dedicated_image.is_some() {
        dedicated_info = dedicated_info
            .with_buffer(dedicated_buffer.unwrap_or(vk::VkBuffer::NULL))
            .with_image(dedicated_image.unwrap_or(vk::VkImage::NULL));
        next = (&raw const dedicated_info).cast::<c_void>();
    }
    if let Some(mask) = device_mask {
        flags_info = flags_info
            .with_flags(vk::VkMemoryAllocateFlagBits::VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT.0)
            .with_deviceMask(mask)
            .with_pNext(next);
        next = (&raw const flags_info).cast::<c_void>();
    }
    if !next.is_null() {
        allocate_info = allocate_info.with_pNext(next);
    }

    let memory = device
        .vkAllocateMemory(&allocate_info, null())
        .map_err(AllocatorError::Vulkan)?;
    let free_memory = memory.table().vkFreeMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let map_memory = memory.table().vkMapMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let unmap_memory = memory.table().vkUnmapMemory.ok_or(AllocatorError::Vulkan(
        vk::VkResult::VK_ERROR_INITIALIZATION_FAILED,
    ))?;
    let device_fns = Arc::new(DeviceFns {
        device: memory.parent().raw(),
        free_memory,
        map_memory,
        unmap_memory,
    });
    let raw = memory.raw();
    core::mem::forget(memory);
    Ok(OwnedMemory::new(
        raw,
        requirements.size,
        host_visible,
        device_fns,
    ))
}
