mod components;

use dioxus::prelude::*;

use crate::components::{
    dashboard_page::DashboardPage, login_page::LoginPage, register_page::RegisterPage,
};

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
