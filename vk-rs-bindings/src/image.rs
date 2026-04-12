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
pub struct ImageDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkBindImageMemory: Option<PFN_vkBindImageMemory>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyImage: Option<PFN_vkDestroyImage>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetImageMemoryRequirements: Option<PFN_vkGetImageMemoryRequirements>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetImageSparseMemoryRequirements: Option<
        PFN_vkGetImageSparseMemoryRequirements,
    >,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetImageSubresourceLayout: Option<PFN_vkGetImageSubresourceLayout>,
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    pub vkGetImageSubresourceLayout2: Option<PFN_vkGetImageSubresourceLayout2>,
    #[cfg(
        any(
            feature = "VK_EXT_host_image_copy",
            feature = "VK_EXT_image_compression_control"
        )
    )]
    pub vkGetImageSubresourceLayout2EXT: Option<PFN_vkGetImageSubresourceLayout2EXT>,
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    pub vkGetImageDrmFormatModifierPropertiesEXT: Option<
        PFN_vkGetImageDrmFormatModifierPropertiesEXT,
    >,
    #[cfg(feature = "VK_KHR_maintenance5")]
    pub vkGetImageSubresourceLayout2KHR: Option<PFN_vkGetImageSubresourceLayout2KHR>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ImageDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkBindImageMemory: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyImage: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetImageMemoryRequirements: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetImageSparseMemoryRequirements: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetImageSubresourceLayout: None,
        #[cfg(feature = "VK_BASE_VERSION_1_4")]
        vkGetImageSubresourceLayout2: None,
        #[cfg(
            any(
                feature = "VK_EXT_host_image_copy",
                feature = "VK_EXT_image_compression_control"
            )
        )]
        vkGetImageSubresourceLayout2EXT: None,
        #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
        vkGetImageDrmFormatModifierPropertiesEXT: None,
        #[cfg(feature = "VK_KHR_maintenance5")]
        vkGetImageSubresourceLayout2KHR: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkBindImageMemory = loader(c"vkBindImageMemory".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroyImage = loader(c"vkDestroyImage".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetImageMemoryRequirements = loader(
                    c"vkGetImageMemoryRequirements".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetImageSparseMemoryRequirements = loader(
                    c"vkGetImageSparseMemoryRequirements".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetImageSubresourceLayout = loader(
                    c"vkGetImageSubresourceLayout".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_4")]
        {
            table.vkGetImageSubresourceLayout2 = loader(
                    c"vkGetImageSubresourceLayout2".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(
            any(
                feature = "VK_EXT_host_image_copy",
                feature = "VK_EXT_image_compression_control"
            )
        )]
        {
            table.vkGetImageSubresourceLayout2EXT = loader(
                    c"vkGetImageSubresourceLayout2EXT".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
        {
            table.vkGetImageDrmFormatModifierPropertiesEXT = loader(
                    c"vkGetImageDrmFormatModifierPropertiesEXT".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_maintenance5")]
        {
            table.vkGetImageSubresourceLayout2KHR = loader(
                    c"vkGetImageSubresourceLayout2KHR".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Image<'dev> {
    pub(crate) raw: VkImage,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ImageDispatchTable,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Drop for Image<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyImage {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'dev> Image<'dev> {
    #[inline]
    pub fn raw(&self) -> VkImage {
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
    pub fn table(&self) -> &ImageDispatchTable {
        self.table
    }
    /// [`vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `memory`
    /// - `memoryOffset`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkBindImageMemory(
        &self,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkBindImageMemory
                .unwrap_unchecked()(self.device().raw(), self.raw, memory, memoryOffset)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
    /// [`vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `image`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyImage(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyImage
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkImage::NULL;
    }
    /// [`vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pMemoryRequirements`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetImageMemoryRequirements(
        &self,
        pMemoryRequirements: *mut VkMemoryRequirements,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageMemoryRequirements
                .unwrap_unchecked()(self.device().raw(), self.raw, pMemoryRequirements)
        }
    }
    /// [`vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pSparseMemoryRequirementCount`: optional: pointer required, values optional if pointer not null
    /// - `pSparseMemoryRequirements`: optional: true, len: pSparseMemoryRequirementCount
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetImageSparseMemoryRequirements(
        &self,
        pSparseMemoryRequirementCount: *mut u32,
        pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageSparseMemoryRequirements
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pSparseMemoryRequirementCount,
                pSparseMemoryRequirements,
            )
        }
    }
    /// [`vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pSubresource`
    /// - `pLayout`
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetImageSubresourceLayout(
        &self,
        pSubresource: *const VkImageSubresource,
        pLayout: *mut VkSubresourceLayout,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageSubresourceLayout
                .unwrap_unchecked()(self.device().raw(), self.raw, pSubresource, pLayout)
        }
    }
    /// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_4`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pSubresource`
    /// - `pLayout`
    #[cfg(feature = "VK_BASE_VERSION_1_4")]
    #[inline(always)]
    pub fn vkGetImageSubresourceLayout2(
        &self,
        pSubresource: *const VkImageSubresource2,
        pLayout: *mut VkSubresourceLayout2,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageSubresourceLayout2
                .unwrap_unchecked()(self.device().raw(), self.raw, pSubresource, pLayout)
        }
    }
    /// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)
    ///
    /// Provided by:
    /// - `VK_EXT_host_image_copy`
    /// - `VK_EXT_image_compression_control`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pSubresource`
    /// - `pLayout`
    #[cfg(
        any(
            feature = "VK_EXT_host_image_copy",
            feature = "VK_EXT_image_compression_control"
        )
    )]
    #[inline(always)]
    pub fn vkGetImageSubresourceLayout2EXT(
        &self,
        pSubresource: *const VkImageSubresource2,
        pLayout: *mut VkSubresourceLayout2,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageSubresourceLayout2EXT
                .unwrap_unchecked()(self.device().raw(), self.raw, pSubresource, pLayout)
        }
    }
    /// [`vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_image_drm_format_modifier`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
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
    #[cfg(feature = "VK_EXT_image_drm_format_modifier")]
    #[inline(always)]
    pub fn vkGetImageDrmFormatModifierPropertiesEXT(
        &self,
        pProperties: *mut VkImageDrmFormatModifierPropertiesEXT,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetImageDrmFormatModifierPropertiesEXT
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
    /// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html)
    ///
    /// Provided by:
    /// - `VK_KHR_maintenance5`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `image`
    /// - `pSubresource`
    /// - `pLayout`
    #[cfg(feature = "VK_KHR_maintenance5")]
    #[inline(always)]
    pub fn vkGetImageSubresourceLayout2KHR(
        &self,
        pSubresource: *const VkImageSubresource2,
        pLayout: *mut VkSubresourceLayout2,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetImageSubresourceLayout2KHR
                .unwrap_unchecked()(self.device().raw(), self.raw, pSubresource, pLayout)
        }
    }
}
