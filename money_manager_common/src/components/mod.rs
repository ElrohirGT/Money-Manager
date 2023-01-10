pub mod buttons;
pub mod dashboard_page;
pub mod debt_list;
pub mod debt_page;
pub mod errors;
pub mod inputs;
pub mod loadings;
pub mod login_page;
pub mod register_page;

use dioxus::{core::UiEvent, events::FormData, prelude::*};

use crate::models::MoneyAmount;

#[derive(PartialEq)]
pub struct ComparisonBarItem<'a> {
    pub color: &'a str,
    pub percentage: f32,
}

#[inline_props]
#[allow(non_snake_case)]
pub fn ComparisonBar<'a>(cx: Scope, items: Vec<ComparisonBarItem<'a>>) -> Element {
    rsx!(cx, div {
        class: "flex items-center grow",
        items.iter().map(|ComparisonBarItem{color, percentage}| {
            let percentage = percentage * 100.;
            rsx!(div {
                key: "{color}",
                class: "{color} h-4",
                style: "width: {percentage:.2}%"
            })
        })
    })
}

pub static SUCCESS_TEXT_COLOR: &str = "text-emerald-500";
pub static FAIL_TEXT_COLOR: &str = "text-red-500";

pub static SUCCESS_BACKGROUND_COLOR: &str = "bg-emerald-500";
pub static FAIL_BACKGROUND_COLOR: &str = "bg-red-500";

#[inline_props]
#[allow(non_snake_case)]
pub fn ProfitDisplayer<'a>(
    cx: Scope,
    profit: MoneyAmount,
    editable: bool,
    class: Option<&'a str>,
    onvaluechanged: Option<EventHandler<'a, String>>,
) -> Element<'a> {
    let class = class.unwrap_or("");
    let editing = use_state(&cx, || false);
    let text_value = use_state(&cx, || profit.amount.to_string());

    let (symbol, text_color) = if profit.amount < 0. {
        ("-", FAIL_TEXT_COLOR)
    } else {
        ("+", SUCCESS_TEXT_COLOR)
    };

    let class = format!(
        "{} {} font-bold text-center bg-transparent",
        class, text_color
    );

    if *editing.get() {
        rsx!(
            cx,
            input {
                class: "{class}",
                value: "{text_value}",
                oninput: move |ev| {
                    let new_amount = ev.value.clone();
                    text_value.set(new_amount);
                },
                onfocusout: move |_| {
                    editing.set(false);
                    if let Some(callback) = onvaluechanged {
                        callback.call(text_value.get().clone());
                    }
                }
            },
        )
    } else {
        rsx!(
            cx,
            p {
                class: "{class}",
                onclick: move |_| editing.set(true && *editable),
                "{symbol}{profit}",
            }
        )
    }
}
