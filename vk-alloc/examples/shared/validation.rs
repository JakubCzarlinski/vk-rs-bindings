use std::ffi::CStr;
use std::ptr::null_mut;

pub(crate) const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

pub(crate) fn require_validation_layer(entry: &vk::Entry<'_>) -> Result<(), String> {
    let mut count = 0;
    entry
        .vkEnumerateInstanceLayerProperties(&raw mut count, null_mut())
        .map_err(|err| format!("failed to enumerate instance layers: {err:?}"))?;
    let mut layers = vec![vk::VkLayerProperties::DEFAULT; count as usize];
    entry
        .vkEnumerateInstanceLayerProperties(&raw mut count, layers.as_mut_ptr())
        .map_err(|err| format!("failed to fetch instance layers: {err:?}"))?;
    let has_validation = layers
        .iter()
        .take(count as usize)
        .any(|layer| unsafe { CStr::from_ptr(layer.layerName.as_ptr()) == VALIDATION_LAYER });
    if has_validation {
        Ok(())
    } else {
        Err(format!(
            "required validation layer {} is not available",
            VALIDATION_LAYER.to_string_lossy()
        ))
    }
}
