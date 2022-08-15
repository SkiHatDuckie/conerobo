use bevy::prelude::*;

// Enum for dropdown nodes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Component)]
pub enum Dropdown {
    Inactive,
    JustActivated,
    Active
}

// Struct for buttons that exit the app when clicked.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Component)]
pub struct ExitButton;