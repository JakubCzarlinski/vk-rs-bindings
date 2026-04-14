use crate::error::ConnectError;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};

pub(crate) fn connect_application() -> Result<Retained<NSApplication>, ConnectError> {
    let mtm = MainThreadMarker::new().ok_or(ConnectError::NotMainThread)?;
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);
    app.finishLaunching();
    Ok(app)
}
