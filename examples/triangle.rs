pub use paper::prelude::*;

fn main() {
    let mut app = Paper::<EmptyApp>::default().with_event_callback(Key::Escape.press(), |mut cmd, _| {
        cmd.close();
    });

    let tri = Shape2D::triangle(Vec2::new(200.0, 0.0), Vec2::splat(150.0));
    let mesh: Mesh = tri.mesh().with_attribute(AttributeType::Color, flatten(vec![PINK_RED, LIME_GREEN, BLUE_PURPLE]));

    let mesh_id = app.add_mesh(mesh);
    let material_id = app.add_material(DefaultMaterial::default());

    app.add_entity(Entity::MeshMaterial { mesh_id, material_id, transform: tri.transform() });

    app.add_entity(Entity::Primitive {
        shape: Shape2D::Triangle { pos: Vec2::new(-200.0, 0.0), size: Vec2::splat(150.0) },
        color: PINK_RED,
    });

    app.run();
}
