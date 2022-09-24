#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

// The lifetime `'a` ensures that the `DocumentProps` members exist
#[inline_props]
pub fn Document<'a>(cx: Scope, h1_color: &'a str, h2_color: &'a str) -> Element {
    let todo_list_entries = [
        "Get comfortable with Dioxus (the library I'm using to make this GUI)",
        "Add more information to the welcome screen (what you're looking at right now)",
        "Create a barebones IDE for basic text editing",
        "Keep codebase clean and simple whenever possible"
    ];
    let todo_list = todo_list_entries.iter().map(|entry| rsx!(
        li { "{entry}" }
    ));

    cx.render(rsx! (
        div {
            h1 {
                color: "{h1_color}",
                "Welcome to ConeRobo" 
            }
            p {
                "Hello! This is a work-in-progress personal project that I'm working on. As you can tell,
                there's still much for me to do. However, I am currently motivated to keep working on it."
            }
            h2 {
                color: "{h2_color}",
                "My current ToDo list"
            }
            ul {
                todo_list
            }
            a {
                href: "https://github.com/SkiHatDuckie/conerobo",
                target: "_blank",
                "Link to the ConeRobo github page"
            }
        }
    ))
}