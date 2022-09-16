use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}

// Colors:
// (26, 24, 36) Xiketic - Good for backgorund
// (67, 66, 69) Onyx
// (171, 177, 179) Silver Chalice
// (241, 246, 255) Alice Blue - Good for background
// (145, 210, 234)  Sky Blue
// (73, 153, 187)  Blue Green - Good for background
// (57, 67, 111)  Purple Navy