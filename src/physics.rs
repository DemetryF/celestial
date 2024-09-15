mod gravity;

use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use atomic_float::AtomicF32;
use egui::Vec2;

use crate::cosmos_object::CosmosObject;

pub const KM_PER_VPX: f32 = 1e5;

pub struct Physics {
    pub objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,
    pub shared_dt: &'static AtomicF32,
    pub shared_time_speed: &'static AtomicF32,

    delta_time: f32,
    time_speed: f32,
}

impl Physics {
    pub fn new(
        objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,
        shared_dt: &'static AtomicF32,
        shared_time_speed: &'static AtomicF32,
    ) -> Self {
        Self {
            objects,
            shared_dt,
            shared_time_speed,
            delta_time: 0.0,
            time_speed: shared_time_speed.load(Ordering::Acquire),
        }
    }

    pub fn start(&mut self) {
        loop {
            let iter_start = Instant::now();

            self.shared_dt.store(self.delta_time, Ordering::Release);
            self.time_speed = self.shared_time_speed.load(Ordering::Relaxed);

            self.update();

            self.delta_time = iter_start.elapsed().as_secs_f32() * self.time_speed;
        }
    }

    pub fn update(&self) {
        let objects = self.objects.read().unwrap();

        // km^3 / kg / sec^2
        let gravitional_const = 6.674e-11 / 1e9;
        // vpx^3 / kg / sec^2
        let gravitional_const = gravitional_const / KM_PER_VPX.powi(3);

        for i in 0..objects.len() {
            let current = &objects[i];
            let mut current = current.write().unwrap();

            current.acceleration = Vec2::ZERO;

            for j in 0..objects.len() {
                if i == j {
                    continue;
                }

                let other = &objects[j];
                let other = other.read().unwrap();

                gravity::gravity(&mut current, &other);
            }

            current.acceleration *= gravitional_const;

            let delta_velocity = current.acceleration * self.delta_time;
            current.velocity += delta_velocity;
        }

        for i in 0..objects.len() {
            let current = &objects[i];
            let mut current = current.write().unwrap();

            let delta_position = current.velocity * self.delta_time;
            current.position += delta_position;
        }
    }
}
