use bevy::{prelude::*, ui::FocusPolicy};

/// Enum for dropdown nodes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Component)]
pub enum Dropdown {
    Inactive,
    JustActivated,
    Active
}

/// Can be added to UI elements to help identify specific widgets.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[derive(Component)]
pub struct UIName {
    pub name: String
}

/// Marker struct for UI buttons.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[derive(Component)]
pub struct UIButton;

/// A UI button with a name that can identify its action when interacted with.
#[derive(Bundle, Clone, Debug)]
pub struct UIButtonBundle {
    pub name: UIName,
    /// Describes the size of the node
    pub node: Node,
    pub button: UIButton,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the node
    pub color: UiColor,
    /// The image of the node
    pub image: UiImage,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for UIButtonBundle {
    fn default() -> Self {
        UIButtonBundle {
            name: Default::default(),
            button: UIButton,
            interaction: Default::default(),
            focus_policy: Default::default(),
            node: Default::default(),
            style: Default::default(),
            color: Default::default(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}