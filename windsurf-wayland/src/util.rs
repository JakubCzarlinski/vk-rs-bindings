use std::num::NonZeroU32;

use wayland_client::WEnum;

pub(crate) fn unpack_enum<T: Copy>(value: WEnum<T>) -> Option<T> {
    match value {
        WEnum::Value(value) => Some(value),
        WEnum::Unknown(_) => None,
    }
}

pub(crate) fn logical_size_to_i32(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

pub(crate) fn nonzero_or(current: u32, next: u32) -> u32 {
    NonZeroU32::new(next).map_or(current, NonZeroU32::get)
}

#[cfg(test)]
mod tests {
    use super::nonzero_or;

    #[test]
    fn preserves_old_size_when_configure_is_zero() {
        assert_eq!(nonzero_or(800, 0), 800);
        assert_eq!(nonzero_or(800, 640), 640);
    }
}
