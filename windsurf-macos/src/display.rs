use core::ptr;

use crate::error::{ConnectError, PumpError};
use crate::state::{SharedDisplayRef, SharedState};
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSEvent, NSEventMask};
use objc2_foundation::{NSDate, NSDefaultRunLoopMode};
use windsurf_core::{Event, EventQueue};

#[derive(Clone)]
pub struct Display {
    pub(crate) shared: SharedDisplayRef,
}

pub struct RawDisplay<'a> {
    pub app: &'a NSApplication,
}

impl Display {
    pub fn connect() -> Result<Self, ConnectError> {
        let mtm = MainThreadMarker::new().ok_or(ConnectError::NotMainThread)?;
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
        app.finishLaunching();

        Ok(Self {
            shared: std::rc::Rc::new(std::cell::RefCell::new(SharedState {
                app,
                next_window_id: std::cell::Cell::new(1),
                pending_events: std::collections::VecDeque::new(),
                windows: std::collections::BTreeMap::new(),
            })),
        })
    }

    pub fn pump(&self, queue: &mut EventQueue) -> Result<(), PumpError> {
        let _mtm = MainThreadMarker::new().ok_or(PumpError::NotMainThread)?;
        let mut shared = self.shared.borrow_mut();
        let stop_at = NSDate::distantPast();
        while let Some(event) = next_event(&shared.app, &stop_at) {
            shared.app.sendEvent(&event);
        }
        shared.app.updateWindows();

        let mut dead_windows = Vec::new();
        let mut emitted = Vec::new();
        for (&id, state) in &mut shared.windows {
            let Some(inner) = state.inner.upgrade() else {
                dead_windows.push(id);
                continue;
            };

            crate::window::sync_metal_layer(&inner);

            if !inner.window.isVisible() && !state.close_requested {
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
        }
        for event in emitted {
            shared.push(event);
        }

        while let Some(event) = shared.pending_events.pop_front() {
            queue.push(event);
        }

        Ok(())
    }

    pub fn raw(&self) -> RawDisplay<'_> {
        let shared = self.shared.borrow();
        let app = unsafe { &*ptr::from_ref(&*shared.app) };
        RawDisplay { app }
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
