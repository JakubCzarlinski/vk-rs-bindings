use crate::{Event, WindowHandle};
use alloc::collections::VecDeque;

extern crate alloc;

/// Tuple-transported event item.
pub type ScopedEvent = (Option<WindowHandle>, Event);

/// Lossless FIFO event queue.
#[derive(Debug)]
pub struct EventQueue {
    entries: VecDeque<ScopedEvent>,
}

impl EventQueue {
    pub const DEFAULT_CAPACITY: usize = 16;

    pub fn new() -> Self {
        Self::with_capacity(Self::DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(capacity),
        }
    }

    /// Push a single scoped event into the queue.
    pub fn push(&mut self, window: Option<WindowHandle>, event: Event) {
        self.push_scoped((window, event));
    }

    /// Push a full `(window, event)` tuple.
    pub fn push_scoped(&mut self, scoped: ScopedEvent) {
        self.entries.push_back(scoped);
    }

    /// Pop the next event in FIFO order.
    pub fn pop(&mut self) -> Option<ScopedEvent> {
        self.entries.pop_front()
    }

    /// Drain all pending events in FIFO order.
    pub fn drain(&mut self) -> Drain<'_> {
        Drain { queue: self }
    }

    /// Drain all events in one pass and dispatch them by scope.
    pub fn dispatch_by_window<F, G>(&mut self, mut on_window: F, mut on_global: G)
    where
        F: FnMut(WindowHandle, Event),
        G: FnMut(Event),
    {
        while let Some((window, event)) = self.pop() {
            if let Some(window) = window {
                on_window(window, event);
            } else {
                on_global(event);
            }
        }
    }

    /// Drain all events for `window` into `target`, preserving order.
    pub fn drain_window_into(&mut self, window: WindowHandle, target: &mut EventQueue) {
        self.drain_matching_into(target, |event_window, _| *event_window == Some(window));
    }

    /// Drain events that are not tied to any window into `target`.
    pub fn drain_global_into(&mut self, target: &mut EventQueue) {
        self.drain_matching_into(target, |event_window, _| event_window.is_none());
    }

    fn drain_matching_into<F>(&mut self, target: &mut EventQueue, mut predicate: F)
    where
        F: FnMut(&Option<WindowHandle>, &Event) -> bool,
    {
        let mut retained = VecDeque::new();

        while let Some((window, event)) = self.pop() {
            if predicate(&window, &event) {
                target.push(window, event);
            } else {
                retained.push_back((window, event));
            }
        }

        self.entries = retained;
    }

    /// Return the number of queued events.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return `true` when there are no pending events.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Remove all pending events without yielding them.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Drain<'a> {
    queue: &'a mut EventQueue,
}

impl Iterator for Drain<'_> {
    type Item = ScopedEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::EventQueue;
    use crate::{Event, WindowHandleAllocator};
    use alloc::vec;
    use alloc::vec::Vec;

    extern crate alloc;

    #[test]
    fn drain_returns_all_events_in_order() {
        let mut handles = WindowHandleAllocator::new();
        let h1 = handles.allocate().unwrap();
        let h2 = handles.allocate().unwrap();

        let mut queue = EventQueue::with_capacity(8);
        queue.push(Some(h1), Event::WindowCreated);
        queue.push(Some(h2), Event::CloseRequested);

        let events: Vec<_> = queue.drain().collect();

        assert_eq!(events.len(), 2);
        assert!(queue.is_empty());
        assert_eq!(events[0], (Some(h1), Event::WindowCreated));
        assert_eq!(events[1], (Some(h2), Event::CloseRequested));
    }

    #[test]
    fn drain_window_and_global_partition_events() {
        let mut handles = WindowHandleAllocator::new();
        let h1 = handles.allocate().unwrap();
        let h2 = handles.allocate().unwrap();

        let mut queue = EventQueue::with_capacity(2);
        queue.push(Some(h1), Event::WindowCreated);
        queue.push(None, Event::Suspended);
        queue.push(Some(h2), Event::CloseRequested);

        let mut window_events = EventQueue::with_capacity(8);
        queue.drain_window_into(h1, &mut window_events);
        let drained_window: Vec<_> = window_events.drain().collect();
        assert_eq!(drained_window, vec![(Some(h1), Event::WindowCreated)]);

        let mut global_events = EventQueue::with_capacity(8);
        queue.drain_global_into(&mut global_events);
        let drained_global: Vec<_> = global_events.drain().collect();
        assert_eq!(drained_global, vec![(None, Event::Suspended)]);

        let remainder: Vec<_> = queue.drain().collect();
        assert_eq!(remainder, vec![(Some(h2), Event::CloseRequested)]);
    }

    #[test]
    fn spill_path_is_lossless_and_fifo() {
        let mut queue = EventQueue::with_capacity(2);
        queue.push(None, Event::WindowCreated);
        queue.push(None, Event::RedrawRequested);
        queue.push(None, Event::CloseRequested);
        queue.push(None, Event::Suspended);

        let drained: Vec<_> = queue.drain().collect();
        assert_eq!(
            drained,
            vec![
                (None, Event::WindowCreated),
                (None, Event::RedrawRequested),
                (None, Event::CloseRequested),
                (None, Event::Suspended),
            ]
        );
    }

    #[test]
    fn steady_state_push_pop_stays_correct() {
        let mut queue = EventQueue::with_capacity(16);
        for _ in 0..512 {
            queue.push(None, Event::Resumed);
            let _ = queue.pop();
        }
        assert!(queue.is_empty());
    }
}
