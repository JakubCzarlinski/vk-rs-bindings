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
pub struct CuModuleNVXDispatchTable {
    #[cfg(feature = "VK_NVX_binary_import")]
    pub vkDestroyCuModuleNVX: Option<PFN_vkDestroyCuModuleNVX>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl CuModuleNVXDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NVX_binary_import")]
        vkDestroyCuModuleNVX: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_NVX_binary_import")]
        {
            table.vkDestroyCuModuleNVX = loader(c"vkDestroyCuModuleNVX".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
pub struct CuModuleNVX<'dev> {
    pub(crate) raw: VkCuModuleNVX,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev CuModuleNVXDispatchTable,
}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'dev> Drop for CuModuleNVX<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyCuModuleNVX {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'dev> CuModuleNVX<'dev> {
    #[inline]
    pub fn raw(&self) -> VkCuModuleNVX {
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
    pub fn table(&self) -> &CuModuleNVXDispatchTable {
        self.table
    }
    /// [`vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html)
    ///
    /// Provided by:
    /// - `VK_NVX_binary_import`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `module`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NVX_binary_import")]
    #[inline(always)]
    pub fn vkDestroyCuModuleNVX(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyCuModuleNVX.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkCuModuleNVX::NULL;
    }
}
