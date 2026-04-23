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
pub struct ShaderInstrumentationARMDispatchTable {
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    pub vkClearShaderInstrumentationMetricsARM: Option<PFN_vkClearShaderInstrumentationMetricsARM>,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    pub vkDestroyShaderInstrumentationARM: Option<PFN_vkDestroyShaderInstrumentationARM>,
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    pub vkGetShaderInstrumentationValuesARM: Option<PFN_vkGetShaderInstrumentationValuesARM>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ShaderInstrumentationARMDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        vkClearShaderInstrumentationMetricsARM: None,
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        vkDestroyShaderInstrumentationARM: None,
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        vkGetShaderInstrumentationValuesARM: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        {
            table.vkClearShaderInstrumentationMetricsARM =
                loader(c"vkClearShaderInstrumentationMetricsARM".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        {
            table.vkDestroyShaderInstrumentationARM =
                loader(c"vkDestroyShaderInstrumentationARM".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_ARM_shader_instrumentation")]
        {
            table.vkGetShaderInstrumentationValuesARM =
                loader(c"vkGetShaderInstrumentationValuesARM".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
pub struct ShaderInstrumentationARM<'dev> {
    pub(crate) raw: VkShaderInstrumentationARM,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ShaderInstrumentationARMDispatchTable,
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'dev> Send for ShaderInstrumentationARM<'dev> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
unsafe impl<'dev> Sync for ShaderInstrumentationARM<'dev> {}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'dev> Drop for ShaderInstrumentationARM<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyShaderInstrumentationARM {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_ARM_shader_instrumentation")]
impl<'dev> ShaderInstrumentationARM<'dev> {
    #[inline]
    pub const fn raw(&self) -> VkShaderInstrumentationARM {
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
    pub const fn table(&self) -> &ShaderInstrumentationARMDispatchTable {
        self.table
    }
    /// [`vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_shader_instrumentation`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `instrumentation`
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    #[inline(always)]
    pub fn vkClearShaderInstrumentationMetricsARM(&self) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkClearShaderInstrumentationMetricsARM
                .unwrap_unchecked()(self.device().raw(), self.raw)
        }
    }
    /// [`vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_shader_instrumentation`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `instrumentation`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    #[inline(always)]
    pub fn vkDestroyShaderInstrumentationARM(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyShaderInstrumentationARM
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkShaderInstrumentationARM::NULL;
    }
    /// [`vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html)
    ///
    /// Provided by:
    /// - `VK_ARM_shader_instrumentation`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `instrumentation`
    /// - `pMetricBlockCount`
    /// - `pMetricValues`
    /// - `flags`: optional: true
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///   - VK_INCOMPLETE
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_ARM_shader_instrumentation")]
    #[inline(always)]
    pub fn vkGetShaderInstrumentationValuesARM(
        &self,
        pMetricBlockCount: *mut u32,
        pMetricValues: *mut core::ffi::c_void,
        flags: VkShaderInstrumentationValuesFlagsARM,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetShaderInstrumentationValuesARM
                .unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pMetricBlockCount,
                pMetricValues,
                flags,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
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
