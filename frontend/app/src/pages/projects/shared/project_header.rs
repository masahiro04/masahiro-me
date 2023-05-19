use crate::pages::{projects::shared::bee::Bee, shared::image::Image};
use yew::prelude::*;

#[function_component]
pub fn ProjectHeader() -> Html {
    html! {
      <div class="max-w-xl mx-auto">
        <a class="w-full group" href="https://github.com/masahiro04" target="_blank" rel="noreferrer">
          <div class="relative">
            <Image class="w-full" source="images/honeycomb.svg" alt="" width=100 height=100 />
            <div class="absolute -scale-x-100 top-[35%] w-[9%] group-hover:duration-1000 group-hover:translate-x-[280%] group-hover:-translate-y-1 group-hover:delay-100">
              <Bee />
            </div>
            <div class="absolute -scale-x-100 bottom-[13%] left-[25%] w-[7%] group-hover:duration-700 group-hover:translate-x-[100%] group-hover:-translate-y-1">
              <Bee />
            </div>
            <div class="absolute rotate-12 -scale-x-100 top-[15%] left-[15%] w-[6.5%] group-hover:duration-700 group-hover:translate-x-[250%] group-hover:-translate-y-1">
              <Bee />
            </div>
            <div class="absolute -rotate-24 bottom-[20%] right-[18%] w-[6%] group-hover:duration-1000 group-hover:-translate-x-[230%] group-hover:-translate-y-1 group-hover:delay-50">
              <Bee />
            </div>
            <div class="absolute top-[20%] right-[8%] w-[9%] group-hover:duration-1000 group-hover:-translate-x-[230%] group-hover:-translate-y-1 group-hover:delay-100">
              <Bee />
            </div>
            <div class="absolute w-full h-full flex justify-center top-0">
              <div class="drop-shadow-lg flex-none self-center text-gray-800 w-[70%] h-[70%]">
                <Image class="w-auto mx-auto" source="images/github.svg" alt=""  />
              </div>
            </div>
          </div>
        </a>
      </div>
    }
}
