use super::pages::posts::post_components::{loading_post, loading_posts};
use super::pages::{
    about::about_index,
    components::layout,
    not_found,
    posts::{post_detail, post_index},
    projects::project_index,
};
use std::str::FromStr;
use url::Url;
use yew::prelude::*;
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
        let first = path_segments.first();
        if let Some(&"about") = first {
            return Ok(Self::AboutIndex {});
        }
        if let Some(&"projects") = first {
            return Ok(Self::Projects {});
        }
        if let (Some(&"pages"), Some(page)) = (first, path_segments.get(1)) {
            return Ok(Self::PostIndex {
                page: page.to_string().parse().unwrap_or(1),
            });
        }
        if let (Some(&"posts"), Some(slug)) = (first, path_segments.get(1)) {
            return Ok(Self::PostDetail {
                slug: slug.to_string(),
            });
        }
        if let Some(&"") = first {
            return Ok(Self::PostIndex { page: 1 });
        }
        Ok(Self::NotFound)
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
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
