use std::sync::{Arc, RwLock};
use std::thread;

use eframe::NativeOptions;
use egui::emath::TSTransform;
use egui::{Pos2, Vec2, ViewportBuilder};

use app::App;
use cosmos_object::CosmosObject;
use physics::Physics;

mod app;
mod cosmos_object;
mod physics;
mod utils;

pub fn main() -> eframe::Result {
    let center = CosmosObject {
        mass: 50.0,
        radius: 10.0,
        position: Pos2::default(),
        velocity: Vec2::ZERO,
    };

    let planet = CosmosObject {
        mass: 1.0,
        radius: 4.0,
        ..Default::default()
    }
    .orbit(&center, 30.0, 0.0, 1.0);

    let objects = vec![RwLock::new(center), RwLock::new(planet)];
    let objects = Arc::new(RwLock::new(objects));

    let mut physics = Physics::new(Arc::clone(&objects));

    thread::spawn(move || physics.start());

    let size = Vec2::splat(800.);

    eframe::run_native(
        "celestial.rs",
        NativeOptions {
            window_builder: Some(Box::new(move |_| {
                ViewportBuilder::default().with_inner_size(size)
            })),
            ..Default::default()
        },
        Box::new(move |_| {
            let transform = TSTransform::new(size / 2.0, 5.0);

            Ok(Box::new(App::new(objects, transform)))
        }),
    )
}
