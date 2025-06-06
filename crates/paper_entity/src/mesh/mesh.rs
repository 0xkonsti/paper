use std::hash::{Hash, Hasher};

use glad_gl::gl;
use paper_utils::calculate_hash;
use uuid::Uuid;

use super::{AttributeType, COLOR_SIZE, POSITION_SIZE};
use crate::{Vertex, VertexAttribute};

pub(crate) const F32_SIZE: u32 = std::mem::size_of::<f32>() as u32;
pub(crate) const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

pub(crate) const BASE_STRIDE: u32 = POSITION_SIZE * F32_SIZE + COLOR_SIZE * F32_SIZE;

#[derive(Debug, Clone)]
pub struct Mesh {
    id: Uuid,

    vao: u32,
    vbo: u32,
    ebo: Option<u32>,

    vertices: Vec<Vertex>,
    attributes: Vec<VertexAttribute>,
    indices: Option<Vec<u32>>,
    stride: u32,

    build: bool,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn add_attribute(&mut self, attribute_type: AttributeType, data: Vec<f32>) {
        self.stride += attribute_type.size() * F32_SIZE;

        self.attributes.push(VertexAttribute::new(&attribute_type));

        if self.vertices.is_empty() {
            self.vertices = data
                .chunks(attribute_type.size() as usize)
                .map(|chunk| {
                    let mut vertex = Vertex::new();
                    vertex.add_attribute(&attribute_type, chunk.to_vec());
                    vertex
                })
                .collect();
        } else {
            for (vertex, chunks) in self.vertices.iter_mut().zip(data.chunks(attribute_type.size() as usize)) {
                vertex.add_attribute(&attribute_type, chunks.to_vec());
            }
        }
    }

    pub fn with_attribute(mut self, attribute_type: AttributeType, data: Vec<f32>) -> Self {
        self.add_attribute(attribute_type, data);
        self
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = Some(indices);
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.set_indices(indices);
        self
    }

    #[cfg(feature = "internal")]
    pub fn finalize(mut self) -> Mesh {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = None;

        let flattened: Vec<f32> = self.vertices.iter().flat_map(|v| v.flatten()).collect();

        unsafe {
            // Create and bind the VAO
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create and bind the VBO
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (flattened.len() * F32_SIZE as usize) as isize,
                flattened.as_ptr() as *const _,
                gl::STATIC_DRAW, // TODO: Make this configurable
            );

            // Set up the vertex attributes layout
            let stride = if self.stride <= BASE_STRIDE {
                // This makes sure unused default attributes are included in the stride
                // (e.g. position, color, etc.)
                BASE_STRIDE
            } else {
                self.stride
            };

            let mut offset = 0;
            for (i, attr) in self.attributes.iter().enumerate() {
                let index = i as u32;
                let size = attr.size as i32;

                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index,
                    size,
                    gl::FLOAT,
                    gl::FALSE,
                    stride as i32,
                    (offset * F32_SIZE) as *const _,
                );

                offset += size as u32;
            }

            // Create and bind the EBO if indices are provided
            if let Some(indices) = self.indices.as_ref() {
                let mut ebo_id = 0;
                gl::GenBuffers(1, &mut ebo_id);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * U32_SIZE as usize) as isize,
                    indices.as_ptr() as *const _,
                    gl::STATIC_DRAW, // TODO: Make this configurable
                );
                ebo = Some(ebo_id);
            }

            gl::BindVertexArray(0); // Unbind the VAO
        };

        self.vao = vao;
        self.vbo = vbo;
        self.ebo = ebo;

        self.build = true;

        self
    }

    #[cfg(feature = "internal")]
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if let Some(indices) = self.indices.as_ref() {
                gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
            }
            gl::BindVertexArray(0);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        let mut mesh = Self {
            id: Uuid::new_v4(),

            vao: 0,
            vbo: 0,
            ebo: None,

            vertices: Vec::new(),
            indices: None,
            attributes: Vec::new(),
            stride: 0,

            build: false,
        };

        mesh.id = Uuid::from_u64_pair(0, calculate_hash(&mesh));

        mesh
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        if !self.build {
            return; // No need to clean up if the mesh wasn't built
        }
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);

            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(1, &ebo);
            }
        }
    }
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices && self.indices == other.indices
    }
}

impl Eq for Mesh {}

impl Hash for Mesh {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.vertices.hash(state);
        self.indices.hash(state);
    }
}
