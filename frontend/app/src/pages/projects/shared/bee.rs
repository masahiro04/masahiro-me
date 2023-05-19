use crate::pages::shared::image::Image;
use yew::prelude::*;

#[function_component]
pub fn Bee() -> Html {
    html! {
        <Image class="drop-shadow-md text-shadow w-full" source="images/bee.svg" alt="" width=100 height=100 />
    }
}
