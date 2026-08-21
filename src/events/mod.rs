//! Event bus for decoupled communication between components.

use parking_lot::Mutex;
use std::sync::Arc;

/// A simple cloneable event bus.
///
/// Events are type-erased at the publish boundary so modules can emit their
/// own event types without creating circular dependencies.
#[derive(Clone, Default)]
pub struct EventBus {
    inner: Arc<Mutex<EventBusInner>>,
}

#[derive(Default)]
struct EventBusInner {
    log: Vec<String>,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.lock();
        f.debug_struct("EventBus")
            .field("event_count", &inner.log.len())
            .finish()
    }
}

impl EventBus {
    /// Create a new event bus.
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish an event. The event is recorded by its `Debug` representation.
    pub fn publish<E: std::fmt::Debug>(&self, event: E) {
        let mut inner = self.inner.lock();
        inner.log.push(format!("{event:?}"));
        tracing::debug!(event = %format!("{event:?}"), "event published");
    }

    /// Number of events published since creation.
    pub fn len(&self) -> usize {
        self.inner.lock().log.len()
    }

    /// Whether no events have been published.
    pub fn is_empty(&self) -> bool {
        self.inner.lock().log.is_empty()
    }

    /// Snapshot of recorded event debug strings.
    pub fn recorded(&self) -> Vec<String> {
        self.inner.lock().log.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_and_records_events() {
        let bus = EventBus::new();
        bus.publish("hello");
        bus.publish(42);
        assert_eq!(bus.len(), 2);
        assert!(!bus.is_empty());
    }
}
