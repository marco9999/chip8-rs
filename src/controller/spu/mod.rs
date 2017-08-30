use std::sync::mpsc::*;
use Core;
use CoreEvent;
use common::constants::spu::*;
use common::types::storage::*;
use controller::*;

pub struct Spu<'a> {
    /// Core manager.
    core: &'a Core,

    /// ControllerEvent queue channel receiver.
    event_queue_rx: Receiver<ControllerEvent>,

    /// ControllerEvent queue channel sender.
    event_queue_tx: SyncSender<ControllerEvent>,
}

unsafe impl<'a> Sync for Spu<'a> {}

impl<'a> Spu<'a> {
    pub fn new(core: &Core) -> Spu {
        let (event_queue_tx, event_queue_rx) = sync_channel::<ControllerEvent>(128);
        Spu {
            core,
            event_queue_tx,
            event_queue_rx,
        }
    }

    fn core(&self) -> &Core {
        self.core
    }
}

impl<'a> Controller for Spu<'a> {
    fn step(&self, event: ControllerEvent) -> Result<(), String> {
        match event {
            ControllerEvent::Tick(mut amount) => {
                while amount > 0 {
                    // Aquire resources.
                    let res = self.core().resources()?;

                    // Check sound register, make source and decrement if non-zero.
                    let counter = res.spu.counter.read(BusContext::Raw, 0);
                    if counter > 0 {
                        res.spu.counter.write(BusContext::Raw, 0, counter - 1);
                        self.core().send_event(CoreEvent::Audio);
                    }
                    
                    // Finished one cycle.
                    amount -= 1;
                }
            },

            _ => {
                unimplemented!("Spu doesn't know how to handle other event types");
            }
        }

        Ok(())
    }

    fn event_iter(&self) -> TryIter<ControllerEvent> {
        self.event_queue_rx.try_iter()
    }

    fn send_event(&self, event: ControllerEvent) {
        self.event_queue_tx.send(event).unwrap();
    }

    fn gen_tick_event(&self, time_delta_us: f64) -> Result<(), String> {
        let clock_state = &mut self.core().resources()?.spu.clock_state;
        let bias = self.core().config().spu_bias;
        clock_state.produce(time_delta_us, bias * CLOCK_SPEED);
        let ticks = clock_state.consume_whole();
        self.event_queue_tx.send(ControllerEvent::Tick(ticks as isize)).unwrap();
        Ok(())
    }
}
