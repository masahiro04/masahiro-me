use app::pages::about::index::about_meta_tags;
use app::pages::posts::detail::post_meta_tags;
// use app::pages::posts::detail::post_meta_tags;
use app::pages::projects::index::projects_meta_tags;
use app::routes::{Route, ServerApp, ServerAppProps};
// use ssr_server::metadata::{insert_metadata, MetadataParams};
use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::str::FromStr;

use axum::body::StreamBody;
use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::Router;
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::server::Server;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use yew::platform::Runtime;

// We use jemalloc as it produces better performance.
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
// ) -> impl IntoResponse {
) -> impl IntoResponse {


// ) -> impl Future<Output = impl IntoResponse> {
// ) -> impl futures::Future<Output = impl IntoResponse> {
    let url = url.to_string();
    println!("url: {}", url);

    let (index_html_top, index_html_head) = index_html_before.split_once("<head>").unwrap();
    let mut index_html_top = index_html_top.to_owned();
    // let meta_tags = match Route::
    let route = Route::from_str(&url).unwrap();
    print!("route: {:?}", route);

    // let mut meta = "".to_string();

    let meta = match route {
        Route::PostIndex { page } => {
            log::debug!("Posts OGP Setting {}", page);
            about_meta_tags()
        }
        Route::PostDetail { slug } => {
            log::debug!("Posts OGP Setting {}", slug);

            // about_meta_tags()
            // post_meta_tags(slug).await
        }
        Route::Projects => projects_meta_tags(),
        Route::AboutIndex => about_meta_tags(),
        // Ok(Route::Preview { id }) => {
        //     log::debug!("Preview OGP Setting {}", id);
        //     about_meta_tags().unwrap()
        // }
        // Ok(Route::Home) => {
        //     log::debug!("Home OGP Setting");
        // }
        _ => "".to_string(),
    };

    if meta != "".to_string() {
        index_html_top.push_str(&meta);
    }

    // match meta {
    //     Ok(meta) => index_html_top.push_str(&meta),
    //     Err(e) => log::warn!("{:#}", e),
    // };
    println!("meta: {}", meta);

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });


    let index_html_before = format!("{}{}", index_html_top, index_html_head);
    // StreamBody::new(
    //     stream::once(async move { index_html_before })
    //         .chain(renderer.render_stream())
    //         .chain(stream::once(async move { index_html_after }))
    //         .map(Result::<_, Infallible>::Ok),
    // )

    let mut body = index_html_before;
    body.push_str(&renderer.render().await);
    body.push_str(&index_html_after);

    // StreamBody::new(
    //     stream::once(async move { index_html_before })
    //         .chain(renderer.render_stream())
    //         .chain(stream::once(async move { index_html_after }))
    //         .map(Result::<_, Infallible>::Ok),
    // )
    Html(body)
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
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

    let app = Router::new().fallback_service(HandleError::new(
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

    println!("You can view the website at: http://localhost:8080/");

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let bind_addr = format!("0.0.0.0:{}", port);
    Server::bind(&bind_addr.parse().unwrap())
        // Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .executor(exec)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
