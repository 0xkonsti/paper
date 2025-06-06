use std::hash::Hash;

use paper_math::Transform;
use uuid::Uuid;

use super::{DEFAULT_MATERIAL_ID, Material, Shader, ShaderUniform, TRANSFORM_UNIFORM};

#[derive(Debug, Clone)]
pub struct DefaultMaterial {
    id: Uuid,

    shader: Shader,
}

impl DefaultMaterial {}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self { id: DEFAULT_MATERIAL_ID, shader: Shader::default() }
    }
}

impl Hash for DefaultMaterial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for DefaultMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DefaultMaterial {}

impl Material for DefaultMaterial {
    fn id(&self) -> Uuid {
        self.id
    }

    fn apply(&self, transform: &Transform) {
        self.shader.use_program();
        self.shader.set_uniform(TRANSFORM_UNIFORM, &ShaderUniform::Mat4(transform.flatten()));
    }
}
