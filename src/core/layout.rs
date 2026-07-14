//! Layout properties.
//! 
//! # How does it work
//! 
//! In plasticism, we invoke twice to collect **Space Requirements** and **Allocate Space**.
//! 
//! First invocation ~~passes down bounds to children~~ and then they return requirements. After that,
//! widgets allocate space by the requirements by another invocation.

use crate::core::utils::Mix;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl Padding {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Padding { top, right, bottom, left }
    }
    pub fn same(space: f64) -> Self {
        Padding { top: space, right: space, bottom: space, left: space }
    }
    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        Padding { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}
impl Mix for Padding {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        Self {
            top: self.top.mix(&other.top, fac),
            right: self.right.mix(&other.right, fac),
            bottom: self.bottom.mix(&other.bottom, fac),
            left: self.left.mix(&other.left, fac),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Margin {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}
impl Margin {
    pub fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Margin { top, right, bottom, left }
    }
    pub fn same(space: f64) -> Self {
        Margin { top: space, right: space, bottom: space, left: space }
    }
    pub fn symmetric(vertical: f64, horizontal: f64) -> Self {
        Margin { top: vertical, right: horizontal, bottom: vertical, left: horizontal }
    }
}
impl Mix for Margin {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        Self {
            top: self.top.mix(&other.top, fac),
            right: self.right.mix(&other.right, fac),
            bottom: self.bottom.mix(&other.bottom, fac),
            left: self.left.mix(&other.left, fac),
        }
    }
}

/// Widget size setting enum.
/// 
/// When allocating space, widgets will first preallocate by children's base length, then
/// allocate remaining space by grow-factors.
/// 
/// See [`SpaceRequirement`](crate::core::measurement::SpaceRequirement) for what generated
/// when calculating layout.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Length {
    /// Acts as `Growable { base: ..., factor: 0 }`
    Fixed {
        length: f64
    },

    Growable {
        base: f64,
        factor: f64,
    },

    /// Acts as `Growable { base: 0, factor: ... }`
    Fraction {
        factor: f64
    },

    /// Acts as `Growable { base: <Min>, factor: 0 }`
    #[default]
    Min,

    /// Acts as `Growable { base: <Min>, factor: ... }`
    GrowFromMin {
        factor: f64
    },

    /// Acts as `Growable { base: <Min>, factor: Infinity }`
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size {
    pub width: Length,
    pub height: Length,
}
impl Size {
    pub fn new(width: Length, height: Length) -> Self {
        Size { width, height }
    }
}
