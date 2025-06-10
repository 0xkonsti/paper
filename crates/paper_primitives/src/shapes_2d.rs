#[cfg(feature = "internal")]
use glad_gl::gl;
use paper_math::{Transform, Vec2};

const TRIANGLE_VERTICES: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];
const RECTANGLE_VERTICES: [f32; 8] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];

const TRIANGLE_INDICES: [u32; 3] = [0, 1, 2];
const RECTANGLE_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

pub enum Shape2D {
    Triangle { position: Vec2, width: f32, height: f32 },
    Rectangle { position: Vec2, width: f32, height: f32 },
    Circle { position: Vec2, radius: f32, segments: u32 },
}

impl Shape2D {
    #[rustfmt::skip]
    pub fn vertices(&self) -> Vec<f32> {
        match self {
            Shape2D::Triangle { width, height, .. } => {
                vec![
                    width * TRIANGLE_VERTICES[0], height * TRIANGLE_VERTICES[1], 0.0,
                    width * TRIANGLE_VERTICES[2], height * TRIANGLE_VERTICES[3], 0.0,
                    width * TRIANGLE_VERTICES[4], height * TRIANGLE_VERTICES[5], 0.0,
                ]
            }
            Shape2D::Rectangle { width, height, .. } => {
                vec![
                    width * RECTANGLE_VERTICES[0],  height * RECTANGLE_VERTICES[1],  0.0,
                    width * RECTANGLE_VERTICES[2],  height * RECTANGLE_VERTICES[3],  0.0,
                    width * RECTANGLE_VERTICES[4],  height * RECTANGLE_VERTICES[5],  0.0,
                    width * RECTANGLE_VERTICES[6],  height * RECTANGLE_VERTICES[7],  0.0,
                ]
            }
            Shape2D::Circle { radius, segments, .. } => {
                let mut vertices = Vec::new();
                for i in 0..*segments {
                    let angle = i as f32 * 2.0 * std::f32::consts::PI / *segments as f32;
                    vertices.push(radius * angle.cos());
                    vertices.push(radius * angle.sin());
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
            Shape2D::Circle { segments, .. } => {
                let mut indices = Vec::new();
                for i in 0..*segments {
                    indices.push(i);
                }
                indices
            }
        }
    }

    pub fn transform(&self) -> Transform {
        match self {
            Shape2D::Triangle { position, .. } => Transform::from_translation(position.extend(0.0)),
            Shape2D::Rectangle { position, .. } => Transform::from_translation(position.extend(0.0)),
            Shape2D::Circle { position, .. } => Transform::from_translation(position.extend(0.0)),
        }
    }

    #[cfg(feature = "internal")]
    pub fn draw_mode(&self) -> gl::types::GLenum {
        match self {
            Shape2D::Triangle { .. } => gl::TRIANGLES,
            Shape2D::Rectangle { .. } => gl::TRIANGLES,
            Shape2D::Circle { segments, .. } if *segments > 0 => gl::TRIANGLE_FAN,
            _ => gl::POINTS, // Default to points for circles with no segments
        }
    }
}
