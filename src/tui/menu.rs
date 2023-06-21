use std::borrow::Cow;

pub const MAIN_MENU: Menu = Menu {
    options: &[
        "Component Menu",
        "Exit TUI"
    ],
    actions: Cow::Borrowed(&[
        Action::Navigation { next_menu: MenuState::ComponentMenu },
        Action::QuitAttempt
    ])
};
pub const COMPONENT_MENU: Menu = Menu {
    options: &[
        "Back to main menu"
    ],
    actions: Cow::Borrowed(&[
        Action::Navigation { next_menu: MenuState::MainMenu }
    ])
};

#[derive(Clone)]
pub enum MenuState {
    MainMenu,
    ComponentMenu
}

#[derive(Clone)]
pub struct Menu {
    pub options: &'static [&'static str],
    pub actions: Cow<'static, [Action]>
}

#[derive(Clone)]
pub enum Action {
    Unavailable,
    QuitAttempt,
    Navigation { next_menu: MenuState }
}