use crate::cursor::apply_cursor_for_window;
use crate::keymap::{key_code_for_event, modifier_key_for_scancode, text_for_event};
use crate::state::SharedState;
use objc2_app_kit::{NSApplication, NSEvent, NSEventType};
use windsurf_core::{ButtonState, CursorEvent, Event, KeyState, PointerButton, WindowId};

pub(crate) fn process_input_event(shared: &mut SharedState, event: &NSEvent) {
    let event_type = event.r#type();
    let window_id = window_id_for_window_number(shared, event.windowNumber());

    match event_type {
        NSEventType::MouseMoved
        | NSEventType::LeftMouseDragged
        | NSEventType::RightMouseDragged
        | NSEventType::OtherMouseDragged => {
            let Some(id) = window_id else {
                return;
            };
            set_pointer_focus(shared, Some(id));
            let position = logical_position_for_event(shared, id, event);
            shared.push(Event::PointerMoved {
                id,
                x: position.x,
                y: position.y,
            });
            shared.push(Event::Cursor(CursorEvent::Moved {
                id,
                x: position.x,
                y: position.y,
            }));
            apply_cursor_for_window(shared, id);
        }
        NSEventType::LeftMouseDown | NSEventType::RightMouseDown | NSEventType::OtherMouseDown => {
            let Some(id) = window_id else {
                return;
            };
            set_pointer_focus(shared, Some(id));
            let button = map_pointer_button(event_type, event.buttonNumber());
            shared.push(Event::PointerButton {
                id,
                button,
                state: ButtonState::Pressed,
            });
            apply_cursor_for_window(shared, id);
        }
        NSEventType::LeftMouseUp | NSEventType::RightMouseUp | NSEventType::OtherMouseUp => {
            let Some(id) = window_id else {
                return;
            };
            let button = map_pointer_button(event_type, event.buttonNumber());
            shared.push(Event::PointerButton {
                id,
                button,
                state: ButtonState::Released,
            });
        }
        NSEventType::ScrollWheel => {
            let Some(id) = window_id else {
                return;
            };
            shared.push(Event::PointerScroll {
                id,
                dx: event.scrollingDeltaX().into(),
                dy: event.scrollingDeltaY().into(),
            });
        }
        NSEventType::MouseEntered => {
            if let Some(id) = window_id {
                set_pointer_focus(shared, Some(id));
                apply_cursor_for_window(shared, id);
            }
        }
        NSEventType::MouseExited => {
            set_pointer_focus(shared, None);
        }
        NSEventType::KeyDown => {
            let Some(id) = window_id.or(shared.keyboard_focus) else {
                return;
            };
            let state = if event.isARepeat() {
                KeyState::Repeated
            } else {
                KeyState::Pressed
            };
            shared.push(Event::Key {
                id,
                key: key_code_for_event(event),
                scancode: event.keyCode(),
                state,
            });
            if let Some(text) = text_for_event(event) {
                shared.push(Event::TextInput { id, text });
            }
        }
        NSEventType::KeyUp => {
            let Some(id) = window_id.or(shared.keyboard_focus) else {
                return;
            };
            shared.push(Event::Key {
                id,
                key: key_code_for_event(event),
                scancode: event.keyCode(),
                state: KeyState::Released,
            });
        }
        NSEventType::FlagsChanged => {
            let Some(id) = window_id.or(shared.keyboard_focus) else {
                return;
            };
            let scancode = event.keyCode();
            if let Some((key, flag)) = modifier_key_for_scancode(scancode) {
                let state = if event.modifierFlags().contains(flag) {
                    KeyState::Pressed
                } else {
                    KeyState::Released
                };
                shared.push(Event::Key {
                    id,
                    key,
                    scancode,
                    state,
                });
            }
        }
        _ => {}
    }
}

pub(crate) fn sync_keyboard_focus(shared: &mut SharedState, app: &NSApplication) {
    let key_window = app.keyWindow();
    let new_focus = key_window
        .as_ref()
        .and_then(|window| window_id_for_window_number(shared, window.windowNumber()));

    if shared.keyboard_focus == new_focus {
        return;
    }

    if let Some(old) = shared.keyboard_focus {
        shared.push(Event::KeyboardFocusOut { id: old });
    }
    if let Some(new_id) = new_focus {
        shared.push(Event::KeyboardFocusIn { id: new_id });
    }
    shared.keyboard_focus = new_focus;
}

fn window_id_for_window_number(shared: &SharedState, window_number: isize) -> Option<WindowId> {
    if window_number == 0 {
        return None;
    }

    shared.windows.iter().find_map(|(&id, state)| {
        let inner = state.inner.upgrade()?;
        (inner.window.windowNumber() == window_number).then_some(id)
    })
}

fn logical_position_for_event(
    shared: &SharedState,
    id: WindowId,
    event: &NSEvent,
) -> windsurf_core::LogicalPosition {
    let point = event.locationInWindow();
    let y = shared
        .windows
        .get(&id)
        .map_or(point.y, |window| f64::from(window.size.height) - point.y);
    windsurf_core::LogicalPosition::new(point.x, y)
}

fn set_pointer_focus(shared: &mut SharedState, new_focus: Option<WindowId>) {
    if shared.pointer_focus == new_focus {
        return;
    }

    if let Some(old) = shared.pointer_focus.take() {
        shared.push(Event::PointerLeft { id: old });
    }
    if let Some(new_id) = new_focus {
        shared.pointer_focus = Some(new_id);
        shared.push(Event::PointerEntered { id: new_id });
    }
}

pub(crate) fn map_pointer_button(event_type: NSEventType, button_number: isize) -> PointerButton {
    match event_type {
        NSEventType::LeftMouseDown | NSEventType::LeftMouseUp => PointerButton::Left,
        NSEventType::RightMouseDown | NSEventType::RightMouseUp => PointerButton::Right,
        NSEventType::OtherMouseDown | NSEventType::OtherMouseUp => match button_number {
            2 => PointerButton::Middle,
            3 => PointerButton::Back,
            4 => PointerButton::Forward,
            other => PointerButton::Other(other as u16),
        },
        _ => PointerButton::Other(button_number as u16),
    }
}

#[cfg(test)]
mod tests {
    use objc2_app_kit::NSEventType;
    use windsurf_core::PointerButton;

    use super::map_pointer_button;

    #[test]
    fn maps_common_buttons() {
        assert_eq!(
            map_pointer_button(NSEventType::LeftMouseDown, 0),
            PointerButton::Left
        );
        assert_eq!(
            map_pointer_button(NSEventType::RightMouseUp, 1),
            PointerButton::Right
        );
        assert_eq!(
            map_pointer_button(NSEventType::OtherMouseDown, 2),
            PointerButton::Middle
        );
    }
}
