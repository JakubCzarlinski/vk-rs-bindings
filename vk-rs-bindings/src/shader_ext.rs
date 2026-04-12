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
pub struct ShaderEXTDispatchTable {
    #[cfg(feature = "VK_EXT_shader_object")]
    pub vkDestroyShaderEXT: Option<PFN_vkDestroyShaderEXT>,
    #[cfg(feature = "VK_EXT_shader_object")]
    pub vkGetShaderBinaryDataEXT: Option<PFN_vkGetShaderBinaryDataEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ShaderEXTDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_EXT_shader_object")]
        vkDestroyShaderEXT: None,
        #[cfg(feature = "VK_EXT_shader_object")]
        vkGetShaderBinaryDataEXT: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_EXT_shader_object")]
        {
            table.vkDestroyShaderEXT =
                loader(c"vkDestroyShaderEXT".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_shader_object")]
        {
            table.vkGetShaderBinaryDataEXT = loader(c"vkGetShaderBinaryDataEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_EXT_shader_object")]
pub struct ShaderEXT<'dev> {
    pub(crate) raw: VkShaderEXT,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ShaderEXTDispatchTable,
}
#[cfg(feature = "VK_EXT_shader_object")]
impl<'dev> Drop for ShaderEXT<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyShaderEXT {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_EXT_shader_object")]
impl<'dev> ShaderEXT<'dev> {
    #[inline]
    pub fn raw(&self) -> VkShaderEXT {
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
    pub fn table(&self) -> &ShaderEXTDispatchTable {
        self.table
    }
    /// [`vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_shader_object`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `shader`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_EXT_shader_object")]
    #[inline(always)]
    pub fn vkDestroyShaderEXT(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyShaderEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkShaderEXT::NULL;
    }
    /// [`vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_shader_object`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `shader`
    /// - `pDataSize`: optional: pointer required, values optional if pointer not null
    /// - `pData`: optional: true, len: pDataSize
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
    #[cfg(feature = "VK_EXT_shader_object")]
    #[inline(always)]
    pub fn vkGetShaderBinaryDataEXT(
        &self,
        pDataSize: *mut usize,
        pData: *mut core::ffi::c_void,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table).vkGetShaderBinaryDataEXT.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pDataSize,
                pData,
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
