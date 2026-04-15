use crate::{Event, WindowId};
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

    /// Drain all events in one pass and dispatch them by scope.
    ///
    /// Events tied to a specific window are sent to `on_window`. Global events
    /// are sent to `on_global`.
    pub fn dispatch_by_window<F, G>(&mut self, mut on_window: F, mut on_global: G)
    where
        F: FnMut(WindowId, Event),
        G: FnMut(Event),
    {
        while let Some(event) = self.events.pop_front() {
            if let Some(window) = event.window_id() {
                on_window(window, event);
            } else {
                on_global(event);
            }
        }
    }

    /// Drain all events for `window` into `target`, preserving order.
    pub fn drain_window_into(&mut self, window: WindowId, target: &mut Self) {
        self.drain_matching_into(target, |event| event.window_id() == Some(window));
    }

    /// Drain events that are not tied to any window into `target`.
    pub fn drain_global_into(&mut self, target: &mut Self) {
        self.drain_matching_into(target, |event| event.window_id().is_none());
    }

    fn drain_matching_into<F>(&mut self, target: &mut Self, mut predicate: F)
    where
        F: FnMut(&Event) -> bool,
    {
        let mut remaining = VecDeque::with_capacity(self.events.len());
        while let Some(event) = self.events.pop_front() {
            if predicate(&event) {
                target.push(event);
            } else {
                remaining.push_back(event);
            }
        }
        self.events = remaining;
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
    use alloc::vec;
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

    #[test]
    fn drain_window_and_global_partition_events() {
        let mut queue = EventQueue::new();
        queue.push(Event::WindowCreated {
            id: WindowId::new(1),
        });
        queue.push(Event::Suspended);
        queue.push(Event::CloseRequested {
            id: WindowId::new(2),
        });

        let mut window_events = EventQueue::new();
        queue.drain_window_into(WindowId::new(1), &mut window_events);
        let drained_window: Vec<_> = window_events.drain().collect();
        assert_eq!(drained_window.len(), 1);
        assert_eq!(
            drained_window[0],
            Event::WindowCreated {
                id: WindowId::new(1)
            }
        );

        let mut global_events = EventQueue::new();
        queue.drain_global_into(&mut global_events);
        let drained_global: Vec<_> = global_events.drain().collect();
        assert_eq!(drained_global, vec![Event::Suspended]);

        let remainder: Vec<_> = queue.drain().collect();
        assert_eq!(
            remainder,
            vec![Event::CloseRequested {
                id: WindowId::new(2)
            }]
        );
    }

    #[test]
    fn dispatch_by_window_splits_scopes_without_intermediate_queues() {
        let mut queue = EventQueue::new();
        queue.push(Event::WindowCreated {
            id: WindowId::new(5),
        });
        queue.push(Event::Resumed);

        let mut window_seen = None;
        let mut global_seen = None;
        queue.dispatch_by_window(
            |window, event| {
                window_seen = Some((window, event));
            },
            |event| {
                global_seen = Some(event);
            },
        );

        assert!(queue.is_empty());
        assert_eq!(
            window_seen,
            Some((
                WindowId::new(5),
                Event::WindowCreated {
                    id: WindowId::new(5)
                }
            ))
        );
        assert_eq!(global_seen, Some(Event::Resumed));
    }
}
