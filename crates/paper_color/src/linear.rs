use paper_utils::Flattenable;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LinearRgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl LinearRgba {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<[f32; 4]> for LinearRgba {
    fn into(self) -> [f32; 4] {
        self.as_array()
    }
}

impl Flattenable<f32> for LinearRgba {
    fn flatten(self) -> Vec<f32> {
        vec![self.r, self.g, self.b, self.a]
    }
}
