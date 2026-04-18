use wayland_client::WEnum;

pub(crate) const fn unpack_enum<T: Copy>(value: WEnum<T>) -> Option<T> {
    match value {
        WEnum::Value(value) => Some(value),
        WEnum::Unknown(_) => None,
    }
}

pub(crate) const fn logical_size_to_i32(value: u32) -> i32 {
    if value > i32::MAX as u32 {
        i32::MAX
    } else {
        value as i32
    }
}

pub(crate) const fn nonzero_or(current: u32, next: u32) -> u32 {
    if next == 0 { current } else { next }
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
