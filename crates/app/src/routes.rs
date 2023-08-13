use crate::pages::posts::shared::{loading_post::LoadingPost, loading_posts::LoadingPosts};
use crate::pages::{
    about::index::AboutIndex,
    not_found::NotFound,
    posts::{detail::PostDetail, index::PostIndex},
    projects::index::Projects,
    shared::layout::Layout,
};
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
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

impl FromStr for Route {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(&format!("http://localhost:8080{}", s))?;
        let Some(path_segments) = url.path_segments() else {
            return Ok(Self::NotFound);
        };
        let path_segments = path_segments.collect::<Vec<_>>();
        if let Some(&"about") = path_segments.get(0) {
            return Ok(Self::AboutIndex {});
        }
        if let Some(&"projects") = path_segments.get(0) {
            return Ok(Self::Projects {});
        }
        if let (Some(&"pages"), Some(page)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::PostIndex {
                page: page.to_string().parse().unwrap_or(1),
            });
        }
        if let (Some(&"posts"), Some(slug)) = (path_segments.get(0), path_segments.get(1)) {
            return Ok(Self::PostDetail {
                slug: slug.to_string(),
            });
        }
        if let Some(&"") = path_segments.get(0) {
            return Ok(Self::PostIndex { page: 1 });
        }
        Ok(Self::NotFound)
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::PostIndex { page } => {
            let fallback = html! {<LoadingPosts />};
            html! {
                <Suspense {fallback}>
                    <PostIndex page={page.clone()} />
                </Suspense>
            }
        }
        Route::PostDetail { slug } => {
            let fallback = html! {<LoadingPost />};
            html! {
                <Suspense {fallback}>
                    <PostDetail slug={slug.clone()} />
                </Suspense>
            }
        }
        Route::Projects => {
            html! { <Projects /> }
        }
        Route::AboutIndex => {
            html! { <AboutIndex /> }
        }
        Route::NotFound => html! { <NotFound />},
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </Router>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}
