#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

mod welcome_screen;
mod navigation_bar;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        navigation_bar::NavigationBar {}
    })
}