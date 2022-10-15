#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase

use dioxus::prelude::*;
mod filebar;
mod editor;

#[inline_props]
pub fn EditorPage(
    cx: Scope,
    text: UseRef<String>,
    num_rows: UseState<u32>,
    filename: UseRef<String>
) -> Element {
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
            editor::LineNumbers { num_rows: num_rows.clone() }
            editor::TextEditor { text: text.clone(), num_rows: num_rows.clone() }
        }
    ))
}