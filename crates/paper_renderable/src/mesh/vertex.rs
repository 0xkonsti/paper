use std::{collections::HashMap, hash::Hash, process::exit};

use log::error;
use paper_utils::hash_f32_array;

use super::{AttributeType, COLOR_SIZE, DEFAULT_COLOR, DEFAULT_POSITION, POSITION_SIZE};

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    position: [f32; POSITION_SIZE as usize],
    color: [f32; COLOR_SIZE as usize],

    custom: HashMap<String, Vec<f32>>,
}

impl Vertex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attribute(&mut self, attribute_type: &AttributeType, data: Vec<f32>) {
        match attribute_type {
            AttributeType::Position => {
                self.position = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid position data length: expected {} but got {}", POSITION_SIZE, data.len());
                    exit(1);
                });
            }
            AttributeType::Color => {
                self.color = data.as_slice().try_into().unwrap_or_else(|_| {
                    error!("Invalid color data length: expected {} but got {}", COLOR_SIZE, data.len());
                    exit(1);
                });
            }
            AttributeType::Custom { name, size } => {
                if data.len() != *size as usize {
                    error!("Invalid custom attribute '{}' data length: expected {} but got {}", name, size, data.len());
                    exit(1);
                }
                self.custom.insert(name.clone(), data);
            }
        }
    }

    pub fn flatten(&self) -> Vec<f32> {
        let mut flat = Vec::new();
        flat.extend_from_slice(&self.position);
        flat.extend_from_slice(&self.color);

        for data in self.custom.values() {
            flat.extend_from_slice(data);
        }

        flat
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self { position: DEFAULT_POSITION, color: DEFAULT_COLOR, custom: HashMap::new() }
    }
}

impl Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        hash_f32_array(state, &self.position);
        hash_f32_array(state, &self.color);

        for data in self.custom.values() {
            hash_f32_array(state, data);
        }
    }
}

impl Eq for Vertex {}
