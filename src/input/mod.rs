use bevy::{prelude::*, app::AppExit};

use crate::utils::{UIButton, Dropdown, UIName};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(process_button_interaction)
            .add_system(update_dropdowns.after(process_button_interaction));
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