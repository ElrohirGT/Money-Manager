use money_manager_common::app;
fn main() {
    let log_level = log::Level::Debug;
    wasm_logger::init(wasm_logger::Config::new(log_level));
    dioxus::web::launch(app);
}
