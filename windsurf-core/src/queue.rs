use crate::Event;
use alloc::collections::{VecDeque, vec_deque};

extern crate alloc;

/// Non-blocking drainable event storage.
///
/// A backend typically owns the producer side and application code owns the
/// consumer side, but the type itself is deliberately just a queue.
#[derive(Debug, Clone, Default)]
pub struct EventQueue {
    events: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a single event into the queue.
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    /// Extend the queue from any iterator of events.
    pub fn extend<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = Event>,
    {
        self.events.extend(events);
    }

    /// Drain all pending events in FIFO order.
    pub fn drain(&mut self) -> vec_deque::Drain<'_, Event> {
        self.events.drain(..)
    }

    /// Return the number of queued events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Return `true` when there are no pending events.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Remove all pending events without yielding them.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::EventQueue;
    use crate::{Event, WindowId};
    use alloc::vec::Vec;
    extern crate alloc;

    #[test]
    fn drain_returns_all_events_in_order() {
        let mut queue = EventQueue::new();
        queue.push(Event::WindowCreated {
            id: WindowId::new(1),
        });
        queue.push(Event::CloseRequested {
            id: WindowId::new(2),
        });

        let events: Vec<Event> = queue.drain().collect();

        assert_eq!(events.len(), 2);
        assert!(queue.is_empty());
        assert_eq!(
            events[0],
            Event::WindowCreated {
                id: WindowId::new(1)
            }
        );
        assert_eq!(
            events[1],
            Event::CloseRequested {
                id: WindowId::new(2)
            }
        );
    }
}
