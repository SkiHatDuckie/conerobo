#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;

#[inline_props]
pub fn EditorScreen(cx: Scope, text: UseRef<String>, num_rows: UseState<u32>) -> Element {
    let inner_text = &*text.read();
    
    cx.render(rsx! (
        div {
            textarea {
                value: "{inner_text}",
                rows: "{num_rows}",
                cols: "50",
                resize: "none",
                onkeydown: move |evt| {
                    match evt.key.as_str() {
                        "Enter" => num_rows.modify(|n| n + 1),
                        "Backspace" => {
                            text.with(|i| {
                                let lines: Vec<&str> = i.split('\n').collect::<Vec<&str>>();
                                if lines[lines.len() - 1] == "" {
                                    num_rows.modify(|n| n - 1)
                                }
                            })
                        },
                        _ => {}
                    }
                },
                oninput: move |evt| text.set(evt.value.clone()),
            }
        }
    ))
}