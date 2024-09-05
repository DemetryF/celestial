use std::sync::{Arc, RwLock};

use egui::Vec2;
use egui::{emath::TSTransform, Color32, Frame, Margin, Pos2, Sense};

use crate::cosmos_object::CosmosObject;
use crate::utils::Painter;

pub struct App {
    pub objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,

    moving: Option<Moving>,
    transform: TSTransform,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.update_moving(ctx);
        self.update_zoom(ctx);

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::ZERO,
                outer_margin: Margin::ZERO,
                fill: Color32::from_gray(27),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let objects = self.objects.read().unwrap();
                let (_, ref painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                let painter = Painter {
                    painter,
                    transform: self.transform,
                };

                for object in objects.iter() {
                    let object = &object.read().unwrap();

                    object.draw(painter)
                }
            });
    }
}

impl App {
    pub fn new(objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>, transform: TSTransform) -> Self {
        Self {
            objects,
            moving: None,
            transform,
        }
    }

    fn update_zoom(&mut self, ctx: &egui::Context) {
        if let Some(real_mouse_pos) = ctx.input(|state| state.pointer.hover_pos()) {
            let delta_scale = ctx.input(|state| state.zoom_delta());

            if delta_scale != 1.0 {
                let new_real_mouse_pos =
                    (real_mouse_pos - self.transform.translation) / delta_scale;

                let delta = new_real_mouse_pos - real_mouse_pos + self.transform.translation;

                self.transform.translation += delta;
                self.transform.scaling *= delta_scale;
            }
        }
    }

    fn update_moving(&mut self, ctx: &egui::Context) {
        let (pressed, released, mouse_pos) = ctx.input(|state| {
            (
                state.pointer.primary_pressed(),
                state.pointer.primary_released(),
                state.pointer.hover_pos(),
            )
        });

        if pressed {
            self.moving = Some(Moving {
                origin: mouse_pos.unwrap(),
                old_translation: self.transform.translation,
            });
        } else if released {
            self.moving = None;
        }

        if let Some(Moving {
            origin,
            old_translation,
        }) = self.moving
        {
            let delta = mouse_pos.unwrap() - origin;

            self.transform.translation = old_translation + delta;
        }
    }
}

#[derive(Clone, Copy)]
pub struct Moving {
    pub origin: Pos2,
    pub old_translation: Vec2,
}