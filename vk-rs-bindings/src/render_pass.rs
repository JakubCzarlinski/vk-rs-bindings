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
pub struct RenderPassDispatchTable {
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    pub vkDestroyRenderPass: Option<PFN_vkDestroyRenderPass>,
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    pub vkGetRenderAreaGranularity: Option<PFN_vkGetRenderAreaGranularity>,
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    pub vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI:
        Option<PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl RenderPassDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        vkDestroyRenderPass: None,
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        vkGetRenderAreaGranularity: None,
        #[cfg(feature = "VK_HUAWEI_subpass_shading")]
        vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI: None,
    };
    #[allow(unused_mut, unused_variables)]
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        {
            table.vkDestroyRenderPass =
                loader(c"vkDestroyRenderPass".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
        {
            table.vkGetRenderAreaGranularity = loader(c"vkGetRenderAreaGranularity".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_HUAWEI_subpass_shading")]
        {
            table.vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
                loader(c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
pub struct RenderPass<'dev> {
    pub(crate) raw: VkRenderPass,
    pub(crate) parent: &'dev crate::device::Device<'dev>,
    pub(crate) table: &'dev RenderPassDispatchTable,
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> Drop for RenderPass<'dev> {
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        }
        if let Some(destroy_fn) = self.table.vkDestroyRenderPass {
            unsafe { destroy_fn(self.parent.raw(), self.raw, core::ptr::null()) };
        }
    }
}
#[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
impl<'dev> RenderPass<'dev> {
    #[inline]
    pub fn raw(&self) -> VkRenderPass {
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
    pub fn table(&self) -> &RenderPassDispatchTable {
        self.table
    }
    /// [`vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html)
    ///
    /// Provided by:
    /// - `VK_GRAPHICS_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `renderPass`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyRenderPass(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkDestroyRenderPass.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pAllocator,
            )
        }
        self.raw = VkRenderPass::NULL;
    }
    /// [`vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html)
    ///
    /// Provided by:
    /// - `VK_GRAPHICS_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `renderPass`
    /// - `pGranularity`
    #[cfg(feature = "VK_GRAPHICS_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetRenderAreaGranularity(&self, pGranularity: *mut VkExtent2D) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (self.table).vkGetRenderAreaGranularity.unwrap_unchecked()(
                self.device().raw(),
                self.raw,
                pGranularity,
            )
        }
    }
    /// [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
    ///
    /// Provided by:
    /// - `VK_HUAWEI_subpass_shading`
    ///
    ///
    /// # Parameters
    /// - `device`
    /// - `renderpass`
    /// - `pMaxWorkgroupSize`: len: 1
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_SURFACE_LOST_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_HUAWEI_subpass_shading")]
    #[inline(always)]
    pub fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
        &self,
        pMaxWorkgroupSize: *mut VkExtent2D,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (self.table)
                .vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI
                .unwrap_unchecked()(self.device().raw(), self.raw, pMaxWorkgroupSize)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_SURFACE_LOST_KHR => Err(r),
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
