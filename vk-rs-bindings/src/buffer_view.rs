#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use crate::commands::*;
use crate::enums::*;
use crate::types::*;
use core::ffi::{c_char, c_void};
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct BufferViewDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyBufferView: Option<PFN_vkDestroyBufferView>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl BufferViewDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyBufferView: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkDestroyBufferView =
                loader(c"vkDestroyBufferView".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct BufferView<'dev> {
    pub(crate) raw: VkBufferView,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev BufferViewDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for BufferView<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyBufferView {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> BufferView<'dev> {
    #[inline]
    pub fn raw(&self) -> VkBufferView {
        self.raw
    }
    #[inline]
    pub fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub fn table(&self) -> &BufferViewDispatchTable {
        self.table
    }
    /// [`vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `bufferView`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyBufferView(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyBufferView.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkBufferView::NULL;
    }
}
