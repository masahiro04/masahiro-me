use crate::pages::components::image;
use yew::prelude::*;

#[function_component(Bee)]
pub fn bee() -> Html {
    html! {
        <image::Image class="drop-shadow-md text-shadow w-full" source="bee.svg" alt="" width=100 height=100 />
    }
}
