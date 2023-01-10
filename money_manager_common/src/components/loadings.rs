use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn TextLoading(cx: Scope) -> Element {
    rsx!(cx, h1 { "Loading..." })
}

#[allow(non_snake_case)]
pub fn LoadingPage(cx: Scope) -> Element {
    rsx!(
        cx,
        div {
            class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
            div {
                class: "flex flex-col shrink-0 p-20 bg-slate-900 gap-y-4 text-white w-1/2 h-9/12",
                TextLoading {}
            }
        }
    )
}
