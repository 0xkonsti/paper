pub use paper::prelude::*;

fn main() {
    let mut app = Paper::default().with_event_callback(Key::Escape.press(), |mut cmd, _| {
        cmd.close();
    });

    let mesh = Mesh::new()
        .with_attribute(AttributeType::Position, vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0])
        .with_attribute(
            AttributeType::Color,
            flatten(vec![
                Srgba::rgba(1.0, 0.0, 0.0, 1.0),
                Srgba::rgba(0.0, 1.0, 0.0, 1.0),
                Srgba::rgba(0.0, 0.0, 1.0, 1.0),
            ]),
        )
        .with_indices(vec![0, 1, 2]);

    let mesh_id = app.add_mesh(mesh);
    let material_id = app.add_material(DefaultMaterial::default());
    app.add_entity(Entity::MeshMaterial { mesh_id, material_id, transform: Transform::from_scale(Vec3::splat(150.0)) });

    app.run();
}
