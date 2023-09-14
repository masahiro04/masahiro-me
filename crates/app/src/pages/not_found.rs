use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <img src="https://masahiro.me/page_not_found.png" />
        </div>
    }
}
