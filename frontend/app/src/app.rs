use crate::routes::RouteOutlet;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <RouteOutlet />
    }
}
