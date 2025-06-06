mod array;
mod default;
mod hash;

pub use array::*;
pub use default::*;
pub use hash::*;

pub mod prelude {
    pub use crate::{default, flatten, flatten_array};
}
