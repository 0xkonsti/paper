use std::hash::{Hash, Hasher};

use paper_utils::hash_f32;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Srgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Srgba {
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }

    pub fn rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: a as f32 / 255.0 }
    }

    pub fn rgb_u8(r: u8, g: u8, b: u8) -> Self {
        Self { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
    }

    pub fn hexa(hex: u64) -> Option<Self> {
        if hex > 0xFFFFFFFF {
            return None;
        }

        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;

        Some(Self { r, g, b, a })
    }

    pub fn hex(hex: u32) -> Option<Self> {
        if hex > 0xFFFFFF {
            return None;
        }

        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;

        Some(Self { r, g, b, a: 1.0 })
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn with_red(mut self, r: f32) -> Self {
        self.r = r;
        self
    }

    pub fn with_green(mut self, g: f32) -> Self {
        self.g = g;
        self
    }

    pub fn with_blue(mut self, b: f32) -> Self {
        self.b = b;
        self
    }
}

impl Hash for Srgba {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_f32(state, self.r);
        hash_f32(state, self.g);
        hash_f32(state, self.b);
        hash_f32(state, self.a);
    }
}

impl From<Srgba> for [f32; 4] {
    fn from(color: Srgba) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

impl From<Srgba> for [f32; 3] {
    fn from(color: Srgba) -> Self {
        [color.r, color.g, color.b]
    }
}

impl From<Srgba> for [f64; 4] {
    fn from(color: Srgba) -> Self {
        [color.r as f64, color.g as f64, color.b as f64, color.a as f64]
    }
}

impl From<Srgba> for [f64; 3] {
    fn from(color: Srgba) -> Self {
        [color.r as f64, color.g as f64, color.b as f64]
    }
}

impl From<Srgba> for [u8; 4] {
    fn from(color: Srgba) -> Self {
        [(color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8, (color.a * 255.0) as u8]
    }
}

impl From<Srgba> for [u8; 3] {
    fn from(color: Srgba) -> Self {
        [(color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8]
    }
}
