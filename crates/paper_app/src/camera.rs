use paper_math::{Mat4, Transform, Vec2};
use paper_utils::default;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2D {
    pub transform: Transform,
    pub zoom:      f32,
    pub viewport:  Vec2,
    pub near:      f32,
    pub far:       f32,
}

impl Camera2D {
    pub fn new(transform: Transform, zoom: f32, viewport: Vec2) -> Self {
        Self { transform, zoom, viewport, ..default() }
    }

    pub fn move_by(&mut self, delta: Vec2) {
        self.transform.translation += delta.extend(0.0);
    }

    pub fn move_vertically(&mut self, delta: f32) {
        self.transform.translation.y += delta;
    }

    pub fn move_horizontally(&mut self, delta: f32) {
        self.transform.translation.x += delta;
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn with_viewport(mut self, viewport: Vec2) -> Self {
        self.viewport = viewport;
        self
    }

    pub(crate) fn projection_matrix(&self) -> [f32; 16] {
        let position = self.transform.translation;
        let half_w = self.viewport.x * 0.5 / self.zoom;
        let half_h = self.viewport.y * 0.5 / self.zoom;
        Mat4::orthographic_rh_gl(
            -half_w + position.x,
            half_w + position.x,
            -half_h + position.y,
            half_h + position.y,
            self.near,
            self.far,
        )
        .to_cols_array()
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            zoom:      1.0,
            viewport:  Vec2::ZERO,
            near:      -1000.0,
            far:       1000.0,
        }
    }
}
