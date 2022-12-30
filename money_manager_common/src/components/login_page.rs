use dioxus::prelude::*;

use crate::{
    components::buttons::{MainButton, SecondaryLink},
    models::User,
    USER,
};

#[allow(non_snake_case)]
pub fn LoginPage(cx: Scope) -> Element {
    let router = use_router(&cx);
    let set_user = use_set(&cx, USER);

    let username = use_state(&cx, || "".to_string());

    rsx!(cx, div {
        class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
        div {
            class: "flex flex-col shrink-0 p-10 items-center justify-center bg-slate-900 gap-y-2 text-white",
            h1 { "Iniciar Sesión" },
            input {
                class: "text-black",
                placeholder: "Usuario",
                oninput: move |ev| username.set(ev.value.clone())
            },
            input {
                class: "text-black",
                placeholder: "Contraseña",
                r#type: "password"
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
                        //TODO Implement login
                        log::info!("WIP login!");
                        set_user(User {
                            id: 1534,
                            username: username.to_string(),
                            email: String::from("elrohirgt@gmail.com")
                        });
                        router.push_route("/dashboard", None, None);
                    }
                }
            }
        }
    })
}
