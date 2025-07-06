mod action;
mod event;
mod key;
mod mouse;

pub use action::*;
pub use event::*;
pub use key::*;
pub use mouse::*;

pub mod prelude {
    pub use crate::{Action, Event, Key, MouseButton};
}
