#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            h1 {
                color: "rgb(200, 150, 60)",
                "Welcome to ConeRobo" 
            }
            p {
                "Hello, world!"
            }
        }
    ))
}