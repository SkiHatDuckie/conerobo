use std::borrow::Cow;

pub const MAIN_MENU: Menu = Menu {
    options: &[
        "Configure GUI Launch",
        "Configure component management launch",
        "Launch individual core module",
        "Launch entire core",
        "Exit TUI"
    ],
    actions: Cow::Borrowed(&[
        Action::Navigation { next_menu: MenuState::GUILaunch },
        Action::Navigation { next_menu: MenuState::ComponentLaunch },
        Action::Unavailable,
        Action::Unavailable,
        Action::QuitAttempt
    ])
};
pub const GUI_LAUNCH: Menu = Menu {
    options: &[
        "Back to main menu"
    ],
    actions: Cow::Borrowed(&[
        Action::Navigation { next_menu: MenuState::MainMenu }
    ])
};
pub const COMPONENT_LAUNCH: Menu = Menu {
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
    GUILaunch,
    ComponentLaunch
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