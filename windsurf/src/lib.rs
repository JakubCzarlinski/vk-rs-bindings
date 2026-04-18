#![doc = include_str!("../README.md")]

use alloc::rc::Rc;
use core::cell::RefCell;
use core::time::Duration;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle};
pub use windsurf_core::*;

extern crate alloc;

/// Generic single-loop frontend over a statically-dispatched backend.
pub struct GenericEventLoop<B: Backend> {
    backend: Rc<RefCell<B>>,
}

/// Generic window object backed by a concrete backend window type.
pub struct GenericWindow<B: Backend> {
    handle: WindowHandle,
    backend: Rc<RefCell<B>>,
    backend_window: Option<B::BackendWindow>,
}

impl<B: Backend> GenericEventLoop<B> {
    pub fn new() -> Result<Self, B::ConnectError> {
        Self::connect()
    }

    pub fn connect() -> Result<Self, B::ConnectError> {
        let backend = B::connect()?;
        Ok(Self {
            backend: Rc::new(RefCell::new(backend)),
        })
    }

    pub fn poll_event(&mut self) -> Result<Option<ScopedEvent>, B::PollError> {
        self.backend.borrow_mut().poll_event()
    }

    pub fn wait_event(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Option<ScopedEvent>, B::PollError> {
        self.backend.borrow_mut().wait_event(timeout)
    }
}

impl<B: Backend> Features for GenericEventLoop<B> {
    fn supported_features(&self) -> FeatureSet {
        self.backend.borrow().supported_features()
    }

    fn set_ime_state(
        &self,
        window: WindowHandle,
        state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        self.backend.borrow_mut().set_ime_state(window, state)
    }

    fn set_cursor(
        &self,
        window: WindowHandle,
        source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        self.backend.borrow_mut().set_cursor(window, source)
    }

    fn set_cursor_mode(
        &self,
        window: WindowHandle,
        mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        self.backend.borrow_mut().set_cursor_mode(window, mode)
    }

    fn start_drag(
        &self,
        window: WindowHandle,
        source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        self.backend.borrow_mut().start_drag(window, source)
    }
}

impl<B: Backend> GenericWindow<B> {
    pub fn new(
        event_loop: &GenericEventLoop<B>,
        attrs: WindowAttributes,
    ) -> Result<Self, B::WindowError> {
        let (handle, backend_window) = event_loop.backend.borrow_mut().create_window(attrs)?;
        Ok(Self {
            handle,
            backend: Rc::clone(&event_loop.backend),
            backend_window: Some(backend_window),
        })
    }

    pub const fn handle(&self) -> WindowHandle {
        self.handle
    }

    pub fn set_title(&self, title: &str) {
        let Some(window) = self.backend_window.as_ref() else {
            return;
        };
        self.backend
            .borrow_mut()
            .set_title(self.handle, window, title);
    }

    pub fn request_redraw(&self) {
        let Some(window) = self.backend_window.as_ref() else {
            return;
        };
        self.backend
            .borrow_mut()
            .request_redraw(self.handle, window);
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let Some(window) = self.backend_window.as_ref() else {
            return (0, 0);
        };
        self.backend.borrow().inner_size(self.handle, window)
    }

    pub fn scale_factor(&self) -> f64 {
        let Some(window) = self.backend_window.as_ref() else {
            return 1.0;
        };
        self.backend.borrow().scale_factor(self.handle, window)
    }
}

impl<B> HasDisplayHandle for GenericWindow<B>
where
    B: Backend,
    B::BackendWindow: HasDisplayHandle,
{
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let Some(window) = self.backend_window.as_ref() else {
            return Err(HandleError::Unavailable);
        };
        window.display_handle()
    }
}

impl<B> HasWindowHandle for GenericWindow<B>
where
    B: Backend,
    B::BackendWindow: HasWindowHandle,
{
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        let Some(window) = self.backend_window.as_ref() else {
            return Err(HandleError::Unavailable);
        };
        window.window_handle()
    }
}

impl<B: Backend> Drop for GenericWindow<B> {
    fn drop(&mut self) {
        let Some(mut backend_window) = self.backend_window.take() else {
            return;
        };

        if let Ok(mut backend) = self.backend.try_borrow_mut() {
            backend.destroy_window(self.handle, &mut backend_window);
        }
    }
}

#[cfg(all(feature = "wayland", target_os = "linux"))]
pub type DefaultBackend = windsurf_wayland::WaylandBackend;

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
pub type DefaultBackend = UnsupportedBackend;

pub type EventLoop = GenericEventLoop<DefaultBackend>;
pub type Window = GenericWindow<DefaultBackend>;

pub type ConnectError = <DefaultBackend as Backend>::ConnectError;
pub type PollError = <DefaultBackend as Backend>::PollError;
pub type WindowError = <DefaultBackend as Backend>::WindowError;

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsupportedError {
    UnsupportedTarget,
}

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
impl std::fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("no default windsurf backend is available for this target/feature set")
    }
}

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
impl core::error::Error for UnsupportedError {}

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
#[derive(Debug, Default)]
pub struct UnsupportedBackend;

#[cfg(not(all(feature = "wayland", target_os = "linux")))]
impl Backend for UnsupportedBackend {
    type ConnectError = UnsupportedError;
    type PollError = UnsupportedError;
    type WindowError = UnsupportedError;
    type BackendWindow = ();

    fn connect() -> Result<Self, Self::ConnectError> {
        Err(UnsupportedError::UnsupportedTarget)
    }

    fn poll_event(&mut self) -> Result<Option<ScopedEvent>, Self::PollError> {
        Err(UnsupportedError::UnsupportedTarget)
    }

    fn wait_event(
        &mut self,
        _timeout: Option<Duration>,
    ) -> Result<Option<ScopedEvent>, Self::PollError> {
        Err(UnsupportedError::UnsupportedTarget)
    }

    fn create_window(
        &mut self,
        _attrs: WindowAttributes,
    ) -> Result<(WindowHandle, Self::BackendWindow), Self::WindowError> {
        Err(UnsupportedError::UnsupportedTarget)
    }

    fn destroy_window(&mut self, _handle: WindowHandle, _window: &mut Self::BackendWindow) {}

    fn set_title(&mut self, _handle: WindowHandle, _window: &Self::BackendWindow, _title: &str) {}

    fn request_redraw(&mut self, _handle: WindowHandle, _window: &Self::BackendWindow) {}

    fn inner_size(&self, _handle: WindowHandle, _window: &Self::BackendWindow) -> (u32, u32) {
        (0, 0)
    }

    fn scale_factor(&self, _handle: WindowHandle, _window: &Self::BackendWindow) -> f64 {
        1.0
    }
}

#[cfg(feature = "wayland")]
pub mod wayland {
    pub use windsurf_wayland::*;
}

#[cfg(feature = "macos")]
pub mod macos {
    pub use windsurf_macos::*;
}
