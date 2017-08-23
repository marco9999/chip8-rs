pub mod cpu;
pub mod spu;

pub use Core;

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
    fn run(&self) {

    }

    fn core(&self) -> &Core;

    fn step(&self, &Event) -> isize;
}