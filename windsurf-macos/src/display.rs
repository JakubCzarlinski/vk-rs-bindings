use crate::app::connect_application;
use crate::cursor::{apply_cursor_for_window, icon_from_source};
use crate::error::{ConnectError, PumpError};
use crate::input::{process_input_event, sync_keyboard_focus};
use crate::state::{SharedDisplayRef, SharedState};
use alloc::collections::{BTreeMap, VecDeque};
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSEvent, NSEventMask};
use objc2_foundation::{NSDate, NSDefaultRunLoopMode};
use windsurf_core::{
    CursorEvent, CursorMode, CursorSource, DragSource, Event, EventQueue, FeatureKind, FeatureSet,
    Features, ImeState, UnsupportedFeature,
};

extern crate alloc;

/// Connection to the process-wide `AppKit` application and windsurf shared state.
#[derive(Clone)]
pub struct Display {
    pub(crate) app: Retained<NSApplication>,
    pub(crate) shared: SharedDisplayRef,
}

/// Borrowed raw `AppKit` display objects.
pub struct RawDisplay<'a> {
    /// Process-wide shared `NSApplication` instance.
    pub app: &'a NSApplication,
}

impl Display {
    /// Connect to `AppKit` and initialize backend state.
    ///
    /// This must be called from the macOS main thread.
    pub fn connect() -> Result<Self, ConnectError> {
        let app = connect_application()?;
        let shared = Rc::new(RefCell::new(SharedState {
            pending_events: VecDeque::new(),
            keyboard_focus: None,
            pointer_focus: None,
            cursor_hidden: false,
            windows: BTreeMap::new(),
        }));

        Ok(Self { app, shared })
    }

    /// Pump pending `AppKit` events into `queue` without blocking.
    ///
    /// This must be called from the macOS main thread.
    pub fn pump(&self, queue: &mut EventQueue) -> Result<(), PumpError> {
        self.pump_appkit()?;

        let mut shared = self.shared.borrow_mut();
        while let Some(event) = shared.pending_events.pop_front() {
            queue.push(event);
        }
        Ok(())
    }

    /// Borrow raw `AppKit` handles used by this display.
    pub fn raw(&self) -> RawDisplay<'_> {
        RawDisplay { app: &self.app }
    }

    fn pump_appkit(&self) -> Result<(), PumpError> {
        let _mtm = MainThreadMarker::new().ok_or(PumpError::NotMainThread)?;
        let app = self.app.clone();
        let stop_at = NSDate::distantPast();

        while let Some(event) = next_event(&app, &stop_at) {
            {
                let mut shared = self.shared.borrow_mut();
                process_input_event(&mut shared, &event);
            }
            app.sendEvent(&event);
        }
        app.updateWindows();

        let mut shared = self.shared.borrow_mut();
        sync_keyboard_focus(&mut shared, &app);

        let mut dead_windows = Vec::new();
        let mut emitted = Vec::new();
        for (&id, state) in &mut shared.windows {
            let Some(inner) = state.inner.upgrade() else {
                dead_windows.push(id);
                continue;
            };

            crate::window::sync_metal_layer(&inner);

            if !inner.window.isVisible() && !inner.window.isMiniaturized() && !state.close_requested
            {
                state.close_requested = true;
                emitted.push(Event::CloseRequested { id });
            }

            let size = crate::window::logical_size(&inner.window);
            if size != state.size {
                state.size = size;
                state.needs_redraw = true;
                emitted.push(Event::WindowResized {
                    id,
                    width: size.width,
                    height: size.height,
                });
            }

            let scale_factor = inner.window.backingScaleFactor();
            if (scale_factor - state.scale_factor).abs() > f64::EPSILON {
                state.scale_factor = scale_factor;
                state.needs_redraw = true;
                emitted.push(Event::ScaleFactorChanged {
                    id,
                    factor: scale_factor,
                });
            }

            if state.needs_redraw {
                state.needs_redraw = false;
                emitted.push(Event::RedrawRequested { id });
            }
        }

        for id in dead_windows {
            shared.windows.remove(&id);
            if shared.pointer_focus == Some(id) {
                shared.pointer_focus = None;
                emitted.push(Event::PointerLeft { id });
            }
            if shared.keyboard_focus == Some(id) {
                shared.keyboard_focus = None;
                emitted.push(Event::KeyboardFocusOut { id });
            }
        }
        for event in emitted {
            shared.push(event);
        }

        Ok(())
    }
}

impl Features for Display {
    fn supported_features(&self) -> FeatureSet {
        FeatureSet::IME
            .with(FeatureSet::CURSOR)
            .with(FeatureSet::DRAG_DROP_DESTINATION)
    }

    fn set_ime_state(
        &self,
        window: windsurf_core::WindowId,
        state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        let mut shared = self.shared.borrow_mut();
        if let Some(window_state) = shared.windows.get_mut(&window)
            && window_state.ime_enabled != state.enabled
        {
            window_state.ime_enabled = state.enabled;
            let event = if state.enabled {
                Event::ImeEnabled { id: window }
            } else {
                Event::ImeDisabled { id: window }
            };
            shared.push(event);
        }
        Ok(())
    }

    fn set_cursor(
        &self,
        window: windsurf_core::WindowId,
        source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        let mut shared = self.shared.borrow_mut();
        let mut emitted = None;
        if let Some(window_state) = shared.windows.get_mut(&window) {
            window_state.cursor_icon = icon_from_source(source);
            emitted = Some(Event::Cursor(CursorEvent::VisibilityChanged {
                id: window,
                visible: window_state.cursor_visible,
            }));
        }
        if let Some(event) = emitted {
            shared.push(event);
        }
        apply_cursor_for_window(&mut shared, window);
        Ok(())
    }

    fn set_cursor_mode(
        &self,
        window: windsurf_core::WindowId,
        mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        let mut shared = self.shared.borrow_mut();
        let mut emitted = Vec::new();
        if let Some(window_state) = shared.windows.get_mut(&window)
            && window_state.cursor_mode != mode
        {
            window_state.cursor_mode = mode;
            emitted.push(Event::Cursor(CursorEvent::ModeChanged { id: window, mode }));

            let visible = !matches!(mode, CursorMode::Hidden);
            if window_state.cursor_visible != visible {
                window_state.cursor_visible = visible;
                emitted.push(Event::Cursor(CursorEvent::VisibilityChanged {
                    id: window,
                    visible,
                }));
            }
        }
        for event in emitted {
            shared.push(event);
        }
        apply_cursor_for_window(&mut shared, window);
        Ok(())
    }

    fn start_drag(
        &self,
        _window: windsurf_core::WindowId,
        _source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}

fn next_event(app: &NSApplication, stop_at: &NSDate) -> Option<Retained<NSEvent>> {
    app.nextEventMatchingMask_untilDate_inMode_dequeue(
        NSEventMask::Any,
        Some(stop_at),
        unsafe { NSDefaultRunLoopMode },
        true,
    )
}
