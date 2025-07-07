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
    fn shaders_from_source(&self) -> bool {
        false
    }
}

impl<M: Material + ?Sized> Material for Box<M> {
    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn vertex_shader(&self) -> &str {
        self.as_ref().vertex_shader()
    }

    fn fragment_shader(&self) -> &str {
        self.as_ref().fragment_shader()
    }

    fn set_shader(&mut self, shader: Shader) {
        self.as_mut().set_shader(shader);
    }

    fn set_uniform(&mut self, name: &str, value: ShaderUniform) {
        self.as_mut().set_uniform(name, value);
    }

    fn bind(&self) {
        self.as_ref().bind();
    }
}
