#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

mod document;

pub fn WelcomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            document::Document {}
        }
    })
}