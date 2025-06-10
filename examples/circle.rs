use paper::{app::enable_logging, prelude::*, renderable::MaterialType};

const SEGMENTS: u32 = 64;

struct Body {
    id: Uuid,
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mass: f32,
}

impl Body {
    fn new(position: Vec2, radius: f32, mass: f32) -> Self {
        Self { id: Uuid::new_v4(), position, velocity: Vec2::ZERO, radius, mass }
    }

    fn with_initial_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }

    fn get_gravitational_force(&self, other: &Body) -> Vec2 {
        let distance = self.position.distance(other.position);
        if distance == 0.0 {
            return Vec2::ZERO; // Avoid division by zero
        }
        let force_magnitude = (self.mass * other.mass) / (distance * distance);
        let direction = (other.position - self.position).normalize();
        direction * force_magnitude
    }

    fn apply_force(&mut self, force: Vec2, delta_time: f64) {
        let acceleration = force / self.mass * delta_time as f32;
        self.velocity += acceleration;
    }

    fn update(&mut self, delta_time: f64) {
        self.position += self.velocity * delta_time as f32;
    }

    fn entity(&self, color: Srgba) -> Entity {
        Entity::from_shape_2d(Shape2D::Circle { position: self.position, radius: self.radius, segments: SEGMENTS })
            .with_material_type(MaterialType::Color(color))
    }
}

struct TwoBodyApp {
    b1: Body,
    b2: Body,

    camera_velocity: Vec2,
}

impl Default for TwoBodyApp {
    fn default() -> Self {
        Self {
            b1: Body::new(Vec2::new(-250.0, 0.0), 10.0, 300000.0).with_initial_velocity(Vec2::new(0.0, 30.0)),
            b2: Body::new(Vec2::new(250.0, 0.0), 30.0, 700000.0).with_initial_velocity(Vec2::new(0.0, -10.0)),

            camera_velocity: Vec2::ZERO,
        }
    }
}

impl PaperApp for TwoBodyApp {
    fn setup(&mut self, mut cmd: Commands) {
        cmd.set_clear_color(Srgba::new(0.1, 0.1, 0.1, 1.0));

        self.b1.id = cmd.add_entity(self.b1.entity(BLUE));
        self.b2.id = cmd.add_entity(self.b2.entity(RED));
    }

    fn update(&mut self, mut cmd: Commands) {
        self.b1.update(cmd.get_delta_time());
        self.b2.update(cmd.get_delta_time());

        cmd.get_transform_mut(&self.b1.id).unwrap().set_translation(self.b1.position.extend(0.0));
        cmd.get_transform_mut(&self.b2.id).unwrap().set_translation(self.b2.position.extend(0.0));

        cmd.camera_mut().move_by(self.camera_velocity);
    }

    fn fixed_update(&mut self, cmd: Commands) {
        let force = self.b1.get_gravitational_force(&self.b2);
        self.b1.apply_force(force, cmd.get_fixed_delta_time());

        let force = self.b2.get_gravitational_force(&self.b1);
        self.b2.apply_force(force, cmd.get_fixed_delta_time());
    }
}

fn main() {
    enable_logging(log::LevelFilter::Off);

    Paper::<TwoBodyApp>::default()
        .with_event_callback(Key::Escape.press(), |mut cmd, _| {
            cmd.close();
        })
        .with_event_callback(Key::W.press(), |_, app| {
            app.camera_velocity.y = -1.0;
        })
        .with_event_callback(Key::S.press(), |_, app| {
            app.camera_velocity.y = 1.0;
        })
        .with_event_callback(Event::AnyKey(vec![Key::W, Key::S], Action::Release), |_, app| {
            app.camera_velocity.y = 0.0;
        })
        .with_event_callback(Key::A.press(), |_, app| {
            app.camera_velocity.x = 1.0;
        })
        .with_event_callback(Key::D.press(), |_, app| {
            app.camera_velocity.x = -1.0;
        })
        .with_event_callback(Event::AnyKey(vec![Key::A, Key::D], Action::Release), |_, app| {
            app.camera_velocity.x = 0.0;
        })
        .with_event_callback(Key::Q.press(), |mut cmd, _| {
            cmd.camera_mut().zoom *= 1.1;
        })
        .with_event_callback(Key::E.press(), |mut cmd, _| {
            cmd.camera_mut().zoom /= 1.1;
        })
        .run();
}
