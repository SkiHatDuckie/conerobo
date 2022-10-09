#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;
mod filebar;

#[inline_props]
pub fn EditorScreen(
    cx: Scope,
    text: UseRef<String>,
    num_rows: UseState<u32>,
    filename: UseRef<String>
) -> Element {
    let inner_text = &*text.read();
    
    cx.render(rsx! (
        div { class: "filebar",
            filebar::FileButtons {
                text: text.clone(),
                num_rows: num_rows.clone(),
                filename: filename.clone(),
            }
            filebar::FilenameInput { filename: filename.clone() }
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

#[inline_props]
fn LineNumbers(cx: Scope, num_rows: UseState<u32>) -> Element {
    let numbered_lines = (1..=*num_rows.get()).map(|_| rsx!(span {}));

    cx.render(rsx! (
        div { class: "line-numbers",
            numbered_lines
        }
    ))
}