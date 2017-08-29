#[macro_use]
extern crate log;
extern crate log4rs;
extern crate futures_cpupool;
extern crate sdl2;

extern crate chip8_rs as chip8;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use futures_cpupool::CpuPool;
use chip8::Core;
use chip8::Config;

fn main() {
    log4rs::init_file("./workspace/config/log.yml", Default::default()).unwrap();
    if cfg!(build = "debug") {
        info!("Started (debug)");
    } else if cfg!(build = "release") {
        info!("Started (release)")
    }

    let config = Config {
        workspace_path: "./workspace/".to_owned(),
        time_delta_us: 20000.0,
        multithreaded_pool: Some(CpuPool::new_num_cpus()),
        cpu_bias: 1.0, 
        spu_bias: 1.0,
        timer_bias: 1.0,
    };
    let mut core = Core::new(Some(config));
    core.reset("./workspace/roms/PONG").unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if let Err(e) = core.run() {
            error!("Encountered error (exiting): {}", e);
            break 'running;
        }
    }
    

    if cfg!(build = "debug") {
        debug!("Memory dumped to workspace/dumps folder chip8-rs");
        core.debug_dump_all("_exit").unwrap();
    }
}
