use dioxus::{core::UiEvent, events::FormData, prelude::*};

use crate::models::Coin;

#[allow(non_snake_case)]
#[inline_props]
pub fn PreferredCurrencySelector<'a>(
    cx: Scope,
    onvaluechanged: Option<EventHandler<'a, UiEvent<FormData>>>,
) -> Element {
    let selected_coin = use_state(&cx, || Coin::default());
    let items = vec![(Coin::Quetzal, "Quetzal"), (Coin::Dollar, "Dolar")]
        .iter()
        .map(|(c, d)| SelectItem {
            value: c.to_currency_code().to_string(),
            display: d,
            selected: c == selected_coin.get(),
            disabled: false,
        })
        .collect();
    rsx!(
        cx,
        HtmlSelect {
            placeholder: "-- Moneda Preferida --",
            options: items,
            onvaluechanged: move |ev: UiEvent<FormData>| {
                let coin: Coin = ev.value.clone().into();
                selected_coin.set(coin);
                if let Some(callback) = onvaluechanged {
                    callback.call(ev);
                }
            }
        }
    )
}

#[derive(Debug)]
pub struct SelectItem<'a> {
    pub value: String,
    pub display: &'a str,
    pub selected: bool,
    pub disabled: bool,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn HtmlSelect<'a>(
    cx: Scope,
    placeholder: Option<&'a str>,
    options: Vec<SelectItem<'a>>,
    onvaluechanged: Option<EventHandler<'a, UiEvent<FormData>>>,
) -> Element<'a> {
    let options = options.iter().map(|SelectItem { value, display, selected, disabled }| {
        match (selected, disabled) {
            (true, true) => rsx!(option { key: "{value}", value: "{value}", disabled: "true", selected: "true", "{display}" }),
            (false, false) => rsx!(option { key: "{value}", value: "{value}", "{display}" }),
            (true, _) => rsx!(option { key: "{value}", value: "{value}", selected: "true", "{display}" }),
            (_, true) => rsx!(option { key: "{value}", value: "{value}", disabled: "true", "{display}" })
        }
    });

    rsx!(cx, select {
        class: "p-1 bg-transparent border-blue-500 border-2 rounded",
        onchange: move |ev| {
            if let Some(callback) = onvaluechanged {
                callback.call(ev);
            }
        },
        placeholder.map(|placeholder| rsx!(option { value: "", disabled: "true", selected: "true", "{placeholder}" })),
        options
    })
}
