#[cfg(feature = "internal")]
use std::os::raw::c_uint;

use paper_math::{Transform, Vec2};

use crate::{AttributeType, Mesh};

const TRIANGLE_VERTICES: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
const RECTANGLE_VERTICES: [f32; 12] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0];

const TRIANGLE_INDICES: [u32; 3] = [0, 1, 2];
const RECTANGLE_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape2D {
    Triangle { pos: Vec2, size: Vec2 },
    Rectangle { pos: Vec2, size: Vec2 },
    Square { pos: Vec2, size: f32 },
    Circle { pos: Vec2, radius: f32, segments: u32 },
}

impl Shape2D {
    pub fn triangle(pos: Vec2, size: Vec2) -> Self {
        Shape2D::Triangle { pos, size }
    }

    pub fn rectangle(pos: Vec2, size: Vec2) -> Self {
        Shape2D::Rectangle { pos, size }
    }

    pub fn square(pos: Vec2, size: f32) -> Self {
        Shape2D::Square { pos, size }
    }

    pub fn circle(pos: Vec2, radius: f32, segments: u32) -> Self {
        Shape2D::Circle { pos, radius, segments }
    }

    pub fn vertices(&self) -> Vec<f32> {
        match self {
            Shape2D::Triangle { .. } => TRIANGLE_VERTICES.to_vec(),
            Shape2D::Rectangle { .. } => RECTANGLE_VERTICES.to_vec(),
            Shape2D::Square { .. } => RECTANGLE_VERTICES.to_vec(),
            Shape2D::Circle { segments, .. } => {
                let mut vertices = Vec::new();
                for i in 0..*segments {
                    let angle = (i as f32 / *segments as f32) * std::f32::consts::PI * 2.0;
                    vertices.push(angle.cos());
                    vertices.push(angle.sin());
                    vertices.push(0.0);
                }
                vertices
            }
        }
    }

    pub fn indices(&self) -> Vec<u32> {
        match self {
            Shape2D::Triangle { .. } => TRIANGLE_INDICES.to_vec(),
            Shape2D::Rectangle { .. } => RECTANGLE_INDICES.to_vec(),
            Shape2D::Square { .. } => RECTANGLE_INDICES.to_vec(),
            Shape2D::Circle { segments, .. } => {
                let mut indices = Vec::new();
                for i in 0..*segments {
                    indices.push(0);
                    indices.push((i + 1) % *segments);
                    indices.push(i);
                }
                indices
            }
        }
    }

    pub fn mesh(&self) -> Mesh {
        let vertices = self.vertices();
        let indices = self.indices();

        Mesh::new().with_attribute(AttributeType::Position, vertices).with_indices(indices)
    }

    pub fn transform(&self) -> Transform {
        match self {
            Shape2D::Triangle { pos, size } => {
                Transform::from_translation(pos.extend(0.0)).with_scale(size.extend(1.0))
            }
            Shape2D::Rectangle { pos, size } => {
                Transform::from_translation(pos.extend(0.0)).with_scale(size.extend(1.0))
            }
            Shape2D::Square { pos, size } => {
                Transform::from_translation(pos.extend(0.0)).with_scale(Vec2::splat(*size).extend(1.0))
            }
            Shape2D::Circle { pos, radius, segments: _ } => {
                Transform::from_translation(pos.extend(0.0)).with_scale(Vec2::splat(*radius).extend(1.0))
            }
        }
    }

    #[cfg(feature = "internal")]
    pub fn draw_mode(&self) -> c_uint {
        use glad_gl::gl;

        match self {
            Shape2D::Triangle { .. } => gl::TRIANGLES,
            Shape2D::Rectangle { .. } => gl::TRIANGLES,
            Shape2D::Circle { segments, .. } if *segments > 0 => gl::TRIANGLE_FAN,
            _ => gl::POINTS, // Default to points for circles with no segments
        }
    }
}

impl From<Shape2D> for Mesh {
    fn from(shape: Shape2D) -> Self {
        shape.mesh()
    }
}

impl From<&Shape2D> for Mesh {
    fn from(shape: &Shape2D) -> Self {
        shape.mesh()
    }
}

impl From<Shape2D> for Transform {
    fn from(shape: Shape2D) -> Self {
        shape.transform()
    }
}

impl From<&Shape2D> for Transform {
    fn from(shape: &Shape2D) -> Self {
        shape.transform()
    }
}
