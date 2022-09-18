#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch_cfg(App, |c| c.with_window(|w| w.with_title("ConeRobo")));
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        WelcomeScreen { h1_color: "rgb(200, 150, 60)" }
    ))
}

#[derive(Props, PartialEq)]
struct WelcomeScreenProps<'a> {
    h1_color: &'a str
}

// The lifetime `'a` ensures that `h1_color` exists
fn WelcomeScreen<'a>(cx: Scope<'a, WelcomeScreenProps<'a>>) -> Element {
    cx.render(rsx! (
        div {
            h1 {
                color: "{cx.props.h1_color}",
                "Welcome to ConeRobo" 
            }
            p {
                "Hello, world!"
            }
        }
    )) 
}