pub mod colors;
mod srgba;

pub use srgba::*;

pub mod prelude {
    pub use crate::{Srgba, colors::*};
}
