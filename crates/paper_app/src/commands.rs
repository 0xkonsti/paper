use paper_color::Srgba;
use paper_input::Event;
use paper_math::{Transform, Vec2};
use paper_renderable::Entity;
use uuid::Uuid;

use crate::Camera2D;

pub trait CommandsTrait {
    fn set_clear_color(&mut self, color: Srgba);

    fn add_entity(&mut self, entity: Entity) -> Uuid;

    fn get_transform(&self, id: &Uuid) -> Option<&Transform>;

    fn get_transform_mut(&mut self, id: &Uuid) -> Option<&mut Transform>;

    fn remove_entity(&mut self, entity: &Entity);

    fn clear_entities(&mut self);

    fn close(&mut self);

    fn trigger_event(&mut self, event: Event);

    fn get_mouse_position(&self) -> Vec2;

    fn get_delta_time(&self) -> f64;

    fn get_fixed_delta_time(&self) -> f64;

    fn camera(&self) -> &Camera2D;

    fn camera_mut(&mut self) -> &mut Camera2D;
}

pub struct Commands<'a> {
    paper: &'a mut dyn CommandsTrait,
}

impl<'a> Commands<'a> {
    pub(crate) fn new(paper: &'a mut dyn CommandsTrait) -> Self {
        Commands { paper }
    }

    pub fn set_clear_color(&mut self, color: Srgba) {
        self.paper.set_clear_color(color);
    }

    pub fn add_entity(&mut self, entity: Entity) -> Uuid {
        self.paper.add_entity(entity)
    }

    pub fn get_transform(&self, id: &Uuid) -> Option<&Transform> {
        self.paper.get_transform(id)
    }

    pub fn get_transform_mut(&mut self, id: &Uuid) -> Option<&mut Transform> {
        self.paper.get_transform_mut(id)
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.paper.remove_entity(entity);
    }

    pub fn clear_entities(&mut self) {
        self.paper.clear_entities();
    }

    pub fn close(&mut self) {
        self.paper.close();
    }

    pub fn trigger_event(&mut self, event: Event) {
        self.paper.trigger_event(event);
    }

    pub fn trigger_events(&mut self, events: impl IntoIterator<Item = Event>) {
        for event in events {
            self.trigger_event(event);
        }
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.paper.get_mouse_position()
    }

    pub fn get_delta_time(&self) -> f64 {
        self.paper.get_delta_time()
    }

    pub fn get_fixed_delta_time(&self) -> f64 {
        self.paper.get_fixed_delta_time()
    }

    pub fn camera(&self) -> &Camera2D {
        self.paper.camera()
    }

    pub fn camera_mut(&mut self) -> &mut Camera2D {
        self.paper.camera_mut()
    }
}
