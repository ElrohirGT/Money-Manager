use dioxus::{
    core::{to_owned, UiEvent},
    events::FormData,
    prelude::{dioxus_elements::textarea, *},
};
use dioxus_free_icons::icons::fa_solid_icons::*;
use dioxus_free_icons::Icon;
use rand::{thread_rng, Rng};

use crate::{
    components::{
        buttons::{PrimaryButton, SecondaryLink},
        errors::ErrorPage,
        inputs::{HtmlSelect, PreferredCurrencySelector, SelectItem},
        loadings::LoadingPage,
        ProfitDisplayer,
    },
    models::{Coin, Contact, Debt, DebtType, MoneyAmount, User},
    USER,
};

#[allow(non_snake_case)]
pub fn DebtPage(cx: Scope) -> Element {
    let page = use_route(&cx);
    let user = use_read(&cx, USER);

    let contact = use_state(&cx, || Contact::default());
    let amount = use_state(&cx, || MoneyAmount::default());
    let debt_type = use_state(&cx, || DebtType::default());
    let date = use_state(&cx, || chrono::Local::now());
    let is_paid = use_state(&cx, || false);
    let description = use_state(&cx, || String::new());

    let id: Option<u32> = page.query();

    let debt_request = use_future(&cx, &id, {
        to_owned![contact, amount, debt_type, date, is_paid, description];
        |id| async move {
            if let Some(id) = id {
                let resp = get_debt(id).await;
                Some(resp.map(|v| {
                    contact.set(v.contact);
                    amount.set(v.amount);
                    debt_type.set(v.debt_type);
                    date.set(v.date);
                    is_paid.set(v.is_paid);
                    description.set(v.description.clone());
                }))
            } else {
                None
            }
        }
    });
    let contact_request = use_future(&cx, user, |user| async move { get_contacts(&user).await });

    let (_, contacts) = match (debt_request.value(), contact_request.value()) {
        // The API call is loading
        (None, None) | (_, None) | (None, _) => return rsx!(cx, LoadingPage {}),
        // An error ocurred during the call to the API
        (Some(Some(Err(err))), _) | (_, Some(Err(err))) => {
            return rsx!(
                cx,
                ErrorPage {
                    code: "502",
                    title: "Internal Server Error",
                    error: "{err}"
                }
            )
        }
        // It got a valid value from the API
        (Some(Some(Ok(d))), Some(Ok(c))) => (d, c),
        // The id was not a valid one so no value was returned
        (Some(None), Some(Ok(c))) => (&(), c),
    };

    let contact_options = contacts
        .iter()
        .map(|c| SelectItem {
            value: c.id.to_string(),
            display: &c.name,
            selected: c.id == contact.id,
            disabled: false,
        })
        .collect();
    log::debug!("Created contact options {:#?}", contact_options);

    rsx!(
        cx,
        div {
            class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
            div {
                class: "flex flex-col shrink-0 p-20 bg-slate-900 gap-y-4 text-white w-1/2 h-9/12",
                ProfitDisplayer {
                    class: "text-2xl"
                    editable: true,
                    profit: amount.get().clone(),
                    onvaluechanged: move |value: String| {
                        log::debug!("The value on the input is: {}", value);
                        let money_amount = match value.parse::<f64>() {
                            Ok(v) => MoneyAmount {
                                coin: amount.coin.clone(),
                                amount: v
                            },
                            Err(_) => MoneyAmount {
                                coin: amount.coin.clone(),
                                amount: 0f64
                            },
                        };

                        amount.set(money_amount);
                    }
                },
                HtmlSelect {
                    placeholder: "-- Contacto --",
                    options: contact_options,
                    onvaluechanged: move |ev: UiEvent<FormData>|
                    {
                        let value = ev.value.clone();
                        if let Some(selected_contact) = contacts.iter().find(|c| c.id.to_string() == value) {
                            log::debug!("The selected contact is {:?}", selected_contact);
                            contact.set(selected_contact.clone());
                        }
                    }
                },
                PreferredCurrencySelector {
                    onvaluechanged: move |ev: UiEvent<FormData>| {
                        let value = ev.value.clone();
                        let coin = Coin::from(value);
                        let money_amount = MoneyAmount {
                            coin,
                            amount: amount.amount
                        };

                        amount.set(money_amount);
                    }
                },
                textarea {
                    class: "bg-transparent border-blue-500 border-2 p-1",
                    placeholder: "Descripción",
                    maxlength: "60",
                    value: "{description}",
                }
                div {
                    class: "flex gap-x-2",
                    SecondaryLink {
                        class: "flex-grow",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: FaChevronLeft
                        },
                        to: "/dashboard"
                    },
                    PrimaryButton {
                        class: "flex-grow"
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: FaFloppyDisk
                        }
                    }
                }
            }
        }
    )
}

pub async fn get_debt(id: u32) -> Result<Debt, std::io::Error> {
    let mut rng = thread_rng();
    Ok(rng.gen())
}

pub async fn get_contacts(user: &User) -> Result<Vec<Contact>, std::io::Error> {
    let mut rng = thread_rng();
    Ok((0..5).map(|_| rng.gen()).collect())
}
