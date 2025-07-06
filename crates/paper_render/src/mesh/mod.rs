mod attribute;
mod mesh;
mod primitive;
mod vertex;

pub use attribute::*;
pub use mesh::*;
pub use primitive::*;
pub use vertex::*;

pub fn positions_from_vec3s(positions: Vec<paper_math::Vec3>) -> Vec<f32> {
    positions.into_iter().flat_map(|pos| pos.to_array().to_vec()).collect()
}
