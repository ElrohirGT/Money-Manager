use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn TextLoading(cx: Scope) -> Element {
    rsx!(cx, h1 { "Loading..." })
}
