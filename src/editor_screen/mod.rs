#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;

// The lifetime `'a` ensures that the screen's members exist
#[inline_props]
pub fn EditorScreen<'a>(cx: Scope<'a>, h1_color: &'a str, h2_color: &'a str) -> Element {
    cx.render(rsx! {
        div {
            textarea {
                rows: "1",
                cols: "50",
                resize: "none"
            }
        }
    })
}