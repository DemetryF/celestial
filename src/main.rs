use std::sync::{Arc, RwLock};
use std::thread;

use atomic_float::AtomicF32;
use eframe::NativeOptions;
use egui::emath::TSTransform;
use egui::{Vec2, ViewportBuilder};

use app::App;
use cosmos_object::CosmosObject;
use physics::{Physics, KM_PER_VPX};

mod app;
mod cosmos_object;
mod physics;
mod utils;

pub fn main() -> eframe::Result {
    static DELTA_TIME: AtomicF32 = AtomicF32::new(0.0);

    // virtual day in real second
    static TIME_SPEED: AtomicF32 = AtomicF32::new(60.0 * 60.0 * 24.0);

    // km^3 / kg / sec^2
    let gravitional_const = 6.674e-11 / 1e9;
    // vpx^3 / kg / sec^2
    let gravitional_const = gravitional_const / KM_PER_VPX.powi(3);

    let sun = CosmosObject {
        mass: 2e30,
        radius: 7e5 / KM_PER_VPX,
        ..Default::default()
    };

    let earth = CosmosObject {
        mass: 6e24,
        radius: 6.5e3 / KM_PER_VPX,
        ..Default::default()
    }
    .orbit(
        &sun,
        149_597_871.0 / KM_PER_VPX,
        0.0,
        1.0,
        gravitional_const,
    );

    let objects = vec![RwLock::new(sun), RwLock::new(earth)];
    let objects = Arc::new(RwLock::new(objects));

    let mut physics = Physics::new(Arc::clone(&objects), &DELTA_TIME, &TIME_SPEED);

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

            let transform = TSTransform::new(size / 2.0, 1.0);

            Ok(Box::new(App::new(objects, transform)))
        }),
    )
}
