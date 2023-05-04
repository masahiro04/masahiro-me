use crate::{log_request, presentation::render::render_meta, usecase::exe::fetch_post_usecase};
use std::collections::HashMap;
use url::form_urlencoded;
use worker::*;

pub async fn handle_get_meta_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let cloned_req = req.clone().unwrap();
    let url = cloned_req.url().unwrap();
    let query_params = url.query().unwrap_or("");
    let parsed_params: HashMap<String, String> = form_urlencoded::parse(query_params.as_bytes())
        .into_owned()
        .collect();
    let slug = match parsed_params.get("slug") {
        Some(slug) => slug,
        None => "",
    };
    let cors = Cors::new()
        .with_origins(vec!["*".to_string()].iter())
        .with_allowed_headers(vec!["*".to_string()])
        .with_methods(vec![Method::Get, Method::Options, Method::Head]);

    let result = match fetch_post_usecase(&ctx.env, &slug).await {
        Ok(post) => post,
        Err(_) => return Response::error("Not found", 404),
    };

    let post = match result {
        Some(post) => post,
        None => return Response::error("Not found", 404),
    };
    let meta = render_meta(&post);
    let mut resp = Response::ok(meta).unwrap().with_cors(&cors).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    Ok(resp)
}
