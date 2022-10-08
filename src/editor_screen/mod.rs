#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;
use crate::file_management;

#[inline_props]
pub fn EditorScreen(cx: Scope, text: UseRef<String>, num_rows: UseState<u32>) -> Element {
    let inner_text = &*text.read();
    
    cx.render(rsx! (
        div {
            FileButtons { text: text.clone(), num_rows: num_rows.clone() }
        }
        div { class: "editor",
            LineNumbers { num_rows: num_rows.clone() }
            textarea {
                value: "{inner_text}",
                rows: "{num_rows}",
                onkeydown: move |evt| {
                    match evt.key.as_str() {
                        "Enter" => num_rows.modify(|n| n + 1),
                        "Backspace" => {
                            text.with(|txt| {
                                let lines: Vec<&str> = txt.split('\n').collect::<Vec<&str>>();
                                if lines[lines.len() - 1] == "" && lines.len() > 1 {
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

// TODO: Add ability to set filename to save and load.
// Since `text` is a shared reference, we need to ensure that it still exists before saving.
// `testfile.py` is temporary.
#[inline_props]
fn FileButtons(cx: Scope, text: UseRef<String>, num_rows: UseState<u32>) -> Element {
    cx.render(rsx! (
        div { class: "file-buttons",
            button {
                onclick: move |_| {
                    text.with(|txt| {
                        file_management::save_file(
                            "testfile.py".to_owned(), txt.clone()
                        )
                    })
                },
                "Save"
            }
            button {
                onclick: move |_| {
                    text.set(file_management::load_file("testfile.py".to_owned()));
                    text.with(|txt| {
                        let lines: Vec<&str> = txt.split('\n').collect::<Vec<&str>>();
                        num_rows.set(lines.len() as u32);
                    })
                },
                "Load"
            }
        }
    ))
}

#[inline_props]
fn LineNumbers(cx: Scope, num_rows: UseState<u32>) -> Element {
    let numbered_lines = (1..=*num_rows.get()).map(|_| rsx!(span {}));

    cx.render(rsx! (
        div { class: "line-numbers",
            numbered_lines
        }
    ))
}