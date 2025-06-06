mod action;
mod key;
mod mouse;

pub use action::*;
pub use key::*;
pub use mouse::*;

pub mod prelude {
    pub use crate::{Action, Key, MouseButton};
}
