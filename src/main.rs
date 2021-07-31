use bevy::{
    prelude::*,
    render::pass::ClearColor
};

struct WindowInitPlugin;

impl Plugin for WindowInitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
                title: "ConeRoboTracker".to_string(),
                width: 480.,
                height: 480.,
                vsync: true,
                ..Default::default()
            })
            .insert_resource(ClearColor(Color::rgb(0.00, 0.00, 0.00)));
    }
}

fn main() {
    App::build()
        .add_plugin(WindowInitPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

// Text component
struct UIText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text
    commands.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                text: Text::with_section(
                    "SkiHatDuckie Was Here!",
                    TextStyle {
                        font: asset_server.load("fonts\\EightBit Atari-Backtalk10.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.30, 0.50, 0.80),
                    },
                    Default::default()
                ),
                ..Default::default()
            })
            .insert(UIText);
}
