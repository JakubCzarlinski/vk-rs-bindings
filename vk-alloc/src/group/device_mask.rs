use crate::error::AllocatorError;
use alloc::boxed::Box;

pub(crate) const fn validate_device_mask(device_mask: u32) -> Result<(), AllocatorError> {
    if device_mask == 0 {
        Err(AllocatorError::InvalidDeviceMask)
    } else {
        Ok(())
    }
}

pub(crate) fn partition_device_mask(device_mask: u32) -> Box<[u32]> {
    (0..32)
        .filter(|&bit| device_mask & (1u32 << bit) != 0)
        .collect::<Box<[u32]>>()
}
