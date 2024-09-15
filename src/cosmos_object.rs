use egui::{Color32, Pos2, Stroke, Vec2};

use crate::app::PhysicalQuantity;
use crate::utils::Painter;

#[derive(Default)]
pub struct CosmosObject {
    /// mass in kilograms
    pub mass: f32,

    /// radius in vpx
    pub radius: f32,

    /// position in vpx
    pub position: Pos2,

    /// velocity in vpx/sec
    pub velocity: Vec2,

    /// acceleration in vpx/sec^2
    pub acceleration: Vec2,
}

impl CosmosObject {
    pub fn draw(&self, painter: Painter) {
        painter.circle(self.position, self.radius, Color32::GRAY, Stroke::NONE);
    }

    pub fn orbit(
        mut self,
        other: &Self,
        orbit_radius: f32,
        anomaly: f32,
        dir: f32,
        gravitional_const: f32,
    ) -> Self {
        let speed = (gravitional_const * other.mass / orbit_radius).sqrt();

        let u = Vec2::angled(anomaly);

        self.position = other.position + u * orbit_radius;
        self.velocity = dir * u.rot90() * speed;

        self
    }

    pub fn get_quantity(&self, quantity: PhysicalQuantity) -> Vec2 {
        match quantity {
            PhysicalQuantity::Velocity => self.velocity,
            PhysicalQuantity::Impulse => self.velocity * self.mass,
            PhysicalQuantity::Acceleration => self.acceleration,
            PhysicalQuantity::Force => self.acceleration * self.mass,
        }
    }
}
