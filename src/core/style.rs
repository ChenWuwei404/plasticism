use std::ops::{Add, Sub};

use grafo::BorderRadii;

use crate::core::utils::Mix;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Radii {
    pub top_right: f64,
    pub bottom_right: f64,
    pub bottom_left: f64,
    pub top_left: f64,
}
impl Radii {
    pub fn new(top_right: f64, bottom_right: f64, bottom_left: f64, top_left: f64) -> Self {
        Radii { top_right, bottom_right, bottom_left, top_left }
    }
    pub fn same(radius: f64) -> Self {
        Radii { top_right: radius, bottom_right: radius, bottom_left: radius, top_left: radius }
    }
    pub fn is_empty(&self) -> bool {
        self.top_right == 0. && self.bottom_right == 0. && self.bottom_left == 0. && self.top_left == 0.
    }
}
impl Add for Radii {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Radii {
            top_right: self.top_right + rhs.top_right,
            bottom_right: self.bottom_right + rhs.bottom_right,
            bottom_left: self.bottom_left + rhs.bottom_left,
            top_left: self.top_left + rhs.top_left,
        }
    }
}
impl Add<f64> for Radii {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Radii {
            top_right: self.top_right + rhs,
            bottom_right: self.bottom_right + rhs,
            bottom_left: self.bottom_left + rhs,
            top_left: self.top_left + rhs,
        }
    }
}
impl Sub for Radii {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Radii {
            top_right: self.top_right - rhs.top_right,
            bottom_right: self.bottom_right - rhs.bottom_right,
            bottom_left: self.bottom_left - rhs.bottom_left,
            top_left: self.top_left - rhs.top_left,
        }
    }
}
impl Sub<f64> for Radii {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Radii {
            top_right: self.top_right - rhs,
            bottom_right: self.bottom_right - rhs,
            bottom_left: self.bottom_left - rhs,
            top_left: self.top_left - rhs,
        }
    }
}
impl Mix for Radii {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        Radii {
            top_right: (1.0 - fac) * self.top_right + fac * other.top_right,
            bottom_right: (1.0 - fac) * self.bottom_right + fac * other.bottom_right,
            bottom_left: (1.0 - fac) * self.bottom_left + fac * other.bottom_left,
            top_left: (1.0 - fac) * self.top_left + fac * other.top_left,
        }
    }
}
impl From<Radii> for BorderRadii {
    fn from(value: Radii) -> Self {
        BorderRadii {
            top_left: value.top_left as f32,
            top_right: value.top_right as f32,
            bottom_left: value.bottom_left as f32,
            bottom_right: value.bottom_right as f32,
        }
    }
}