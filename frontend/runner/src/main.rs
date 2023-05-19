use yew::{self, function_component, html, Html};

use app::routes::App;

extern crate lazy_static;
extern crate wee_alloc;

#[macro_export]
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[function_component(Appp)]
pub fn app() -> Html {
    html! {
    <div>{ "aaa" }</div>
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // yew::Renderer::<Appp>::new().hydrate();
    yew::Renderer::<App>::new().render();
}

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// // #[tokio::main(flavor = "current_thread")]
// #[async_std::main]
// async fn main() {
//     console_log!("sentinel1");
//     wasm_logger::init(wasm_logger::Config::default());
//     console_log!("sentinel2");
//     let renderer = yew::ServerRenderer::<App>::new();
//     console_log!("sentinel3");
//
//     let rendered = renderer.render().await;
//     console_log!("sentinel4");
//
//     // Prints: <div>Hello, World!</div>
//     // println!("{}", rendered);
//     console_log!("{}", rendered);
// }
