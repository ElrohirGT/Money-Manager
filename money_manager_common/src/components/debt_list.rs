use chrono::Locale;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon};

use crate::{
    components::buttons::PrimaryButton,
    models::{Debt, DebtType},
};

use super::{FAIL_TEXT_COLOR, SUCCESS_TEXT_COLOR};

#[inline_props]
#[allow(non_snake_case)]
pub fn DebtList<'a>(cx: Scope, debts: &'a Vec<Debt>, class: &'a str) -> Element {
    rsx!(cx, div {
        class: "{class}",
        div {
            class: "flex flex-col",
            debts.iter().map(|d| rsx!(DebtListItem { key: "{d.id}", debt: d }))
        }
    })
}

#[inline_props]
#[allow(non_snake_case)]
pub fn DebtListItem<'a>(cx: Scope, debt: &'a Debt) -> Element {
    let (symbol, text_color) = match debt.debt_type {
        DebtType::PersonalDebt => ("-", FAIL_TEXT_COLOR),
        DebtType::ContactDebt => ("+", SUCCESS_TEXT_COLOR),
    };

    let date = debt.date.format_localized("%e %B %Y, %T", Locale::es_GT);
    let (paid_effect, pay_button) = if debt.is_paid {
        ("text-opacity-25", rsx!(div {}))
    } else {
        (
            "",
            rsx!(PrimaryButton {
                class: "flex-col align-center"
                Icon {
                    width: 15,
                    height: 15,
                    fill: "white",
                    icon: FaCoins,
                }
            }),
        )
    };

    rsx!(cx,
        div {
            class: "flex inline-block odd:bg-slate-700 {paid_effect} text-white",
            p { class: "p-4 w-1/4", "{debt.contact.name}" },
            p { class: "p-4 w-1/2 text-center", "{date}"},
            p { class: "p-4 {text_color} text-right w-1/4", "{symbol}{debt.amount:.2}" },
            div {
                class: "bg-slate-900 w-1/12 flex",
                pay_button
            }
        }
    )
}
