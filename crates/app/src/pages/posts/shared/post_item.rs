use super::categories::Categories;
use crate::domain::entities::post::Post;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostItemProps {
    pub post: Post,
}

#[function_component(PostItem)]
pub fn post_item(props: &PostItemProps) -> Html {
    html! {
        <div class="relative w-full group cursor-pointer">
            <Link<Route> to={Route::PostDetail { slug: props.post.slug().to_string() }}>
                <div class="py-3 bg-white rounded-md max-w-full bg-opacity-60 font-semibold text-gray-600 truncate shadow-sm duration-500 px-3 sm:px-6 text-sm sm:text-base group-hover:shadow-lg group-hover:scale-[1.01] group-hover:bg-opacity-90">
                    {props.post.title()}
                    <div class="flex overflow-x-auto text-gray-400 font-thin text-sm">
                        <time dateTime={props.post.date().to_string()} class="text-gray-500">
                            {props.post.date() }
                        </time>
                        <div class="ml-2 my-auto">
                            <Categories categories={props.post.categories().to_vec()} is_link={false} />
                        </div>
                    </div>
                </div>
            </Link<Route>>
        </div>
    }
}
