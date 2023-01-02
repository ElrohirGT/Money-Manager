use std::collections::HashMap;

use dioxus::{core::to_owned, prelude::*};
use serde::Deserialize;

use crate::{
    components::buttons::{MainButton, SecondaryLink},
    models::{Coin, User},
    EXCHANGE_RATES, USER,
};

#[allow(non_snake_case)]
pub fn LoginPage(cx: Scope) -> Element {
    let router = use_router(&cx);
    let set_user = use_set(&cx, USER);
    let set_exchanges = use_set(&cx, EXCHANGE_RATES);

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
                        cx.spawn({
                            to_owned![router, set_user, username, set_exchanges];
                            async move {
                                //TODO Implement login
                                log::info!("WIP login!");
                                let client = reqwest::Client::new();

                                log::debug!("Retrieving user info...");

                                let user = User {
                                    id: 1534,
                                    username: username.to_string(),
                                    email: String::from("elrohirgt@gmail.com"),
                                    preferred_coin: Coin::Quetzal
                                };
                                log::debug!("User info retrieved!");

                                let api_call = format!("https://api.apilayer.com/exchangerates_data/latest?symbols=USD%2CGTQ&base={}", user.preferred_coin.to_currency_code());

                                log::debug!("Retrieving exchange rates...");
                                let api_response = client.get(api_call)
                                    .header("apikey", "wqg2plg3sgDwAVWoc26sQsW9ywaA4crl")
                                    .send()
                                    .await
                                    .expect("Couldn't retrieve the exchange rates!");
                                log::debug!("Exchange rates received!");

                                log::debug!("Converting to ApiResponse object...");
                                let api_response = api_response.json::<ApiResponse>()
                                    .await
                                    .expect("API Response is not correct format!");
                                log::debug!("Conversion successfull!");

                                log::debug!("Converting api response codes into Coin enum...");
                                let rates = api_response.rates.into_iter().map(|(coin, rate)| {
                                    let coin = match &*coin {
                                        "USD" => Coin::Dollar,
                                        "GTQ" => Coin::Quetzal,
                                        _ => unreachable!()
                                    };
                                    (coin, rate)
                                }).collect();
                                log::debug!("Done converting!");
                                log::debug!("{:#?}", rates);

                                set_exchanges(rates);

                                set_user(user);
                                router.push_route("/dashboard", None, None);
                            }
                        })
                    }
                }
            }
        }
    })
}

#[derive(Deserialize)]
struct ApiResponse {
    pub rates: HashMap<String, f64>,
}
