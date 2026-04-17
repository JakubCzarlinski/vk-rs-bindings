use windsurf::{Event, EventQueue, WindowId};

fn main() {
    let mut queue = EventQueue::new();
    let id = WindowId::new(1);

    queue.push(Event::WindowCreated { id });
    queue.push(Event::RedrawRequested { id });

    for event in queue.drain() {
        match event {
            Event::RedrawRequested { id } => println!("redraw {}", id.raw()),
            other => println!("event: {other:?}"),
        }
    }
}
