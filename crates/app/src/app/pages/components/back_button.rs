use crate::pages::components::image::Image;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(BackButton)]
pub fn back_button() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.back());

    html! {
        <div
            onclick={onclick}
            class="relative cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 flex items-center w-24 justify-center text-gray-600 text-sm -translate-y-1 hover:shadow-md sm:-translate-y-2"
        >
            <span class="mr-1">{"Back"}</span>
            <Image class="w-4 h-4 sm:w-5 sm:h-5" source="back_arrow.svg" alt="back arrow icon" />
        </div>
    }
}
