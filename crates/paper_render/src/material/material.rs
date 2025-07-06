use crate::{Shader, ShaderUniform};

pub(crate) type Uniform = (ShaderUniform, bool); // (value, dirty)

pub trait Material {
    fn name(&self) -> &str;

    fn vertex_shader(&self) -> &str;

    fn fragment_shader(&self) -> &str;

    fn set_shader(&mut self, shader: Shader);

    fn set_uniform(&mut self, name: &str, value: ShaderUniform);

    fn bind(&self);

    #[cfg(feature = "internal")]
    fn from_source(&self) -> bool {
        false
    }
}
