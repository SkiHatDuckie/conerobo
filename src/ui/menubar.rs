use bevy::prelude::*;

use crate::utils::{Dropdown, UIButtonBundle, UIName};

pub fn spawn_branch(parent: &mut ChildBuilder, font_handle: Handle<Font>) {
    // UI Entities
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

    // Append entities to node tree
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
}