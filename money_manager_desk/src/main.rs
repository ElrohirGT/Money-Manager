use money_manager_common::app;

fn main() {
    pretty_env_logger::init_timed();
    dioxus::desktop::launch(app);
}
