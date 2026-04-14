use crate::display::Display;
use crate::window::Window;
use core::ptr::NonNull;
use raw_window_handle::{
    AppKitWindowHandle, DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};

impl HasDisplayHandle for Display {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::appkit())
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(DisplayHandle::appkit())
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let raw = AppKitWindowHandle::new(NonNull::from(&*self.inner.view).cast());
        Ok(unsafe { WindowHandle::borrow_raw(raw.into()) })
    }
}
