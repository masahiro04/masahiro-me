use crate::pages::shared::image::Image;
use yew::prelude::*;

#[function_component(Bee)]
pub fn bee() -> Html {
    html! {
        <Image class="drop-shadow-md text-shadow w-full" source="bee.svg" alt="" width=100 height=100 />
    }
}
