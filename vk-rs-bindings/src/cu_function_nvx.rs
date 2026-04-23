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
pub struct CuFunctionNVXDispatchTable {
    #[cfg(feature = "VK_NVX_binary_import")]
    pub vkDestroyCuFunctionNVX: Option<PFN_vkDestroyCuFunctionNVX>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl CuFunctionNVXDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_NVX_binary_import")]
        vkDestroyCuFunctionNVX: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_NVX_binary_import")]
        {
            table.vkDestroyCuFunctionNVX = loader(c"vkDestroyCuFunctionNVX".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
pub struct CuFunctionNVX<'dev> {
    pub(crate) raw: VkCuFunctionNVX,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev CuFunctionNVXDispatchTable,
}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'dev> Send for CuFunctionNVX<'dev> {}
#[cfg(feature = "VK_NVX_binary_import")]
unsafe impl<'dev> Sync for CuFunctionNVX<'dev> {}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'dev> Drop for CuFunctionNVX<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyCuFunctionNVX {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_NVX_binary_import")]
impl<'dev> CuFunctionNVX<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkCuFunctionNVX {
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
    pub const fn table(&self) -> &CuFunctionNVXDispatchTable {
        self.table
    }
    /// [`vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html)
    ///
    /// Provided by:
    /// - `VK_NVX_binary_import`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `function`
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_NVX_binary_import")]
    #[inline(always)]
    pub fn vkDestroyCuFunctionNVX(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyCuFunctionNVX.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkCuFunctionNVX::NULL;
    }
}
