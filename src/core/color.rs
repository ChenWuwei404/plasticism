use crate::core::utils::{Mix, oklab_to_rgb, oklch_to_oklab};

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r: r as f64 / 255.0, g: g as f64 / 255.0, b: b as f64 / 255.0, alpha: 1.0 }
}
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r: r as f64 / 255.0, g: g as f64 / 255.0, b: b as f64 / 255.0, alpha: a as f64 / 255.0 }
}

/// Generic color struct, whose fields varying from 0.0 to 1.0
/// 
/// # Examples
/// 
/// ```rust
/// use plasticism::core::color::Color;
/// 
/// let red = Color::rgb(1.0, 0.0, 0.0);
/// 
/// let red_grafo: grafo::Color = red.into();
/// let red_style: protextinator::style::FontColor = red.into();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub alpha: f64,
}
impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, alpha: 1.0 }
    }
    pub fn rgba(r: f64, g: f64, b: f64, alpha: f64) -> Color {
        Color { r, g, b, alpha }
    }
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            alpha: self.alpha.clamp(0.0, 1.0),
        }
    }

    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, alpha: 1.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, alpha: 1.0 };
    pub const TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, alpha: 0.0 };
}
impl From<Color> for grafo::Color {
    fn from(value: Color) -> Self {
        let clamped = value.clamp();
        grafo::Color::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
    }
}
impl Mix for Color {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        if self.alpha == 0.0 {
            return Color { r: other.r, g: other.g, b: other.b, alpha: fac * other.alpha }
        }
        if other.alpha == 0.0 {
            return Color { r: self.r, g: self.g, b: self.b, alpha: (1.0 - fac) * self.alpha }
        }
        return Color {
            r: self.r.mix(&other.r, fac),
            g: self.g.mix(&other.g, fac),
            b: self.b.mix(&other.b, fac),
            alpha: self.alpha.mix(&other.alpha, fac),
        }
    }
}

/// [Oklab](https://bottosson.github.io/posts/oklab/) color, implemented `Into<T>` for
/// `Color`, `grafo::Color` and `protextinator::style::FontColor`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
    pub alpha: f64,
}
impl Oklab {
    pub fn lab(l: f64, a: f64, b: f64) -> Oklab {
        Oklab { l, a, b, alpha: 1.0 }
    }
    pub fn laba(l: f64, a: f64, b: f64, alpha: f64) -> Oklab {
        Oklab { l, a, b, alpha }
    }

    pub const BLACK: Oklab = Oklab { l: 0.0, a: 0.0, b: 0.0, alpha: 1.0 };
    pub const WHITE: Oklab = Oklab { l: 1.0, a: 0.0, b: 0.0, alpha: 1.0 };
    pub const TRANSPARENT: Oklab = Oklab { l: 0.0, a: 0.0, b: 0.0, alpha: 0.0 };
}
impl From<Oklab> for Color {
    fn from(value: Oklab) -> Self {
        let (r, g, b) = oklab_to_rgb(value.l, value.a, value.b, false);
        Color::rgba(r, g, b, value.alpha)
    }
}
impl From<Oklab> for grafo::Color {
    fn from(value: Oklab) -> Self {
        let rgba: Color = value.into();
        let clamped = rgba.clamp();
        grafo::Color::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
    }
}
impl Mix for Oklab {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        if self.alpha == 0.0 {
            return Oklab { l: other.l, a: other.a, b: other.b, alpha: fac * other.alpha }
        }
        if other.alpha == 0.0 {
            return Oklab { l: self.l, a: self.a, b: self.b, alpha: (1.0 - fac) * self.alpha }
        }
        return Oklab {
            l: self.l.mix(&other.l, fac),
            a: self.a.mix(&other.a, fac),
            b: self.b.mix(&other.b, fac),
            alpha: self.alpha.mix(&other.alpha, fac),
        }
    }
}

/// [Oklch](https://bottosson.github.io/posts/oklab/) color, implemented `Into<T>` for
/// `Oklab`, `Color`, `grafo::Color` and `protextinator::style::FontColor`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
    pub alpha: f64,
}
impl Oklch {
    pub fn lch(l: f64, c: f64, h: f64) -> Oklch {
        Oklch { l, c, h, alpha: 1.0 }
    }
    pub fn lcha(l: f64, c: f64, h: f64, alpha: f64) -> Oklch{
        Oklch { l, c, h, alpha }
    }

    pub const BLACK: Oklch = Oklch { l: 0.0, c: 0.0, h: 0.0, alpha: 1.0 };
    pub const WHITE: Oklch = Oklch { l: 1.0, c: 0.0, h: 0.0, alpha: 1.0 };
    pub const TRANSPARENT: Oklch = Oklch { l: 0.0, c: 0.0, h: 0.0, alpha: 0.0 };
}
impl From<Oklch> for Oklab {
    fn from(value: Oklch) -> Self {
        let (l, a, b) = oklch_to_oklab(value.l, value.c, value.h);
        Oklab { l, a, b, alpha: value.alpha }
    }
}
impl From<Oklch> for Color {
    fn from(value: Oklch) -> Self {
        let (l, a, b) = oklch_to_oklab(value.l, value.c, value.h);
        let (r, g, b) = oklab_to_rgb(l, a, b, false);
        Color::rgba(r, g, b, value.alpha)
    }
}
impl From<Oklch> for grafo::Color {
    fn from(value: Oklch) -> Self {
        let rgba: Color = value.into();
        let clamped = rgba.clamp();
        grafo::Color::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
    }
}
impl Mix for Oklch {
    fn mix(&self, other: &Self, fac: f64) -> Self {
        if self.alpha == 0.0 {
            return Oklch { l: other.l, c: other.c, h: other.h, alpha: fac * other.alpha }
        }
        if other.alpha == 0.0 {
            return Oklch { l: self.l, c: self.c, h: self.h, alpha: (1.0 - fac) * self.alpha }
        }
        return Oklch {
            l: self.l.mix(&other.l, fac),
            c: self.c.mix(&other.c, fac),
            h: self.h.mix(&other.h, fac),
            alpha: self.alpha.mix(&other.alpha, fac),
        }
    }
}
