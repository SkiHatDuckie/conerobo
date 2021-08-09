use bevy::{
    prelude::*, 
};
use super::components::*;

// Constants
const WIN_WIDTH: f32 = 560.0;
const WIN_HEIGHT: f32 = 560.0;

pub fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Add entities to the environment
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Paddle
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb_u8(245, 200, 175).into()),
        transform: Transform::from_xyz(0.0, -250.0, 0.0),
        sprite: Sprite::new(Vec2::new(100.0, 25.0)),
        ..Default::default()
    })
    .insert(Paddle { speed: 400.0 })
    .insert(Collider::Paddle);

    // Ball
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb_u8(245, 200, 175).into()),
        transform: Transform::from_xyz(0.0, -25.0, 1.0),
        sprite: Sprite::new(Vec2::new(25.0, 25.0)),
        ..Default::default()
    })
    .insert(Ball {
        velocity: 400.0 * Vec3::new(0.4, 0.4, 0.0).normalize(),
    });

     // Add walls
     let wall_material = materials.add(Color::rgb_u8(75, 185, 225).into());
     let wall_thickness = 10.0;
     let bounds = Vec2::new(WIN_WIDTH, WIN_HEIGHT);
     // Left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
    // Right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
    // Bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid)
    .insert(Collider::Bottom);
    // Top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);

    // Scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts\\EightBit Atari-Backtalk10.ttf"),
                        font_size: 20.0,
                        color: Color::rgb_u8(235, 235, 165),
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts\\EightBit Atari-Backtalk10.ttf"),
                        font_size: 20.0,
                        color: Color::rgb_u8(235, 235, 165),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}
