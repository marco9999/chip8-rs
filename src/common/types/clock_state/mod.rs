#[derive(Debug)]
pub struct ClockState {
    ticks: f64
}

impl ClockState {
    pub fn new() -> ClockState {
        ClockState {
            ticks: 0.0
        }
    }

    pub fn produce(&mut self, time_us: f64, clock_speed: f64) {
        self.ticks += time_us / 1e6 * clock_speed;
    }

    pub fn consume_whole(&mut self) -> isize{
        let whole_ticks = self.ticks as isize;
        self.ticks -= whole_ticks as f64;
        whole_ticks
    }
}