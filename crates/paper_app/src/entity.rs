use std::ops::{Deref, DerefMut};

use log::error;
use paper_math::Transform;
use paper_render::ShaderUniform;

use crate::{Paper, PaperApp};

pub const PROJECTION_UNIFORM: &str = "uProjection";
pub const MODEL_UNIFORM: &str = "uModel";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeshId(uuid::Uuid);

impl MeshId {
    pub fn new() -> Self {
        MeshId(uuid::Uuid::new_v4())
    }
}

impl Deref for MeshId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MeshId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialId(uuid::Uuid);

impl MaterialId {
    pub fn new() -> Self {
        MaterialId(uuid::Uuid::new_v4())
    }
}

impl Deref for MaterialId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MaterialId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    pub fn new() -> Self {
        EntityId(uuid::Uuid::new_v4())
    }
}

impl Deref for EntityId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Entity {
    MeshMaterial { mesh_id: MeshId, material_id: MaterialId, transform: Transform },
}

impl Entity {
    pub(crate) fn transform(&self) -> &Transform {
        match self {
            Entity::MeshMaterial { transform, .. } => transform,
        }
    }

    pub(crate) fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Entity::MeshMaterial { transform, .. } => transform,
        }
    }

    pub(crate) fn draw<T: PaperApp>(&self, paper: &mut Paper<T>) {
        match self {
            Entity::MeshMaterial { mesh_id, material_id, transform } => {
                let Some(mesh) = paper.meshes.get(mesh_id) else {
                    error!("Mesh with ID {mesh_id:?} not found");
                    return;
                };

                let Some(material) = paper.materials.get_mut(material_id) else {
                    error!("Material with ID {material_id:?} not found");
                    return;
                };

                material.set_uniform(PROJECTION_UNIFORM, ShaderUniform::Mat4(paper.camera.projection_matrix()));
                material.set_uniform(MODEL_UNIFORM, ShaderUniform::Mat4(transform.flatten()));

                material.bind();
                mesh.draw();
            }
        }
    }
}
