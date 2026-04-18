use windsurf::{Event, EventQueue, WindowHandleAllocator};

fn main() {
    let mut handles = WindowHandleAllocator::new();
    let window = handles.allocate().expect("window handle available");

    let mut queue = EventQueue::new();
    queue.push(Some(window), Event::WindowCreated).unwrap();
    queue.push(Some(window), Event::RedrawRequested).unwrap();

    for (scope, event) in queue.drain() {
        println!("scope={scope:?} event={event:?}");
    }
}
