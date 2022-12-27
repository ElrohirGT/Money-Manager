use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::*;

use dioxus_free_icons::Icon;

pub fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    let button_class =
        "inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded";

    cx.render(rsx!(
        div {
            class: "flex flex-col shrink-0 p-6 items-center justify-center",
            button {
                class: "{button_class}",
                onclick: move |_|{count += 1},
                "High Five!"
            },
            h1 { "High Five counter: {count}"},
            button {
                class: "{button_class}",
                onclick: move |_|{count -= 1},
                "",
                Icon {
                    width: 30,
                    height: 30,
                    fill: "white",
                    icon: FaMinus
                }},
        }
    ))
}
