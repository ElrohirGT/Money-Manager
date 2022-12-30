mod components;
mod models;

use dioxus::prelude::*;
use models::User;

use crate::components::{
    dashboard_page::DashboardPage, login_page::LoginPage, register_page::RegisterPage,
};

pub static USER: Atom<User> = |_| User::default();

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        Router {
            Route { to: "/", LoginPage {} },
            Route { to: "/register", RegisterPage {}},
            Route { to: "/dashboard", DashboardPage {} },
            Redirect { from: "", to: "/" }
        }

    ))
}
