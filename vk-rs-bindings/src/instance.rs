//! Instance-tier dispatch table and safe [`Instance`] wrapper.
#![allow(
    non_snake_case,
    unused_imports,
    clippy::too_many_arguments,
    clippy::missing_safety_doc
)]
use crate::commands::*;
use crate::entry::VulkanLib;
use crate::enums::*;
use crate::types::*;
use core::ffi::{c_char, c_void};
use core::ptr;
/// Raw instance-tier function pointer table.
///
/// Fields are `Option<PFN_*>`; `None` means absent at load time.
/// Use [`Instance`] for the safe API.
#[cfg(feature = "VK_BASE_VERSION_1_0")]
#[derive(Debug, Clone)]
pub struct InstanceDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkDestroyInstance: Option<PFN_vkDestroyInstance>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkEnumeratePhysicalDevices: Option<PFN_vkEnumeratePhysicalDevices>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetDeviceProcAddr: Option<PFN_vkGetDeviceProcAddr>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkGetInstanceProcAddr: Option<PFN_vkGetInstanceProcAddr>,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    pub vkEnumeratePhysicalDeviceGroups: Option<PFN_vkEnumeratePhysicalDeviceGroups>,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub vkCreateDebugReportCallbackEXT: Option<PFN_vkCreateDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub vkDebugReportMessageEXT: Option<PFN_vkDebugReportMessageEXT>,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub vkDestroyDebugReportCallbackEXT: Option<PFN_vkDestroyDebugReportCallbackEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkCreateDebugUtilsMessengerEXT: Option<PFN_vkCreateDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkDestroyDebugUtilsMessengerEXT: Option<PFN_vkDestroyDebugUtilsMessengerEXT>,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub vkSubmitDebugUtilsMessageEXT: Option<PFN_vkSubmitDebugUtilsMessageEXT>,
    #[cfg(feature = "VK_EXT_directfb_surface")]
    pub vkCreateDirectFBSurfaceEXT: Option<PFN_vkCreateDirectFBSurfaceEXT>,
    #[cfg(feature = "VK_EXT_headless_surface")]
    pub vkCreateHeadlessSurfaceEXT: Option<PFN_vkCreateHeadlessSurfaceEXT>,
    #[cfg(feature = "VK_EXT_metal_surface")]
    pub vkCreateMetalSurfaceEXT: Option<PFN_vkCreateMetalSurfaceEXT>,
    #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
    pub vkCreateImagePipeSurfaceFUCHSIA: Option<PFN_vkCreateImagePipeSurfaceFUCHSIA>,
    #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
    pub vkCreateStreamDescriptorSurfaceGGP: Option<PFN_vkCreateStreamDescriptorSurfaceGGP>,
    #[cfg(feature = "VK_KHR_android_surface")]
    pub vkCreateAndroidSurfaceKHR: Option<PFN_vkCreateAndroidSurfaceKHR>,
    #[cfg(feature = "VK_KHR_device_group_creation")]
    pub vkEnumeratePhysicalDeviceGroupsKHR: Option<PFN_vkEnumeratePhysicalDeviceGroupsKHR>,
    #[cfg(feature = "VK_KHR_display")]
    pub vkCreateDisplayPlaneSurfaceKHR: Option<PFN_vkCreateDisplayPlaneSurfaceKHR>,
    #[cfg(feature = "VK_KHR_surface")]
    pub vkDestroySurfaceKHR: Option<PFN_vkDestroySurfaceKHR>,
    #[cfg(feature = "VK_KHR_wayland_surface")]
    pub vkCreateWaylandSurfaceKHR: Option<PFN_vkCreateWaylandSurfaceKHR>,
    #[cfg(feature = "VK_KHR_win32_surface")]
    pub vkCreateWin32SurfaceKHR: Option<PFN_vkCreateWin32SurfaceKHR>,
    #[cfg(feature = "VK_KHR_xcb_surface")]
    pub vkCreateXcbSurfaceKHR: Option<PFN_vkCreateXcbSurfaceKHR>,
    #[cfg(feature = "VK_KHR_xlib_surface")]
    pub vkCreateXlibSurfaceKHR: Option<PFN_vkCreateXlibSurfaceKHR>,
    #[cfg(feature = "VK_MVK_ios_surface")]
    pub vkCreateIOSSurfaceMVK: Option<PFN_vkCreateIOSSurfaceMVK>,
    #[cfg(feature = "VK_MVK_macos_surface")]
    pub vkCreateMacOSSurfaceMVK: Option<PFN_vkCreateMacOSSurfaceMVK>,
    #[cfg(feature = "VK_NN_vi_surface")]
    pub vkCreateViSurfaceNN: Option<PFN_vkCreateViSurfaceNN>,
    #[cfg(feature = "VK_OHOS_surface")]
    pub vkCreateSurfaceOHOS: Option<PFN_vkCreateSurfaceOHOS>,
    #[cfg(feature = "VK_QNX_screen_surface")]
    pub vkCreateScreenSurfaceQNX: Option<PFN_vkCreateScreenSurfaceQNX>,
    #[cfg(feature = "VK_SEC_ubm_surface")]
    pub vkCreateUbmSurfaceSEC: Option<PFN_vkCreateUbmSurfaceSEC>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl InstanceDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkDestroyInstance: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkEnumeratePhysicalDevices: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetDeviceProcAddr: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkGetInstanceProcAddr: None,
        #[cfg(feature = "VK_BASE_VERSION_1_1")]
        vkEnumeratePhysicalDeviceGroups: None,
        #[cfg(feature = "VK_EXT_debug_report")]
        vkCreateDebugReportCallbackEXT: None,
        #[cfg(feature = "VK_EXT_debug_report")]
        vkDebugReportMessageEXT: None,
        #[cfg(feature = "VK_EXT_debug_report")]
        vkDestroyDebugReportCallbackEXT: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkCreateDebugUtilsMessengerEXT: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkDestroyDebugUtilsMessengerEXT: None,
        #[cfg(feature = "VK_EXT_debug_utils")]
        vkSubmitDebugUtilsMessageEXT: None,
        #[cfg(feature = "VK_EXT_directfb_surface")]
        vkCreateDirectFBSurfaceEXT: None,
        #[cfg(feature = "VK_EXT_headless_surface")]
        vkCreateHeadlessSurfaceEXT: None,
        #[cfg(feature = "VK_EXT_metal_surface")]
        vkCreateMetalSurfaceEXT: None,
        #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
        vkCreateImagePipeSurfaceFUCHSIA: None,
        #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
        vkCreateStreamDescriptorSurfaceGGP: None,
        #[cfg(feature = "VK_KHR_android_surface")]
        vkCreateAndroidSurfaceKHR: None,
        #[cfg(feature = "VK_KHR_device_group_creation")]
        vkEnumeratePhysicalDeviceGroupsKHR: None,
        #[cfg(feature = "VK_KHR_display")]
        vkCreateDisplayPlaneSurfaceKHR: None,
        #[cfg(feature = "VK_KHR_surface")]
        vkDestroySurfaceKHR: None,
        #[cfg(feature = "VK_KHR_wayland_surface")]
        vkCreateWaylandSurfaceKHR: None,
        #[cfg(feature = "VK_KHR_win32_surface")]
        vkCreateWin32SurfaceKHR: None,
        #[cfg(feature = "VK_KHR_xcb_surface")]
        vkCreateXcbSurfaceKHR: None,
        #[cfg(feature = "VK_KHR_xlib_surface")]
        vkCreateXlibSurfaceKHR: None,
        #[cfg(feature = "VK_MVK_ios_surface")]
        vkCreateIOSSurfaceMVK: None,
        #[cfg(feature = "VK_MVK_macos_surface")]
        vkCreateMacOSSurfaceMVK: None,
        #[cfg(feature = "VK_NN_vi_surface")]
        vkCreateViSurfaceNN: None,
        #[cfg(feature = "VK_OHOS_surface")]
        vkCreateSurfaceOHOS: None,
        #[cfg(feature = "VK_QNX_screen_surface")]
        vkCreateScreenSurfaceQNX: None,
        #[cfg(feature = "VK_SEC_ubm_surface")]
        vkCreateUbmSurfaceSEC: None,
    };
    /// Resolve all instance commands from the given loader closure.
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkDestroyInstance =
                loader(c"vkDestroyInstance".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkEnumeratePhysicalDevices = loader(c"vkEnumeratePhysicalDevices".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetDeviceProcAddr =
                loader(c"vkGetDeviceProcAddr".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkGetInstanceProcAddr = loader(c"vkGetInstanceProcAddr".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_1")]
        {
            table.vkEnumeratePhysicalDeviceGroups =
                loader(c"vkEnumeratePhysicalDeviceGroups".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_report")]
        {
            table.vkCreateDebugReportCallbackEXT =
                loader(c"vkCreateDebugReportCallbackEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_report")]
        {
            table.vkDebugReportMessageEXT = loader(c"vkDebugReportMessageEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_report")]
        {
            table.vkDestroyDebugReportCallbackEXT =
                loader(c"vkDestroyDebugReportCallbackEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkCreateDebugUtilsMessengerEXT =
                loader(c"vkCreateDebugUtilsMessengerEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkDestroyDebugUtilsMessengerEXT =
                loader(c"vkDestroyDebugUtilsMessengerEXT".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_debug_utils")]
        {
            table.vkSubmitDebugUtilsMessageEXT = loader(c"vkSubmitDebugUtilsMessageEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_directfb_surface")]
        {
            table.vkCreateDirectFBSurfaceEXT = loader(c"vkCreateDirectFBSurfaceEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_headless_surface")]
        {
            table.vkCreateHeadlessSurfaceEXT = loader(c"vkCreateHeadlessSurfaceEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_EXT_metal_surface")]
        {
            table.vkCreateMetalSurfaceEXT = loader(c"vkCreateMetalSurfaceEXT".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
        {
            table.vkCreateImagePipeSurfaceFUCHSIA =
                loader(c"vkCreateImagePipeSurfaceFUCHSIA".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
        {
            table.vkCreateStreamDescriptorSurfaceGGP =
                loader(c"vkCreateStreamDescriptorSurfaceGGP".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_android_surface")]
        {
            table.vkCreateAndroidSurfaceKHR = loader(c"vkCreateAndroidSurfaceKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_device_group_creation")]
        {
            table.vkEnumeratePhysicalDeviceGroupsKHR =
                loader(c"vkEnumeratePhysicalDeviceGroupsKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_display")]
        {
            table.vkCreateDisplayPlaneSurfaceKHR =
                loader(c"vkCreateDisplayPlaneSurfaceKHR".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_surface")]
        {
            table.vkDestroySurfaceKHR =
                loader(c"vkDestroySurfaceKHR".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_wayland_surface")]
        {
            table.vkCreateWaylandSurfaceKHR = loader(c"vkCreateWaylandSurfaceKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_win32_surface")]
        {
            table.vkCreateWin32SurfaceKHR = loader(c"vkCreateWin32SurfaceKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_xcb_surface")]
        {
            table.vkCreateXcbSurfaceKHR = loader(c"vkCreateXcbSurfaceKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_KHR_xlib_surface")]
        {
            table.vkCreateXlibSurfaceKHR = loader(c"vkCreateXlibSurfaceKHR".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_MVK_ios_surface")]
        {
            table.vkCreateIOSSurfaceMVK = loader(c"vkCreateIOSSurfaceMVK".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_MVK_macos_surface")]
        {
            table.vkCreateMacOSSurfaceMVK = loader(c"vkCreateMacOSSurfaceMVK".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_NN_vi_surface")]
        {
            table.vkCreateViSurfaceNN =
                loader(c"vkCreateViSurfaceNN".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_OHOS_surface")]
        {
            table.vkCreateSurfaceOHOS =
                loader(c"vkCreateSurfaceOHOS".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_QNX_screen_surface")]
        {
            table.vkCreateScreenSurfaceQNX = loader(c"vkCreateScreenSurfaceQNX".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_SEC_ubm_surface")]
        {
            table.vkCreateUbmSurfaceSEC = loader(c"vkCreateUbmSurfaceSEC".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
    /// Resolve all instance commands via `vkGetInstanceProcAddr(instance, …)`.
    pub fn load_for_instance<F>(instance: VkInstance, mut get_proc: F) -> Self
    where
        F: FnMut(VkInstance, *const c_char) -> Option<unsafe extern "system" fn()>,
    {
        Self::load(|name| get_proc(instance, name))
    }
}
/// Safe `VkInstance` wrapper.
///
/// Lifetime `'lib` ties this instance to the [`VulkanLib`] it came from;
/// it cannot outlive it.
///
/// Holds [`InstanceDispatchTable`] by value.
///
/// # Cleanup
/// `Instance` automatically destroys itself on `Drop` if it has not already
/// been destroyed by the user.  The user does not need to call `vkDestroyInstance`.
#[cfg(feature = "VK_BASE_VERSION_1_0")]
pub struct Instance<'lib> {
    pub(crate) raw: VkInstance,
    pub(crate) table: InstanceDispatchTable,
    pub(crate) physical_device_table: crate::physical_device::PhysicalDeviceDispatchTable,
    #[cfg(feature = "VK_EXT_debug_report")]
    pub(crate) debug_report_callback_ext_table:
        crate::debug_report_callback_ext::DebugReportCallbackEXTDispatchTable,
    #[cfg(feature = "VK_EXT_debug_utils")]
    pub(crate) debug_utils_messenger_ext_table:
        crate::debug_utils_messenger_ext::DebugUtilsMessengerEXTDispatchTable,
    #[cfg(feature = "VK_KHR_surface")]
    pub(crate) surface_khr_table: crate::surface_khr::SurfaceKHRDispatchTable,
    _lib: core::marker::PhantomData<&'lib VulkanLib>,
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'lib> Send for Instance<'lib> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
unsafe impl<'lib> Sync for Instance<'lib> {}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'lib> Instance<'lib> {
    /// Wrap a raw `VkInstance` with a pre-loaded dispatch table.
    ///
    /// # Safety
    /// `raw` must be a valid live `VkInstance` for `'lib`.
    #[inline]
    pub unsafe fn from_raw(
        raw: VkInstance,
        table: InstanceDispatchTable,
        physical_device_table: crate::physical_device::PhysicalDeviceDispatchTable,
        #[cfg(feature = "VK_EXT_debug_report")]
        debug_report_callback_ext_table: crate::debug_report_callback_ext::DebugReportCallbackEXTDispatchTable,
        #[cfg(feature = "VK_EXT_debug_utils")]
        debug_utils_messenger_ext_table: crate::debug_utils_messenger_ext::DebugUtilsMessengerEXTDispatchTable,
        #[cfg(feature = "VK_KHR_surface")]
        surface_khr_table: crate::surface_khr::SurfaceKHRDispatchTable,
    ) -> Self {
        Self {
            raw,
            table,
            physical_device_table,
            #[cfg(feature = "VK_EXT_debug_report")]
            debug_report_callback_ext_table,
            #[cfg(feature = "VK_EXT_debug_utils")]
            debug_utils_messenger_ext_table,
            #[cfg(feature = "VK_KHR_surface")]
            surface_khr_table,
            _lib: core::marker::PhantomData,
        }
    }
    /// The raw `VkInstance` handle.
    #[inline(always)]
    pub fn raw(&self) -> VkInstance {
        self.raw
    }
    /// The underlying dispatch table.
    #[inline(always)]
    pub fn table(&self) -> &InstanceDispatchTable {
        &self.table
    }
    /// [`vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `instance`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkDestroyInstance(&mut self, pAllocator: *const VkAllocationCallbacks) {
        if self.raw.0.is_null() {
            return;
        }
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table).vkDestroyInstance.unwrap_unchecked()(self.raw, pAllocator)
        }
        self.raw = VkInstance::NULL;
    }
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline]
    pub fn vkEnumeratePhysicalDevices<'inst>(
        &'inst self,
    ) -> Result<alloc::boxed::Box<[crate::physical_device::PhysicalDevice<'inst>]>, VkResult> {
        use crate::physical_device::PhysicalDevice;
        let fp = unsafe { self.table.vkEnumeratePhysicalDevices.unwrap_unchecked() };
        let mut count = 0;
        let r = unsafe { fp(self.raw, &mut count, core::ptr::null_mut()) };
        if match r {
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_INITIALIZATION_FAILED
            | VkResult::VK_ERROR_UNKNOWN => true,
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => true,
            _ => r < VkResult::VK_SUCCESS,
        } {
            return Err(r);
        }
        if count == 0 {
            return Ok(alloc::boxed::Box::<
                [crate::physical_device::PhysicalDevice<'inst>; 0],
            >::new([]));
        }
        let mut raw_gpus =
            alloc::boxed::Box::<[VkPhysicalDevice]>::new_uninit_slice(count as usize);
        let r = unsafe { fp(self.raw, &mut count, raw_gpus.as_mut_ptr().cast()) };
        if let Err(e) = {
            match r {
                VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
                VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
                | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
                | VkResult::VK_ERROR_INITIALIZATION_FAILED
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
        } {
            return Err(e);
        }
        let raw_gpus = unsafe { raw_gpus.assume_init() };
        Ok(raw_gpus
            .into_iter()
            .map(|raw| PhysicalDevice {
                raw,
                instance: self,
                table: &self.physical_device_table,
            })
            .collect())
    }
    /// [`vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `device`
    /// - `pName`: len: null-terminated
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetDeviceProcAddr(
        &self,
        device: VkDevice,
        pName: *const core::ffi::c_char,
    ) -> PFN_vkVoidFunction {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table).vkGetDeviceProcAddr.unwrap_unchecked()(device, pName)
        }
    }
    /// [`vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `instance`: optional: true
    /// - `pName`: len: null-terminated
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkGetInstanceProcAddr(&self, pName: *const core::ffi::c_char) -> PFN_vkVoidFunction {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table).vkGetInstanceProcAddr.unwrap_unchecked()(self.raw, pName)
        }
    }
    /// [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_1`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `instance`
    /// - `pPhysicalDeviceGroupCount`: optional: pointer required, values optional if pointer not null
    /// - `pPhysicalDeviceGroupProperties`: optional: true, len: pPhysicalDeviceGroupCount
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
    ///   - VK_ERROR_INITIALIZATION_FAILED
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    #[inline(always)]
    pub fn vkEnumeratePhysicalDeviceGroups(
        &self,
        pPhysicalDeviceGroupCount: *mut u32,
        pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (&self.table)
                .vkEnumeratePhysicalDeviceGroups
                .unwrap_unchecked()(
                self.raw,
                pPhysicalDeviceGroupCount,
                pPhysicalDeviceGroupProperties,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_INITIALIZATION_FAILED
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
    /// [`vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_report`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pCallback`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_EXT_debug_report")]
    #[inline]
    pub fn vkCreateDebugReportCallbackEXT<'ret>(
        &'ret self,
        pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::debug_report_callback_ext::DebugReportCallbackEXT<'ret>, VkResult> {
        let mut handle = VkDebugReportCallbackEXT::NULL;
        let r = unsafe {
            (&self.table)
                .vkCreateDebugReportCallbackEXT
                .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
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
        .map(
            |_| crate::debug_report_callback_ext::DebugReportCallbackEXT {
                raw: handle,
                parent: self,
                table: &self.debug_report_callback_ext_table,
            },
        )
    }
    /// [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_report`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `flags`
    /// - `objectType`
    /// - `object`: object_type: objectType
    /// - `location`
    /// - `messageCode`
    /// - `pLayerPrefix`: len: null-terminated
    /// - `pMessage`: len: null-terminated
    #[cfg(feature = "VK_EXT_debug_report")]
    #[inline(always)]
    pub fn vkDebugReportMessageEXT(
        &self,
        flags: VkDebugReportFlagsEXT,
        objectType: VkDebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        messageCode: i32,
        pLayerPrefix: *const core::ffi::c_char,
        pMessage: *const core::ffi::c_char,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table).vkDebugReportMessageEXT.unwrap_unchecked()(
                self.raw,
                flags,
                objectType,
                object,
                location,
                messageCode,
                pLayerPrefix,
                pMessage,
            )
        }
    }
    /// [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_report`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `callback`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_EXT_debug_report")]
    #[inline(always)]
    pub fn vkDestroyDebugReportCallbackEXT(
        &self,
        callback: VkDebugReportCallbackEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table)
                .vkDestroyDebugReportCallbackEXT
                .unwrap_unchecked()(self.raw, callback, pAllocator)
        }
    }
    /// [`vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pMessenger`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline]
    pub fn vkCreateDebugUtilsMessengerEXT<'ret>(
        &'ret self,
        pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::debug_utils_messenger_ext::DebugUtilsMessengerEXT<'ret>, VkResult> {
        let mut handle = VkDebugUtilsMessengerEXT::NULL;
        let r = unsafe {
            (&self.table)
                .vkCreateDebugUtilsMessengerEXT
                .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
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
        .map(
            |_| crate::debug_utils_messenger_ext::DebugUtilsMessengerEXT {
                raw: handle,
                parent: self,
                table: &self.debug_utils_messenger_ext_table,
            },
        )
    }
    /// [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `messenger`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline(always)]
    pub fn vkDestroyDebugUtilsMessengerEXT(
        &self,
        messenger: VkDebugUtilsMessengerEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table)
                .vkDestroyDebugUtilsMessengerEXT
                .unwrap_unchecked()(self.raw, messenger, pAllocator)
        }
    }
    /// [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_debug_utils`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `messageSeverity`
    /// - `messageTypes`
    /// - `pCallbackData`
    #[cfg(feature = "VK_EXT_debug_utils")]
    #[inline(always)]
    pub fn vkSubmitDebugUtilsMessageEXT(
        &self,
        messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table)
                .vkSubmitDebugUtilsMessageEXT
                .unwrap_unchecked()(
                self.raw, messageSeverity, messageTypes, pCallbackData
            )
        }
    }
    /// [`vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_directfb_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_EXT_directfb_surface")]
    #[inline]
    pub fn vkCreateDirectFBSurfaceEXT<'ret>(
        &'ret self,
        pCreateInfo: *const VkDirectFBSurfaceCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateDirectFBSurfaceEXT.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_headless_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_EXT_headless_surface")]
    #[inline]
    pub fn vkCreateHeadlessSurfaceEXT<'ret>(
        &'ret self,
        pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateHeadlessSurfaceEXT.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html)
    ///
    /// Provided by:
    /// - `VK_EXT_metal_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_EXT_metal_surface")]
    #[inline]
    pub fn vkCreateMetalSurfaceEXT<'ret>(
        &'ret self,
        pCreateInfo: *const VkMetalSurfaceCreateInfoEXT,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateMetalSurfaceEXT.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html)
    ///
    /// Provided by:
    /// - `VK_FUCHSIA_imagepipe_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_FUCHSIA_imagepipe_surface")]
    #[inline]
    pub fn vkCreateImagePipeSurfaceFUCHSIA<'ret>(
        &'ret self,
        pCreateInfo: *const VkImagePipeSurfaceCreateInfoFUCHSIA,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table)
                .vkCreateImagePipeSurfaceFUCHSIA
                .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html)
    ///
    /// Provided by:
    /// - `VK_GGP_stream_descriptor_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_GGP_stream_descriptor_surface")]
    #[inline]
    pub fn vkCreateStreamDescriptorSurfaceGGP<'ret>(
        &'ret self,
        pCreateInfo: *const VkStreamDescriptorSurfaceCreateInfoGGP,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table)
                .vkCreateStreamDescriptorSurfaceGGP
                .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_android_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_android_surface")]
    #[inline]
    pub fn vkCreateAndroidSurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateAndroidSurfaceKHR.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html)
    ///
    /// Provided by:
    /// - `VK_KHR_device_group_creation`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `instance`
    /// - `pPhysicalDeviceGroupCount`: optional: pointer required, values optional if pointer not null
    /// - `pPhysicalDeviceGroupProperties`: optional: true, len: pPhysicalDeviceGroupCount
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
    ///   - VK_ERROR_INITIALIZATION_FAILED
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_device_group_creation")]
    #[inline(always)]
    pub fn vkEnumeratePhysicalDeviceGroupsKHR(
        &self,
        pPhysicalDeviceGroupCount: *mut u32,
        pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (&self.table)
                .vkEnumeratePhysicalDeviceGroupsKHR
                .unwrap_unchecked()(
                self.raw,
                pPhysicalDeviceGroupCount,
                pPhysicalDeviceGroupProperties,
            )
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_INITIALIZATION_FAILED
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
    /// [`vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_display`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_display")]
    #[inline]
    pub fn vkCreateDisplayPlaneSurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table)
                .vkCreateDisplayPlaneSurfaceKHR
                .unwrap_unchecked()(self.raw, pCreateInfo, pAllocator, &mut handle)
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `surface`: optional: true
    /// - `pAllocator`: optional: true
    #[cfg(feature = "VK_KHR_surface")]
    #[inline(always)]
    pub fn vkDestroySurfaceKHR(
        &self,
        surface: VkSurfaceKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) {
        unsafe {
            // SAFETY: table is fully loaded at creation.
            (&self.table).vkDestroySurfaceKHR.unwrap_unchecked()(self.raw, surface, pAllocator)
        }
    }
    /// [`vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_wayland_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_wayland_surface")]
    #[inline]
    pub fn vkCreateWaylandSurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateWaylandSurfaceKHR.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_win32_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_win32_surface")]
    #[inline]
    pub fn vkCreateWin32SurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateWin32SurfaceKHR.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_xcb_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_xcb_surface")]
    #[inline]
    pub fn vkCreateXcbSurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateXcbSurfaceKHR.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html)
    ///
    /// Provided by:
    /// - `VK_KHR_xlib_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_KHR_xlib_surface")]
    #[inline]
    pub fn vkCreateXlibSurfaceKHR<'ret>(
        &'ret self,
        pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateXlibSurfaceKHR.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html)
    ///
    /// Provided by:
    /// - `VK_MVK_ios_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_MVK_ios_surface")]
    #[inline]
    pub fn vkCreateIOSSurfaceMVK<'ret>(
        &'ret self,
        pCreateInfo: *const VkIOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateIOSSurfaceMVK.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html)
    ///
    /// Provided by:
    /// - `VK_MVK_macos_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_MVK_macos_surface")]
    #[inline]
    pub fn vkCreateMacOSSurfaceMVK<'ret>(
        &'ret self,
        pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateMacOSSurfaceMVK.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html)
    ///
    /// Provided by:
    /// - `VK_NN_vi_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_NATIVE_WINDOW_IN_USE_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_NN_vi_surface")]
    #[inline]
    pub fn vkCreateViSurfaceNN<'ret>(
        &'ret self,
        pCreateInfo: *const VkViSurfaceCreateInfoNN,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateViSurfaceNN.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_UNKNOWN => Err(r),
            #[cfg(feature = "VK_BASE_VERSION_1_0")]
            VkResult::VK_ERROR_VALIDATION_FAILED => Err(r),
            #[cfg(feature = "VK_KHR_surface")]
            VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => Err(r),
            _ => {
                if r >= VkResult::VK_SUCCESS {
                    Ok(r)
                } else {
                    Err(r)
                }
            }
        }
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html)
    ///
    /// Provided by:
    /// - `VK_OHOS_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_SURFACE_LOST_KHR
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_OHOS_surface")]
    #[inline]
    pub fn vkCreateSurfaceOHOS<'ret>(
        &'ret self,
        pCreateInfo: *const VkSurfaceCreateInfoOHOS,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateSurfaceOHOS.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY | VkResult::VK_ERROR_UNKNOWN => Err(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html)
    ///
    /// Provided by:
    /// - `VK_QNX_screen_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_QNX_screen_surface")]
    #[inline]
    pub fn vkCreateScreenSurfaceQNX<'ret>(
        &'ret self,
        pCreateInfo: *const VkScreenSurfaceCreateInfoQNX,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateScreenSurfaceQNX.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
    /// [`vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html)
    ///
    /// Provided by:
    /// - `VK_SEC_ubm_surface`
    ///
    ///
    /// # Parameters
    /// - `instance`
    /// - `pCreateInfo`
    /// - `pAllocator`: optional: true
    /// - `pSurface`
    ///
    /// # Returns
    ///
    /// **Success Codes:**
    ///   - VK_SUCCESS
    ///
    /// **Error Codes:**
    ///   - VK_ERROR_OUT_OF_HOST_MEMORY
    ///   - VK_ERROR_OUT_OF_DEVICE_MEMORY
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_SEC_ubm_surface")]
    #[inline]
    pub fn vkCreateUbmSurfaceSEC<'ret>(
        &'ret self,
        pCreateInfo: *const VkUbmSurfaceCreateInfoSEC,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::surface_khr::SurfaceKHR<'ret>, VkResult> {
        let mut handle = VkSurfaceKHR::NULL;
        let r = unsafe {
            (&self.table).vkCreateUbmSurfaceSEC.unwrap_unchecked()(
                self.raw,
                pCreateInfo,
                pAllocator,
                &mut handle,
            )
        };
        match r {
            VkResult::VK_SUCCESS => Ok(r),
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
        .map(|_| crate::surface_khr::SurfaceKHR {
            raw: handle,
            parent: self,
            table: &self.surface_khr_table,
        })
    }
}
#[cfg(feature = "VK_BASE_VERSION_1_0")]
impl<'lib> Drop for Instance<'lib> {
    #[inline]
    fn drop(&mut self) {
        if self.raw.0.is_null() {
            return;
        } else if let Some(destroy) = self.table.vkDestroyInstance {
            unsafe { destroy(self.raw, core::ptr::null()) };
        }
    }
}
