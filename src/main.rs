use bevy::{
    prelude::*,
    render::pass::ClearColor
};


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "ConeRoboTracker".to_string(),
            width: 480.,
            height: 480.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)))
        .add_plugins(DefaultPlugins)
        .run();
}
