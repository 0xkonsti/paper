mod app;
mod camera;
mod commands;
mod entity;
mod paper;

pub use app::*;
pub use camera::*;
pub use commands::*;
pub use entity::*;
pub use paper::*;

pub mod prelude {
    pub use crate::{
        app::{EmptyApp, PaperApp},
        camera::Camera2D,
        entity::{Entity, EntityId, MaterialId, MeshId},
        paper::Paper,
    };
}
