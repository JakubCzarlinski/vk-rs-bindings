use crate::state::SharedState;
use objc2::rc::Retained;
use objc2_app_kit::NSCursor;
use windsurf_core::WindowId;
use windsurf_extra::{CursorIcon, CursorMode, CursorSource};

extern crate alloc;

pub(crate) fn icon_from_source(source: &CursorSource) -> CursorIcon {
    match source {
        CursorSource::Icon(icon) => *icon,
        CursorSource::Rgba { .. } => CursorIcon::Default,
    }
}

pub(crate) fn apply_cursor_for_window(shared: &mut SharedState, id: WindowId) {
    let Some(window_state) = shared.windows.get(&id) else {
        return;
    };

    let window_is_focused = matches!(shared.pointer_focus, Some(focused) if focused == id);
    let should_be_visible = window_state.cursor_visible
        && !matches!(window_state.cursor_mode, CursorMode::Hidden)
        && window_is_focused;

    if should_be_visible {
        if shared.cursor_hidden {
            NSCursor::unhide();
            shared.cursor_hidden = false;
        }

        if let Some(inner) = window_state.inner.upgrade() {
            cursor_from_icon(window_state.cursor_icon).set();
            inner.window.invalidateCursorRectsForView(&inner.view);
        }
        return;
    }

    if matches!(window_state.cursor_mode, CursorMode::Hidden) && window_is_focused {
        if !shared.cursor_hidden {
            NSCursor::hide();
            shared.cursor_hidden = true;
        }
        return;
    }

    if shared.cursor_hidden {
        NSCursor::unhide();
        shared.cursor_hidden = false;
    }
}

fn cursor_from_icon(icon: CursorIcon) -> Retained<NSCursor> {
    match icon {
        CursorIcon::Pointer => NSCursor::pointingHandCursor(),
        CursorIcon::Text => NSCursor::IBeamCursor(),
        CursorIcon::NotAllowed => NSCursor::operationNotAllowedCursor(),
        CursorIcon::Grab | CursorIcon::Move => NSCursor::openHandCursor(),
        CursorIcon::Grabbing => NSCursor::closedHandCursor(),
        CursorIcon::EwResize | CursorIcon::ColResize => NSCursor::resizeLeftRightCursor(),
        CursorIcon::NsResize | CursorIcon::RowResize => NSCursor::resizeUpDownCursor(),
        CursorIcon::Crosshair | CursorIcon::NeswResize | CursorIcon::NwseResize => {
            NSCursor::crosshairCursor()
        }
        CursorIcon::ZoomIn => NSCursor::zoomInCursor(),
        CursorIcon::ZoomOut => NSCursor::zoomOutCursor(),
        CursorIcon::Wait | CursorIcon::Progress | CursorIcon::Default => NSCursor::arrowCursor(),
        CursorIcon::Help => NSCursor::contextualMenuCursor(),
    }
}

#[cfg(test)]
mod tests {
    use windsurf_extra::{CursorIcon, CursorSource};

    use super::icon_from_source;

    #[test]
    fn maps_custom_rgba_to_default_icon() {
        let source = CursorSource::Rgba {
            width: 1,
            height: 1,
            hotspot_x: 0,
            hotspot_y: 0,
            pixels: super::alloc::vec::Vec::from([255, 255, 255, 255]),
        };
        assert_eq!(icon_from_source(&source), CursorIcon::Default);
    }
}
