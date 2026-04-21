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
pub struct MicromapEXTDispatchTable {
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    pub vkDestroyMicromapEXT: Option<PFN_vkDestroyMicromapEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl MicromapEXTDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_EXT_opacity_micromap")]
        vkDestroyMicromapEXT: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_EXT_opacity_micromap")]
        {
            table.vkDestroyMicromapEXT = loader(c"vkDestroyMicromapEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
pub struct MicromapEXT<'dev> {
    pub(crate) raw: VkMicromapEXT,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev MicromapEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
unsafe impl<'dev> Send for MicromapEXT<'dev> {}
#[cfg(feature = "VK_EXT_opacity_micromap")]
unsafe impl<'dev> Sync for MicromapEXT<'dev> {}
#[cfg(feature = "VK_EXT_opacity_micromap")]
impl<'dev> Drop for MicromapEXT<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyMicromapEXT {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_EXT_opacity_micromap")]
impl<'dev> MicromapEXT<'dev> {
    #[inline]
    pub fn raw(&self) -> VkMicromapEXT {
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
    pub fn table(&self) -> &MicromapEXTDispatchTable {
        self.table
    }
    /// [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_opacity_micromap`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `micromap`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_EXT_opacity_micromap")]
    #[inline(always)]
    pub fn vkDestroyMicromapEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyMicromapEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkMicromapEXT::NULL;
    }
}
