use std::{collections::HashMap, fmt::Debug, process::exit};

use glad_gl::gl;
use log::{debug, error};
use paper_color::Srgba;
use paper_entity::{Entity, Material, Mesh};
use paper_input::Event;
use paper_math::{Transform, Vec2};
use paper_window::{Window, WindowConfig};
use uuid::Uuid;

use crate::{Commands, CommandsTrait, EmptyApp, PaperApp};

pub type EventCallback<T> = Box<dyn Fn(Commands, &mut T)>;

pub struct Paper<T: PaperApp = EmptyApp> {
    window: Window,
    max_fps: Option<f64>,

    mouse_position: Vec2,
    delta_time: f64,

    triggered_events: Vec<Event>,
    event_callbacks: HashMap<Event, Vec<EventCallback<T>>>,

    entities: Vec<(Uuid, Uuid, Transform)>, // (mesh_id, material_id, transform)
    entity_map: HashMap<Uuid, usize>, // maps entity ID to its index in the entities vector (for access from outside)
    meshes: HashMap<Uuid, Mesh>,
    materials: HashMap<Uuid, Box<dyn Material>>,
}

pub fn enable_logging() {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Debug).init();
}

impl<T: PaperApp> Paper<T> {
    pub fn new(window_config: WindowConfig) -> Self {
        let _ = env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Debug).try_init();

        let Some(window) = Window::new(window_config) else {
            error!("Failed to create window");
            exit(1);
        };

        Paper {
            window,
            max_fps: None,

            mouse_position: Vec2::ZERO,
            delta_time: 0.,

            triggered_events: Vec::new(),
            event_callbacks: HashMap::new(),

            entities: Vec::new(),
            entity_map: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut app = T::default();

        app.setup(Commands::new(self));

        let frame_time = 1. / self.max_fps.unwrap_or(60.);
        let mut last_frame = std::time::Instant::now();
        let mut delta_time = 0.;

        while !self.window.should_close() {
            let now = std::time::Instant::now();
            delta_time += now.duration_since(last_frame).as_secs_f64();
            last_frame = now;

            if self.max_fps.is_some() && delta_time < frame_time {
                continue;
            }

            self.delta_time = delta_time;
            delta_time = 0.;

            self.window.poll_events();

            self.events(&mut app);

            app.update(Commands::new(self));

            self.render();

            self.window.swap_buffers();
        }

        app.cleanup(Commands::new(self));
    }

    pub fn add_entity(&mut self, entity: Entity) -> Uuid {
        let e_mesh = entity.mesh();
        let e_material_type = entity.material_type();

        let mut mesh_id = e_mesh.id();
        if let Some(mesh) = self.meshes.get(&mesh_id) {
            mesh_id = mesh.id();
        } else {
            self.meshes.insert(mesh_id, e_mesh.clone().finalize());
            debug!("Added new mesh to Paper | Count: {}", self.meshes.len());
        }

        let mut material_id = e_material_type.id();
        if let Some(material) = self.materials.get(&e_material_type.id()) {
            material_id = material.id();
        } else {
            self.materials.insert(e_material_type.id(), e_material_type.get_material());
            debug!("Added new material to Paper | Count: {}", self.materials.len());
        }

        let index = self.entities.len();
        let entity_id = Uuid::new_v4();
        self.entities.push((mesh_id, material_id, entity.transform));
        self.entity_map.insert(entity_id, index);

        debug!("Added new entity to Paper | Count: {}", self.entities.len());
        debug!("Entity ID: {entity_id}, Mesh ID: {mesh_id}, Material ID: {material_id}");

        entity_id
    }

    pub fn with_entity(mut self, entity: Entity, id: Option<&mut Uuid>) -> Self {
        let new_id = self.add_entity(entity);
        if let Some(id) = id {
            *id = new_id;
        }
        self
    }

    pub fn add_event_callback<F: Fn(Commands, &mut T) + Clone + 'static>(&mut self, event: Event, callback: F) {
        match event {
            Event::AnyKey(keys, action) => {
                for key in keys {
                    self.add_event_callback(Event::Key(key, action), callback.clone());
                }
                return;
            }
            Event::AnyMouseButton(buttons, action) => {
                for button in buttons {
                    self.add_event_callback(Event::MouseButton(button, action), callback.clone());
                }
                return;
            }
            _ => {}
        }

        debug!("Adding event callback for {event:?}");
        self.event_callbacks.entry(event).or_default().push(Box::new(callback));
    }

    pub fn with_event_callback<F: Fn(Commands, &mut T) + Clone + 'static>(mut self, event: Event, callback: F) -> Self {
        self.add_event_callback(event, callback);
        self
    }

    pub fn set_clear_color(&mut self, color: Srgba) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn with_clear_color(mut self, color: Srgba) -> Self {
        self.set_clear_color(color);
        self
    }

    pub fn set_max_fps(&mut self, fps: f64) {
        self.max_fps = Some(fps);
    }

    pub fn with_max_fps(mut self, fps: f64) -> Self {
        self.set_max_fps(fps);
        self
    }

    fn render(&self) {
        self.window.clear();

        for (mesh_id, mat_id, transform) in &self.entities {
            let Some(mesh) = self.meshes.get(mesh_id) else {
                error!("Mesh with ID {mesh_id} not found");
                continue;
            };

            let Some(material) = self.materials.get(mat_id) else {
                error!("Material with ID {mat_id} not found");
                continue;
            };

            material.apply(transform);
            mesh.draw()
        }
    }

    fn events(&mut self, app: &mut T) {
        let mut events = self.triggered_events.clone();
        self.triggered_events.clear();

        for (_, event) in glfw::flush_messages(&self.window.events) {
            let event = event.into();
            events.push(event);
        }

        for event in events {
            match event {
                Event::MouseEnter(true) => {
                    self.mouse_position = self.window.get_cursor_pos();
                }
                Event::MouseMove(x, y) => {
                    self.mouse_position = Vec2::new(x as f32, y as f32);
                }
                _ => {}
            }

            self.handle_event(&event, app);
        }
    }

    fn handle_event(&mut self, event: &Event, app: &mut T) {
        let temp_callbacks = std::mem::take(&mut self.event_callbacks);

        if let Some(callbacks) = temp_callbacks.get(event) {
            debug!("Handling event: {event:?}");
            self.call_callbacks(callbacks, app);
        }

        self.event_callbacks = temp_callbacks;
    }

    fn call_callbacks(&mut self, callbacks: &Vec<EventCallback<T>>, app: &mut T) {
        for callback in callbacks {
            debug!("Calling event callback");
            callback(Commands::new(self), app);
        }
    }
}

impl<T: PaperApp> Debug for Paper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paper").field("window", &self.window).finish()
    }
}

impl<T: PaperApp> Default for Paper<T> {
    fn default() -> Self {
        Self::new(WindowConfig::default())
    }
}

impl<T: PaperApp> CommandsTrait for Paper<T> {
    fn set_clear_color(&mut self, color: Srgba) {
        self.set_clear_color(color);
    }

    fn add_entity(&mut self, entity: Entity) {
        self.add_entity(entity);
    }

    fn remove_entity(&mut self, entity: &Entity) {
        // TODO: COULD REMOVE MESHES AND MATERIALS IF THEY ARE NOT USED BY ANY OTHER ENTITY
        self.entities
            .retain(|e| e.0 != entity.mesh().id() && e.1 != entity.material_type().id() && e.2 != entity.transform);
    }

    fn clear_entities(&mut self) {
        self.entities.clear();
    }

    fn close(&mut self) {
        debug!("Closing application");
        self.window.close();
    }

    fn trigger_event(&mut self, event: Event) {
        debug!("Triggering event: {event:?}");
        self.triggered_events.push(event);
    }

    fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    fn get_delta_time(&self) -> f64 {
        self.delta_time
    }
}
