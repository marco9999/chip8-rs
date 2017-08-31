#![feature(const_fn)]
#![feature(associated_type_defaults)]

extern crate num;
extern crate rand;
#[macro_use]
extern crate log;
#[macro_use] 
extern crate serde_derive;
//extern crate bincode;
extern crate futures;
extern crate futures_cpupool;

pub mod common;
pub mod resources;
pub mod controller;

use std::cell::UnsafeCell;
use std::sync::mpsc::*;
use futures::Future;
use futures_cpupool::CpuPool;
use futures_cpupool::CpuFuture;
use common::types::storage::*;
use common::types::storage::register::Register;
use common::types::primative::udword;
use resources::Resources;
use resources::cpu::KEYS;
use controller::Controller;
use controller::cpu::Cpu;
use controller::spu::Spu;
use controller::timer::Timer;
use common::constants::cpu::{VERTICAL_RES, HORIZONTAL_RES};

pub struct Config {
    pub workspace_path: String,
    pub time_delta_us: f64,
    pub multithreaded_pool: Option<CpuPool>, 
    pub cpu_bias: f64,
    pub spu_bias: f64,
    pub timer_bias: f64,

    pub video_callback: Option<fn(&[bool; HORIZONTAL_RES * VERTICAL_RES])>,
    pub audio_callback: Option<fn()>,
}

/// Events that are communicated from the controllers to the core,
/// relating to user interaction, etc (host functionality).
enum CoreEvent {
    /// A render event, originating from a controller to update the screen.
    Video,

    /// An audio event, originating from a controller to play a beep sound.
    Audio,
}

pub struct Core {
    config: Config,
    resources: Option<Box<UnsafeCell<Resources>>>,
    controllers: Vec<Box<Controller>>,
    multithreaded_futures: Vec<CpuFuture<(), String>>,
    event_queue_rx: Receiver<CoreEvent>,
    event_queue_tx: SyncSender<CoreEvent>,
}

impl Core {
    /// Creates a new core.
    /// You must call reset() afterwards to create the resources and controllers.
    /// This is to prevent moves when constructing, causing the controllers
    /// core references pointing to invalid locations.
    pub fn new(config: Option<Config>) -> Core {
        let (event_queue_tx, event_queue_rx) = sync_channel::<CoreEvent>(128);
        match config {
            Some(config) => {
                Core {
                    config: config,
                    resources: None,
                    controllers: Vec::new(),
                    multithreaded_futures: Vec::new(),
                    event_queue_rx,
                    event_queue_tx,
                }
            },
            None => { 
                Core {
                    config: Config {
                        workspace_path: "./workspace/".to_owned(),
                        time_delta_us: 20000.0,
                        multithreaded_pool: None,
                        cpu_bias: 1.0, 
                        spu_bias: 1.0,
                        timer_bias: 1.0,
                        video_callback: None,
                        audio_callback: None,
                    },
                    resources: None,
                    controllers: Vec::new(),
                    multithreaded_futures: Vec::new(),
                    event_queue_rx,
                    event_queue_tx,
                }
            },
        }
    }

    /// Resets the core, initialising the Core state.
    /// Performs the following:
    ///  - Allocates resources.
    ///  - Resets all controllers.
    ///  - Loads the default font set.
    ///  - Loads the rom from the path given.
    pub fn reset(&mut self, rom_path: &str) -> Result<(), String> {
        self.resources = Some(Box::new(UnsafeCell::new(Resources::new())));

        self.controllers.clear();
        unsafe {
            let self_ptr =  self as *const Core;
            self.controllers.push(Box::new(Cpu::new(&*self_ptr)));
            self.controllers.push(Box::new(Spu::new(&*self_ptr)));
            self.controllers.push(Box::new(Timer::new(&*self_ptr)));
        }

        self.load_font_set()?;
        self.load_rom(rom_path)?;

        Ok(())
    }

    /// Runs through each of the controllers that update the machine state.
    /// Each run will update the state for the time step defined at initialisation.
    /// Also handles any events received from the controllers.
    pub fn run(&mut self) -> Result<(), String> {
        if cfg!(build = "debug") {
            unsafe {
                static mut TIME_US: f64 = 0.0;
                info!("Emulated time elapsed (s) = {:.6}", TIME_US / 1e6);
                TIME_US += self.config.time_delta_us;
            }
        }

        // Generate the clock tick event for each controller, using the time slice set.
        for ref cont in self.controllers.iter() {
            cont.gen_tick_event(self.config.time_delta_us)?;
        }

        // Run the controllers, either in multi-threaded or single-threaded mode.
        match self.config.multithreaded_pool {
            Some(ref pool) => {
                for cont in self.controllers.iter() {
                    unsafe { 
                        let cont_static = &*(cont.as_ref() as *const Controller); 
                        self.multithreaded_futures.push(
                            pool.spawn_fn(move || {
                                cont_static.run()
                            })
                        );
                    }
                }

                for future in self.multithreaded_futures.drain(..) {
                    future.wait()?;
                }
            },
            None => {
                for cont in self.controllers.iter() {
                    cont.run()?;
                }
            },
        }

        // Handle any host interface events from controllers.
        for event in self.event_queue_rx.try_iter() {
            match event {
                CoreEvent::Video => {
                    if let Some(ref f) = self.config.video_callback {
                        f(&self.resources()?.cpu.framebuffer);
                    }
                },
                CoreEvent::Audio => {
                    if let Some(ref f) = self.config.audio_callback {
                        f();
                    }
                },
            }
        }

        Ok(())
    }

    /// Dumps all resources memory to workspace/dumps/file.bin.
    #[cfg(build = "debug")]
    pub fn debug_dump_all(&self, postfix_tag: &str) -> Result<(), String> {
        if let Err(_) = self.resources()?.memory.dump_file(&self.workspace_path(&format!("dumps/memory{}.bin", postfix_tag))) {
            return Err("Something went wrong writing the memory dump file.".to_owned());
        }

        Ok(())
    }

    /// Sets/clears the key specified.
    pub fn set_key(&self, key: usize, pressed: bool) -> Result<(), String> {
        if key > 0xF {
            return Err("Key not within valid range".to_owned());
        }

        let res = self.resources()?;
        res.cpu.keys.write_bitfield(BusContext::Raw, 0, &KEYS[key], pressed as udword);
        Ok(())
    }

    /// Returns a relative path within the workspace.
    /// Workspace contains config files, save files, log files, etc.
    fn workspace_path(&self, rel_path: &str) -> String {
        self.config.workspace_path.clone() + rel_path
    }

    /// Returns a reference to mutable resources. 
    fn resources(&self) -> Result<&mut Resources, String> {
        unsafe {
            match self.resources {
                Some(ref res) => {
                    Ok(&mut *res.get())
                },
                None => {
                    Err("Core has not been initialised".to_owned())
                },
            }
        }
    }

    /// Returns a reference to the shared config. 
    fn config(&self) -> &Config {
        &self.config
    }

    /// Initialises the default Chip8 font set and loads it into memory starting at offset 0x0. 
    /// See http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#font.
    fn load_font_set(&self) -> Result<(), String> {
        let char_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let char_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
        let char_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
        let char_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
        let char_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
        let char_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let char_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
        let char_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
        let char_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
        let char_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let char_a: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
        let char_b: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
        let char_c: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
        let char_d: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
        let char_e: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
        let char_f: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

        let res = self.resources()?;

        res.memory.write_slice(BusContext::Raw, 0x0, &char_0);
        res.memory.write_slice(BusContext::Raw, 0x5, &char_1);
        res.memory.write_slice(BusContext::Raw, 0xA, &char_2);
        res.memory.write_slice(BusContext::Raw, 0xF, &char_3);
        res.memory.write_slice(BusContext::Raw, 0x14, &char_4);
        res.memory.write_slice(BusContext::Raw, 0x19, &char_5);
        res.memory.write_slice(BusContext::Raw, 0x1E, &char_6);
        res.memory.write_slice(BusContext::Raw, 0x23, &char_7);
        res.memory.write_slice(BusContext::Raw, 0x28, &char_8);
        res.memory.write_slice(BusContext::Raw, 0x2D, &char_9);
        res.memory.write_slice(BusContext::Raw, 0x32, &char_a);
        res.memory.write_slice(BusContext::Raw, 0x37, &char_b);
        res.memory.write_slice(BusContext::Raw, 0x3C, &char_c);
        res.memory.write_slice(BusContext::Raw, 0x41, &char_d);
        res.memory.write_slice(BusContext::Raw, 0x46, &char_e);
        res.memory.write_slice(BusContext::Raw, 0x4B, &char_f);

        Ok(())
    }

    /// Loads in a Chip8 rom at 0x200.
    fn load_rom(&self, rom_path: &str) -> Result<(), String> {
        if let Err(_) = self.resources()?.memory.read_file(0x200, rom_path) {
            return Err("Something went wrong loading rom file.".to_owned());
        }
        Ok(())
    }

    /// Sends an event to the back of the event queue attached to the core.
    /// Used from controllers to do callbacks.
    fn send_event(&self, event: CoreEvent) {
        self.event_queue_tx.send(event).unwrap();
    }
}
