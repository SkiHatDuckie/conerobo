#[derive(Clone, Copy, PartialEq)]
pub struct MenuState(pub i32);

#[derive(Clone)]
pub struct Menu {
    pub title: &'static str,
    pub state: MenuState,
    pub options: &'static [MenuOption]
}

pub struct MenuOption {
    pub option_str: &'static str,
    pub action: Action
}

pub enum Action {
    // Unavailable,
    QuitAttempt,
    Navigation { next_menu: MenuState }
}