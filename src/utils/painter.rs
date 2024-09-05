use egui::emath::TSTransform;
use egui::epaint::CircleShape;
use egui::{Color32, Pos2, Shape, Stroke};

#[derive(Clone, Copy)]
pub struct Painter<'frame> {
    pub painter: &'frame egui::Painter,
    pub transform: TSTransform,
}

impl<'frame> Painter<'frame> {
    pub fn draw(&self, shape: impl Into<Shape>) {
        let mut shape = shape.into();

        shape.transform(self.transform);

        self.painter.add(shape);
    }

    pub fn circle(
        &self,
        center: Pos2,
        radius: f32,
        fill_color: Color32,
        stroke: impl Into<Stroke>,
    ) {
        self.draw(CircleShape {
            center,
            radius,
            fill: fill_color.into(),
            stroke: stroke.into(),
        });
    }
}
