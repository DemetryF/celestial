mod gravity;

use std::sync::{Arc, RwLock};
use std::time::Instant;

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
            let (_, [current, right @ ..]) = objects.split_at(i) else {
                continue;
            };

            for other in right {
                let mut current = current.write().unwrap();
                let mut other = other.write().unwrap();

                gravity::gravity(&mut current, &mut other, self.delta_time);
            }

            let mut current = current.write().unwrap();

            let delta = current.velocity * self.delta_time;

            current.position += delta;
        }
    }
}
