use grafo::{DrawCommandError, Renderer, Shape, ShapeDrawCommandOptions, Stroke};

use crate::core::{color::Color, measurement::Rect, style::Radii};

use super::Atom;

pub struct Rectangle {
    color: Color,
    radii: Radii
}
impl Rectangle {
    pub fn new() -> Self {
        Rectangle { color: Color::default(), radii: Radii::default() }
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }
    pub fn set_radii(&mut self, radii: Radii) {
        self.radii = radii
    }
}
impl Atom for Rectangle {
    fn draw(&self, rect: &Rect, renderer: &mut Renderer, parent_shape_id: Option<usize>) -> Result<usize, DrawCommandError> {
        let shape = if self.radii.is_empty() {
            Shape::rect(
                (*rect).into(),
                Stroke { width: 0.0, color: Color::TRANSPARENT.into()}
            )
        } else {
            Shape::rounded_rect(
                (*rect).into(),
                self.radii.into(),
                Stroke { width: 0.0, color: Color::TRANSPARENT.into()}
            )
        };
        renderer.add_shape(
            shape,
            parent_shape_id,
            None,
            ShapeDrawCommandOptions::default()
                .color(self.color.into())
        )
    }
}

pub struct RectangleBorder {
    color: Color,
    width: f64,
    radii: Radii
}
impl RectangleBorder {
    pub fn new() -> Self {
        RectangleBorder { color: Color::WHITE, width: 1.0, radii: Radii::default() }
    }

    pub fn set_color(&mut self, color: impl Into<Color>) {
        self.color = color.into();
    }
    pub fn set_width(&mut self, width: f64) {
        self.width = width
    }
    pub fn set_radii(&mut self, radii: Radii) {
        self.radii = radii
    }
}
impl Atom for RectangleBorder {
    fn draw(&self, rect: &Rect, renderer: &mut Renderer, parent_shape_id: Option<usize>) -> Result<usize, DrawCommandError> {
        let shape = if self.radii.is_empty() {
            Shape::rect(
                (*rect).into(),
                Stroke { width: self.width as f32, color: self.color.into()}
            )
        } else {
            Shape::rounded_rect(
                (*rect).into(),
                self.radii.into(),
                Stroke { width: self.width as f32, color: self.color.into()}
            )
        };
        renderer.add_shape(
            shape,
            parent_shape_id,
            None,
            ShapeDrawCommandOptions::default()
        )
    }
}
