use paper_math::{Mat4, Transform, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Camera2D {
    pub transform: Transform,
    pub zoom: f32,
    pub viewport: Vec2,
}

impl Camera2D {
    pub fn new(transform: Transform, zoom: f32, viewport: Vec2) -> Self {
        Self { transform, zoom, viewport }
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

    pub(crate) fn projection_matrix(&self) -> Mat4 {
        let position = self.transform.translation;
        let half_w = self.viewport.x * 0.5 / self.zoom;
        let half_h = self.viewport.y * 0.5 / self.zoom;
        Mat4::orthographic_rh_gl(
            -half_w + position.x,
            half_w + position.x,
            -half_h + position.y,
            half_h + position.y,
            -1.0,
            1.0,
        )
    }
}
