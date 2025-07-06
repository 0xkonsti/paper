pub(crate) const POSITION_SIZE: u32 = 3;
pub(crate) const COLOR_SIZE: u32 = 4;

pub(crate) const DEFAULT_POSITION: [f32; POSITION_SIZE as usize] = [0.0; POSITION_SIZE as usize];
pub(crate) const DEFAULT_COLOR: [f32; COLOR_SIZE as usize] = [0.0; COLOR_SIZE as usize];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeType {
    Position,
    Color,

    Custom { name: String, size: u32 },
}

impl AttributeType {
    pub fn name(&self) -> &str {
        match self {
            AttributeType::Position => "position",
            AttributeType::Color => "color",

            AttributeType::Custom { name, .. } => name,
        }
    }

    pub const fn size(&self) -> u32 {
        match self {
            AttributeType::Position => POSITION_SIZE,
            AttributeType::Color => COLOR_SIZE,

            AttributeType::Custom { size, .. } => *size,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexAttribute {
    pub name: String,
    pub size: u32,

    __type: AttributeType,
}

impl VertexAttribute {
    pub fn new(attribute_type: &AttributeType) -> Self {
        Self {
            name:   attribute_type.name().to_string(),
            size:   attribute_type.size(),
            __type: attribute_type.clone(),
        }
    }
}
