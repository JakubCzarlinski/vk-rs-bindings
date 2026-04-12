//! Vulkan library loader, pre-instance dispatch table, and `Entry` wrapper.
//!
//! # Hierarchy
//!
//! ```text
//! VulkanLib
//!   └── Entry<'lib>          (pre-instance commands)
//!         └── Instance<'lib> (instance commands)
//!               └── Device<'inst> (device commands)
//!                     └── CommandBuffer<'dev>
//! ```
//!
//! # Quick start
//!
//! ```rust,no_run
//! let lib      = VulkanLib::load()?;
//! let entry    = Entry::new(&lib);
//! let instance = entry.vkCreateInstance(&info, ptr::null())?;
//! let gpus     = instance.vkEnumeratePhysicalDevices()?;
//! let device   = instance.vkCreateDevice(gpus[0], &dev_info, ptr::null())?;
//! ```
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
#[cfg(target_os = "windows")]
const VULKAN_LIB_NAMES: &[&str] = &["vulkan-1.dll"];
#[cfg(target_os = "macos")]
const VULKAN_LIB_NAMES: &[&str] = &["libMoltenVK.dylib", "libvulkan.dylib", "libvulkan.1.dylib"];
#[cfg(target_os = "ios")]
const VULKAN_LIB_NAMES: &[&str] = &["libMoltenVK.dylib"];
#[cfg(target_os = "android")]
const VULKAN_LIB_NAMES: &[&str] = &["libvulkan.so"];
#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android"),
))]
const VULKAN_LIB_NAMES: &[&str] = &["libvulkan.so.1", "libvulkan.so"];
/// Error from [`VulkanLib::load`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// No platform Vulkan library could be opened.
    LibraryNotFound,
    /// Library opened but `vkGetInstanceProcAddr` was missing.
    MissingGetInstanceProcAddr,
}
impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoadError::LibraryNotFound => {
                write!(f, "Vulkan loader not found (tried: {:?})", VULKAN_LIB_NAMES)
            }
            LoadError::MissingGetInstanceProcAddr => {
                write!(f, "Vulkan library missing `vkGetInstanceProcAddr`")
            }
        }
    }
}
/// Handle to the platform Vulkan shared library.
///
/// Must outlive all [`Entry`], [`Instance`], and [`Device`] values
/// derived from it - the lifetime chain is enforced by the borrow checker.
///
/// # Cleanup
/// No cleanup required. The library is closed when this value is dropped.
pub struct VulkanLib {
    _lib: libloading::Library,
    pub(crate) get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
}
impl VulkanLib {
    /// Open the platform Vulkan loader.
    pub fn load() -> Result<Self, LoadError> {
        for &name in VULKAN_LIB_NAMES {
            let Ok(lib) = (unsafe { libloading::Library::new(name) }) else {
                continue;
            };
            let sym = unsafe { lib.get::<PFN_vkGetInstanceProcAddr>(c"vkGetInstanceProcAddr") };
            let sym = match sym {
                Ok(s) => *s,
                Err(_) => return Err(LoadError::MissingGetInstanceProcAddr),
            };
            return Ok(Self {
                _lib: lib,
                get_instance_proc_addr: sym,
            });
        }
        Err(LoadError::LibraryNotFound)
    }
}
/// Pre-instance function pointer table.
///
/// Fields are `Option<PFN_*>`; `None` means absent at load time.
/// Use [`Entry`] for the safe API.
#[derive(Debug, Clone)]
pub struct EntryDispatchTable {
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkCreateInstance: Option<PFN_vkCreateInstance>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkEnumerateInstanceExtensionProperties: Option<PFN_vkEnumerateInstanceExtensionProperties>,
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    pub vkEnumerateInstanceLayerProperties: Option<PFN_vkEnumerateInstanceLayerProperties>,
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    pub vkEnumerateInstanceVersion: Option<PFN_vkEnumerateInstanceVersion>,
}
impl EntryDispatchTable {
    pub const EMPTY: Self = Self {
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkCreateInstance: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkEnumerateInstanceExtensionProperties: None,
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        vkEnumerateInstanceLayerProperties: None,
        #[cfg(feature = "VK_BASE_VERSION_1_1")]
        vkEnumerateInstanceVersion: None,
    };
    /// Resolve all pre-instance commands from the given loader closure.
    pub fn load<F>(mut loader: F) -> Self
    where
        F: FnMut(*const c_char) -> Option<unsafe extern "system" fn()>,
    {
        let mut table = Self::EMPTY;
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkCreateInstance =
                loader(c"vkCreateInstance".as_ptr()).map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkEnumerateInstanceExtensionProperties =
                loader(c"vkEnumerateInstanceExtensionProperties".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_0")]
        {
            table.vkEnumerateInstanceLayerProperties =
                loader(c"vkEnumerateInstanceLayerProperties".as_ptr())
                    .map(|f| unsafe { core::mem::transmute(f) });
        }
        #[cfg(feature = "VK_BASE_VERSION_1_1")]
        {
            table.vkEnumerateInstanceVersion = loader(c"vkEnumerateInstanceVersion".as_ptr())
                .map(|f| unsafe { core::mem::transmute(f) });
        }
        table
    }
}
/// Pre-instance Vulkan entry point.
///
/// Borrows the [`VulkanLib`] it was created from; cannot outlive it.
///
/// Obtain via [`Entry::new`], then call [`Entry::vkCreateInstance`] to
/// get an [`Instance`].
///
/// # Cleanup
/// No cleanup required.  Entry commands leave no Vulkan objects alive.
pub struct Entry<'lib> {
    table: EntryDispatchTable,
    /// Retained so that `vkCreateInstance` can resolve the instance-tier
    /// table via `vkGetInstanceProcAddr` after the instance is created.
    lib: &'lib VulkanLib,
}
impl<'lib> Entry<'lib> {
    /// Create an `Entry` by loading all pre-instance commands from `lib`.
    #[inline]
    pub fn new(lib: &'lib VulkanLib) -> Self {
        let table = EntryDispatchTable::load(|name| unsafe {
            (lib.get_instance_proc_addr)(VkInstance::NULL, name)
        });
        Self { table, lib }
    }
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    /// Create a Vulkan instance.
    ///
    /// On success returns an [`Instance`] whose lifetime is tied to this
    /// `Entry` (and therefore to the underlying [`VulkanLib`]).
    ///
    /// # Safety
    /// `pCreateInfo` must point to a valid, fully-initialized
    /// `VkInstanceCreateInfo`.
    #[inline]
    pub fn vkCreateInstance(
        &self,
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
    ) -> Result<crate::instance::Instance<'lib>, VkResult> {
        use crate::instance::{Instance, InstanceDispatchTable};
        let fp = unsafe { self.table.vkCreateInstance.unwrap_unchecked() };
        let mut raw = VkInstance::NULL;
        let r = unsafe { fp(pCreateInfo, pAllocator, &mut raw) };
        if let Err(e) = {
            match r {
                VkResult::VK_SUCCESS => Ok(r),
                VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
                | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
                | VkResult::VK_ERROR_INITIALIZATION_FAILED
                | VkResult::VK_ERROR_LAYER_NOT_PRESENT
                | VkResult::VK_ERROR_EXTENSION_NOT_PRESENT
                | VkResult::VK_ERROR_INCOMPATIBLE_DRIVER
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
        let table = InstanceDispatchTable::load(|name| unsafe {
            (self.lib.get_instance_proc_addr)(raw, name)
        });
        let pd_table = crate::physical_device::PhysicalDeviceDispatchTable::load(|name| unsafe {
            (self.lib.get_instance_proc_addr)(raw, name)
        });
        #[cfg(feature = "VK_EXT_debug_report")]
        let debug_report_callback_ext_table =
            crate::debug_report_callback_ext::DebugReportCallbackEXTDispatchTable::load(
                |name| unsafe { (self.lib.get_instance_proc_addr)(raw, name) },
            );
        #[cfg(feature = "VK_EXT_debug_utils")]
        let debug_utils_messenger_ext_table =
            crate::debug_utils_messenger_ext::DebugUtilsMessengerEXTDispatchTable::load(
                |name| unsafe { (self.lib.get_instance_proc_addr)(raw, name) },
            );
        #[cfg(feature = "VK_KHR_surface")]
        let surface_khr_table = crate::surface_khr::SurfaceKHRDispatchTable::load(|name| unsafe {
            (self.lib.get_instance_proc_addr)(raw, name)
        });
        Ok(unsafe {
            Instance::from_raw(
                raw,
                table,
                pd_table,
                #[cfg(feature = "VK_EXT_debug_report")]
                debug_report_callback_ext_table,
                #[cfg(feature = "VK_EXT_debug_utils")]
                debug_utils_messenger_ext_table,
                #[cfg(feature = "VK_KHR_surface")]
                surface_khr_table,
            )
        })
    }
    /// [`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `pLayerName`: optional: true, len: null-terminated
    /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
    /// - `pProperties`: optional: true, len: pPropertyCount
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
    ///   - VK_ERROR_LAYER_NOT_PRESENT
    ///   - VK_ERROR_UNKNOWN
    ///   - VK_ERROR_VALIDATION_FAILED
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkEnumerateInstanceExtensionProperties(
        &self,
        pLayerName: *const core::ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut VkExtensionProperties,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (&self.table)
                .vkEnumerateInstanceExtensionProperties
                .unwrap_unchecked()(pLayerName, pPropertyCount, pProperties)
        };
        match r {
            VkResult::VK_SUCCESS | VkResult::VK_INCOMPLETE => Ok(r),
            VkResult::VK_ERROR_OUT_OF_HOST_MEMORY
            | VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY
            | VkResult::VK_ERROR_LAYER_NOT_PRESENT
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
    /// [`vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_0`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `pPropertyCount`: optional: pointer required, values optional if pointer not null
    /// - `pProperties`: optional: true, len: pPropertyCount
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
    #[cfg(feature = "VK_BASE_VERSION_1_0")]
    #[inline(always)]
    pub fn vkEnumerateInstanceLayerProperties(
        &self,
        pPropertyCount: *mut u32,
        pProperties: *mut VkLayerProperties,
    ) -> Result<VkResult, VkResult> {
        let r = unsafe {
            (&self.table)
                .vkEnumerateInstanceLayerProperties
                .unwrap_unchecked()(pPropertyCount, pProperties)
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
    /// [`vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html)
    ///
    /// Provided by:
    /// - `VK_BASE_VERSION_1_1`
    ///
    /// - **Export Scopes:** Vulkan, VulkanSC
    ///
    /// # Parameters
    /// - `pApiVersion`
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
    #[cfg(feature = "VK_BASE_VERSION_1_1")]
    #[inline(always)]
    pub fn vkEnumerateInstanceVersion(&self, pApiVersion: *mut u32) -> Result<VkResult, VkResult> {
        let r = unsafe { (&self.table).vkEnumerateInstanceVersion.unwrap_unchecked()(pApiVersion) };
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
    }
}
