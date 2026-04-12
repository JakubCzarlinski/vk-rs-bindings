#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use core::ffi::{c_char, c_void};
use crate::commands::*;
use crate::types::*;
use crate::enums::*;
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct ImageViewDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyImageView: Option<PFN_vkDestroyImageView>,
    #[cfg(feature = "VK_NVX_image_view_handle")]
    pub vkGetImageViewAddressNVX: Option<PFN_vkGetImageViewAddressNVX>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ImageViewDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyImageView: None,
        #[cfg(feature = "VK_NVX_image_view_handle")]
        vkGetImageViewAddressNVX: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroyImageView = loader(c"vkDestroyImageView".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NVX_image_view_handle")]
        {
            table.vkGetImageViewAddressNVX = loader(c"vkGetImageViewAddressNVX".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct ImageView<'dev> {
    pub(crate) raw: VkImageView,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ImageViewDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for ImageView<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyImageView {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> ImageView<'dev> {
    #[inline]
    pub fn raw(&self) -> VkImageView {
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
    pub fn table(&self) -> &ImageViewDispatchTable {
        self.table
    }
    /// [`vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `imageView`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyImageView(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyImageView
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkImageView::NULL;
    }
    /// [`vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html)
    ///
    /// Provided by:
    /// - `VK_NVX_image_view_handle`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `imageView`
    /// - `pProperties`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_NVX_image_view_handle")]
    #[inline(always)]
    pub fn vkGetImageViewAddressNVX(
        &self,
        pProperties: *mut VkImageViewAddressPropertiesNVX,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetImageViewAddressNVX
                .unwrap_unchecked()(self.device().raw(), self.raw, pProperties)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
