#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

pub mod file_management;
pub mod navigation_bar;
pub mod welcome_page;
pub mod editor_page;

pub fn launch_gui() {
    dioxus::desktop::launch_cfg(
        App, 
        |config| config.with_custom_index(include_str!("index.html").to_owned())
    );
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { [include_str!("style.css")] }
        body {
            navigation_bar::NavigationBar {}
        }
    })
}