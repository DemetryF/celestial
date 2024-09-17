use egui::emath::{Rot2, TSTransform};
use egui::epaint::{CircleShape, PathShape};
use egui::{Color32, Pos2, Shape, Stroke, Vec2};

#[derive(Clone, Copy)]
pub struct Painter<'frame> {
    pub raw: &'frame egui::Painter,
    pub transform: TSTransform,
}

impl<'frame> Painter<'frame> {
    pub fn draw(&self, shape: impl Into<Shape>) {
        let mut shape = shape.into();

        shape.transform(self.transform);

        self.raw.add(shape);
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
            fill: fill_color,
            stroke: stroke.into(),
        });
    }

    pub fn line(&self, points: [Pos2; 2], stroke: impl Into<Stroke>) {
        self.draw(Shape::LineSegment {
            points,
            stroke: stroke.into().into(),
        })
    }

    pub fn vec(&self, origin: Pos2, vec: Vec2, stroke: Stroke) {
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.);

        let tip_length = stroke.width * 0.6;
        let tip = origin + vec;
        let dir = vec.normalized();

        self.line([origin, tip], stroke);

        self.draw(PathShape {
            points: vec![
                tip,
                tip - tip_length * (rot * dir),
                tip - tip_length * (rot.inverse() * dir),
            ],
            closed: true,
            fill: stroke.color,
            stroke: stroke.into(),
        });
    }
}
