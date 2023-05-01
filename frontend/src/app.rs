use crate::routes::RouteOutlet;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <RouteOutlet />
    }
}
