use dioxus::prelude::*;

fn main() {
    pretty_env_logger::init_timed();
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    rsx!(cx, div {
        link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css"}
        money_manager_common::app{}
    })
}
