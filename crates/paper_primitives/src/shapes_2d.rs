use paper_math::{Transform, Vec2};

const TRIANGLE_VERTICES: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];
const RECTANGLE_VERTICES: [f32; 8] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];

const TRIANGLE_INDICES: [u32; 3] = [0, 1, 2];
const RECTANGLE_INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

pub enum Shape2D {
    Triangle { position: Vec2, width: f32, height: f32 },
    Rectangle { position: Vec2, width: f32, height: f32 },
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
        }
    }

    pub fn indices(&self) -> Vec<u32> {
        match self {
            Shape2D::Triangle { .. } => TRIANGLE_INDICES.to_vec(),
            Shape2D::Rectangle { .. } => RECTANGLE_INDICES.to_vec(),
        }
    }

    pub fn transform(&self) -> Transform {
        match self {
            Shape2D::Triangle { position, .. } => Transform::from_translation(position.extend(0.0)),
            Shape2D::Rectangle { position, .. } => Transform::from_translation(position.extend(0.0)),
        }
    }
}
