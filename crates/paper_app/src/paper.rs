use std::fmt::Debug;

use glfw::Context;
use hashbrown::HashMap;
use log::{debug, error, info};
use paper_color::DEEP_BLUE;
use paper_input::Event;
use paper_math::Vec2;
use paper_render::{ColorMaterial, InternalMesh, Material, Mesh, Shader, ShaderUniform};
use paper_utils::default;
use paper_window::{Window, prelude::WindowConfig};

use crate::{
    Camera2D, EmptyApp, Entity, EntityId, MaterialId, MeshId, PaperApp,
    commands::{Commandable, Commands},
};

const FIXED_TIME_STEP: f64 = 1.0 / 24.0;

pub type EventCallback<T> = Box<dyn Fn(Commands, &mut T)>;

pub struct Paper<T: PaperApp = EmptyApp> {
    window: Window,

    max_fps:    Option<f64>,
    delta_time: f64,

    fixed_time_step:  f64,
    fixed_delta_time: f64,

    mouse_pos: Vec2,

    pub(crate) camera: Camera2D,

    triggered_events: Vec<Event>,
    event_callbacks:  HashMap<Event, Vec<EventCallback<T>>>,
    current_events:   Vec<Event>,

    entities: HashMap<EntityId, (Entity, Vec<(String, ShaderUniform)>)>,

    pub(crate) meshes:    HashMap<MeshId, InternalMesh>,
    pub(crate) materials: HashMap<MaterialId, Box<dyn Material>>,
    // material_name_map:    HashMap<String, MaterialId>,
}

impl<T: PaperApp> Paper<T> {
    pub fn new(config: &WindowConfig) -> Self {
        let _ = env_logger::Builder::from_default_env()
            .format_target(false)
            .format_indent(Some(29))
            .filter_level(log::LevelFilter::Debug)
            .try_init();

        let Some(window) = Window::new(config) else {
            error!("Failed to create window");
            std::process::exit(1);
        };

        info!("Paper application initialized");

        Self {
            window,

            max_fps: None,
            delta_time: 0.0,

            fixed_time_step: FIXED_TIME_STEP,
            fixed_delta_time: 0.0,

            mouse_pos: Vec2::ZERO,

            camera: Camera2D { viewport: Vec2::new(config.width as f32, config.height as f32), ..default() },

            triggered_events: Vec::new(),
            event_callbacks: HashMap::new(),
            current_events: Vec::new(),

            entities: HashMap::new(),

            meshes: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let mut app = T::default();

        app.setup(Commands::new(self));

        let frame_time = self.max_fps.map(|fps| 1.0 / fps);
        let mut last_frame = std::time::Instant::now();
        let mut delta_time = 0.0;
        let mut fixed_delta_time = 0.0;

        self.window.set_clear_color(DEEP_BLUE);

        while !self.window.p_window.should_close() {
            let now = std::time::Instant::now();
            delta_time += now.duration_since(last_frame).as_secs_f64();
            fixed_delta_time += delta_time;
            last_frame = now;

            self.window.glfw.poll_events();
            let mut events = self.events(&mut app);
            self.current_events.append(&mut events);

            if fixed_delta_time >= self.fixed_time_step {
                self.fixed_delta_time = fixed_delta_time;

                app.fixed_update(Commands::new(self));

                fixed_delta_time -= self.fixed_time_step;
            }

            if let Some(frame_time) = frame_time {
                if delta_time < frame_time {
                    continue;
                }

                self.delta_time = delta_time;
                delta_time -= frame_time;
            }

            app.update(Commands::new(self));

            self.render();

            self.current_events.clear();
            self.triggered_events.clear();
        }

        app.cleanup(Commands::new(self));
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

    pub fn set_camera(&mut self, camera: Camera2D) {
        self.camera = camera;
    }

    pub fn with_camera(mut self, camera: Camera2D) -> Self {
        self.set_camera(camera);
        self
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> MeshId {
        let mesh_id = MeshId::new(&mesh);
        if self.meshes.contains_key(&mesh_id) {
            debug!("Mesh with ID {mesh_id:?} already exists, returning existing mesh ID");
            return mesh_id;
        }
        let internal_mesh = InternalMesh::build(mesh);
        self.meshes.insert(mesh_id, internal_mesh);
        debug!("Added mesh with ID: {mesh_id:?}");
        mesh_id
    }

    pub fn add_material<M: Material + 'static>(&mut self, mut material: M) -> MaterialId {
        let material_id = MaterialId::new();
        if material.shaders_from_source() {
            material.set_shader(Shader::from_source(material.vertex_shader(), material.fragment_shader()));
        } else {
            material.set_shader(Shader::new(material.vertex_shader(), material.fragment_shader()));
        }
        self.materials.insert(material_id, Box::new(material));
        debug!("Added material with ID: {material_id:?}");
        material_id
    }

    pub fn set_material_uniform(&mut self, material_id: MaterialId, name: &str, value: ShaderUniform) {
        let Some(material) = self.materials.get_mut(&material_id) else {
            error!("Material with ID {material_id:?} not found");
            return;
        };
        material.set_uniform(name, value);
        debug!("Set uniform '{name}' for material with ID: {material_id:?} = ({:?})", material.name());
    }

    pub fn add_entity(&mut self, mut entity: Entity) -> EntityId {
        let id = EntityId::new();
        let uniforms = entity.uniforms();

        if let Entity::Primitive { shape, .. } = entity {
            let mesh = shape.mesh();
            let mesh_id = self.add_mesh(mesh);
            let material_id = self.add_material(ColorMaterial::default());
            entity = Entity::MeshMaterial { mesh_id, material_id, transform: shape.transform() };
            debug!("Transformed primitive entity into MeshMaterial");
        }

        self.entities.insert(id, (entity, uniforms));
        debug!("Added entity with ID: {id:?}");
        id
    }

    pub fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id).map(|(entity, _)| entity)
    }

    pub fn remove_entity(&mut self, id: &EntityId) -> Option<Entity> {
        let entity = self.entities.remove(id);

        if self.entities.remove(id).is_none() {
            error!("Failed to remove entity with ID: {id:?} (not found)");
        } else {
            debug!("Removed entity with ID: {id:?}");
        }

        entity.map(|(entity, _)| entity)
    }

    // ---------------< PRIVATE >---------------

    fn render(&mut self) {
        self.window.clear();

        let entities = self.entities.values().cloned().collect::<Vec<_>>();

        for (entity, uniforms) in entities {
            entity.draw(self, uniforms);
        }

        self.window.p_window.swap_buffers();
    }

    fn events(&mut self, app: &mut T) -> Vec<Event> {
        let mut events: Vec<Event> = self.triggered_events.drain(..).collect();

        for (_, event) in glfw::flush_messages(&self.window.events) {
            events.push(event.into());
        }

        for event in &events {
            match *event {
                Event::MouseEnter(true) => {
                    let cursor_pos = self.window.p_window.get_cursor_pos();
                    self.mouse_pos = Vec2::new(cursor_pos.0 as f32, cursor_pos.1 as f32);
                }
                Event::MouseMove(x, y) => {
                    self.mouse_pos = Vec2::new(x as f32, y as f32);
                }
                _ => {}
            }

            self.handle_event(event, app);
        }

        app.event_handler(Commands::new(self), &events);

        events
    }

    fn handle_event(&mut self, event: &Event, app: &mut T) {
        let event_callbacks = std::mem::take(&mut self.event_callbacks);

        if let Some(callbacks) = event_callbacks.get(event) {
            debug!("Handling event: {event:?}");
            self.call_callbacks(callbacks, app);
        }

        self.event_callbacks = event_callbacks;
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
        f.debug_struct("Paper")
            .field("window", &self.window)
            .field("max_fps", &self.max_fps)
            .field("delta_time", &self.delta_time)
            .field("fixed_time_step", &self.fixed_time_step)
            .field("fixed_delta_time", &self.fixed_delta_time)
            .field("mouse_pos", &self.mouse_pos)
            .field("triggered_events", &self.triggered_events)
            .finish()
    }
}

impl Default for Paper {
    fn default() -> Self {
        Self::new(&WindowConfig::default())
    }
}

impl<T: PaperApp> Commandable for Paper<T> {
    fn close(&mut self) {
        self.window.p_window.set_should_close(true);
        info!("Closing Paper application");
    }

    fn events(&self) -> &Vec<Event> {
        &self.current_events
    }
}
