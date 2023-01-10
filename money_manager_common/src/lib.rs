mod components;
mod models;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::{Coin, User};

use crate::components::{
    dashboard_page::DashboardPage, debt_page::DebtPage, login_page::LoginPage,
    register_page::RegisterPage,
};

pub static USER: Atom<User> = |_| User::default();
pub static EXCHANGE_RATES: Atom<HashMap<Coin, f64>> = |_| HashMap::new();

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(Router {
        Route { to: "/", LoginPage {} },
        Route { to: "/register", RegisterPage {}},
        Route { to: "/dashboard", DashboardPage {} },
        Route { to: "/debt/:id", DebtPage {} }
        Redirect { from: "", to: "/" }
    }))
}
