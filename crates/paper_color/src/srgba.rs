use paper_utils::Flattenable;

use crate::LinearRgba;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Srgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Srgba {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }

    pub const fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self::rgba_u8(r, g, b, 255)
    }

    pub const fn hexa(rgba: u32) -> Self {
        let r = (rgba >> 24) as u8;
        let g = (rgba >> 16) as u8;
        let b = (rgba >> 8) as u8;
        let a = rgba as u8;
        Self::rgba_u8(r, g, b, a)
    }

    pub const fn hex(rgb: u32) -> Self {
        let r = (rgb >> 16) as u8;
        let g = (rgb >> 8) as u8;
        let b = rgb as u8;
        Self::rgb_u8(r, g, b)
    }

    /// Into linear space [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_correct(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }
        if value <= 0.04045 { value / 12.92 } else { f32::powf((value + 0.055) / 1.055, 2.4) }
    }

    /// from linear space [gamma correction](https://en.wikipedia.org/wiki/Gamma_correction).
    pub fn gamma_correct_inv(value: f32) -> f32 {
        if value <= 0.0 {
            return value;
        }

        if value <= 0.0031308 { value * 12.92 } else { (1.055 * f32::powf(value, 1.0 / 2.4)) - 0.055 }
    }
}

impl From<LinearRgba> for Srgba {
    #[inline]
    fn from(value: LinearRgba) -> Self {
        Self {
            r: Srgba::gamma_correct_inv(value.r),
            g: Srgba::gamma_correct_inv(value.g),
            b: Srgba::gamma_correct_inv(value.b),
            a: value.a,
        }
    }
}

impl From<Srgba> for LinearRgba {
    #[inline]
    fn from(value: Srgba) -> Self {
        Self {
            r: Srgba::gamma_correct(value.r),
            g: Srgba::gamma_correct(value.g),
            b: Srgba::gamma_correct(value.b),
            a: value.a,
        }
    }
}

impl Flattenable<f32> for Srgba {
    fn flatten(self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}
