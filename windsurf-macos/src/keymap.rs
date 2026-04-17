use alloc::string::{String, ToString};
use objc2_app_kit::{NSEvent, NSEventModifierFlags};
use windsurf_core::KeyCode;

extern crate alloc;

pub(crate) fn key_code_for_event(event: &NSEvent) -> KeyCode {
    key_code_from_scancode(u32::from(event.keyCode()))
}

pub(crate) fn text_for_event(event: &NSEvent) -> Option<String> {
    let text = event.characters()?.to_string();
    if text.is_empty() || text.chars().all(char::is_control) {
        return None;
    }
    Some(text)
}

pub(crate) fn key_code_from_scancode(scancode: u32) -> KeyCode {
    match scancode {
        53 => KeyCode::Escape,
        36 | 76 => KeyCode::Enter,
        48 => KeyCode::Tab,
        51 => KeyCode::Backspace,
        49 => KeyCode::Space,
        114 => KeyCode::Insert,
        117 => KeyCode::Delete,
        115 => KeyCode::Home,
        119 => KeyCode::End,
        116 => KeyCode::PageUp,
        121 => KeyCode::PageDown,
        126 => KeyCode::ArrowUp,
        125 => KeyCode::ArrowDown,
        123 => KeyCode::ArrowLeft,
        124 => KeyCode::ArrowRight,
        56 | 60 => KeyCode::Shift,
        59 | 62 => KeyCode::Control,
        58 | 61 => KeyCode::Alt,
        55 | 54 => KeyCode::Meta,
        57 => KeyCode::CapsLock,
        71 => KeyCode::NumLock,
        107 => KeyCode::F14,
        122 => KeyCode::F1,
        120 => KeyCode::F2,
        99 => KeyCode::F3,
        118 => KeyCode::F4,
        96 => KeyCode::F5,
        97 => KeyCode::F6,
        98 => KeyCode::F7,
        100 => KeyCode::F8,
        101 => KeyCode::F9,
        109 => KeyCode::F10,
        103 => KeyCode::F11,
        111 => KeyCode::F12,
        105 => KeyCode::F13,
        113 => KeyCode::F15,
        106 => KeyCode::F16,
        64 => KeyCode::F17,
        79 => KeyCode::F18,
        80 => KeyCode::F19,
        90 => KeyCode::F20,
        _ => KeyCode::Unknown,
    }
}

pub(crate) fn modifier_key_for_scancode(scancode: u16) -> Option<(KeyCode, NSEventModifierFlags)> {
    match scancode {
        56 | 60 => Some((KeyCode::Shift, NSEventModifierFlags::Shift)),
        59 | 62 => Some((KeyCode::Control, NSEventModifierFlags::Control)),
        58 | 61 => Some((KeyCode::Alt, NSEventModifierFlags::Option)),
        55 | 54 => Some((KeyCode::Meta, NSEventModifierFlags::Command)),
        57 => Some((KeyCode::CapsLock, NSEventModifierFlags::CapsLock)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use windsurf_core::KeyCode;

    use super::key_code_from_scancode;

    #[test]
    fn maps_known_scancodes() {
        assert_eq!(key_code_from_scancode(122), KeyCode::F1);
        assert_eq!(key_code_from_scancode(53), KeyCode::Escape);
        assert_eq!(key_code_from_scancode(999), KeyCode::Unknown);
    }
}
