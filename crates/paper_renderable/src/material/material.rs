use std::hash::{Hash, Hasher};

use paper_color::Srgba;
use paper_math::Transform;
use uuid::Uuid;

use super::DefaultMaterial;
use crate::material::color_material::ColorMaterial;

pub(crate) const DEFAULT_MATERIAL_ID: Uuid = Uuid::nil();
pub(crate) const COLOR_MATERIAL_ID: Uuid = Uuid::from_u128(1);

pub(crate) const TRANSFORM_UNIFORM: &str = "uModel";
pub(crate) const COLOR_UNIFORM: &str = "uColor";

#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub enum MaterialType {
    #[default]
    Default,
    Color(Srgba),
}

impl MaterialType {
    pub fn id(&self) -> Uuid {
        match self {
            MaterialType::Default => DEFAULT_MATERIAL_ID,
            MaterialType::Color(color) => {
                Uuid::from_u64_pair(COLOR_MATERIAL_ID.as_u64_pair().1, paper_utils::calculate_hash(color))
            }
        }
    }

    pub fn get_material(&self) -> Box<dyn Material> {
        match self {
            MaterialType::Default => Box::new(DefaultMaterial::default()),
            MaterialType::Color(color) => Box::new(ColorMaterial::new(*color)),
        }
    }
}

pub trait Material: std::fmt::Debug {
    fn id(&self) -> Uuid;
    fn apply(&self, transform: &Transform);
}

impl Hash for Box<dyn Material> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for Box<dyn Material> {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Box<dyn Material> {}
