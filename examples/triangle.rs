use paper::{app::enable_logging, entity::AttributeType, prelude::*};

fn main() {
    enable_logging();

    let colors: Vec<f32> = flatten([RED.to_vec(), GREEN.to_vec(), BLUE.to_vec()]);

    let triangle =
        Entity::from_shape_2d(Shape2D::Triangle { position: Vec2::new(0.0, 0.0), width: 50.0, height: 50.0 })
            .with_attribute(AttributeType::Color, colors);

    Paper::<EmptyApp>::default()
        .with_entity(triangle, None)
        .with_event_callback(Key::Escape.press(), |mut cmd, _| {
            cmd.close();
        })
        .run();
}
