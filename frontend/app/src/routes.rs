use crate::pages::pages::Pages;
use crate::pages::{about::About, not_found::NotFound, post::PostDetail, projects::Projects};
use crate::presentation::layout::BaseLayout;
use yew::prelude::*;
use yew_router::prelude::*;
//
#[derive(Clone, Routable, PartialEq)]
pub enum RootRoutes {
    #[at("/pages/:page")]
    Pages { page: i32 },
    #[at("/posts/:slug")]
    PostDetail { slug: String },
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}
//
fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::Pages { page } => html! {<Pages page={page.clone()} />},
        RootRoutes::PostDetail { slug } => html! {<PostDetail slug={slug.clone()} />},
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::About => html! { <About /> },
        RootRoutes::NotFound => html! { <NotFound />},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
                <>{"aaa"}</>
        </BrowserRouter>
    }
}
