use crate::display::Display;
use crate::error::WindowError;
use crate::state::{WindowInner, WindowState};
use alloc::rc::Rc;
use objc2::{ClassType, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBackingStoreType, NSColor, NSView, NSWindow, NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use windsurf_core::{Event, LogicalSize, WindowAttributes, WindowId};

extern crate alloc;

pub struct Window {
    pub(crate) shared: crate::state::SharedDisplayRef,
    pub(crate) inner: Rc<WindowInner>,
    pub(crate) id: WindowId,
}

pub struct RawWindow<'a> {
    pub window: &'a NSWindow,
    pub view: &'a NSView,
    pub layer: &'a CAMetalLayer,
}

impl Window {
    pub fn new(display: &Display, attrs: &WindowAttributes) -> Result<Self, WindowError> {
        let mtm = MainThreadMarker::new().ok_or(WindowError::NotMainThread)?;
        let mut shared = display.shared.borrow_mut();

        let id = WindowId::new(shared.next_window_id.get());
        shared.next_window_id.set(id.raw() + 1);

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
        window.makeKeyAndOrderFront(None);
        #[allow(deprecated)]
        shared.app.activateIgnoringOtherApps(true);

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
            },
        );
        shared.push(Event::WindowCreated { id });

        Ok(Self {
            shared: Rc::clone(&display.shared),
            inner,
            id,
        })
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn set_title(&self, title: &str) {
        self.inner.window.setTitle(&NSString::from_str(title));
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = logical_size(&self.inner.window);
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f64 {
        self.inner.window.backingScaleFactor()
    }

    pub fn request_redraw(&self) {
        if let Some(state) = self.shared.borrow_mut().windows.get_mut(&self.id)
            && !state.needs_redraw
        {
            state.needs_redraw = true;
        }
    }

    pub fn raw_window_handle(&self) -> Result<RawWindowHandle, HandleError> {
        self.window_handle().map(Into::into)
    }

    pub fn raw_display_handle(&self) -> Result<RawDisplayHandle, HandleError> {
        self.display_handle().map(Into::into)
    }

    pub fn raw(&self) -> RawWindow<'_> {
        RawWindow {
            window: &self.inner.window,
            view: &self.inner.view,
            layer: &self.inner.layer,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if MainThreadMarker::new().is_none() {
            return;
        }

        let mut shared = self.shared.borrow_mut();
        if shared.windows.remove(&self.id).is_some() {
            shared.push(Event::WindowDestroyed { id: self.id });
        }
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
