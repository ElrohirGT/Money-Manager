use dioxus::prelude::*;

use crate::components::buttons::{MainButton, SecondaryLink};

#[allow(non_snake_case)]
pub fn LoginPage(cx: Scope) -> Element {
    let router = use_router(&cx);
    rsx!(cx, div {
        class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
        div {
            class: "flex flex-col shrink-0 p-10 items-center justify-center bg-slate-900 gap-y-2 text-white",
            h1 { "Iniciar Sesión" },
            input {
                class: "text-black",
                placeholder: "Usuario"
            },
            input {
                class: "text-black",
                placeholder: "Contraseña"
            }
            div {
                class: "flex flex-row gap-x-2",
                SecondaryLink {
                    to: "/register",
                    text: "Registrarse"
                }
                MainButton {
                    text: "Entrar"
                    onclick: move |_| {
                        log::info!("WIP login!");
                        // let state =
                        router.push_route("/dashboard", None, None);
                    }
                }
            }
        }
    })
}
