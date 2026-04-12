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
pub struct FramebufferDispatchTable {
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    pub vkDestroyFramebuffer: Option<PFN_vkDestroyFramebuffer>,
    #[cfg(feature = "VK_QCOM_tile_properties")]
    pub vkGetFramebufferTilePropertiesQCOM: Option<
        PFN_vkGetFramebufferTilePropertiesQCOM,
    >,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl FramebufferDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        vkDestroyFramebuffer: None,
        #[cfg(feature = "VK_QCOM_tile_properties")]
        vkGetFramebufferTilePropertiesQCOM: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        {
            table.vkDestroyFramebuffer = loader(c"vkDestroyFramebuffer".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_QCOM_tile_properties")]
        {
            table.vkGetFramebufferTilePropertiesQCOM = loader(
                    c"vkGetFramebufferTilePropertiesQCOM".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub struct Framebuffer<'dev> {
    pub(crate) raw: VkFramebuffer,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev FramebufferDispatchTable,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> Drop for Framebuffer<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyFramebuffer {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> Framebuffer<'dev> {
    #[inline]
    pub fn raw(&self) -> VkFramebuffer {
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
    pub fn table(&self) -> &FramebufferDispatchTable {
        self.table
    }
    /// [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html)
    ///
    /// Provided by:
    /// - `VK_GRAPHICS_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `framebuffer`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyFramebuffer(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyFramebuffer
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkFramebuffer::NULL;
    }
    /// [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html)
    ///
    /// Provided by:
    /// - `VK_QCOM_tile_properties`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `framebuffer`
    /// - `pPropertiesCount`: optional: pointer required, values optional if pointer not null
    /// - `pProperties`: optional: true, len: pPropertiesCount
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///   - VK_INCOMPLETE
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_QCOM_tile_properties")]
    #[inline(always)]
    pub fn vkGetFramebufferTilePropertiesQCOM(
        &self,
        pPropertiesCount: *mut u32,
        pProperties: *mut VkTilePropertiesQCOM,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetFramebufferTilePropertiesQCOM
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pPropertiesCount,
                pProperties,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => if r >= VkResult::VK_SUCCESS { Ok(r) } else { Err(r) }
        }
    }
}
