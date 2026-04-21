use crate::error::AllocatorError;
use crate::memory::block::SharedSource;
use crate::stats::StatsState;
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::ptr::null_mut;
use vk::{Buffer, DeviceMemory, Image};

pub struct Allocation {
    block_handle: u32,
    offset: u64,
    size: u64,
    mapped_ptr: *mut u8,
    arena_id: u32,
    source: SharedSource,
    stats: Option<Arc<StatsState>>,
}

unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

impl Allocation {
    pub(crate) const fn new(
        block_handle: u32,
        arena_id: u32,
        offset: u64,
        size: u64,
        mapped_ptr: *mut u8,
        source: SharedSource,
        stats: Option<Arc<StatsState>>,
    ) -> Self {
        Self {
            block_handle,
            offset,
            size,
            mapped_ptr,
            arena_id,
            source,
            stats,
        }
    }

    pub const fn block_handle(&self) -> u32 {
        self.block_handle
    }

    pub const fn offset(&self) -> u64 {
        self.offset
    }

    pub const fn size(&self) -> u64 {
        self.size
    }

    pub const fn arena_id(&self) -> u32 {
        self.arena_id
    }

    pub fn memory(&self) -> vk::VkDeviceMemory {
        self.source.raw_memory()
    }

    pub const fn mapped_ptr(&self) -> *mut u8 {
        self.mapped_ptr
    }

    pub const fn mapped_slice_mut<T>(&mut self, len: usize) -> Option<&mut [T]> {
        if self.mapped_ptr.is_null() {
            None
        } else {
            Some(unsafe { core::slice::from_raw_parts_mut(self.mapped_ptr.cast::<T>(), len) })
        }
    }
}

impl Drop for Allocation {
    fn drop(&mut self) {
        self.source.free_range(self.offset, self.size);
        if let Some(stats) = &self.stats {
            stats.on_free();
        }
        self.mapped_ptr = null_mut();
    }
}

pub struct AllocatedBuffer<'vk> {
    buffer: Buffer<'vk>,
    allocation: Allocation,
}

impl<'vk> AllocatedBuffer<'vk> {
    pub(crate) const fn new(buffer: Buffer<'vk>, allocation: Allocation) -> Self {
        Self { buffer, allocation }
    }

    pub const fn buffer(&self) -> &Buffer<'vk> {
        &self.buffer
    }

    pub const fn allocation(&self) -> &Allocation {
        &self.allocation
    }

    pub fn allocation_mut(&mut self) -> &mut Allocation {
        &mut self.allocation
    }
}

pub struct AllocatedImage<'vk> {
    image: Image<'vk>,
    allocation: Allocation,
}

impl<'vk> AllocatedImage<'vk> {
    pub(crate) fn new(image: Image<'vk>, allocation: Allocation) -> Self {
        Self { image, allocation }
    }

    pub fn image(&self) -> &Image<'vk> {
        &self.image
    }

    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HostImportBufferCreateInfo {
    pub host_ptr: *mut u8,
    pub size: u64,
    pub handle_type: vk::VkExternalMemoryHandleTypeFlagBits,
}

impl HostImportBufferCreateInfo {
    pub const DEFAULT: Self = Self {
        host_ptr: null_mut(),
        size: 0,
        handle_type: vk::VkExternalMemoryHandleTypeFlagBits::VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT,
    };

    pub const fn new(host_ptr: *mut u8, size: u64) -> Self {
        Self {
            host_ptr,
            size,
            ..Self::DEFAULT
        }
    }

    #[must_use]
    pub const fn with_handle_type(
        mut self,
        handle_type: vk::VkExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.handle_type = handle_type;
        self
    }
}

pub struct ImportedHostBuffer<'vk> {
    buffer: Buffer<'vk>,
    memory: DeviceMemory<'vk>,
    host_ptr: *mut u8,
    size: u64,
}

unsafe impl Send for ImportedHostBuffer<'_> {}
unsafe impl Sync for ImportedHostBuffer<'_> {}

impl<'vk> ImportedHostBuffer<'vk> {
    pub(crate) const fn new(
        buffer: Buffer<'vk>,
        memory: DeviceMemory<'vk>,
        host_ptr: *mut u8,
        size: u64,
    ) -> Self {
        Self {
            buffer,
            memory,
            host_ptr,
            size,
        }
    }

    pub const fn buffer(&self) -> &Buffer<'vk> {
        &self.buffer
    }

    pub const fn memory(&self) -> &DeviceMemory<'vk> {
        &self.memory
    }

    pub const fn host_ptr(&self) -> *mut u8 {
        self.host_ptr
    }

    pub const fn size(&self) -> u64 {
        self.size
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LargeBufferCreateInfo {
    pub preferred_chunk_size: Option<u64>,
}

impl Default for LargeBufferCreateInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl LargeBufferCreateInfo {
    pub const DEFAULT: Self = Self {
        preferred_chunk_size: None,
    };

    pub const fn new() -> Self {
        Self::DEFAULT
    }

    #[must_use]
    pub const fn with_preferred_chunk_size(mut self, preferred_chunk_size: u64) -> Self {
        self.preferred_chunk_size = Some(preferred_chunk_size);
        self
    }
}

pub struct LargeBuffer<'vk> {
    total_size: u64,
    chunk_size: u64,
    segments: Box<[AllocatedBuffer<'vk>]>,
}

impl<'vk> LargeBuffer<'vk> {
    pub(crate) fn new(
        total_size: u64,
        chunk_size: u64,
        segments: Vec<AllocatedBuffer<'vk>>,
    ) -> Self {
        Self {
            total_size,
            chunk_size,
            segments: segments.into_boxed_slice(),
        }
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    pub fn chunk_size(&self) -> u64 {
        self.chunk_size
    }

    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    pub fn segments(&self) -> &[AllocatedBuffer<'vk>] {
        &self.segments
    }

    pub fn segments_mut(&mut self) -> &mut [AllocatedBuffer<'vk>] {
        &mut self.segments
    }

    pub fn segment_for_offset(&self, offset: u64) -> Option<LargeBufferSegment<'_, 'vk>> {
        if offset >= self.total_size {
            return None;
        }
        let index = (offset / self.chunk_size) as usize;
        let global_offset = (index as u64) * self.chunk_size;
        let local_offset = offset - global_offset;
        let segment = self.segments.get(index)?;
        Some(LargeBufferSegment {
            index,
            global_offset,
            local_offset,
            size: segment.allocation().size(),
            segment,
        })
    }

    pub fn write_bytes(&mut self, offset: u64, bytes: &[u8]) -> Result<(), AllocatorError> {
        if offset + bytes.len() as u64 > self.total_size {
            return Err(AllocatorError::OutOfBounds);
        }
        let mut remaining = bytes;
        let mut cursor = offset;
        while !remaining.is_empty() {
            let index = (cursor / self.chunk_size) as usize;
            let global_offset = (index as u64) * self.chunk_size;
            let local_offset = (cursor - global_offset) as usize;
            let segment = self
                .segments
                .get_mut(index)
                .ok_or(AllocatorError::OutOfBounds)?;
            let ptr = segment.allocation().mapped_ptr();
            if ptr.is_null() {
                return Err(AllocatorError::HostVisibleRequired);
            }
            let writable =
                (segment.allocation().size() as usize - local_offset).min(remaining.len());
            unsafe {
                core::ptr::copy_nonoverlapping(remaining.as_ptr(), ptr.add(local_offset), writable);
            }
            remaining = &remaining[writable..];
            cursor += writable as u64;
        }
        Ok(())
    }
}

pub struct LargeBufferSegment<'a, 'vk> {
    pub index: usize,
    pub global_offset: u64,
    pub local_offset: u64,
    pub size: u64,
    pub segment: &'a AllocatedBuffer<'vk>,
}
