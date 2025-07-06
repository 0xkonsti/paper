mod material;
mod mesh;

pub use material::*;
pub use mesh::*;

pub mod prelude {
    pub use crate::{
        material::{DefaultMaterial, Material},
        mesh::{AttributeType, Mesh},
    };
}
