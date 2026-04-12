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
pub struct ShaderModuleDispatchTable {
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    pub vkDestroyShaderModule: Option<PFN_vkDestroyShaderModule>,
    #[cfg(feature = "VK_EXT_shader_module_identifier")]
    pub vkGetShaderModuleIdentifierEXT: Option<PFN_vkGetShaderModuleIdentifierEXT>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl ShaderModuleDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        vkDestroyShaderModule: None,
        #[cfg(feature = "VK_EXT_shader_module_identifier")]
        vkGetShaderModuleIdentifierEXT: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
        {
            table.vkDestroyShaderModule = loader(c"vkDestroyShaderModule".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_shader_module_identifier")]
        {
            table.vkGetShaderModuleIdentifierEXT = loader(
                    c"vkGetShaderModuleIdentifierEXT".as_ptr(),
                )
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
pub struct ShaderModule<'dev> {
    pub(crate) raw: VkShaderModule,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev ShaderModuleDispatchTable,
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> Drop for ShaderModule<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyShaderModule {
            unsafe { destroy_fn(self.parent.raw, self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
impl<'dev> ShaderModule<'dev> {
    #[inline]
    pub fn raw(&self) -> VkShaderModule {
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
    pub fn table(&self) -> &ShaderModuleDispatchTable {
        self.table
    }
    /// [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html)
    ///
    /// Provided by:
    /// - `VK_COMPUTE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan
    ///
    /// # Parameters
    /// - `device`
    /// - `shaderModule`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_COMPUTE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyShaderModule(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkDestroyShaderModule
                .unwrap_unchecked()(self.device().raw(), self.raw, pAllocator)
        }
        self.raw = VkShaderModule::NULL;
    }
    /// [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_shader_module_identifier`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `shaderModule`
    /// - `pIdentifier`
    #[cfg(feature = "VK_EXT_shader_module_identifier")]
    #[inline(always)]
    pub fn vkGetShaderModuleIdentifierEXT(
        &self,
        pIdentifier: *mut VkShaderModuleIdentifierEXT,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table)
                .vkGetShaderModuleIdentifierEXT
                .unwrap_unchecked()(self.device().raw(), self.raw, pIdentifier)
        }
    }
}
