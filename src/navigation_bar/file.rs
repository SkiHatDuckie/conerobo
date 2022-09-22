#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

pub fn Dropdown(cx: Scope) -> Element {
    cx.render(rsx!(
        button {
            onclick: move |_| println!("Hello!"),
            "Hello"
        }
    ))
}