use paper_input::Event;
use paper_math::{Quat, Transform, Vec2, Vec3};
use paper_render::{Material, Mesh, ShaderUniform};

use crate::{Entity, EntityId, MaterialId, MeshId};

pub(crate) trait Commandable {
    fn close(&mut self);

    fn events(&self) -> &Vec<Event>;

    fn delta_time(&self) -> f32;

    fn fixed_delta_time(&self) -> f32;

    fn add_mesh(&mut self, mesh: Mesh) -> MeshId;

    fn add_material(&mut self, material: Box<dyn Material>) -> MaterialId;

    fn add_entity(&mut self, entity: Entity) -> EntityId;

    fn remove_entity(&mut self, entity_id: EntityId) -> Option<Entity>;

    fn set_material_uniform(&mut self, material_id: MaterialId, name: &str, value: ShaderUniform);

    fn get_entity_transform(&self, id: &EntityId) -> Option<&Transform>;

    fn get_entity_transform_mut(&mut self, id: &EntityId) -> Option<&mut Transform>;

    fn set_entity_transform(&mut self, id: &EntityId, transform: Transform);
}

pub struct Commands<'a> {
    ca: &'a mut dyn Commandable,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(ca: &'a mut dyn Commandable) -> Self {
        Self { ca }
    }

    pub fn close(&mut self) {
        self.ca.close();
    }

    pub fn events(&self) -> &Vec<Event> {
        self.ca.events()
    }

    pub fn delta_time(&self) -> f32 {
        self.ca.delta_time()
    }

    pub fn fixed_delta_time(&self) -> f32 {
        self.ca.fixed_delta_time()
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        self.ca.add_mesh(mesh)
    }

    pub fn add_material<M: Material + 'static>(&mut self, material: M) -> MaterialId {
        self.ca.add_material(Box::new(material))
    }

    pub fn add_entity(&mut self, entity: Entity) -> EntityId {
        self.ca.add_entity(entity)
    }

    pub fn remove_entity(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.ca.remove_entity(entity_id)
    }

    pub fn set_material_uniform(&mut self, material_id: MaterialId, name: &str, value: ShaderUniform) {
        self.ca.set_material_uniform(material_id, name, value);
    }

    pub fn get_entity_transform(&self, id: &EntityId) -> Option<&Transform> {
        self.ca.get_entity_transform(id)
    }

    pub fn get_entity_transform_mut(&mut self, id: &EntityId) -> Option<&mut Transform> {
        self.ca.get_entity_transform_mut(id)
    }

    pub fn set_entity_transform(&mut self, id: &EntityId, transform: Transform) {
        self.ca.set_entity_transform(id, transform);
    }

    pub fn set_entity_translation(&mut self, id: &EntityId, translation: Vec2) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.set_translation(translation.extend(0.0));
        }
    }

    pub fn translate_entity(&mut self, id: &EntityId, translation: Vec2) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.translate(translation.extend(0.0));
        }
    }

    pub fn set_entity_rotation(&mut self, id: &EntityId, rotation: Quat) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.set_rotation(rotation);
        }
    }

    pub fn rotate_entity(&mut self, id: &EntityId, axis: Vec3, angle: f32) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.rotate_around(axis, angle);
        }
    }

    pub fn set_entity_scale(&mut self, id: &EntityId, scale: Vec3) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.set_scale(scale);
        }
    }

    pub fn scale_entity(&mut self, id: &EntityId, scale: Vec3) {
        if let Some(transform) = self.get_entity_transform_mut(id) {
            transform.scale(scale);
        }
    }
}
