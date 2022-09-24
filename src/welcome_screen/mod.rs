#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

mod document;

// The lifetime `'a` ensures that the screen's members exist
#[inline_props]
pub fn WelcomeScreen<'a>(cx: Scope<'a>, h1_color: &'a str, h2_color: &'a str) -> Element {
    cx.render(rsx! {
        div {
            document::Document { h1_color: h1_color, h2_color: h2_color }
        }
    })
}