#[macro_use]
extern crate log;
extern crate piston_window;
extern crate time;
extern crate rand;

use self::time::{Duration, Timespec};

pub mod logger;
pub mod gamegrid;
pub mod piece;


#[derive(Debug)]
pub struct CycleTimer {
    last_cycle: Timespec,
    cycle_time: Duration,
}

impl CycleTimer {
    pub fn new(cycle_time: i64) -> CycleTimer {
        CycleTimer {
            last_cycle: time::get_time(),
            cycle_time: Duration::milliseconds(cycle_time),
        }
    }

    pub fn reset(&mut self) {
        self.last_cycle = time::get_time();
    }

    fn check(&self) -> bool {
        let current_time = time::get_time();

        self.last_cycle + self.cycle_time < current_time
    }

    pub fn cycle(&mut self) -> bool {
        if self.check() {
            self.reset();
            true
        } else {
            false
        }
    }
}
