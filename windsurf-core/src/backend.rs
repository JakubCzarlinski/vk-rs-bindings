use crate::{
    CursorMode, CursorSource, DragSource, Event, FeatureKind, FeatureSet, ImeState,
    UnsupportedFeature, WindowAttributes, WindowHandle,
};
use core::time::Duration;

/// Backend contract used by the frontend `EventLoop`/`Window` wrappers.
///
/// Implementations are expected to use static dispatch (no trait objects) and
/// push window identity via the transport tuple `(Option<WindowHandle>, Event)`.
pub trait LoopBackend: Sized {
    type ConnectError;
    type PollError;
    type WindowError;
    type BackendWindow;

    fn connect() -> Result<Self, Self::ConnectError>;

    fn poll_event(&mut self) -> Result<Option<(Option<WindowHandle>, Event)>, Self::PollError>;

    fn wait_event(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<Option<(Option<WindowHandle>, Event)>, Self::PollError>;

    fn create_window(
        &mut self,
        attrs: WindowAttributes,
    ) -> Result<(WindowHandle, Self::BackendWindow), Self::WindowError>;

    fn destroy_window(&mut self, handle: WindowHandle, window: &mut Self::BackendWindow);

    fn set_title(&mut self, handle: WindowHandle, window: &Self::BackendWindow, title: &str);

    fn request_redraw(&mut self, handle: WindowHandle, window: &Self::BackendWindow);

    fn inner_size(&self, handle: WindowHandle, window: &Self::BackendWindow) -> (u32, u32);

    fn scale_factor(&self, handle: WindowHandle, window: &Self::BackendWindow) -> f64;

    fn supported_features(&self) -> FeatureSet {
        FeatureSet::empty()
    }

    fn set_ime_state(
        &mut self,
        _window: WindowHandle,
        _state: &ImeState,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Ime))
    }

    fn set_cursor(
        &mut self,
        _window: WindowHandle,
        _source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn set_cursor_mode(
        &mut self,
        _window: WindowHandle,
        _mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::Cursor))
    }

    fn start_drag(
        &mut self,
        _window: WindowHandle,
        _source: DragSource,
    ) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(FeatureKind::DragDropSource))
    }
}
