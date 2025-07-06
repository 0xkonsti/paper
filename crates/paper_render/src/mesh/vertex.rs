use std::hash::Hash;

use log::error;

use crate::{AttributeType, COLOR_SIZE, DEFAULT_COLOR, DEFAULT_POSITION, POSITION_SIZE};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vertex {
    position: [f32; POSITION_SIZE as usize],
    color:    [f32; COLOR_SIZE as usize],

    custom: Vec<(String, Vec<f32>)>,
}

impl Vertex {
    pub fn add_attribute(&mut self, attribute_type: &AttributeType, data: Vec<f32>) {
        match attribute_type {
            AttributeType::Position => {
                self.position = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid position data length: expected {} but got {}", POSITION_SIZE, data.len());
                    std::process::exit(1);
                });
            }
            AttributeType::Color => {
                self.color = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid color data length: expected {} but got {}", COLOR_SIZE, data.len());
                    std::process::exit(1);
                });
            }
            AttributeType::Custom { name, size } => {
                if data.len() != *size as usize {
                    error!("Invalid custom attribute '{}' data length: expected {} but got {}", name, size, data.len());
                    std::process::exit(1);
                }
                self.custom.push((name.clone(), data));
            }
        }
    }

    pub fn flatten(&self) -> Vec<f32> {
        let mut flat = Vec::new();
        flat.extend_from_slice(&self.position);
        flat.extend_from_slice(&self.color);

        for (_, data) in &self.custom {
            flat.extend_from_slice(data);
        }

        flat
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: DEFAULT_POSITION, color: DEFAULT_COLOR, custom: Vec::new() }
    }
}

impl Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for value in &self.position {
            value.to_bits().hash(state);
        }

        for value in &self.color {
            value.to_bits().hash(state);
        }

        for (name, data) in &self.custom {
            name.hash(state);
            for value in data {
                value.to_bits().hash(state);
            }
        }
    }
}
