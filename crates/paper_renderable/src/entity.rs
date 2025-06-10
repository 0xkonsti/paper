use paper_math::Transform;
use paper_primitives::Shape2D;
use paper_utils::default;

use crate::{AttributeType, Material, MaterialType, Mesh};

#[derive(Debug)]
pub struct Entity {
    mesh: Mesh,
    material_type: MaterialType,
    material: Option<Box<dyn Material>>,

    pub transform: Transform,
}

impl Entity {
    pub fn new(mesh: Mesh, material_type: MaterialType, transform: Transform) -> Self {
        Self { mesh, material_type, material: None, transform }
    }

    pub fn from_shape_2d(shape: Shape2D) -> Self {
        let mut mesh =
            Mesh::new().with_attribute(AttributeType::Position, shape.vertices()).with_indices(shape.indices());

        mesh.set_draw_mode(shape.draw_mode());

        Self { mesh, material_type: MaterialType::Default, material: None, transform: shape.transform() }
    }

    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = mesh;
        self
    }

    pub fn with_material_type(mut self, material_type: MaterialType) -> Self {
        self.material_type = material_type;
        self
    }

    pub fn with_material(mut self, material: Box<dyn Material>) -> Self {
        self.material = Some(material);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_attribute(mut self, attribute_type: AttributeType, data: Vec<f32>) -> Self {
        self.mesh.add_attribute(attribute_type, data);
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.mesh.set_indices(indices);
        self
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn material_type(&self) -> &MaterialType {
        &self.material_type
    }

    pub fn material(&self) -> Option<&dyn Material> {
        self.material.as_ref().map(|m| m.as_ref())
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self { mesh: default(), material_type: MaterialType::Default, material: None, transform: Transform::default() }
    }
}
