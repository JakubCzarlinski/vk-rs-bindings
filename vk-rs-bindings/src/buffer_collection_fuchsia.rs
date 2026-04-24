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
pub struct BufferCollectionFUCHSIADispatchTable {
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub vkDestroyBufferCollectionFUCHSIA: Option<PFN_vkDestroyBufferCollectionFUCHSIA>,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub vkGetBufferCollectionPropertiesFUCHSIA: Option<PFN_vkGetBufferCollectionPropertiesFUCHSIA>,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub vkSetBufferCollectionBufferConstraintsFUCHSIA:
        Option<PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA>,
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    pub vkSetBufferCollectionImageConstraintsFUCHSIA:
        Option<PFN_vkSetBufferCollectionImageConstraintsFUCHSIA>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl BufferCollectionFUCHSIADispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        vkDestroyBufferCollectionFUCHSIA: None,
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        vkGetBufferCollectionPropertiesFUCHSIA: None,
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        vkSetBufferCollectionBufferConstraintsFUCHSIA: None,
        #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
        vkSetBufferCollectionImageConstraintsFUCHSIA: None,
    };
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self {
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            vkDestroyBufferCollectionFUCHSIA: loader(c"vkDestroyBufferCollectionFUCHSIA".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            vkGetBufferCollectionPropertiesFUCHSIA: loader(
                c"vkGetBufferCollectionPropertiesFUCHSIA".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            vkSetBufferCollectionBufferConstraintsFUCHSIA: loader(
                c"vkSetBufferCollectionBufferConstraintsFUCHSIA".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
            #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
            vkSetBufferCollectionImageConstraintsFUCHSIA: loader(
                c"vkSetBufferCollectionImageConstraintsFUCHSIA".as_ptr(),
            )
            .map(|f| unsafe { core::mem::transmute(f) }),
        }
    }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
pub struct BufferCollectionFUCHSIA<'dev> {
    pub(crate) raw: VkBufferCollectionFUCHSIA,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev BufferCollectionFUCHSIADispatchTable,
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'dev> Send for BufferCollectionFUCHSIA<'dev> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
unsafe impl<'dev> Sync for BufferCollectionFUCHSIA<'dev> {}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'dev> Drop for BufferCollectionFUCHSIA<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyBufferCollectionFUCHSIA {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_FUCHSIA_buffer_collection")]
impl<'dev> BufferCollectionFUCHSIA<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkBufferCollectionFUCHSIA {
        self.raw
    }
    #[inline]
    pub const fn parent(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn device(&self) -> &'dev crate::device::Device<'dev> {
        self.parent
    }
    #[inline]
    pub const fn instance(&self) -> &'dev crate::instance::Instance<'dev> {
        self.parent.instance()
    }
    #[inline]
    pub const fn table(&self) -> &BufferCollectionFUCHSIADispatchTable {
        self.table
    }
    /// [`vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html)
    ///
    /// Provided by:
    /// - `VK_FUCHSIA_buffer_collection`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `collection`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    #[inline(always)]
    pub fn vkDestroyBufferCollectionFUCHSIA(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyBufferCollectionFUCHSIA
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkBufferCollectionFUCHSIA::NULL;
    }
    /// [`vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html)
    ///
    /// Provided by:
    /// - `VK_FUCHSIA_buffer_collection`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `collection`
    /// - `pProperties`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_INITIALIZATION_FAILED`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    #[inline(always)]
    pub fn vkGetBufferCollectionPropertiesFUCHSIA(
        &self,
        pProperties: *mut VkBufferCollectionPropertiesFUCHSIA,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetBufferCollectionPropertiesFUCHSIA
                .unwrap_unchecked()(self.device().raw(), self.raw, pProperties)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html)
    ///
    /// Provided by:
    /// - `VK_FUCHSIA_buffer_collection`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `collection`
    /// - `pBufferConstraintsInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_INITIALIZATION_FAILED`
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    #[inline(always)]
    pub fn vkSetBufferCollectionBufferConstraintsFUCHSIA(
        &self,
        pBufferConstraintsInfo: *const VkBufferConstraintsInfoFUCHSIA,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkSetBufferCollectionBufferConstraintsFUCHSIA
                .unwrap_unchecked()(
                self.device().raw(), self.raw, pBufferConstraintsInfo
            )
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
    /// [`vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html)
    ///
    /// Provided by:
    /// - `VK_FUCHSIA_buffer_collection`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `collection`
    /// - `pImageConstraintsInfo`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - `VK_SUCCESS`
    ///
    /// **Error Codes:**
    ///   - `VK_ERROR_INITIALIZATION_FAILED`
    ///   - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///   - `VK_ERROR_FORMAT_NOT_SUPPORTED`
    ///   - `VK_ERROR_UNKNOWN`
    ///   - `VK_ERROR_VALIDATION_FAILED`
    #[cfg(feature = "VK_FUCHSIA_buffer_collection")]
    #[inline(always)]
    pub fn vkSetBufferCollectionImageConstraintsFUCHSIA(
        &self,
        pImageConstraintsInfo: *const VkImageConstraintsInfoFUCHSIA,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkSetBufferCollectionImageConstraintsFUCHSIA
                .unwrap_unchecked()(self.device().raw(), self.raw, pImageConstraintsInfo)
        };
        if r >= VkResult::VK_SUCCESS {
            Ok(r)
        } else {
            Err(r)
        }
    }
}
