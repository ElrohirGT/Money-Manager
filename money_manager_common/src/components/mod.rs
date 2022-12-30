pub mod buttons;
pub mod dashboard_page;
pub mod debt_list;
pub mod inputs;
pub mod loadings;
pub mod login_page;
pub mod register_page;

use dioxus::prelude::*;

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
pub fn ProfitDisplayer(cx: Scope, profit: f64) -> Element {
    let (symbol, text_color) = if profit < &0. {
        ("", FAIL_TEXT_COLOR)
    } else {
        ("+", SUCCESS_TEXT_COLOR)
    };
    rsx!(cx, p {
        class: "{text_color} font-bold text-center",
        "{symbol}{profit:.2}"
    })
}
