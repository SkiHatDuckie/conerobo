#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct EditorProps<'a> {
    h1_color: &'a str,
    h2_color: &'a str
}

// The lifetime `'a` ensures that the `EditorProps` members exist
pub fn EditorScreen<'a>(cx: Scope<'a, EditorProps<'a>>) -> Element {
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