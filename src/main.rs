#[macro_use]
extern crate log;
extern crate log4rs;
extern crate futures_cpupool;

extern crate chip8_rs as chip8;

use futures_cpupool::CpuPool;
use chip8::Core;
use chip8::Config;

fn main() {
    log4rs::init_file("./workspace/config/log.yml", Default::default()).unwrap();
    info!("Started");

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

    loop {
        if let Err(e) = core.run() {
            error!("Encountered error (exiting): {}", e);
            break;
        }
    }

    if cfg!(build = "debug") {
        debug!("Memory dumped to workspace/dumps folder chip8-rs");
        core.debug_dump_all("_exit").unwrap();
    }
}
