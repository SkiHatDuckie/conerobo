mod input;
mod ui;
mod utils;

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "ConeRobo".to_owned(),
            width: 600.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(input::InputPlugin)
        .add_plugin(ui::UIPlugin)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

//let colors = [
//    Color::rgb_u8(26, 24, 36),     // Xiketic - Good for backgorund
//    Color::rgb_u8(67, 66, 69),     // Onyx
//    Color::rgb_u8(171, 177, 179),  // Silver Chalice
//    Color::rgb_u8(241, 246, 255),  // Alice Blue - Good for background
//    Color::rgb_u8(145, 210, 234),  // Sky Blue
//    Color::rgb_u8(73, 153, 187),   // Blue Green - Good for background
//    Color::rgb_u8(57, 67, 111)     // Purple Navy
//];