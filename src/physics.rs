mod gravity;

use std::sync::{Arc, RwLock};
use std::time::Instant;

use egui::Vec2;

use crate::cosmos_object::CosmosObject;

pub struct Physics {
    objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,
    delta_time: f32,
}

impl Physics {
    pub fn new(objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>) -> Self {
        Self {
            objects,
            delta_time: Default::default(),
        }
    }

    pub fn start(&mut self) {
        loop {
            let start_time = Instant::now();

            self.update();

            self.delta_time = start_time.elapsed().as_secs_f32() * 5.0;
        }
    }

    pub fn update(&self) {
        let objects = self.objects.read().unwrap();

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

            let delta_velocity = current.acceleration * self.delta_time;
            current.velocity += delta_velocity;

            let delta_position = current.velocity * self.delta_time;
            current.position += delta_position;
        }
    }
}
