use std::sync::atomic::Ordering;

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
        self.delta_time.load(Ordering::Relaxed)
    }

    pub fn set_delta_time(&self, delta_time: f32) {
        self.delta_time.store(delta_time, Ordering::Relaxed)
    }

    pub fn time_speed(&self) -> f32 {
        self.time_speed.load(Ordering::Relaxed)
    }

    pub fn zoom_time_speed(&self, zoom_delta: f32) {
        self.time_speed
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |time_speed| {
                Some(time_speed * zoom_delta)
            })
            .unwrap();
    }

    pub fn elapsed(&self) -> f32 {
        self.elapsed.load(Ordering::Relaxed)
    }

    pub fn update_elapsed(&self, delta_time: f32) {
        self.elapsed.fetch_add(delta_time, Ordering::Relaxed);
    }
}
