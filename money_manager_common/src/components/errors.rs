use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn ErrorPage<'a>(cx: Scope, code: &'a str, title: &'a str, error: &'a str) -> Element<'a> {
    rsx!(
        cx,
        div {
            class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
            div {
                class: "p-20 bg-slate-900 gap-y-4 text-white w-1/2 h-9/12",
                ErrorComponent { code: code, title: title, error: error }
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn ErrorComponent<'a>(cx: Scope, code: &'a str, title: &'a str, error: &'a str) -> Element<'a> {
    rsx!(
        cx,
        div {
            class: "flex flex-col",
            h1 { class: "text-xl", "{code}" },
            h2 { class: "text-lg", "{title}" },
            p { class: "text-justify", "{error}" }
        }
    )
}

pub enum Errors {
    CouldntParseCoinCode,
}
