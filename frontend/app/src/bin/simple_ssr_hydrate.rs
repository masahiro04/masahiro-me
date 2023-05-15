// use simple_ssr::App;
use app::App;

use yew::Renderer;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().hydrate();
}
