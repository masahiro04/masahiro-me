use app::route;
use yew::{function_component, html, Html};

#[function_component(Hoge)]
pub fn hoge() -> Html {
    html! {
        <div>
            {"hoge"}
        </div>
    }
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    // TODO: yewのappをrenderするとwasm-validatorのエラーが出る
    yew::Renderer::<route::App>::new().hydrate();
    // yew::Renderer::<Hoge>::new().hydrate();
}
