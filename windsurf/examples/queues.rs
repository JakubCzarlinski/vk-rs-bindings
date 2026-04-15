use windsurf::{Event, EventQueue, GamepadEvent, GamepadId, WindowId};

fn main() {
    let mut queue = EventQueue::new();
    let id = WindowId::new(1);

    queue.push(Event::WindowCreated { id });
    queue.push(Event::RedrawRequested { id });
    queue.push(Event::Gamepad(GamepadEvent::Connected {
        gamepad: GamepadId(9),
        name: String::from("Demo Pad"),
    }));

    for event in queue.drain() {
        match event {
            Event::RedrawRequested { id } => println!("redraw {}", id.raw()),
            other => println!("event: {other:?}"),
        };
    }
}
