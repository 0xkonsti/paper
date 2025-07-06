use hashbrown::HashMap;

use crate::{Material, Shader, ShaderUniform, Uniform};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ColorMaterial {
    shader:   Shader,
    uniforms: HashMap<String, Uniform>,
}

impl Material for ColorMaterial {
    fn name(&self) -> &str {
        "ColorMaterial"
    }

    fn vertex_shader(&self) -> &str {
        include_str!("shader/color/vert.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("shader/color/frag.glsl")
    }

    fn set_shader(&mut self, shader: Shader) {
        self.shader = shader;
    }

    fn set_uniform(&mut self, name: &str, value: ShaderUniform) {
        let Some((current_value, dirty)) = self.uniforms.get_mut(name) else {
            self.uniforms.insert(name.to_string(), (value, true));
            return;
        };
        if *current_value != value {
            *current_value = value;
            *dirty = true;
        }
    }

    fn bind(&self) {
        self.shader.use_program();
        for (name, (value, dirty)) in &self.uniforms {
            if *dirty {
                self.shader.set_uniform(name, value);
            }
        }
    }

    #[cfg(feature = "internal")]
    fn from_source(&self) -> bool {
        true
    }
}
