#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;

#[inline_props]
pub fn EditorScreen(cx: Scope, text: UseRef<String>) -> Element {
    let inner_text = &*text.read();

    cx.render(rsx! (
        div {
            textarea {
                rows: "1",
                cols: "50",
                resize: "none",
                oninput: move |evt| text.set(evt.value.clone()),
                "{inner_text}"
            }
        }
    ))
}