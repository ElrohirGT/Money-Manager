use dioxus::prelude::*;

use crate::components::buttons::{MainButton, SecondaryLink};

#[allow(non_snake_case)]
pub fn RegisterPage(cx: Scope) -> Element {
    rsx!(cx, div {
        class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
        div {
            class: "flex flex-col shrink-0 p-10 items-center justify-center bg-slate-900 gap-y-2 text-white",
            h1 { "Registrarse" },
            input {
                class: "text-black",
                placeholder: "Usuario"
            },
            input {
                class: "text-black",
                placeholder: "Correo"
            },
            input {
                class: "text-black",
                placeholder: "Contraseña"
            },
            select {
                class: "text-black p-2",
                // placeholder: "--Moneda Preferida--",
                option { value: "", disabled: "true", selected: "true", "--Moneda Preferida--" }
                option { value: "GTQ", "Quetzal" },
                option { value: "USD", "US Dollar" }
            }
            div {
                class: "flex flex-row gap-x-2",
                SecondaryLink {
                    text: "Iniciar sesión",
                    to: "/"
                }
                MainButton {
                    text: "Registrarse"
                }
            }
        }
    })
}
