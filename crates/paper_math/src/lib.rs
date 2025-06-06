mod transform;

pub use transform::*;

#[rustfmt::skip]
pub mod prelude {
    pub use crate::{
        transform::Transform,
        Vec2, Vec3, Vec4, Quat, Vec3A,
        IVec2, IVec3, IVec4, UVec2, UVec3, UVec4,
        BVec2, BVec3, BVec4, BVec4A,
        Mat2, Mat3, Mat3A, Mat4,
        Vec2Swizzles, Vec3Swizzles, Vec4Swizzles,
    };
}

pub use glam::*;
