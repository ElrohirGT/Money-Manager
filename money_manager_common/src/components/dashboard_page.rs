use crate::{
    components::{
        buttons::{MainButton, SecondaryButton},
        debt_list::DebtList,
        loadings::TextLoading,
        ComparisonBar, ProfitDisplayer,
    },
    models::{Contact, DashboardData, Debt, DebtType},
    USER,
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng, Rng};

use super::{ComparisonBarItem, FAIL_BACKGROUND_COLOR, SUCCESS_BACKGROUND_COLOR};

#[allow(non_snake_case)]
pub fn DashboardPage(cx: Scope) -> Element {
    let user = use_read(&cx, USER);
    if user.username.is_empty() {
        return rsx!(cx, Redirect { to: "/" });
    }

    let request = use_future(&cx, user, |u| async move { get_dashboard_data(u.id).await });

    let components = match request.value() {
        Some(Ok(DashboardData {
            personal_debt,
            contact_debt,
            history,
        })) => {
            let total = personal_debt + contact_debt;
            let profit = contact_debt - personal_debt;

            let personal_percentage = (personal_debt / total) as f32;
            let contact_percentage = (contact_debt / total) as f32;

            let bar_items = vec![
                ComparisonBarItem {
                    color: SUCCESS_BACKGROUND_COLOR,
                    percentage: contact_percentage,
                },
                ComparisonBarItem {
                    color: FAIL_BACKGROUND_COLOR,
                    percentage: personal_percentage,
                },
            ];

            rsx!(
                h1 { "{user.username}" },
                h2 { "{user.email}" },
                ComparisonBar { items: bar_items },
                ProfitDisplayer { profit: profit }
                MainButton {
                    Icon {
                        width: 30,
                        height: 30,
                        fill: "white",
                        icon: FaMagnifyingGlass
                    }
                },
                SecondaryButton {
                    Icon {
                        width: 30,
                        height: 30,
                        fill: "white",
                        icon: FaPlus
                    }
                },
                DebtList {
                    class: "overflow-y-auto max-h-96",
                    debts: &history
                }
            )
        }
        Some(Err(err)) => rsx!(h1{ "An error ocurrred\n{err}"}),
        None => rsx!(TextLoading {}),
    };

    rsx!(
        cx,
        div {
            class: "flex items-center align-center justify-center h-screen w-screen bg-slate-800",
            div {
                class: "flex flex-col shrink-0 p-20 bg-slate-900 gap-y-4 text-white w-1/2 h-9/12",
                components
            }
        }
    )
}

async fn get_dashboard_data(user_id: u32) -> Result<DashboardData, std::io::Error> {
    // let api_key = "730722fe";
    //TODO Actually call the app API instead of this dummy data
    let _ = reqwest::get("http://www.omdbapi.com/?i=tt3896198&apikey=730722fe").await;

    let mut rng = thread_rng();
    let contacts: Vec<Contact> = (0..12).map(|_| rng.gen()).collect();

    let range = Uniform::new(1, 5);
    let history: Vec<Debt> = contacts
        .iter()
        .map(|c| {
            (0..range.sample(&mut rng))
                .map(|_| {
                    let mut debt: Debt = rng.gen();
                    debt.contact = c.clone();
                    debt
                })
                .collect::<Vec<Debt>>()
        })
        .flatten()
        .collect();

    Ok(DashboardData {
        personal_debt: history
            .iter()
            .filter(|d| {
                if let DebtType::PersonalDebt = d.debt_type {
                    true
                } else {
                    false
                }
            })
            .map(|d| d.amount)
            .sum(),
        contact_debt: history
            .iter()
            .filter(|d| {
                if let DebtType::ContactDebt = d.debt_type {
                    true
                } else {
                    false
                }
            })
            .map(|d| d.amount)
            .sum(),
        history,
    })
}
