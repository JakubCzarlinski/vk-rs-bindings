#[cfg(target_os = "linux")]
use std::collections::BTreeMap;
#[cfg(target_os = "linux")]
use std::sync::mpsc::{self, Sender};
#[cfg(target_os = "linux")]
use std::thread;
#[cfg(target_os = "linux")]
use std::time::Duration;
#[cfg(target_os = "linux")]
use windsurf::{Event, EventLoop, LogicalSize, Window, WindowAttributes, WindowHandle};

#[cfg(target_os = "linux")]
enum WorkerMsg {
    Event(Event),
    Stop,
}

#[cfg(target_os = "linux")]
fn spawn_window_worker(id: WindowHandle) -> Sender<WorkerMsg> {
    let (tx, rx) = mpsc::channel::<WorkerMsg>();
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                WorkerMsg::Event(event) => println!("[worker {id:?}] {event:?}"),
                WorkerMsg::Stop => break,
            }
        }
    });
    tx
}

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut event_loop = EventLoop::connect()?;
    let left = Window::new(
        &event_loop,
        WindowAttributes {
            title: String::from("shared-state: left"),
            size: LogicalSize::new(400, 300),
            ..WindowAttributes::default()
        },
    )?;
    let right = Window::new(
        &event_loop,
        WindowAttributes {
            title: String::from("shared-state: right"),
            size: LogicalSize::new(300, 400),
            ..WindowAttributes::default()
        },
    )?;

    let mut windows: BTreeMap<WindowHandle, Window> = BTreeMap::new();
    windows.insert(left.handle(), left);
    windows.insert(right.handle(), right);

    let mut workers: BTreeMap<WindowHandle, Sender<WorkerMsg>> = BTreeMap::new();
    for handle in windows.keys().copied() {
        workers.insert(handle, spawn_window_worker(handle));
    }

    for window in windows.values() {
        window.request_redraw();
    }

    loop {
        if let Some((scope, event)) = event_loop.wait_event(Some(Duration::from_millis(8)))? {
            match (scope, &event) {
                (Some(window), Event::CloseRequested) => {
                    if let Some(sender) = workers.remove(&window) {
                        let _ = sender.send(WorkerMsg::Stop);
                    }
                    windows.remove(&window);
                    if windows.is_empty() {
                        break;
                    }
                }
                (Some(window), _) => {
                    if let Some(sender) = workers.get(&window) {
                        let _ = sender.send(WorkerMsg::Event(event));
                    }
                }
                (None, _) => {
                    println!("[global] {event:?}");
                }
            }
        }
    }

    for (_, sender) in workers {
        let _ = sender.send(WorkerMsg::Stop);
    }

    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    panic!("The multi_window_shared_state example targets Linux Wayland");
}
