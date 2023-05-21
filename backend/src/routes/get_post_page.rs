use crate::{
    log_request,
    presentation::render::render_post,
    usecase::exe::{fetch_post_usecase, fetch_related_posts_usecase},
};
use worker::*;

pub async fn handle_get_post_page_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let slug: &str = ctx.param("slug").unwrap();
    let kv_namespace = String::from("BLOG_CONTENT");
    let kv = &ctx.env.kv(&kv_namespace);
    let store = match kv {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to retrieve KV store: {}", e);
            std::process::exit(1);
        }
    };

    let result = match fetch_post_usecase(store, &slug).await {
        Ok(post) => post,
        Err(_) => return Response::error("Not found", 404),
    };

    let post = match result {
        Some(post) => post,
        None => return Response::error("Not found", 404),
    };

    let category_ids = post
        .categories()
        .iter()
        .map(|c| c.id().to_string())
        .collect::<Vec<_>>()
        .join("");
    console_log!("category_ids: {}", category_ids);

    let related_posts = match fetch_related_posts_usecase(store, &category_ids).await {
        Ok(posts) => posts,
        Err(_) => vec![],
    };

    let html = render_post(&post, &related_posts);

    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    Ok(resp)
}
