use bevy::{
    prelude::*, 
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

// Constants
const WIN_WIDTH: f32 = 560.0;
const WIN_HEIGHT: f32 = 560.0;

// Initialize window
struct WindowInitPlugin;

impl Plugin for WindowInitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(WindowDescriptor {
                title: "Simple Game".to_string(),
                width: WIN_WIDTH,
                height: WIN_HEIGHT,
                vsync: true,
                resizable: false,
                ..Default::default()
            })
            .insert_resource(ClearColor(Color::rgb_u8(10, 25, 30)));
    }
}

// Simple paddle game as my introduction to developing RL AI
fn main() {
    App::build()
        .add_plugin(WindowInitPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(paddle_movement_system.system())
        .add_system(ball_collision_system.system())
        .add_system(ball_movement_system.system())
        .run();
}

struct Paddle {
    speed: f32,
}

struct Ball {
    velocity: Vec3,
}

enum Collider {
    Solid,
    Paddle,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add entities to the environment
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Paddle
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb_u8(245, 245, 175).into()),
        transform: Transform::from_xyz(0.0, -245.0, 0.0),
        sprite: Sprite::new(Vec2::new(100.0, 25.0)),
        ..Default::default()
    })
    .insert(Paddle { speed: 400.0 })
    .insert(Collider::Paddle);

    // ball
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
     // left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
    // right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
    // top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness, wall_thickness)),
        ..Default::default()
    })
    .insert(Collider::Solid);
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((paddle, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(220.0).max(-220.0);
    }
}

fn ball_movement_system(
    time: Res<Time>, 
    mut ball_query: Query<(&Ball, &mut Transform)>
) {
    // clamp the timestep to stop the ball from escaping when the game starts
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * delta_seconds;
    }
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    if let Ok((mut ball, ball_transform, sprite)) = ball_query.single_mut() {
        let ball_size = sprite.size;
        let velocity = &mut ball.velocity;

        // check collision with walls
        for (collider, transform, sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                sprite.size,
            );
            if let Some(collision) = collision {
                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = velocity.x > 0.0,
                    Collision::Right => reflect_x = velocity.x < 0.0,
                    Collision::Top => reflect_y = velocity.y < 0.0,
                    Collision::Bottom => reflect_y = velocity.y > 0.0,
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    velocity.x = -velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    velocity.y = -velocity.y;
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is
                // also in collision
                if let Collider::Solid = *collider {
                    break;
                }
            }
        }
    }
}
