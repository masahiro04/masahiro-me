use super::pages::posts::post_components::{loading_post, loading_posts};
use super::pages::{
    about::about_index,
    components::layout,
    not_found,
    posts::{post_detail, post_index},
    projects::project_index,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Root,
    #[at("/pages/:page")]
    PostIndex { page: i32 },
    #[at("/posts/:slug")]
    PostDetail { slug: String },
    #[at("/projects")]
    Projects,
    #[at("/about")]
    AboutIndex,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(RootRedirect)]
fn root_redirect() -> Html {
    let navigator = use_navigator().unwrap();

    use_effect_with((), move |_| {
        navigator.replace(&Route::PostIndex { page: 1 });
        || ()
    });

    html! { <div></div> }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Root => {
            html! { <RootRedirect /> }
        }
        Route::PostIndex { page } => {
            let fallback = html! {<loading_posts::LoadingPosts />};
            html! {
                <Suspense {fallback}>
                    <post_index::PostIndex page={page} />
                </Suspense>
            }
        }
        Route::PostDetail { slug } => {
            let fallback = html! {<loading_post::LoadingPost />};
            html! {
                <Suspense {fallback}>
                    <post_detail::PostDetail slug={slug} />
                </Suspense>
            }
        }
        Route::Projects => {
            html! { <project_index::ProjectIndex /> }
        }
        Route::AboutIndex => {
            html! { <about_index::AboutIndex /> }
        }
        Route::NotFound => html! { <not_found::NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <layout::Layout>
                <Switch<Route> render={switch} />
            </layout::Layout>
        </BrowserRouter>
    }
}
