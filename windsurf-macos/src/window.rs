use crate::display::Display;
use crate::drag::{WindowDragDelegate, attach_file_drop_delegate};
use crate::error::WindowError;
use crate::state::{SharedState, WindowInner, WindowState};
use alloc::rc::Rc;
use core::cell::RefCell;
use objc2::rc::Retained;
use objc2::{ClassType, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBackingStoreType, NSColor, NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use windsurf_core::{CursorIcon, CursorMode, Event, LogicalSize, WindowAttributes, WindowId};

extern crate alloc;

/// macOS AppKit-backed window.
pub struct Window {
    pub(crate) shared: Rc<RefCell<SharedState>>,
    pub(crate) inner: Rc<WindowInner>,
    _drag_delegate: Retained<WindowDragDelegate>,
    pub(crate) id: WindowId,
}

/// Borrowed raw `AppKit` window objects.
pub struct RawWindow<'a> {
    /// Backing `NSWindow`.
    pub window: &'a NSWindow,
    /// Content `NSView`.
    pub view: &'a NSView,
    /// Backing `CAMetalLayer` attached to `view`.
    pub layer: &'a CAMetalLayer,
}

impl Window {
    /// Create and show a new `AppKit` window.
    ///
    /// This must be called from the macOS main thread.
    pub fn new(display: &Display, attrs: &WindowAttributes) -> Result<Self, WindowError> {
        let mtm = MainThreadMarker::new().ok_or(WindowError::NotMainThread)?;
        let mut shared = display.shared.borrow_mut();

        let id = next_available_window_id(&shared).ok_or(WindowError::NoAvailableWindowId)?;

        let rect = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(f64::from(attrs.size.width), f64::from(attrs.size.height)),
        );
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                rect,
                window_style_mask(attrs.decorations),
                NSBackingStoreType::Buffered,
                false,
            )
        };
        unsafe { window.setReleasedWhenClosed(false) };
        window.setTitle(&NSString::from_str(&attrs.title));
        window.setOpaque(!attrs.transparent);
        if attrs.transparent {
            window.setBackgroundColor(Some(&NSColor::clearColor()));
        }
        if let Some(min_size) = attrs.min_size {
            window.setContentMinSize(NSSize::new(
                f64::from(min_size.width),
                f64::from(min_size.height),
            ));
        }
        if let Some(max_size) = attrs.max_size {
            window.setContentMaxSize(NSSize::new(
                f64::from(max_size.width),
                f64::from(max_size.height),
            ));
        }

        let view = NSView::initWithFrame(NSView::alloc(mtm), rect);
        view.setAutoresizingMask(NSAutoresizingMaskOptions(
            NSAutoresizingMaskOptions::ViewWidthSizable.0
                | NSAutoresizingMaskOptions::ViewHeightSizable.0,
        ));
        view.setWantsLayer(true);

        let layer = CAMetalLayer::new();
        layer.setOpaque(!attrs.transparent);
        view.setLayer(Some(layer.as_super()));
        window.setContentView(Some(&view));
        window.setAcceptsMouseMovedEvents(true);

        let drag_delegate = attach_file_drop_delegate(mtm, &display.shared, &window, id);

        window.makeKeyAndOrderFront(None);
        #[allow(deprecated)]
        display.app.activateIgnoringOtherApps(true);

        let inner = Rc::new(WindowInner {
            window,
            view,
            layer,
        });
        sync_metal_layer(&inner);
        let size = logical_size(&inner.window);
        let scale_factor = inner.window.backingScaleFactor();

        shared.windows.insert(
            id,
            WindowState {
                inner: Rc::downgrade(&inner),
                size,
                scale_factor,
                needs_redraw: true,
                close_requested: false,
                ime_enabled: false,
                cursor_mode: CursorMode::Normal,
                cursor_visible: true,
                cursor_icon: CursorIcon::Default,
            },
        );
        shared.push(Event::WindowCreated { id });

        Ok(Self {
            shared: Rc::clone(&display.shared),
            inner,
            _drag_delegate: drag_delegate,
            id,
        })
    }

    /// Return this window's stable backend identifier.
    pub fn id(&self) -> WindowId {
        self.id
    }

    /// Update the title shown by the system window manager.
    pub fn set_title(&self, title: &str) {
        self.inner.window.setTitle(&NSString::from_str(title));
    }

    /// Return the current logical inner size.
    pub fn inner_size(&self) -> (u32, u32) {
        let size = logical_size(&self.inner.window);
        (size.width, size.height)
    }

    /// Return the current backing scale factor.
    pub fn scale_factor(&self) -> f64 {
        self.inner.window.backingScaleFactor()
    }

    /// Schedule a redraw event for this window on the next pump.
    pub fn request_redraw(&self) {
        if let Some(state) = self.shared.borrow_mut().windows.get_mut(&self.id)
            && !state.needs_redraw
        {
            state.needs_redraw = true;
        }
    }

    /// Borrow this window as `raw_window_handle::RawWindowHandle`.
    pub fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError> {
        self.window_handle().map(Into::into)
    }

    /// Borrow this window's display as `raw_window_handle::RawDisplayHandle`.
    pub fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError> {
        self.display_handle().map(Into::into)
    }

    /// Borrow raw `AppKit` objects used by this window.
    pub fn raw(&self) -> RawWindow<'_> {
        RawWindow {
            window: &self.inner.window,
            view: &self.inner.view,
            layer: &self.inner.layer,
        }
    }
}

fn next_available_window_id(shared: &SharedState) -> Option<WindowId> {
    (1_u8..=u8::MAX)
        .map(WindowId::new)
        .find(|id| !shared.windows.contains_key(id))
}

impl Drop for Window {
    fn drop(&mut self) {
        if MainThreadMarker::new().is_none() {
            return;
        }

        let mut shared = self.shared.borrow_mut();
        if shared.windows.remove(&self.id).is_some() {
            if shared.pointer_focus == Some(self.id) {
                shared.pointer_focus = None;
                shared.push(Event::PointerLeft { id: self.id });
            }
            if shared.keyboard_focus == Some(self.id) {
                shared.keyboard_focus = None;
                shared.push(Event::KeyboardFocusOut { id: self.id });
            }
            shared.push(Event::WindowDestroyed { id: self.id });
        }
        self.inner.window.setDelegate(None);
        if self.inner.window.isVisible() {
            self.inner.window.close();
        }
    }
}

pub(crate) fn logical_size(window: &NSWindow) -> LogicalSize {
    let rect = window.contentLayoutRect();
    LogicalSize {
        width: rect.size.width.max(0.0).round() as u32,
        height: rect.size.height.max(0.0).round() as u32,
    }
}

pub(crate) fn sync_metal_layer(inner: &WindowInner) {
    let frame = inner.view.frame();
    let backing = inner.view.convertRectToBacking(frame);
    inner.layer.setFrame(frame);
    inner
        .layer
        .setContentsScale(inner.window.backingScaleFactor());
    inner.layer.setDrawableSize(backing.size);
}

fn window_style_mask(decorations: bool) -> NSWindowStyleMask {
    if decorations {
        NSWindowStyleMask(
            NSWindowStyleMask::Titled.0
                | NSWindowStyleMask::Closable.0
                | NSWindowStyleMask::Miniaturizable.0
                | NSWindowStyleMask::Resizable.0,
        )
    } else {
        NSWindowStyleMask(NSWindowStyleMask::Borderless.0 | NSWindowStyleMask::Resizable.0)
    }
}
