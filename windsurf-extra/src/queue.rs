use crate::ExtraEvent;
use alloc::collections::{VecDeque, vec_deque};

extern crate alloc;

/// Drainable queue for [`ExtraEvent`] values.
#[derive(Debug, Clone, Default)]
pub struct ExtraEventQueue {
    events: VecDeque<ExtraEvent>,
}

impl ExtraEventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a single extra event into the queue.
    pub fn push(&mut self, event: ExtraEvent) {
        self.events.push_back(event);
    }

    /// Extend the queue from any iterator of extra events.
    pub fn extend<I>(&mut self, events: I)
    where
        I: IntoIterator<Item = ExtraEvent>,
    {
        self.events.extend(events);
    }

    /// Drain all queued extra events in FIFO order.
    pub fn drain(&mut self) -> vec_deque::Drain<'_, ExtraEvent> {
        self.events.drain(..)
    }

    /// Return the number of queued extra events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Return `true` when there are no pending extra events.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Remove all queued extra events without yielding them.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::ExtraEventQueue;
    use crate::{ExtraEvent, GamepadEvent, GamepadId};

    use alloc::vec::Vec;

    extern crate alloc;
    #[test]
    fn extra_queue_drains() {
        let mut queue = ExtraEventQueue::new();
        queue.push(ExtraEvent::Gamepad(GamepadEvent::Disconnected {
            gamepad: GamepadId(7),
        }));

        let drained: Vec<_> = queue.drain().collect();

        assert_eq!(drained.len(), 1);
        assert!(queue.is_empty());
    }
}
