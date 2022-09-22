#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

mod file;

pub fn NavigationBar(cx: Scope) -> Element {
    let active_dropdown = use_state(&cx, || 0);
    let dropdown = match active_dropdown.get() {
        1 => { rsx!( file::Dropdown {} ) },
        _ => { rsx!( div {} ) }
    };

    cx.render(rsx! (
        div {
            button {
                onclick: move |_| {
                    match active_dropdown.get() {
                        1 => active_dropdown.set(0),
                        _ => active_dropdown.set(1)
                    }
                },
                "File"
            }
        }
        div {
            dropdown
        }
    ))
}