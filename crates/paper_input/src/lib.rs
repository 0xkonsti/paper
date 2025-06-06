mod action;
mod key;
mod mouse;
mod event;

pub use action::*;
pub use event::*;
pub use key::*;
pub use mouse::*;

pub mod prelude {
    pub use crate::{Action, Event, Key, MouseButton};
}
