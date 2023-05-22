use crate::pages::{
    about::index::AboutIndex,
    not_found::NotFound,
    posts::{detail::PostDetail, index::PostIndex},
    projects::index::Projects,
    shared::layout::Layout,
};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
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

pub fn switch(routes: RootRoutes) -> Html {
    match routes {
        RootRoutes::PostIndex { page } => html! {<PostIndex page={page.clone()} />},
        RootRoutes::PostDetail { slug } => html! {<PostDetail slug={slug.clone()} />},
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::AboutIndex => html! { <AboutIndex /> },
        RootRoutes::NotFound => html! { <NotFound />},
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<RootRoutes> render={switch} />
        </Router>
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
