use std::sync::{Arc, RwLock};
use std::thread;

use eframe::NativeOptions;
use egui::emath::TSTransform;
use egui::{Vec2, ViewportBuilder};

use app::App;
use cosmos_object::CosmosObject;
use physics::Physics;

mod app;
mod cosmos_object;
mod physics;
mod utils;

pub fn main() -> eframe::Result {
    let center = CosmosObject {
        mass: 100.0,
        ..Default::default()
    };

    let planet = CosmosObject {
        mass: 5.0,
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
        Box::new(move |ctx| {
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert(
                "segoe".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/Segoe UI.ttf")),
            );

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "segoe".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("segoe".to_owned());

            ctx.egui_ctx.set_fonts(fonts);

            let transform = TSTransform::new(size / 2.0, 5.0);

            Ok(Box::new(App::new(objects, transform)))
        }),
    )
}
