use glam::{Mat4, Quat, Vec3};
use paper_utils::default;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation:    Quat,
    pub scale:       Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self { translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::ONE };

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { translation, rotation, scale }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self { translation, ..default() }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self { rotation, ..default() }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self { scale, ..default() }
    }

    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_translation(&mut self, translation: Vec3) -> &mut Self {
        self.translation = translation;
        self
    }

    pub fn set_rotation(&mut self, rotation: Quat) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn set_scale(&mut self, scale: Vec3) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn flatten(&self) -> [f32; 16] {
        self.matrix().to_cols_array()
    }

    pub fn from_flattened(flattened: [f32; 16]) -> Self {
        let (scale, rotation, translation) = Mat4::from_cols_array(&flattened).to_scale_rotation_translation();
        Self { translation, rotation, scale }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
