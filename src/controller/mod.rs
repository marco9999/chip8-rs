pub mod cpu;
pub mod spu;
pub mod timer;

use std::sync::mpsc::*;

pub enum Event {
    /// A clock tick event, containing the amount of whole ticks
    /// that occurred. 
    Tick(isize),

    /// An input event, containing the key and the pressed state of
    /// the key.
    Input(usize, bool),
}

pub trait Controller {
    /// Consumes all events in the event queue by calling step for
    /// each one.
    fn run(&self) {
        for event in self.event_iter() {
            self.step(event);
        }
    }
    
    /// Steps through the controllers state and updates it for a
    /// single event.
    fn step(&self, Event);

    /// Returns an iterator to the events currently in event queue.
    /// Non-blocking.
    fn event_iter(&self) -> TryIter<Event>;

    /// Sends an event to the back of the event queue attached to
    /// this controller.
    fn send_event(&self, event: Event);

    /// Generates and sends a clock tick event, calculated from the 
    /// time delta given (in us). The controller implementing this 
    /// is responsible for using the correct clock speed and bias.
    fn gen_tick_event(&self, time_delta_us: f64);
}