use std::sync;

use atomic_float::AtomicF32;

pub struct SimulationState {
    delta_time: AtomicF32,
    time_speed: AtomicF32,
    elapsed: AtomicF32,
}

impl SimulationState {
    pub const fn new(time_speed: f32) -> Self {
        Self {
            delta_time: AtomicF32::new(0.0),
            elapsed: AtomicF32::new(0.0),
            time_speed: AtomicF32::new(time_speed),
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time.load(sync::atomic::Ordering::SeqCst)
    }

    pub fn set_delta_time(&self, delta_time: f32) {
        self.delta_time
            .store(delta_time, sync::atomic::Ordering::Relaxed)
    }

    pub fn time_speed(&self) -> f32 {
        self.time_speed.load(sync::atomic::Ordering::SeqCst)
    }

    pub fn set_time_speed(&self, time_speed: f32) {
        self.time_speed
            .store(time_speed, sync::atomic::Ordering::Relaxed)
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed.load(sync::atomic::Ordering::SeqCst)
    }

    pub fn set_elapsed(&self, elapsed: f32) {
        self.elapsed.store(elapsed, sync::atomic::Ordering::Relaxed)
    }
}
