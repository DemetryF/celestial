mod gravity;
mod simulation_state;

use std::sync::{Arc, RwLock};
use std::time::Instant;

use egui::Vec2;

use crate::cosmos_object::CosmosObject;

pub use simulation_state::SimulationState;

pub const KM_PER_VPX: f32 = 1e5;
pub const GRAVITIONAL_CONST: f32 = 6.674e-20 / KM_PER_VPX / KM_PER_VPX / KM_PER_VPX;

pub struct Physics {
    pub objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,

    pub sim_state: &'static SimulationState,

    delta_time: f32,
}

impl Physics {
    pub fn new(
        objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,
        sim_state: &'static SimulationState,
    ) -> Self {
        Self {
            objects,
            sim_state,
            delta_time: 0.0,
        }
    }

    pub fn start(&mut self) {
        loop {
            let iter_start = Instant::now();

            self.sim_state.set_delta_time(self.delta_time);
            let time_speed = self.sim_state.time_speed();

            self.sim_state
                .set_elapsed(self.sim_state.elapsed() + self.delta_time);

            self.update();

            self.delta_time = iter_start.elapsed().as_secs_f32() * time_speed;
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

            current.acceleration *= GRAVITIONAL_CONST;

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
