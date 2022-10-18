#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

pub fn Document(cx: Scope) -> Element {
    let todo_list_entries = [
        "Update this welcome screen as needed",
        "Figure out how ConeRobo Component management is gonna work",
        "Create an API for making ConeRobo Components",
        "Keep codebase clean and simple whenever possible",
        "Learn Common Lisp (the language I chose for Component management",
    ];
    let todo_list = todo_list_entries.iter().map(|entry| rsx!(
        li { "{entry}" }
    ));

    cx.render(rsx! (
        div {
            h1 { "Welcome to ConeRobo" }
            p {
                "Hello! This is a work-in-progress personal project that I'm working on. As you can tell,
                there's still much for me to do. However, I am currently motivated to keep working on it."
            }
            h2 { "My current ToDo list" }
            ul { todo_list }
            a {
                href: "https://github.com/SkiHatDuckie/conerobo",
                target: "_blank",
                "Link to the ConeRobo github page"
            }
        }
    ))
}