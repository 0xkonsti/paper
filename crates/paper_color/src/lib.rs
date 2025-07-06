mod linear;
mod pallet;
mod srgba;

pub use linear::*;
pub use pallet::*;
pub use srgba::*;

pub mod prelude {
    pub use crate::{LinearRgba, Srgba, pallet::*};
}
