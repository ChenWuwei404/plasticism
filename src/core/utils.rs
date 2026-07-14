use std::f64::consts::PI;
use std::ops::{Add, Mul};

pub fn degree_to_rad(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn rad_to_degree(rad: f64) -> f64 {
    rad * 180.0 / PI
}

pub fn oklab_to_rgb(l: f64, a: f64, b: f64, clamp: bool) -> (f64, f64, f64) {
    const M1_INV: [[f64; 3]; 3] = [
        [1.0, 0.3963377774, 0.2158037573],
        [1.0, -0.1055613458, -0.0638541728],
        [1.0, -0.0894841775, -1.2914855480],
    ];

    const M2_INV: [[f64; 3]; 3] = [
        [4.0767416621, -3.3077115913, 0.2309699292],
        [-1.2684380046, 2.6097574011, -0.3413193965],
        [-0.0041960863, -0.7034186147, 1.7076147010],
    ];

    fn mat_vec_mul(mat: &[[f64; 3]; 3], vec: &[f64; 3]) -> [f64; 3] {
        let mut result = [0.0; 3];
        for i in 0..3 {
            result[i] = mat[i][0] * vec[0] + mat[i][1] * vec[1] + mat[i][2] * vec[2];
        }
        result
    }

    fn gamma_correction(x: f64) -> f64 {
        if x <= 0.0031308 {
            12.92 * x
        } else {
            1.055 * x.powf(1.0 / 2.4) - 0.055
        }
    }

    let lab = [l, a, b];
    let lms_linear = mat_vec_mul(&M1_INV, &lab);

    let lms = [lms_linear[0].powi(3), lms_linear[1].powi(3), lms_linear[2].powi(3)];

    let rgb_linear = mat_vec_mul(&M2_INV, &lms);

    let mut rgb = [
        gamma_correction(rgb_linear[0]),
        gamma_correction(rgb_linear[1]),
        gamma_correction(rgb_linear[2]),
    ];

    if clamp {
        for val in &mut rgb {
            if *val < 0.0 {
                *val = 0.0;
            } else if *val > 1.0 {
                *val = 1.0;
            }
        }
    }

    (rgb[0], rgb[1], rgb[2])
}

pub fn oklch_to_oklab(l: f64, c: f64, h: f64) -> (f64, f64, f64) {
    (l, c * degree_to_rad(h).cos(), c * degree_to_rad(h).sin())
}

pub trait Mix {
    fn mix(&self, other: &Self, fac: f64) -> Self;
}

impl<T> Mix for T 
where 
    T: Copy + Mul<f64, Output = T> + Add<T, Output = T>,
{
    fn mix(&self, other: &Self, fac: f64) -> Self {
        *self * (1.0 - fac) + *other * fac
    }
}