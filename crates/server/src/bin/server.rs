// 参照よくなさそう
use app::pages::about::about_index::about_meta_tags;
use app::pages::posts::post_detail::post_meta_tags;
use app::pages::posts::post_index::posts_meta_tags;
use app::pages::projects::project_index::projects_meta_tags;
use app::route;
use infrastructure::repositories::post_repository::post_from_api::PostFromApi;
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
    #[clap(short, long, value_parser = clap::value_parser!(PathBuf))]
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
    let (index_html_top, index_html_head) = match index_html_before.split_once("<head>") {
        Some((top, head)) => (top.to_string(), head.to_string()),
        None => (index_html_before, "".to_string()),
    };
    let mut index_html_top = index_html_top.to_owned();
    let route = route::Route::from_str(&url).unwrap();

    let meta = match route {
        route::Route::PostIndex { page } => posts_meta_tags(),
        route::Route::PostDetail { slug } => {
            log::debug!("Posts OGP Setting {}", slug);
            let meta_future = tokio::spawn(async move {
                let api_response = fetch_data_from_api(&slug).await;
                let post_from_api = match api_response {
                    Ok(body) => body,
                    Err(err) => panic!("error: {}", err),
                };

                let post = post_from_api.into_post().unwrap();
                let keywords = post
                    .categories()
                    .into_iter()
                    .map(|category| category.name().to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                post_meta_tags(
                    post.title(),
                    post.excerpt(),
                    &keywords,
                    &post.featured_media(),
                )
            });
            meta_future.await.unwrap_or_else(|_| "".to_string())
        }
        route::Route::Projects => projects_meta_tags(),
        route::Route::AboutIndex => about_meta_tags(),
        _ => "".to_string(),
    };

    if meta != "".to_string() {
        index_html_top.push_str(&meta);
    }

    let renderer =
        yew::ServerRenderer::<route::ServerApp>::with_props(move || route::ServerAppProps {
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
    Executor::default();

    // env_logger::init();

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

    let addr = ([0, 0, 0, 0], port).into();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
