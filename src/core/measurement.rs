//! Those structs are only designed to use in layout calculation and rendering.
//! 
//! For layout styling, see [`crate::core::layout`]

use std::ops::{Add, Div, Mul, Sub};

use winit::dpi::{LogicalPosition, LogicalSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Space {
    pub width: f64,
    pub height: f64,
}
impl Space {
    pub fn new(width: f64, height: f64) -> Self {
        Space { width, height }
    }
}
impl From<Space> for LogicalSize<f64> {
    fn from(value: Space) -> Self {
        LogicalSize { width: value.width, height: value.height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}
impl From<Point> for LogicalPosition<f64> {
    fn from(value: Point) -> Self {
        LogicalPosition { x: value.x, y: value.y }
    }
}

pub struct Insets {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl Insets {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Insets { top, right, bottom, left }
    }
    pub fn same(space: f64) -> Self {
        Insets { top: space, right: space, bottom: space, left: space }
    }
    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        Insets { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}
impl Add for Insets {
    type Output = Insets;
    fn add(self, rhs: Self) -> Self::Output {
        Insets {
            top: self.top + rhs.top,
            right: self.right + rhs.right,
            bottom: self.bottom + rhs.bottom,
            left: self.left + rhs.left,
        }
    }
}
impl Sub for Insets {
    type Output = Insets;
    fn sub(self, rhs: Self) -> Self::Output {
        Insets {
            top: self.top - rhs.top,
            right: self.right - rhs.right,
            bottom: self.bottom - rhs.bottom,
            left: self.left - rhs.left,
        }
    }
}
impl Mul<f64> for Insets {
    type Output = Insets;
    fn mul(self, rhs: f64) -> Self::Output {
        Insets {
            top: self.top * rhs,
            right: self.right * rhs,
            bottom: self.bottom * rhs,
            left: self.left * rhs,
        }
    }
}
impl Div<f64> for Insets {
    type Output = Insets;
    fn div(self, rhs: f64) -> Self::Output {
        Insets {
            top: self.top / rhs,
            right: self.right / rhs,
            bottom: self.bottom / rhs,
            left: self.left / rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect { x, y, width, height }
    }
    pub fn merge(point: Point, space: Space) -> Self {
        Rect { x: point.x, y: point.y, width: space.width, height: space.height }
    }
    pub fn expand(&self, rhs: Insets) -> Self {
        Rect {
            x: self.x - rhs.left,
            y: self.y - rhs.top,
            width: self.width + rhs.left + rhs.right,
            height: self.height + rhs.top + rhs.bottom,
        }
    }
    pub fn inset(&self, rhs: Insets) -> Self {
        Rect {
            x: self.x + rhs.left,
            y: self.y + rhs.top,
            width: self.width - rhs.left - rhs.right,
            height: self.height - rhs.top - rhs.bottom,
        }
    }
}
impl From<Rect> for Point {
    fn from(value: Rect) -> Self {
        Point { x: value.x, y: value.y }
    }
}
impl From<Rect> for Space {
    fn from(value: Rect) -> Self {
        Space { width: value.width, height: value.height }
    }
}
impl From<Rect> for LogicalPosition<f64> {
    fn from(value: Rect) -> Self {
        LogicalPosition { x: value.x, y: value.y }
    }
}
impl From<Rect> for LogicalSize<f64> {
    fn from(value: Rect) -> Self {
        LogicalSize { width: value.width, height: value.height }
    }
}
/// Impl for [`grafo::Shape`]
impl From<Rect> for [(f32, f32); 2] {
    fn from(value: Rect) -> Self {
        [(value.x as f32, value.y as f32), (value.width as f32, value.height as f32)]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LengthRequirement {
    pub base: f64,
    pub factor: f64
}
impl LengthRequirement {
    pub fn new(base: f64, factor: f64) -> Self {
        LengthRequirement { base, factor }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpaceRequirement {
    pub width: LengthRequirement,
    pub height: LengthRequirement,
}
impl SpaceRequirement {
    pub fn new(width: LengthRequirement, height: LengthRequirement) -> Self {
        SpaceRequirement { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpaceAllocation {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
impl From<SpaceAllocation> for Rect {
    fn from(value: SpaceAllocation) -> Self {
        Rect { x: value.x, y: value.y, width: value.width, height: value.height }
    }
}
