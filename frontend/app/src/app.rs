use crate::routes::RouteOutlet;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <Suspense {fallback}>
            <RouteOutlet />
        </Suspense>
    }
}
