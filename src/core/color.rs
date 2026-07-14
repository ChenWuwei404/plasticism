use crate::core::utils::{oklab_to_rgb, oklch_to_oklab};

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
impl From<Color> for protextinator::style::FontColor {
    fn from(value: Color) -> Self {
        let clamped = value.clamp();
        protextinator::style::FontColor::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
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
impl From<Oklab> for protextinator::style::FontColor {
    fn from(value: Oklab) -> Self {
        let rgba: Color = value.into();
        let clamped = rgba.clamp();
        protextinator::style::FontColor::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
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
impl From<Oklch> for protextinator::style::FontColor {
    fn from(value: Oklch) -> Self {
        let rgba: Color = value.into();
        let clamped = rgba.clamp();
        protextinator::style::FontColor::rgba(
            (clamped.r * 255.0) as u8,
            (clamped.g * 255.0) as u8,
            (clamped.b * 255.0) as u8,
            (clamped.alpha * 255.0) as u8,
        )
    }
}
