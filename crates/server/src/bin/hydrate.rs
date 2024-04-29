use app::route;
use yew::{function_component, html, Html};

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // TODO: yewのappをrenderするとwasm-validatorのエラーが出る
    yew::Renderer::<route::App>::new().hydrate();
}
