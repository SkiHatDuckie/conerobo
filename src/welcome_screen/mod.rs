#![allow(non_snake_case)]
// ^ Dioxus components use UpperCammelCase 

use dioxus::prelude::*;

mod document;

#[derive(Props, PartialEq)]
pub struct WelcomeScreenProps<'a> {
    h1_color: &'a str,
    h2_color: &'a str
}

// The lifetime `'a` ensures that the `WelcomeScreenProps` members exist
pub fn WelcomeScreen<'a>(cx: Scope<'a, WelcomeScreenProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            document::Document { h1_color: cx.props.h1_color, h2_color: cx.props.h2_color }
        }
    })
}