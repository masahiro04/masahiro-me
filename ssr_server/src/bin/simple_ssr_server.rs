use app::infrastructure::repositories::post_repository::PostFromApi;
use app::pages::about::index::about_meta_tags;
use app::pages::posts::detail::post_meta_tags;
use app::pages::posts::index::posts_meta_tags;
use app::pages::projects::index::projects_meta_tags;
use app::routes::{Route, ServerApp, ServerAppProps};
use reqwest::Client;
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::str::FromStr;

use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{header::CACHE_CONTROL, StatusCode, Uri};
use axum::response::{IntoResponse, Redirect, Response};

use axum::routing::get;
use axum::Router;
use clap::Parser;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use yew::platform::Runtime;

// We use jemalloc as it produces better performance.
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

pub async fn fetch_data_from_api(slug: &str) -> Result<PostFromApi, reqwest::Error> {
    let url = format!("https://api.masahiro.me/api/posts/{}", slug);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let body = response.json::<PostFromApi>().await?;
    Ok(body)
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let url = url.to_string();
    let (index_html_top, index_html_head) = index_html_before.split_once("<head>").unwrap();
    let mut index_html_top = index_html_top.to_owned();
    let route = Route::from_str(&url).unwrap();

    let meta = match route {
        Route::PostIndex { page } => posts_meta_tags(),
        Route::PostDetail { slug } => {
            log::debug!("Posts OGP Setting {}", slug);
            let meta_future = tokio::spawn(async move {
                let api_response = fetch_data_from_api(&slug).await;
                let post_from_api = match api_response {
                    Ok(body) => body,
                    Err(err) => panic!("error: {}", err),
                };

                let title = post_from_api.title;
                let description = post_from_api.excerpt;
                let featured_media = post_from_api.featured_media;
                let keywords = post_from_api
                    .categories
                    .iter()
                    .map(|category| category.name.clone())
                    .collect::<Vec<String>>()
                    .join(",");
                post_meta_tags(&title, &description, &keywords, &featured_media)
            });
            meta_future.await.unwrap_or_else(|_| "".to_string())
        }
        Route::Projects => projects_meta_tags(),
        Route::AboutIndex => about_meta_tags(),
        _ => "".to_string(),
    };

    if meta != "".to_string() {
        index_html_top.push_str(&meta);
    }

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    let index_html_before = format!("{}{}", index_html_top, index_html_head);

    let mut body = index_html_before;
    body.push_str(&renderer.render().await);
    body.push_str(&index_html_after);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CACHE_CONTROL, "public, max-age=86400, s-maxage=86400")
        .body(body)
        .unwrap();

    response
}
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
// pub fn health() -> impl IntoResponse {
async fn redirect_to_pages(_: ()) -> impl IntoResponse {
    Redirect::to("/pages/1")
}

async fn redirect_to_sitemap(_: ()) -> impl IntoResponse {
    Redirect::to("http://api.masahiro.me/sitemap")
}

#[tokio::main]
async fn main() {
    let exec = Executor::default();

    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    // NOTE: ここでheadにデータを入れることでSSRを実現できそう
    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();
    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {e}"),
        )
    };

    let app = Router::new()
        .route("/", get(redirect_to_pages))
        .route("/sitemap", get(redirect_to_sitemap))
        .fallback_service(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    get(render)
                        .with_state((index_html_before.clone(), index_html_after.clone()))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    println!("You can view the website at: http://0.0.0.0:8080/");

    let port = match std::env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => 8080,
    };

    let addr = ([0, 0, 0, 0], 8080).into();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
