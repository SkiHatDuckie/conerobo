use bevy::{
    prelude::*,
    ui::FocusPolicy,
};

// Marker struct for dropdown nodes
#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component, Default)]
pub struct Dropdown;

// A UI node designed to funcion as a dropdown.
#[derive(Bundle, Clone, Debug, Default)]
pub struct DropdownNodeBundle {
    // Describes the size of the node
    pub node: Node,
    // Describes the style including flexbox settings
    pub style: Style,
    // Describes the color of the node
    pub color: UiColor,
    // Describes the image of the node
    pub image: UiImage,
    // Marker component that signals this node is a dropdown
    pub dropdown: Dropdown,
    // Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    // The transform of the node
    pub transform: Transform,
    // The global transform of the node
    pub global_transform: GlobalTransform,
    // Describes the visibility properties of the node
    pub visibility: Visibility,
    // Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}