use tokio::time;
use rand::Rng;
use std::time::Instant;
use std::{ops::Range, time::Duration};

pub struct Waiter {}

impl Waiter {
    // Seconds
    pub fn get_random_duration(range: Range<f32>) -> Duration {
        let mut rng = rand::thread_rng();

        let t = rng.gen_range(range);

        Duration::from_secs_f32(t)
    }
}

pub struct Stopwatch {
    start: Instant,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    pub fn elapsed_millis(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}