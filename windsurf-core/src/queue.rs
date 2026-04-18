use crate::{Event, WindowHandle};
use core::mem::MaybeUninit;

/// Tuple-transported event item.
pub type ScopedEvent = (Option<WindowHandle>, Event);

/// Push failure when the fixed-capacity queue is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueOverflow;

/// Fixed-capacity FIFO event queue with overflow accounting.
#[derive(Debug)]
pub struct EventQueue<const CAP: usize = 1024> {
    entries: [MaybeUninit<ScopedEvent>; CAP],
    head: usize,
    len: usize,
    dropped: usize,
}

impl<const CAP: usize> EventQueue<CAP> {
    pub fn new_with_capacity() -> Self {
        Self {
            entries: [const { MaybeUninit::uninit() }; CAP],
            head: 0,
            len: 0,
            dropped: 0,
        }
    }

    /// Push a single scoped event into the queue.
    pub fn push(
        &mut self,
        window: Option<WindowHandle>,
        event: Event,
    ) -> Result<(), QueueOverflow> {
        self.push_scoped((window, event))
    }

    /// Push a full `(window, event)` tuple.
    pub fn push_scoped(&mut self, scoped: ScopedEvent) -> Result<(), QueueOverflow> {
        if self.len == CAP {
            self.dropped = self.dropped.saturating_add(1);
            return Err(QueueOverflow);
        }

        let tail = (self.head + self.len) % CAP;
        self.entries[tail].write(scoped);
        self.len += 1;
        Ok(())
    }

    /// Pop the next event in FIFO order.
    pub fn pop(&mut self) -> Option<ScopedEvent> {
        if self.len == 0 {
            return None;
        }

        let index = self.head;
        self.head = (self.head + 1) % CAP;
        self.len -= 1;

        Some(unsafe { self.entries[index].assume_init_read() })
    }

    /// Drain all pending events in FIFO order.
    pub fn drain(&mut self) -> Drain<'_, CAP> {
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
    pub fn drain_window_into<const TARGET_CAP: usize>(
        &mut self,
        window: WindowHandle,
        target: &mut EventQueue<TARGET_CAP>,
    ) {
        self.drain_matching_into(target, |event_window, _| *event_window == Some(window));
    }

    /// Drain events that are not tied to any window into `target`.
    pub fn drain_global_into<const TARGET_CAP: usize>(
        &mut self,
        target: &mut EventQueue<TARGET_CAP>,
    ) {
        self.drain_matching_into(target, |event_window, _| event_window.is_none());
    }

    fn drain_matching_into<const TARGET_CAP: usize, F>(
        &mut self,
        target: &mut EventQueue<TARGET_CAP>,
        mut predicate: F,
    ) where
        F: FnMut(&Option<WindowHandle>, &Event) -> bool,
    {
        let initial = self.len;
        for _ in 0..initial {
            let Some((window, event)) = self.pop() else {
                break;
            };
            if predicate(&window, &event) {
                let _ = target.push(window, event);
            } else {
                let _ = self.push(window, event);
            }
        }
    }

    /// Return the number of queued events.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Return `true` when there are no pending events.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Remove all pending events without yielding them.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Number of events dropped due to overflow since queue creation.
    pub const fn dropped_count(&self) -> usize {
        self.dropped
    }

    /// Read and reset overflow counter.
    pub fn take_dropped_count(&mut self) -> usize {
        let dropped = self.dropped;
        self.dropped = 0;
        dropped
    }
}

impl EventQueue<1024> {
    pub fn new() -> Self {
        Self::new_with_capacity()
    }
}

impl<const CAP: usize> Default for EventQueue<CAP> {
    fn default() -> Self {
        Self::new_with_capacity()
    }
}

impl<const CAP: usize> Drop for EventQueue<CAP> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct Drain<'a, const CAP: usize> {
    queue: &'a mut EventQueue<CAP>,
}

impl<const CAP: usize> Iterator for Drain<'_, CAP> {
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

        let mut queue = EventQueue::<8>::new_with_capacity();
        queue.push(Some(h1), Event::WindowCreated).unwrap();
        queue.push(Some(h2), Event::CloseRequested).unwrap();

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

        let mut queue = EventQueue::<8>::new_with_capacity();
        queue.push(Some(h1), Event::WindowCreated).unwrap();
        queue.push(None, Event::Suspended).unwrap();
        queue.push(Some(h2), Event::CloseRequested).unwrap();

        let mut window_events = EventQueue::<8>::new_with_capacity();
        queue.drain_window_into(h1, &mut window_events);
        let drained_window: Vec<_> = window_events.drain().collect();
        assert_eq!(drained_window, vec![(Some(h1), Event::WindowCreated)]);

        let mut global_events = EventQueue::<8>::new_with_capacity();
        queue.drain_global_into(&mut global_events);
        let drained_global: Vec<_> = global_events.drain().collect();
        assert_eq!(drained_global, vec![(None, Event::Suspended)]);

        let remainder: Vec<_> = queue.drain().collect();
        assert_eq!(remainder, vec![(Some(h2), Event::CloseRequested)]);
    }

    #[test]
    fn overflow_is_counted_and_observable() {
        let mut queue = EventQueue::<1>::new_with_capacity();
        queue.push(None, Event::Suspended).unwrap();
        assert!(queue.push(None, Event::Resumed).is_err());
        assert_eq!(queue.dropped_count(), 1);
        assert_eq!(queue.take_dropped_count(), 1);
        assert_eq!(queue.take_dropped_count(), 0);
    }

    #[test]
    fn steady_state_push_pop_does_not_allocate() {
        let mut queue = EventQueue::<16>::new_with_capacity();
        for _ in 0..512 {
            queue.push(None, Event::Resumed).unwrap();
            let _ = queue.pop();
        }
        assert_eq!(queue.dropped_count(), 0);
    }
}
