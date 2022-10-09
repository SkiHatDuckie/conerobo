#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

mod file_management;
mod navigation_bar;
mod welcome_page;
mod editor_page;

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { [include_str!("./style.css")] }
        body {
            navigation_bar::NavigationBar {}
        }
    })
}