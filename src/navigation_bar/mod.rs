#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;
use crate::welcome_screen;
use crate::editor_screen;

pub fn NavigationBar(cx: Scope) -> Element {
    let active_screen = use_state(&cx, || 0);
    let screen = match active_screen.get() {
        1 => { rsx!( welcome_screen::WelcomeScreen {
            h1_color: "rgb(200, 160, 70)",
            h2_color: "rgb(50, 70, 200)"
        })},
        2 => { rsx!( editor_screen::EditorScreen {
            h1_color: "rgb(200, 160, 70)",
            h2_color: "rgb(50, 70, 200)"
        })},
        _ => { rsx!( div {} ) }
    };

    cx.render(rsx! (
        div {
            button {
                onclick: move |_| {
                    match active_screen.get() {
                        1 => active_screen.set(0),
                        _ => active_screen.set(1)
                    }
                },
                "Welcome!"
            }
            button {
                onclick: move |_| {
                    match active_screen.get() {
                        2 => active_screen.set(0),
                        _ => active_screen.set(2)
                    }
                },
                "Editor"
            }
        }
        div {
            screen
        }
    ))
}