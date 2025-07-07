use paper::prelude::*;

struct TwoBodyApp {
    body_1: Body,
    body_2: Body,
}

impl PaperApp for TwoBodyApp {
    fn new(mut cmd: Commands) -> Self {
        let pos_1 = Vec2 { x: 100.0, y: 0.0 };
        let pos_2 = Vec2 { x: -100.0, y: 0.0 };

        let entity_1 = cmd.add_entity(Entity::Primitive {
            shape: Shape2D::Circle { pos: pos_1, radius: 15.0, segments: 16 },
            color: WHITE,
        });

        let entity_2 = cmd.add_entity(Entity::Primitive {
            shape: Shape2D::Circle { pos: pos_2, radius: 15.0, segments: 16 },
            color: WHITE,
        });

        let body_1 = Body { id: entity_1, position: pos_1, velocity: Vec2::new(0.0, -15.0), mass: 700000.0 };

        let body_2 = Body { id: entity_2, position: pos_2, velocity: Vec2::new(0.0, 30.0), mass: 300000.0 };

        Self { body_1, body_2 }
    }

    fn update(&mut self, mut cmd: Commands) {
        // Calculate gravitational force between the two bodies
        let dt = cmd.delta_time();

        self.body_1.update(dt);
        self.body_2.update(dt);

        // Update entity positions in the app
        cmd.set_entity_translation(&self.body_1.id, self.body_1.position);
        cmd.set_entity_translation(&self.body_2.id, self.body_2.position);
    }

    fn fixed_update(&mut self, cmd: Commands) {
        let dt = cmd.fixed_delta_time();

        self.body_1.apply_force(self.body_1.get_gravitational_force(&self.body_2), dt);
        self.body_2.apply_force(self.body_2.get_gravitational_force(&self.body_1), dt);
    }
}

struct Body {
    id:       EntityId,
    position: Vec2,
    velocity: Vec2,
    mass:     f32,
}

impl Body {
    fn get_gravitational_force(&self, other: &Body) -> Vec2 {
        let distance = self.position.distance(other.position);
        if distance == 0.0 {
            return Vec2::ZERO;
        }
        let force_magnitude = other.mass / (distance * distance);
        let direction = (other.position - self.position).normalize();
        direction * force_magnitude
    }

    fn apply_force(&mut self, force: Vec2, dt: f32) {
        let acceleration = force * dt;
        self.velocity += acceleration;
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

fn main() {
    let mut app = Paper::<TwoBodyApp>::default();

    app.run();
}
