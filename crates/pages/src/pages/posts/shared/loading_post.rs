use yew::prelude::*;
#[function_component(LoadingPost)]
pub fn loading_post() -> Html {
    html! {
        <>
            <div class="relative cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 flex items-center w-24 justify-center text-gray-600 text-sm -translate-y-1 hover:shadow-md sm:-translate-y-2">
                <div class="h-4 w-20 sm:w-5 sm:h-5 bg-gray-300 animate-pulse" />
            </div>
            <div class="relative overflow-hidden py-8 bg-white bg-opacity-50 rounded-md shadow-md sm:py-16">
                <div class="relative px-4 sm:px-6 lg:px-8">
                    <div class="mx-auto max-w-prose">
                        <div class="mx-auto w-100 text-center mb-4">
                            <div class="h-8 w-100 bg-gray-300 animate-pulse text-center mb-4" />
                        </div>
                        {for (0..4).map(|_| { html! {
                            <>
                                <div class="h-4 w-1/2 bg-gray-300 animate-pulse mb-4"></div>
                                <div class="h-4 w-3/4 bg-gray-300 animate-pulse mb-4"></div>
                                <div class="h-4 w-3/4 bg-gray-300 animate-pulse mb-4"></div>
                            </>
                        } })}
                    </div>
                </div>
            </div>
        </>
    }
}
