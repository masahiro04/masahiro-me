use crate::pages::{
    about::index::AboutIndex,
    not_found::NotFound,
    posts::{detail::PostDetail, index::PostIndex},
    projects::index::Projects,
    shared::layout::Layout,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum RootRoutes {
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

fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::PostIndex { page } => html! {<PostIndex page={page.clone()} />},
        RootRoutes::PostDetail { slug } => html! {<PostDetail slug={slug.clone()} />},
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::AboutIndex => html! { <AboutIndex /> },
        RootRoutes::NotFound => html! { <NotFound />},
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<RootRoutes> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}
