mod config;
mod window;

pub use config::*;
pub use window::*;

pub mod prelude {
    pub use crate::config::{Samples, WindowConfig, WindowMode};
}
