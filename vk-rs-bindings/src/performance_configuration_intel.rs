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
pub struct PerformanceConfigurationINTELDispatchTable {
    #[cfg(feature = "VK_INTEL_performance_query")]
    pub vkReleasePerformanceConfigurationINTEL: Option<PFN_vkReleasePerformanceConfigurationINTEL>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl PerformanceConfigurationINTELDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_INTEL_performance_query")]
        vkReleasePerformanceConfigurationINTEL: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_INTEL_performance_query")]
        {
            table.vkReleasePerformanceConfigurationINTEL =
                loader(c"vkReleasePerformanceConfigurationINTEL".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_INTEL_performance_query")]
pub struct PerformanceConfigurationINTEL<'dev> {
    pub(crate) raw: VkPerformanceConfigurationINTEL,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev PerformanceConfigurationINTELDispatchTable,
}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'dev> Send for PerformanceConfigurationINTEL<'dev> {}
#[cfg(feature = "VK_INTEL_performance_query")]
unsafe impl<'dev> Sync for PerformanceConfigurationINTEL<'dev> {}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'dev> Drop for PerformanceConfigurationINTEL<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
    }
}
#[cfg(feature = "VK_INTEL_performance_query")]
impl<'dev> PerformanceConfigurationINTEL<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkPerformanceConfigurationINTEL {
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
    pub const fn table(&self) -> &PerformanceConfigurationINTELDispatchTable {
        self.table
    }
    /// [`vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html)
    ///
    /// Provided by:
    /// - `VK_INTEL_performance_query`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `configuration`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_TOO_MANY_OBJECTS
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_INTEL_performance_query")]
    #[inline(always)]
    pub fn vkReleasePerformanceConfigurationINTEL(&self) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkReleasePerformanceConfigurationINTEL
                .unwrap_unchecked()(self.device().raw(), self.raw)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_TOO_MANY_OBJECTS
            | VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
    }
}
