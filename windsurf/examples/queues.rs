use windsurf::{Event, EventQueue, ExtraEvent, ExtraEventQueue, GamepadEvent, GamepadId, WindowId};

fn main() {
    let mut core = EventQueue::new();
    let mut extras = ExtraEventQueue::new();
    let id = WindowId::new(1);

    core.push(Event::WindowCreated { id });
    core.push(Event::RedrawRequested { id });
    extras.push(ExtraEvent::Gamepad(GamepadEvent::Connected {
        gamepad: GamepadId(9),
        name: String::from("Demo Pad"),
    }));

    for event in core.drain() {
        match event {
            Event::RedrawRequested { id } => println!("redraw {}", id.raw()),
            other => println!("core event: {other:?}"),
        }
    }

    for event in extras.drain() {
        println!("extra event: {event:?}");
    }
}
