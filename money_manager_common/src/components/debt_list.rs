use dioxus::prelude::*;

use crate::models::{Debt, DebtType};

use super::{FAIL_TEXT_COLOR, SUCCESS_TEXT_COLOR};

#[inline_props]
#[allow(non_snake_case)]
pub fn DebtList<'a>(cx: Scope, debts: &'a Vec<Debt>, class: &'a str) -> Element {
    rsx!(cx, div {
        class: "{class}",
        table {
            class: "w-full",
            caption {class: "p-2 text-xl font-bold", "Historial"}
            tr {
                th { class: "p-2 text-lg", "Contacto" }
                th { class: "p-2 text-lg", "Cantidad" }
            }
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

    rsx!(cx, tr {
        class: "even:bg-slate-700",
        td { class: "p-4", "{debt.contact.name}" },
        td { class: "p-4 {text_color} text-right", "{symbol}{debt.amount:.2}" }
    })
}
