#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        WelcomeScreen {
            h1_color: "rgb(200, 160, 70)",
            h2_color: "rgb(50, 70, 200)"
        }
    ))
}

#[derive(Props, PartialEq)]
struct WelcomeScreenProps<'a> {
    h1_color: &'a str,
    h2_color: &'a str
}

// The lifetime `'a` ensures that `h1_color` exists
fn WelcomeScreen<'a>(cx: Scope<'a, WelcomeScreenProps<'a>>) -> Element {
    let todo_list_entries = [
        "Get comfortable with Dioxus (the library I'm using to make this GUI)",
        "Create a navigation bar for GUI",
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
                color: "{cx.props.h1_color}",
                "Welcome to ConeRobo" 
            }
            p {
                "Hello, world!"
            }
            h2 {
                color: "{cx.props.h2_color}",
                "My current ToDo list"
            }
            ul {
                todo_list
            }
        }
    ))
}