use std::sync::{Arc, RwLock};

use egui::emath::TSTransform;
use egui::{Align2, FontId, Frame, Key, Margin, Rounding, Sense, Stroke};
use egui::{Color32, Pos2, Rect, Vec2};

use crate::cosmos_object::CosmosObject;
use crate::utils::Painter;

const BACKGROUND_COLOR: Color32 = Color32::from_gray(27);
const GRID_COLOR: Color32 = Color32::from_gray(60);
const KM_PER_VPIXEL: f32 = 5e4;

pub struct App {
    pub objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,

    moving: Option<Moving>,
    adding: Option<Adding>,
    transform: TSTransform,
    adding_mass: f32,
    showed_quantity: Option<PhysicalQuantity>,
    quantity_scale: [f32; 4],
    cell_size: f32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.update_moving(ctx);
        self.update_adding(ctx);
        self.update_zoom(ctx);
        self.update_showed_quantity(ctx);

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::ZERO,
                outer_margin: Margin::ZERO,
                fill: BACKGROUND_COLOR,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let objects = self.objects.read().unwrap();
                let (_, ref painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                let painter = Painter {
                    raw: painter,
                    transform: self.transform,
                };

                self.draw_grid(painter, ui.min_size());

                for object in objects.iter() {
                    let object = &object.read().unwrap();

                    object.draw(painter);

                    if let Some(quantity) = self.showed_quantity {
                        let scale = self.quantity_scale[quantity as usize];

                        painter.vec(
                            object.position,
                            object.get_quantity(quantity) * scale,
                            quantity.color(),
                        );
                    }
                }

                self.draw_scale_info(painter, ui.min_size());

                let Some(mouse_pos) = ctx.input(|state| state.pointer.hover_pos()) else {
                    return;
                };

                if let Some(Adding { origin }) = self.adding {
                    let position = self.transform.inverse() * origin;
                    let velocity = (origin - mouse_pos) / self.transform.scaling;

                    CosmosObject {
                        mass: self.adding_mass,
                        position,
                        velocity,
                        ..Default::default()
                    }
                    .draw(painter);

                    let scale = self.quantity_scale[PhysicalQuantity::Velocity as usize];

                    painter.vec(position, velocity * scale, Color32::LIGHT_RED);

                    ctx.input(|state| {
                        for event in state.events.iter() {
                            if let egui::Event::MouseWheel { delta, .. } = event {
                                self.adding_mass *= 1.7f32.powf(delta.y);
                            }
                        }
                    });
                }
            });
    }
}

impl App {
    pub fn new(objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>, transform: TSTransform) -> Self {
        Self {
            objects,
            transform,

            moving: None,
            adding: None,
            adding_mass: 20.0,
            showed_quantity: None,
            quantity_scale: [1.0, 1.0, 1.0, 1.0],
            cell_size: 20.0,
        }
    }

    fn update_zoom(&mut self, ctx: &egui::Context) {
        if self.adding.is_some() {
            return;
        }

        ctx.input(|state| {
            for event in state.events.iter() {
                let egui::Event::MouseWheel { delta, .. } = event else {
                    continue;
                };

                let zoom_delta = 1.7f32.powf(delta.y);

                if state.modifiers.ctrl {
                    self.cell_size *= zoom_delta;
                } else if state.modifiers.shift {
                    if let Some(quantity) = self.showed_quantity {
                        self.quantity_scale[quantity as usize] *= zoom_delta;
                    }
                } else {
                    if let Some(real_mouse_pos) = state.pointer.hover_pos() {
                        self.zoom_relative_to(zoom_delta, real_mouse_pos);
                    }
                }
            }
        });
    }

    fn zoom_relative_to(&mut self, delta_scale: f32, point: Pos2) {
        if delta_scale != 1.0 {
            let new_real_mouse_pos = (point - self.transform.translation) / delta_scale;

            let delta = new_real_mouse_pos - point + self.transform.translation;

            self.transform.translation += delta;
            self.transform.scaling *= delta_scale;
        }
    }

    fn update_moving(&mut self, ctx: &egui::Context) {
        let (pressed, released, Some(mouse_pos)) = ctx.input(|state| {
            (
                state.pointer.primary_pressed(),
                state.pointer.primary_released(),
                state.pointer.hover_pos(),
            )
        }) else {
            return;
        };

        if pressed {
            self.moving = Some(Moving {
                origin: mouse_pos,
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
            let delta = mouse_pos - origin;

            self.transform.translation = old_translation + delta;
        }
    }

    fn update_adding(&mut self, ctx: &egui::Context) {
        let (pressed, released, Some(mouse_pos)) = ctx.input(|state| {
            (
                state.pointer.secondary_pressed(),
                state.pointer.secondary_released(),
                state.pointer.hover_pos(),
            )
        }) else {
            return;
        };

        if pressed {
            self.adding = Some(Adding { origin: mouse_pos })
        } else if released {
            let adding = self.adding.take().unwrap();

            let position = self.transform.inverse() * adding.origin;
            let velocity = (adding.origin - mouse_pos) / self.transform.scaling;

            let mut objects = self.objects.write().unwrap();

            objects.push(RwLock::new(CosmosObject {
                mass: self.adding_mass,
                position,
                velocity,
                ..Default::default()
            }));
        }
    }

    fn update_showed_quantity(&mut self, ctx: &egui::Context) {
        self.showed_quantity = ctx.input(|state| {
            if state.key_pressed(Key::Escape) {
                return None;
            }

            let pressed = if state.key_pressed(Key::V) {
                Some(PhysicalQuantity::Velocity)
            } else if state.key_pressed(Key::I) {
                Some(PhysicalQuantity::Impulse)
            } else if state.key_pressed(Key::A) {
                Some(PhysicalQuantity::Acceleration)
            } else if state.key_pressed(Key::F) {
                Some(PhysicalQuantity::Force)
            } else {
                None
            };

            if let Some(pressed) = pressed {
                if Some(pressed) == self.showed_quantity {
                    None
                } else {
                    Some(pressed)
                }
            } else {
                self.showed_quantity
            }
        });
    }

    fn draw_grid(&self, painter: Painter, ui_size: Vec2) {
        let start = self.transform.inverse() * Pos2::ZERO;
        let end = self.transform.inverse() * ui_size.to_pos2();

        let start = (start / self.cell_size).floor();
        let end = (end / self.cell_size).ceil();

        let stroke = Stroke::new(self.cell_size / 20.0, GRID_COLOR);

        for x in start.x as isize..end.x as isize {
            let points = [
                Pos2::new(x as f32, start.y) * self.cell_size,
                Pos2::new(x as f32, end.y) * self.cell_size,
            ];

            painter.line(points, stroke);
        }

        for y in start.y as isize..end.y as isize {
            let points = [
                Pos2::new(start.x, y as f32) * self.cell_size,
                Pos2::new(end.x, y as f32) * self.cell_size,
            ];

            painter.line(points, stroke);
        }
    }

    fn draw_scale_info(&self, painter: Painter, field_size: Vec2) {
        let size = Vec2::new(field_size.x, field_size.y * 0.05);
        let start = field_size.to_pos2() - size;

        painter.raw.rect(
            Rect::from_min_size(start, size),
            Rounding::ZERO,
            BACKGROUND_COLOR,
            Stroke::new(1.0, GRID_COLOR),
        );

        let height = size.y / 1.5;
        let segment_size = size.y / 1.5;
        let protrusion_size = segment_size / 6.0;

        let stroke = Stroke::new(3.0, GRID_COLOR);

        draw_segment(
            painter.raw,
            start + Vec2::splat(size.y / 2.0),
            segment_size,
            protrusion_size,
            stroke,
        );

        fn draw_segment(
            painter: &egui::Painter,
            pos: Pos2,
            size: f32,
            protrusion_size: f32,
            stroke: Stroke,
        ) {
            let half_protrusion = protrusion_size / 2.0;
            let protrusion = Vec2::new(0.0, half_protrusion);

            let points = [pos, pos + Vec2::new(size, 0.0)];

            painter.line_segment(points, stroke);
            painter.line_segment([points[0] + protrusion, points[0] - protrusion], stroke);
            painter.line_segment([points[1] + protrusion, points[1] - protrusion], stroke);
        }

        const KM_PER_PC: f32 = 30.8568e9;
        const KM_PER_LYR: f32 = 9.4607304725808e12;

        let km_on_side = self.cell_size * KM_PER_VPIXEL;
        let pc_on_side = km_on_side / KM_PER_PC;
        let lyr_per_side = km_on_side / KM_PER_LYR;

        let text = format!("{lyr_per_side:e}lyr = {pc_on_side:e}pc = {km_on_side:e}km");
        let font_size = height * 0.7;

        painter.raw.text(
            start + Vec2::new(size.y + segment_size, size.y * 0.5 - font_size / 2.0),
            Align2::LEFT_TOP,
            text,
            FontId::new(font_size, egui::FontFamily::Monospace),
            Color32::from_gray(150),
        );
    }
}

#[derive(Clone, Copy)]
pub struct Moving {
    pub origin: Pos2,
    pub old_translation: Vec2,
}

#[derive(Clone, Copy)]
pub struct Adding {
    pub origin: Pos2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PhysicalQuantity {
    Velocity = 0,
    Impulse = 1,
    Acceleration = 2,
    Force = 3,
}

impl PhysicalQuantity {
    pub fn color(self) -> Color32 {
        match self {
            PhysicalQuantity::Velocity => Color32::LIGHT_RED,
            PhysicalQuantity::Impulse => Color32::LIGHT_BLUE,
            PhysicalQuantity::Acceleration => Color32::LIGHT_GREEN,
            PhysicalQuantity::Force => Color32::LIGHT_YELLOW,
        }
    }
}
