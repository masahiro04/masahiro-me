use crate::presentation::link::Link;
use crate::routes::RootRoutes;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PaginationProps {
    pub is_loading: bool,
    pub has_next_page: bool,
    pub current_page: i32,
}

#[function_component(Loading)]
fn loading() -> Html {
    html! {
        <div class="flex items-center justify-center space-x-4 mt-8">
            <div class="cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 w-24 text-gray-600 text-sm sm:text-base sm:px-4 sm:w-32 hover:shadow-md">
                <span class="w-5 h-5">
                    <div class="bg-gray-300 animate-pulse h-4 w-16 m-auto" />
                </span>
            </div>
        </div>
    }
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    if props.is_loading {
        return html! { <Loading /> };
    }

    html! {
    <div class="flex items-center justify-center space-x-4 mt-8">
      { if props.current_page != 1 {
            html! {
                <Link href={RootRoutes::Pages{ page: props.current_page - 1 } }>
                  <div class="relative cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 flex items-center w-24 justify-center text-gray-600 text-sm sm:text-base sm:px-4 sm:w-32 hover:shadow-md">
                    <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path
                        fillRule="evenodd"
                        d="M7.28 7.72a.75.75 0 010 1.06l-2.47 2.47H21a.75.75 0 010 1.5H4.81l2.47 2.47a.75.75 0 11-1.06 1.06l-3.75-3.75a.75.75 0 010-1.06l3.75-3.75a.75.75 0 011.06 0z"
                        clipPath="evenodd"
                      ></path>
                    </svg>
                    <span class="ml-1 flex">
                      <span class="hidden sm:block">
                        {format!("Page {}", props.current_page - 1)}
                      </span>
                      <span class="whitespace-nowrap sm:hidden">
                        {format!("P. {}", props.current_page - 1)}
                       </span>
                    </span>
                  </div>
                </Link>
            }
      } else { html!{} }}

      { if props.has_next_page  {
            html!{
                <Link href={RootRoutes::Pages{ page: props.current_page + 1 } }>
                  <div class="relative cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 flex items-center w-24 justify-center text-gray-600 text-sm sm:text-base sm:px-4 sm:w-32 hover:shadow-md">
                    <span class="mr-1">{"Next"}</span>
                    <svg class="w-5 h-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                      <path
                        fillRule="evenodd"
                        d="M16.72 7.72a.75.75 0 011.06 0l3.75 3.75a.75.75 0 010 1.06l-3.75 3.75a.75.75 0 11-1.06-1.06l2.47-2.47H3a.75.75 0 010-1.5h16.19l-2.47-2.47a.75.75 0 010-1.06z"
                        clipPath="evenodd"
                      ></path>
                    </svg>
                  </div>
                </Link>
            }
        } else { html!{} }}
    </div>

    }
}
