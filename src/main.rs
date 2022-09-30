#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

mod navigation_bar;
mod welcome_screen;
mod editor_screen;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { [include_str!("./style.css")] }
        navigation_bar::NavigationBar {}
    })
}