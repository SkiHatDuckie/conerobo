mod utils;

use utils::{
    Dropdown, UIName, UIButton, UIButtonBundle
};
use bevy::{
    prelude::*,
    winit::WinitSettings,
    app::AppExit,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "ConeRobo".to_owned(),
            width: 600.0,
            height: 600.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(process_button_interaction)
        .add_system(update_dropdowns.after(process_button_interaction))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("fonts/FSEX300.ttf");

    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // UI Entities
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
    let menu_bar = NodeBundle {
        style: Style {
            justify_content: JustifyContent::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            size: Size::new(Val::Percent(100.0), Val::Px(22.0)),
            ..default()
        },
        color: Color::rgb_u8(241, 246, 255).into(),
        ..default()
    };
    let file_button = UIButtonBundle {
        name: UIName { name: "dropdown".to_owned() },
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };
    let file_button_text = TextBundle::from_section(
        "File", 
        TextStyle {
            font: font_handle.clone(),
            font_size: 20.0,
            color: Color::rgb_u8(26, 24, 36)
        }
    );
    let file_button_dropdown = NodeBundle {
        style: Style {
            justify_content: JustifyContent::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(22.0),
                ..default()
            },
            size: Size::new(Val::Auto, Val::Auto),
            ..default()
        },
        color: Color::rgb_u8(171, 177, 179).into(),
        visibility: Visibility { is_visible: false },
        ..default()
    };
    let exit_button = UIButtonBundle {
        name: UIName { name: "exit".to_owned() },
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };
    let exit_button_text = TextBundle::from_section(
        "Exit", 
        TextStyle {
            font: font_handle.clone(),
            font_size: 20.0,
            color: Color::rgb_u8(26, 24, 36)
        }
    );

    // Create UI node tree
    commands
        .spawn_bundle(root_node)
        .with_children(|parent| {
            parent
                .spawn_bundle(menu_bar)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(file_button)
                        .with_children(|parent| {
                            parent.spawn_bundle(file_button_text);
                            parent
                                .spawn_bundle(file_button_dropdown)
                                .insert(Dropdown::Inactive)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(exit_button)
                                        .with_children(|parent| {
                                            parent.spawn_bundle(exit_button_text);
                                        });
                                });
                        });
                });
        });
}

fn update_dropdowns(
    buttons: Res<Input<MouseButton>>, 
    mut dropdown_query: Query<(&mut Visibility, &mut Dropdown)>
) {
    if buttons.any_just_released([MouseButton::Left, MouseButton::Right]) {
        for mut dropdown in dropdown_query.iter_mut() {
            if *dropdown.1 == Dropdown::JustActivated {
                *dropdown.1 = Dropdown::Active;
            }
        }
    }
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        for mut dropdown in dropdown_query.iter_mut() {
            if *dropdown.1 == Dropdown::Active {
                dropdown.0.is_visible = false;
            }
        }
    }
}

fn process_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &UIName, &Children),
        (Changed<Interaction>, With<UIButton>),
    >,
    mut dropdown_query: Query<(&mut Visibility, &mut Dropdown)>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, ui_name, children) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match ui_name.name.as_str() {
                    "dropdown" => {
                        for child in children {
                            // Change the visibility of a dropdown, if the button has one as a child.
                            if let Ok(mut dropdown) = dropdown_query.get_mut(*child) {
                                dropdown.0.is_visible = true;
                                *dropdown.1 = Dropdown::JustActivated;
                            }
                        }
                    }
                    "exit" => { exit.send(AppExit); },
                    _ => {}
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
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