use std::hash::Hash;

use glad_gl::gl;

use crate::{AttributeType, COLOR_SIZE, POSITION_SIZE, Vertex, VertexAttribute};

pub(crate) const F32_SIZE: u32 = size_of::<f32>() as u32;
pub(crate) const U32_SIZE: u32 = size_of::<u32>() as u32;

pub(crate) const BASE_STRIDE: u32 = POSITION_SIZE * F32_SIZE + COLOR_SIZE * F32_SIZE;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Mesh {
    vertices:   Vec<Vertex>,
    attributes: Vec<VertexAttribute>,
    indices:    Option<Vec<u32>>,
    stride:     u32,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_attribute(&mut self, attribute_type: AttributeType, data: Vec<f32>) {
        self.stride += attribute_type.size() * F32_SIZE;

        self.attributes.push(VertexAttribute::new(&attribute_type));

        if self.vertices.is_empty() {
            self.vertices = data
                .chunks(attribute_type.size() as usize)
                .map(|chunk| {
                    let mut vertex = Vertex::default();
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
}

#[cfg(feature = "internal")]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InternalMesh {
    pub mesh: Mesh,

    vao: u32,
    vbo: u32,
    ebo: Option<u32>,

    draw_mode: gl::types::GLenum,
}

#[cfg(feature = "internal")]
impl InternalMesh {
    pub fn build(mesh: Mesh) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = None;

        let flattened: Vec<f32> = mesh.vertices.iter().flat_map(|v| v.flatten()).collect();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (flattened.len() * F32_SIZE as usize) as isize,
                flattened.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = if mesh.stride <= BASE_STRIDE { BASE_STRIDE } else { mesh.stride };

            let mut offset = 0;
            for (i, attr) in mesh.attributes.iter().enumerate() {
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

            if let Some(indices) = mesh.indices.as_ref() {
                let mut ebo_id = 0;
                gl::GenBuffers(1, &mut ebo_id);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * U32_SIZE as usize) as isize,
                    indices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
                ebo = Some(ebo_id);
            }

            gl::BindVertexArray(0); // Unbind the VAO
        };

        Self { mesh, vao, vbo, ebo, draw_mode: gl::TRIANGLES }
    }

    pub fn set_draw_mode(&mut self, mode: gl::types::GLenum) {
        self.draw_mode = mode;
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            if let Some(indices) = self.mesh.indices.as_ref() {
                gl::DrawElements(self.draw_mode, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            } else {
                gl::DrawArrays(self.draw_mode, 0, self.mesh.vertices.len() as i32);
            }
            gl::BindVertexArray(0); // Unbind the VAO
        }
    }
}

impl Eq for Mesh {}

impl Hash for Mesh {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vertices.hash(state);
        self.attributes.hash(state);
        self.indices.hash(state);
        self.stride.hash(state);
    }
}
