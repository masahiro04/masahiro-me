mod app;
mod domain;
mod infrastructure;
mod pages;
mod presentation;
mod routes;
mod usecase;

use app::App;
use yew::start_app;

extern crate lazy_static;
extern crate wee_alloc;

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    start_app::<App>();
}
