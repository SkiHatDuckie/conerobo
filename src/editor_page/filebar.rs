#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;
use crate::file_management;

#[inline_props]
pub fn FileButtons(
    cx: Scope,
    text: UseRef<String>,
    num_rows: UseState<u32>,
    filename: UseRef<String>,
) -> Element {
    cx.render(rsx! (
        button { class: "file-button",
            onclick: move |_| {
                text.with(|txt| {
                    filename.with(|name| {
                        file_management::save_file(
                            name.clone(), txt.clone()
                        )
                    })
                })
            },
            "Save"
        }
        button { class: "file-button",
            onclick: move |_| {
                text.set(
                    filename.with(|name| {
                        file_management::load_file(name.clone())
                    })
                );
                text.with(|txt| {
                    let lines: Vec<&str> = txt.split('\n').collect::<Vec<&str>>();
                    num_rows.set(lines.len() as u32);
                })
            },
            "Load"
        }
    ))
}

#[inline_props]
pub fn FilenameInput(cx: Scope, filename: UseRef<String>) -> Element {
    let inner_filename = &*filename.read();

    cx.render(rsx!(
        label { class: "filename-label",
            " File: "
        }
        input { class: "filename-input",
            value: "{inner_filename}",
            oninput: move |evt| { filename.set(evt.value.clone()) }
        }
    ))
}