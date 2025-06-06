use std::hash::{Hash, Hasher};

use paper_math::Transform;
use uuid::Uuid;

use super::DefaultMaterial;

pub(crate) const DEFAULT_MATERIAL_ID: Uuid = Uuid::nil();

pub(crate) const TRANSFORM_UNIFORM: &str = "uModel";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MaterialType {
    #[default]
    Default,
}

impl MaterialType {
    pub fn id(&self) -> Uuid {
        match self {
            MaterialType::Default => DEFAULT_MATERIAL_ID,
        }
    }

    pub fn get_material(&self) -> Box<dyn Material> {
        match self {
            MaterialType::Default => Box::new(DefaultMaterial::default()),
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
