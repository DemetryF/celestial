use egui::{Color32, Pos2, Stroke, Vec2};

use crate::utils::Painter;

#[derive(Default)]
pub struct CosmosObject {
    pub mass: f32,

    pub position: Pos2,
    pub velocity: Vec2,
}

impl CosmosObject {
    pub fn draw(&self, painter: Painter) {
        painter.circle(self.position, self.radius(), Color32::GRAY, Stroke::NONE);
    }

    pub fn orbit(mut self, other: &Self, orbit_radius: f32, anomaly: f32, dir: f32) -> Self {
        let speed = (other.mass / orbit_radius).sqrt();

        let u = Vec2::angled(anomaly);

        self.position = other.position + u * orbit_radius;
        self.velocity = dir * u.rot90() * speed;

        self
    }

    pub fn radius(&self) -> f32 {
        self.mass.sqrt()
    }
}
