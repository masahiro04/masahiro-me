use crate::routes::get_post::handle_get_post_request;
use crate::routes::get_post_page::handle_get_post_page_request;
use crate::routes::get_posts::handle_get_posts_request;
use crate::routes::get_posts_page::handle_get_pages_request;
use crate::routes::get_projects_page::handle_get_projects_request;
use crate::routes::sitemap::handle_get_sitemap_request;
use crate::routes::{get_about_page::handle_get_about_request, get_meta::handle_get_meta_request};
use worker::*;

pub async fn run(req: Request, env: Env) -> Result<Response> {
    let router = Router::new();
    router
        .get("/", |_, _| {
            Response::ok("Welcome to Masahiro's tech note content API")
        })
        .get_async("/api/posts", handle_get_posts_request)
        .get_async("/api/posts/:slug", handle_get_post_request)
        .get_async("/pages", handle_get_pages_request)
        .get_async("/pages/:id", handle_get_pages_request)
        .get_async("/posts/:slug", handle_get_post_page_request)
        .get_async("/projects", handle_get_projects_request)
        .get_async("/about", handle_get_about_request)
        .get_async("/sitemap", handle_get_sitemap_request)
        .get_async("/meta", handle_get_meta_request)
        .run(req, env)
        .await
}
