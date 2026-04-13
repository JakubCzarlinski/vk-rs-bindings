use std::fs::File;
use std::os::fd::OwnedFd;
use std::ptr::NonNull;

use memmap2::MmapOptions;
use windsurf_core::{Key, KeyState};
use xkbcommon_dl as xkb;

pub(crate) struct XkbState {
    context: NonNull<xkb::xkb_context>,
    keymap: Option<NonNull<xkb::xkb_keymap>>,
    state: Option<NonNull<xkb::xkb_state>>,
}

impl XkbState {
    pub(crate) fn new() -> Option<Self> {
        let handle = xkb::xkbcommon_option()?;
        let context = NonNull::new(unsafe {
            (handle.xkb_context_new)(xkb::xkb_context_flags::XKB_CONTEXT_NO_FLAGS)
        })?;
        Some(Self {
            context,
            keymap: None,
            state: None,
        })
    }

    pub(crate) fn load_keymap(&mut self, fd: OwnedFd, size: usize) {
        let Some(handle) = xkb::xkbcommon_option() else {
            return;
        };
        if size == 0 {
            return;
        }

        let file = File::from(fd);
        let Ok(mapped) = (unsafe { MmapOptions::new().len(size).map(&file) }) else {
            return;
        };
        let Some(keymap_len) = mapped_keymap_len(mapped.as_ref()) else {
            return;
        };

        // The compositor-owned keymap is immutable. Mapping it avoids an extra
        // copy before `xkbcommon` parses it into its own internal structures.
        let keymap = NonNull::new(unsafe {
            (handle.xkb_keymap_new_from_buffer)(
                self.context.as_ptr(),
                mapped.as_ptr().cast(),
                keymap_len,
                xkb::xkb_keymap_format::XKB_KEYMAP_FORMAT_TEXT_V1,
                xkb::xkb_keymap_compile_flags::XKB_KEYMAP_COMPILE_NO_FLAGS,
            )
        });
        let state = keymap
            .and_then(|keymap| NonNull::new(unsafe { (handle.xkb_state_new)(keymap.as_ptr()) }));

        if let Some(old_state) = self.state.take() {
            unsafe { (handle.xkb_state_unref)(old_state.as_ptr()) };
        }
        if let Some(old_keymap) = self.keymap.take() {
            unsafe { (handle.xkb_keymap_unref)(old_keymap.as_ptr()) };
        }

        self.state = state;
        self.keymap = keymap;
    }

    pub(crate) fn update_modifiers(
        &mut self,
        depressed: u32,
        latched: u32,
        locked: u32,
        group: u32,
    ) {
        let Some(handle) = xkb::xkbcommon_option() else {
            return;
        };
        let Some(state) = self.state else {
            return;
        };

        unsafe {
            (handle.xkb_state_update_mask)(
                state.as_ptr(),
                depressed,
                latched,
                locked,
                group,
                group,
                group,
            );
        }
    }

    pub(crate) fn translate(&mut self, key: u32, state: KeyState) -> Option<Key> {
        let handle = xkb::xkbcommon_option()?;
        let xkb_state = self.state?;
        let keycode = key.saturating_add(8);
        let direction = match state {
            KeyState::Released => xkb::xkb_key_direction::XKB_KEY_UP,
            KeyState::Pressed | KeyState::Repeated => xkb::xkb_key_direction::XKB_KEY_DOWN,
        };

        unsafe {
            (handle.xkb_state_update_key)(xkb_state.as_ptr(), keycode, direction);
        }

        let sym = unsafe { (handle.xkb_state_key_get_one_sym)(xkb_state.as_ptr(), keycode) };
        if let Some(text) = key_utf8(handle, xkb_state, keycode)
            && !text.is_empty()
            && !text.chars().all(char::is_control)
        {
            return Some(Key::Character(text));
        }

        Some(map_keysym(sym))
    }
}

impl Drop for XkbState {
    fn drop(&mut self) {
        let Some(handle) = xkb::xkbcommon_option() else {
            return;
        };
        if let Some(state) = self.state.take() {
            unsafe { (handle.xkb_state_unref)(state.as_ptr()) };
        }
        if let Some(keymap) = self.keymap.take() {
            unsafe { (handle.xkb_keymap_unref)(keymap.as_ptr()) };
        }
        unsafe { (handle.xkb_context_unref)(self.context.as_ptr()) };
    }
}

fn key_utf8(
    handle: &xkb::XkbCommon,
    state: NonNull<xkb::xkb_state>,
    keycode: u32,
) -> Option<String> {
    const STACK_BUFFER_SIZE: usize = 64;

    let len = unsafe {
        (handle.xkb_state_key_get_utf8)(state.as_ptr(), keycode, std::ptr::null_mut(), 0)
    };
    if len <= 0 {
        return None;
    }

    let required_len = usize::try_from(len).ok()?;
    if required_len < STACK_BUFFER_SIZE {
        let mut buffer = [0_u8; STACK_BUFFER_SIZE];
        let written = unsafe {
            (handle.xkb_state_key_get_utf8)(
                state.as_ptr(),
                keycode,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
            )
        };
        return decode_utf8(&buffer, written);
    }

    let mut buffer = vec![0_u8; required_len + 1];
    let written = unsafe {
        (handle.xkb_state_key_get_utf8)(
            state.as_ptr(),
            keycode,
            buffer.as_mut_ptr().cast(),
            buffer.len(),
        )
    };
    decode_utf8(&buffer, written)
}

fn decode_utf8(buffer: &[u8], written: i32) -> Option<String> {
    if written <= 0 {
        return None;
    }

    let written = usize::try_from(written).ok()?;
    let text = std::str::from_utf8(&buffer[..written]).ok()?;
    Some(String::from(text))
}

fn mapped_keymap_len(bytes: &[u8]) -> Option<usize> {
    bytes
        .iter()
        .rposition(|byte| *byte != 0)
        .map(|index| index + 1)
}

fn map_keysym(sym: u32) -> Key {
    use xkb::keysyms;

    match sym {
        keysyms::Escape => Key::Escape,
        keysyms::Return => Key::Enter,
        keysyms::Tab => Key::Tab,
        keysyms::BackSpace => Key::Backspace,
        keysyms::space => Key::Space,
        keysyms::Insert => Key::Insert,
        keysyms::Delete => Key::Delete,
        keysyms::Home => Key::Home,
        keysyms::End => Key::End,
        keysyms::Page_Up => Key::PageUp,
        keysyms::Page_Down => Key::PageDown,
        keysyms::Up => Key::ArrowUp,
        keysyms::Down => Key::ArrowDown,
        keysyms::Left => Key::ArrowLeft,
        keysyms::Right => Key::ArrowRight,
        keysyms::Shift_L | keysyms::Shift_R => Key::Shift,
        keysyms::Control_L | keysyms::Control_R => Key::Control,
        keysyms::Alt_L | keysyms::Alt_R => Key::Alt,
        keysyms::Meta_L | keysyms::Meta_R | keysyms::Super_L | keysyms::Super_R => Key::Meta,
        keysyms::Caps_Lock => Key::CapsLock,
        keysyms::Num_Lock => Key::NumLock,
        keysyms::Scroll_Lock => Key::ScrollLock,
        keysyms::Pause => Key::Pause,
        keysyms::Print => Key::PrintScreen,
        keysyms::Menu => Key::ContextMenu,
        keysyms::F1 => Key::F1,
        keysyms::F2 => Key::F2,
        keysyms::F3 => Key::F3,
        keysyms::F4 => Key::F4,
        keysyms::F5 => Key::F5,
        keysyms::F6 => Key::F6,
        keysyms::F7 => Key::F7,
        keysyms::F8 => Key::F8,
        keysyms::F9 => Key::F9,
        keysyms::F10 => Key::F10,
        keysyms::F11 => Key::F11,
        keysyms::F12 => Key::F12,
        keysyms::F13 => Key::F13,
        keysyms::F14 => Key::F14,
        keysyms::F15 => Key::F15,
        keysyms::F16 => Key::F16,
        keysyms::F17 => Key::F17,
        keysyms::F18 => Key::F18,
        keysyms::F19 => Key::F19,
        keysyms::F20 => Key::F20,
        keysyms::F21 => Key::F21,
        keysyms::F22 => Key::F22,
        keysyms::F23 => Key::F23,
        keysyms::F24 => Key::F24,
        _ => Key::Unknown,
    }
}
