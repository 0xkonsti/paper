use paper_color::Srgba;
use paper_math::{Mat4, Transform};
use paper_utils::calculate_hash;
use uuid::Uuid;

use crate::{
    Material, PROJECTION_UNIFORM, Shader, ShaderUniform,
    material::{COLOR_MATERIAL_ID, COLOR_UNIFORM, TRANSFORM_UNIFORM},
};

#[derive(Debug, Clone)]
pub struct ColorMaterial {
    id: Uuid,
    color: Srgba,

    shader: Shader,
}

impl ColorMaterial {
    pub fn new(color: Srgba) -> Self {
        let id = Uuid::from_u64_pair(COLOR_MATERIAL_ID.as_u64_pair().1, calculate_hash(&color));

        Self {
            id,
            color,
            shader: Shader::from_source(
                include_str!("shader/color/color.vert"),
                include_str!("shader/color/color.frag"),
            ),
        }
    }
}

impl Default for ColorMaterial {
    fn default() -> Self {
        Self::new(Srgba::WHITE)
    }
}

impl Material for ColorMaterial {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply(&self, transform: &Transform, projection: &Mat4) {
        self.shader.use_program();
        self.shader.set_uniform(PROJECTION_UNIFORM, &ShaderUniform::Mat4(projection.to_cols_array()));
        self.shader.set_uniform(TRANSFORM_UNIFORM, &ShaderUniform::Mat4(transform.flatten()));
        self.shader.set_uniform(COLOR_UNIFORM, &ShaderUniform::Vec4(self.color.to_array()));
    }
}
