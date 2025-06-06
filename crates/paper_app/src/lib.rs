mod app;
mod camera;
mod commands;
mod paper;

pub use app::*;
pub use commands::*;
pub use paper::*;

pub mod prelude {
    pub use crate::{
        app::{EmptyApp, PaperApp},
        commands::Commands,
        paper::Paper,
    };
}
