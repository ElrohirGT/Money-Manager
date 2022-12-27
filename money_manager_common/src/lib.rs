use dioxus::events::MouseEvent;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::*;

use dioxus_free_icons::Icon;

pub fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            class: "flex flex-col shrink-0 p-6 items-center justify-center",
            MainButton {
                onclick: move |_| { count +=1 },
                Icon {
                    width: 30,
                    height: 30,
                    fill: "white",
                    icon: FaPlus
                }
            }
            h1 { "High Five counter: {count}"},
            MainButton {
                onclick: move |_| { count-=1 },
                    Icon {
                    width: 30,
                    height: 30,
                    fill: "white",
                    icon: FaMinus
                }
            }
        }
    ))
}

#[allow(non_snake_case)]
#[inline_props]
pub fn MainButton<'a>(
    cx: Scope<'a>,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
    text: Option<String>,
) -> Element {
    let button_class =
        "inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded";

    let text = match &text {
        Some(v) => v,
        None => "",
    };

    rsx!(cx, button {
        class: "{button_class}",
        onclick: move |ev| { onclick.call(ev) },
        "{text}",
        children
    })
}
