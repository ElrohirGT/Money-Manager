use dioxus::events::MouseEvent;
use dioxus::prelude::*;

static MAIN_BUTTON_CLASS: &str =
    "inline-block bg-blue-500 text-white font-bold py-2 px-4 rounded transition hover:bg-blue-700";
static SECONDARY_BUTTON_CLASS: &str = "inline-block bg-transparent border-transparent border-2 text-blue-500 font-bold py-2 px-4 rounded transition hover:border-blue-500";

#[allow(non_snake_case)]
#[inline_props]
pub fn MainLink<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    text: Option<&'a str>,
    to: &'a str,
) -> Element {
    rsx!(
        cx,
        Button {
            class: "{MAIN_BUTTON_CLASS}",
            text: text.unwrap_or(""),
            to: "{to}",
            children
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn SecondaryLink<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    text: Option<&'a str>,
    to: &'a str,
) -> Element {
    rsx!(
        cx,
        Button {
            class: "{SECONDARY_BUTTON_CLASS}",
            text: text.unwrap_or(""),
            to: "{to}",
            children
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn MainButton<'a>(
    cx: Scope<'a>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
    text: Option<&'a str>,
) -> Element {
    rsx!(
        cx,
        Button {
            class: "{MAIN_BUTTON_CLASS}",
            onclick: move |ev| {
                if let Some(callback) = onclick {
                    callback.call(ev)
                }
            },
            text: text.unwrap_or(""),
            children
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn SecondaryButton<'a>(
    cx: Scope<'a>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
    text: Option<&'a str>,
) -> Element {
    rsx!(
        cx,
        Button {
            class: "{SECONDARY_BUTTON_CLASS}",
            onclick: move |ev| {
                if let Some(callback) = onclick {
                    callback.call(ev)
                }
            },
            text: text.unwrap_or(""),
            children
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn Button<'a>(
    cx: Scope<'a>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
    text: &'a str,
    to: Option<&'a str>,
    class: &'a str,
) -> Element {
    match to {
        Some(route) => rsx!(
            cx,
            Link {
                class: "{class}",
                to: route,
                "{text}",
                children,
            }
        ),
        None => rsx!(cx, button {
            class: "{class}",
            onclick: move |ev| {
                if let Some(callback) = onclick {
                    callback.call(ev)
                }
            },
            "{text}",
            children
        }),
    }
}
