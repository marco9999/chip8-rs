pub struct Event {
    /// Event type that occurred.
    source: EventSource,
    /// Number of times the event occurred.
    amount: usize,
}

pub enum EventSource {
    /// A clock tick event.
    Tick,
}

pub trait Controller {
    pub fn step(&Event) -> usize;
}

pub struct CoreResources {
    core: Weak<Core>,
}

pub impl CoreResources {
    pub fn new(core: Weak<Core>) -> CoreResources {
        core
    }

    pub fn core(&self) -> Weak<Core> {
        core
    }
}