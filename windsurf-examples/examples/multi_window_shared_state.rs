#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::collections::{BTreeMap, BTreeSet};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::error::Error;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::sync::mpsc::{self, Sender};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::thread;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::time::Duration;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use windsurf::{
    CursorIcon, CursorMode, CursorSource, Display, Event, EventQueue, FeatureSet, Features,
    ImePurpose, ImeState, KeyCode, KeyState, Window, WindowAttributes, WindowId,
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
enum WorkerMsg {
    Core(Event),
    Stop,
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn spawn_window_worker(id: WindowId) -> Sender<WorkerMsg> {
    let (tx, rx) = mpsc::channel::<WorkerMsg>();
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            match msg {
                WorkerMsg::Core(event) => {
                    println!("[worker {id:?}] core: {event:?}");
                    if matches!(event, Event::TextInput { .. }) {
                        // Simulate a slow per-window handler.
                        thread::sleep(Duration::from_millis(250));
                    }
                }
                WorkerMsg::Stop => break,
            }
        }
    });
    tx
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> Result<(), Box<dyn Error>> {
    let display = Display::connect()?;
    let mut events = EventQueue::new();
    let features = display.supported_features();

    let left = Window::new(
        &display,
        &WindowAttributes {
            title: String::from("shared-state: left"),
            ..WindowAttributes::default()
        },
    )?;
    let right = Window::new(
        &display,
        &WindowAttributes {
            title: String::from("shared-state: right"),
            ..WindowAttributes::default()
        },
    )?;

    let mut windows: BTreeMap<WindowId, Window> = BTreeMap::new();
    windows.insert(left.id(), left);
    windows.insert(right.id(), right);

    let mut workers: BTreeMap<WindowId, Sender<WorkerMsg>> = BTreeMap::new();
    for id in windows.keys().copied() {
        workers.insert(id, spawn_window_worker(id));
    }

    let mut hidden_mode_windows = BTreeSet::new();

    if features.contains(FeatureSet::CURSOR) {
        let mut ids = windows.keys().copied();
        if let Some(id) = ids.next() {
            let _ = display.set_cursor(id, &CursorSource::Icon(CursorIcon::Pointer));
        }
        if let Some(id) = ids.next() {
            let _ = display.set_cursor(id, &CursorSource::Icon(CursorIcon::Text));
        }
    }

    if features.contains(FeatureSet::IME) {
        for id in windows.keys().copied() {
            let _ = display.set_ime_state(
                id,
                &ImeState {
                    enabled: true,
                    purpose: ImePurpose::Normal,
                    cursor_area: None,
                },
            );
        }
    }

    for window in windows.values() {
        window.request_redraw();
    }

    println!(
        "multi-window threaded demo started: {} windows, features={features:?}",
        windows.len()
    );
    println!("space: toggle cursor hidden/normal for focused window");
    println!("text input handling in one window worker intentionally sleeps");

    loop {
        display.pump(&mut events)?;

        let mut close_requests = BTreeSet::new();

        events.dispatch_by_window(
            |id, event| {
                match &event {
                    Event::KeyboardFocusIn { id: event_id } => {
                        if let Some(window) = windows.get(event_id) {
                            window.set_title("shared-state: focused");
                        }
                    }
                    Event::KeyboardFocusOut { id: event_id } => {
                        if let Some(window) = windows.get(event_id) {
                            window.set_title("shared-state: unfocused");
                        }
                    }
                    Event::Key {
                        id: event_id,
                        key,
                        state,
                        ..
                    } if *state == KeyState::Pressed
                        && *key == KeyCode::Space
                        && features.contains(FeatureSet::CURSOR) =>
                    {
                        let is_hidden = hidden_mode_windows.contains(event_id);
                        let next_mode = if is_hidden {
                            hidden_mode_windows.remove(event_id);
                            CursorMode::Normal
                        } else {
                            hidden_mode_windows.insert(*event_id);
                            CursorMode::Hidden
                        };
                        let _ = display.set_cursor_mode(*event_id, next_mode);
                    }
                    Event::CloseRequested { id: event_id } => {
                        close_requests.insert(*event_id);
                    }
                    _ => {}
                }

                if let Some(worker) = workers.get(&id) {
                    let _ = worker.send(WorkerMsg::Core(event));
                }
            },
            |event| println!("[global] {event:?}"),
        );

        for id in close_requests {
            if let Some(sender) = workers.remove(&id) {
                let _ = sender.send(WorkerMsg::Stop);
            }
            hidden_mode_windows.remove(&id);
            windows.remove(&id);
        }

        if windows.is_empty() {
            println!("all windows closed, exiting");
            break;
        }

        thread::sleep(Duration::from_millis(8));
    }

    for (_, sender) in workers {
        let _ = sender.send(WorkerMsg::Stop);
    }

    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn main() {
    panic!("The multi_window_shared_state example targets Linux Wayland and macOS AppKit");
}
