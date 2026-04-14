#[cfg(any(target_os = "linux", target_os = "macos"))]
mod basic_window_app;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod basic_window_vulkan;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    basic_window_app::main()
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn main() {
    panic!("The basic_window example currently targets Linux Wayland and macOS AppKit");
}
