use bevy::prelude::*;

use crate::utils::{TextInput, CharVector, Focus, TextInputBundle, Multiline};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[derive(Component)]
pub struct TextboxText;

pub fn spawn_branch(parent: &mut ChildBuilder, font_handle: &Handle<Font>) {
    // UI entities
    let editor_surface = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(22.0),
                ..default()
            },
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };
    let textbox_surface = TextInputBundle {
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(20.0),
                top: Val::Px(0.0),
                ..default()
            },
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    };
    let textbox_text = TextBundle::from_section(
        "Unfocused",
        TextStyle { 
            font: font_handle.clone(), 
            font_size: 16.0, 
            color: Color::rgb_u8(241, 246, 255) 
        }
    );

    // Append entities to node tree
    parent
        .spawn_bundle(editor_surface)
        .with_children(|parent| {
            parent
                .spawn_bundle(textbox_surface)
                .insert(Multiline)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(textbox_text)
                        .insert(TextboxText);
                });
        });
}

// This is all temporary code for checking if focusing on an object works.
pub fn update_textbox_on_focus(
    mut textbox_query: Query<(&mut CharVector, &Focus), (Changed<Focus>, With<TextInput>)>,
    mut text_query: Query<&mut Text, With<TextboxText>>
) {
    textbox_query.for_each_mut(|(mut char_vec, focus)| {
        let mut msg: Vec<char> = match focus.is_focused {
            true => "Focused!".chars().collect(),
            false => "Unfocused".chars().collect()
        };
        char_vec.text.clear();
        char_vec.text.append(&mut msg);

        text_query.for_each_mut(|mut text| {
            text.sections[0].value = char_vec.text.iter().collect::<String>();
        });
    });
}