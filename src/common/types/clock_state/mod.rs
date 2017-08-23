#[derive(Serialize, Deserialize, Debug)]
pub struct ClockState {
    ticks: f64
}

impl ClockState {
    pub fn new() -> ClockState {
        ClockState {
            ticks: 0.0
        }
    }

    pub fn produce(&mut self, time_us: f64, speed: f64) {
        self.ticks += time_us / 10e6 * speed;
    }

    pub fn consume(&mut self, ticks: f64) {
        self.ticks -= ticks;
    }
}