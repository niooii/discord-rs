use async_std::task;
use rand::Rng;
use std::time::Instant;
use std::{ops::Range, time::Duration};

pub struct Waiter {}

impl Waiter {
    pub fn new() -> Waiter {
        Waiter {}
    }

    // Seconds
    pub async fn wait_random(range: Range<f32>) {
        let mut rng = rand::thread_rng();

        let t = rng.gen_range(range);

        task::sleep(Duration::from_secs_f32(t)).await;
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