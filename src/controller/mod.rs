pub struct Event {
    /// Event type that occurred.
    source: EventSource,
    /// Number of times the event occurred.
    amount: isize,
}

pub enum EventSource {
    /// A clock tick event.
    Tick,
}

pub trait Controller {
    fn run(&self, event: &Event) -> isize {
        let mut amount = event.amount;
        while amount > 0 {
            amount -= self.step(event);
        }
        amount
    }

    fn step(&Event) -> isize;
}