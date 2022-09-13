mod menubar;
mod editor;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_ui)
        .add_system(editor::update_textbox_on_focus);
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FSEX300.ttf");

    let root_node = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::SpaceBetween,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        color: Color::rgb_u8(73, 153, 187).into(),
        ..default()
    };

    // Create UI node tree
    commands
        .spawn_bundle(root_node)
        .with_children(|parent| {
            editor::spawn_branch(parent, &font_handle);
            menubar::spawn_branch(parent, &font_handle);
        });
}