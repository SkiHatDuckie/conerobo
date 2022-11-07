pub const MENUS: [Menu; 3] = [
    Menu {
        state: State::MainMenu,
        options: Options {
            vec: &[
                "Configure GUI Launch",
                "Configure component management launch",
                "Launch individual core module",
                "Launch entire core",
                "Exit TUI"
            ]
        }
    },
    Menu {
        state: State::GUILaunch,
        options: Options {
            vec: &[
                "Back to main menu"
            ]
        }
    },
    Menu {
        state: State::ComponentLaunch,
        options: Options {
            vec: &[
                "Back to main menu"
            ]
        }
    }
];

#[derive(Clone)]
pub enum State {
    MainMenu,
    GUILaunch,
    ComponentLaunch,
}

#[derive(Clone)]
pub struct Options {
    pub vec: &'static [&'static str],
}

#[derive(Clone)]
pub struct Menu {
    pub state: State,
    pub options: Options
}