use wayland_client::protocol::{wl_keyboard, wl_pointer, wl_seat};
use wayland_client::{Connection, Dispatch, QueueHandle};
use windsurf_core::{ButtonState, Event, KeyCode, KeyState, PointerButton};
use windsurf_extra::{CursorEvent, ExtraEvent};

use crate::state::State;
use crate::util::unpack_enum;

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        state: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _data: &(),
        _conn: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities { capabilities } = event {
            let Some(capabilities) = unpack_enum(capabilities) else {
                return;
            };

            if capabilities.contains(wl_seat::Capability::Pointer) && state.pointer.is_none() {
                state.pointer = Some(seat.get_pointer(qh, ()));
                if let (Some(manager), Some(pointer)) =
                    (state.cursor_shape_manager.as_ref(), state.pointer.as_ref())
                {
                    state.cursor_shape_device = Some(manager.get_pointer(pointer, qh, ()));
                }
            }
            if !capabilities.contains(wl_seat::Capability::Pointer) {
                if let Some(device) = state.cursor_shape_device.take() {
                    device.destroy();
                }
                state.pointer = None;
                state.pointer_focus = None;
                state.pointer_enter_serial = None;
            }
            if capabilities.contains(wl_seat::Capability::Keyboard) && state.keyboard.is_none() {
                state.keyboard = Some(seat.get_keyboard(qh, ()));
            }
            if !capabilities.contains(wl_seat::Capability::Keyboard) {
                state.keyboard = None;
                state.keyboard_focus = None;
            }
        }
    }
}

impl Dispatch<wl_pointer::WlPointer, ()> for State {
    fn event(
        state: &mut Self,
        _pointer: &wl_pointer::WlPointer,
        event: wl_pointer::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_pointer::Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
                ..
            } => {
                if let Some(window) = state.window_for_surface(&surface) {
                    state.pointer_enter_serial = Some(serial);
                    state.pointer_focus = Some(window);
                    state.push(Event::PointerEntered { id: window });
                    state.push(Event::PointerMoved {
                        id: window,
                        x: surface_x,
                        y: surface_y,
                    });
                    state.push_extra(ExtraEvent::Cursor(CursorEvent::Moved {
                        id: window,
                        x: surface_x,
                        y: surface_y,
                    }));
                    state.apply_cursor(window);
                }
            }
            wl_pointer::Event::Leave { surface, .. } => {
                if let Some(window) = state.window_for_surface(&surface) {
                    state.pointer_focus = None;
                    state.pointer_enter_serial = None;
                    state.push(Event::PointerLeft { id: window });
                }
            }
            wl_pointer::Event::Motion {
                surface_x,
                surface_y,
                ..
            } => {
                if let Some(window) = state.pointer_focus {
                    state.push(Event::PointerMoved {
                        id: window,
                        x: surface_x,
                        y: surface_y,
                    });
                    state.push_extra(ExtraEvent::Cursor(CursorEvent::Moved {
                        id: window,
                        x: surface_x,
                        y: surface_y,
                    }));
                }
            }
            wl_pointer::Event::Button {
                button,
                state: button_state,
                ..
            } => {
                if let Some(window) = state.pointer_focus
                    && let Some(button_state) = unpack_enum(button_state).map(map_button_state)
                {
                    state.push(Event::PointerButton {
                        id: window,
                        button: map_pointer_button(button),
                        state: button_state,
                    });
                }
            }
            wl_pointer::Event::Axis { axis, value, .. } => {
                if let Some(window) = state.pointer_focus {
                    let Some(axis) = unpack_enum(axis) else {
                        return;
                    };
                    let (dx, dy) = match axis {
                        wl_pointer::Axis::VerticalScroll => (0.0, value),
                        wl_pointer::Axis::HorizontalScroll => (value, 0.0),
                        _ => (0.0, 0.0),
                    };
                    state.push(Event::PointerScroll { id: window, dx, dy });
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        state: &mut Self,
        _keyboard: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            wl_keyboard::Event::Enter { surface, .. } => {
                if let Some(window) = state.window_for_surface(&surface) {
                    state.keyboard_focus = Some(window);
                    state.push(Event::KeyboardFocusIn { id: window });
                }
            }
            wl_keyboard::Event::Leave { surface, .. } => {
                if let Some(window) = state.window_for_surface(&surface) {
                    state.keyboard_focus = None;
                    state.push(Event::KeyboardFocusOut { id: window });
                }
            }
            wl_keyboard::Event::Keymap { format, fd, size } => {
                if let Some(format) = unpack_enum(format)
                    && matches!(format, wl_keyboard::KeymapFormat::XkbV1)
                    && let Some(xkb) = state.xkb.as_mut()
                {
                    xkb.load_keymap(fd, size as usize);
                }
            }
            wl_keyboard::Event::Modifiers {
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
                ..
            } => {
                if let Some(xkb) = state.xkb.as_mut() {
                    xkb.update_modifiers(mods_depressed, mods_latched, mods_locked, group);
                }
            }
            wl_keyboard::Event::Key {
                key,
                state: key_state,
                ..
            } => {
                let Some(window) = state.keyboard_focus else {
                    return;
                };

                let Some(key_state) = unpack_enum(key_state) else {
                    return;
                };
                let state_value = map_key_state(key_state);
                let translated = state
                    .xkb
                    .as_mut()
                    .and_then(|xkb| xkb.translate(key, state_value));
                let logical = translated
                    .as_ref()
                    .map_or(KeyCode::Unknown, |translation| translation.key);
                state.push(Event::Key {
                    id: window,
                    key: logical,
                    scancode: key,
                    state: state_value,
                });
                if let Some(text) = translated.and_then(|translation| translation.text) {
                    state.push(Event::TextInput { id: window, text });
                }
            }
            _ => {}
        }
    }
}

fn map_button_state(state: wl_pointer::ButtonState) -> ButtonState {
    match state {
        wl_pointer::ButtonState::Pressed => ButtonState::Pressed,
        _ => ButtonState::Released,
    }
}

fn map_key_state(state: wl_keyboard::KeyState) -> KeyState {
    match state {
        wl_keyboard::KeyState::Pressed => KeyState::Pressed,
        _ => KeyState::Released,
    }
}

pub(crate) fn map_pointer_button(button: u32) -> PointerButton {
    match button {
        0x110 => PointerButton::Left,
        0x111 => PointerButton::Right,
        0x112 => PointerButton::Middle,
        0x116 => PointerButton::Back,
        0x115 => PointerButton::Forward,
        other => PointerButton::Other(other as u16),
    }
}

#[cfg(test)]
mod tests {
    use windsurf_core::PointerButton;

    use super::map_pointer_button;

    #[test]
    fn maps_common_linux_pointer_buttons() {
        assert_eq!(map_pointer_button(0x110), PointerButton::Left);
        assert_eq!(map_pointer_button(0x111), PointerButton::Right);
        assert_eq!(map_pointer_button(0x112), PointerButton::Middle);
    }
}
