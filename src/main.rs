#![allow(non_snake_case)]

use rust_dioxus::App;
fn main() {
    start();
}

#[cfg(feature="web")]
pub fn start(){
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(App);
}

#[cfg(feature="desktop")]
pub fn start(){
    tracing_subscriber::fmt::init();
    dioxus_desktop::launch(App);
}

