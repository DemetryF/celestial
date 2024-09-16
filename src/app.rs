use std::sync::{Arc, RwLock};

use egui::emath::TSTransform;
use egui::{Align2, FontId, Frame, Key, Margin, Rounding, Sense, Stroke};
use egui::{Color32, Pos2, Rect, Vec2};

use crate::cosmos_object::CosmosObject;
use crate::physics::{SimulationState, KM_PER_VPX};
use crate::utils::{format_time, format_time_ord, Painter};

const BACKGROUND_COLOR: Color32 = Color32::from_gray(27);
const GRID_COLOR: Color32 = Color32::from_gray(60);

pub struct App {
    pub objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,

    sim_state: &'static SimulationState,

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
                let (_, ref painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                let painter = Painter {
                    raw: painter,
                    transform: self.transform,
                };

                self.draw_grid(painter, ui.min_size());

                let objects = self.objects.read().unwrap();

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

                self.show_info(painter, ui.min_size());

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
    pub fn new(
        objects: Arc<RwLock<Vec<RwLock<CosmosObject>>>>,
        transform: TSTransform,
        sim_state: &'static SimulationState,
    ) -> Self {
        Self {
            objects,
            transform,

            sim_state,

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
            // mouse coords in virtual space
            let vmouse = self.transform.inverse() * point;

            // apply zoom to transform
            self.transform.scaling *= delta_scale;

            // new real coords of point, the mouse pointed to,
            let new_rmouse = self.transform * vmouse;

            // shift to which transform.translation should be shifted
            let shift = point - new_rmouse;

            // apply shift
            self.transform.translation += shift;
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

    fn show_info(&self, painter: Painter, field_size: Vec2) {
        let box_size = Vec2::new(field_size.x, field_size.y * 0.1);
        let box_start = field_size.to_pos2() - box_size;

        painter.raw.rect(
            Rect::from_min_size(box_start, box_size),
            Rounding::ZERO,
            BACKGROUND_COLOR,
            Stroke::new(1.0, GRID_COLOR),
        );

        fn draw_segment(
            painter: &egui::Painter,
            pos: Pos2,
            size: f32,
            protrusion_size: f32,
            stroke: Stroke,
        ) {
            let half_protrusion = protrusion_size / 2.0;
            let protrusion = Vec2::new(0., half_protrusion);

            let points = [pos, pos + Vec2::new(size, 0.0)];

            painter.line_segment(points, stroke);
            painter.line_segment([points[0] + protrusion, points[0] - protrusion], stroke);
            painter.line_segment([points[1] + protrusion, points[1] - protrusion], stroke);
        }

        let height = box_size.y / 3.;
        let segment_size = box_size.y / 3.;
        let protrusion_size = segment_size / 12.;

        let stroke = Stroke::new(3., GRID_COLOR);

        draw_segment(
            painter.raw,
            box_start + Vec2::splat(box_size.y / 4.0),
            segment_size,
            protrusion_size,
            stroke,
        );

        const KM_PER_PC: f32 = 30.8568e9;
        const KM_PER_LYR: f32 = 9.4607304725808e12;

        let km_on_side = self.cell_size * KM_PER_VPX;
        let pc_on_side = km_on_side / KM_PER_PC;
        let lyr_per_side = km_on_side / KM_PER_LYR;

        let scale_info_text =
            format!("{lyr_per_side:.2e}lyr = {pc_on_side:.2e}pc = {km_on_side:.2e}km");

        let font_size = height * 0.7;

        let scale_info_text_pos = box_start
            + Vec2::new(
                box_size.y / 2. + segment_size,
                box_size.y * 0.3 - font_size / 2.,
            );

        let font_id = FontId::new(font_size, egui::FontFamily::Monospace);

        painter.raw.text(
            scale_info_text_pos,
            Align2::LEFT_TOP,
            scale_info_text,
            font_id.clone(),
            Color32::from_gray(150),
        );

        let elapsed_text = format_time_ord(self.sim_state.elapsed() as usize);

        painter.raw.text(
            Pos2::new(box_size.x - box_size.y / 4., scale_info_text_pos.y),
            Align2::RIGHT_TOP,
            elapsed_text,
            font_id.clone(),
            Color32::from_gray(150),
        );

        if let Some(quantity) = self.showed_quantity {
            let stroke = Stroke::new(3., quantity.color());

            draw_segment(
                painter.raw,
                box_start + Vec2::new(box_size.y / 4., box_size.y * 3. / 4.),
                segment_size,
                protrusion_size,
                stroke,
            );

            let quantity_in_side = km_on_side / self.quantity_scale[quantity as usize];

            let quantity_info_text = format!("{quantity_in_side:.2e}{}", quantity.unit_name());

            let quantity_info_pos = box_start
                + Vec2::new(
                    box_size.y / 2. + segment_size,
                    box_size.y * 0.7 - font_size / 2.,
                );

            painter.raw.text(
                quantity_info_pos,
                Align2::LEFT_TOP,
                quantity_info_text,
                font_id.clone(),
                Color32::from_gray(150),
            );
        }

        let time_speed_info_text = format!(
            "{} in 1 sec",
            format_time(self.sim_state.time_speed() as usize)
        );

        let time_speed_info_pos = box_start
            + Vec2::new(
                box_size.x - box_size.y / 4.,
                box_size.y * 0.7 - font_size / 2.,
            );

        painter.raw.text(
            time_speed_info_pos,
            Align2::RIGHT_TOP,
            time_speed_info_text,
            font_id.clone(),
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
            PhysicalQuantity::Impulse => Color32::from_rgb(156, 245, 255),
            PhysicalQuantity::Acceleration => Color32::LIGHT_GREEN,
            PhysicalQuantity::Force => Color32::from_rgb(250, 255, 105),
        }
    }

    pub fn unit_name(self) -> &'static str {
        match self {
            PhysicalQuantity::Velocity => "km/sec",
            PhysicalQuantity::Impulse => "kg*km/sec",
            PhysicalQuantity::Acceleration => "km/sec^2",
            PhysicalQuantity::Force => "kg*km/sec^2",
        }
    }
}
