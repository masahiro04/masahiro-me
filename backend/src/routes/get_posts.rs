use super::cors::cors;
use crate::{
    log_request,
    usecase::exe::{fetch_posts_usecase, fetch_related_posts_usecase},
};
use std::collections::HashMap;
use url::form_urlencoded;
use worker::*;

pub async fn handle_get_posts_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let cloned_req = req.clone().unwrap();
    let url = cloned_req.url().unwrap();
    let query_params = url.query().unwrap_or("");
    let parsed_params: HashMap<String, String> = form_urlencoded::parse(query_params.as_bytes())
        .into_owned()
        .collect();
    let category_ids = match parsed_params.get("category_ids") {
        Some(category_ids) => category_ids,
        None => "",
    };
    let per_page = match parsed_params.get("per_page") {
        Some(_per_page) => _per_page,
        None => "10",
    };
    let offset = match parsed_params.get("offset") {
        Some(_offset) => _offset,
        None => "0",
    };

    let kv_namespace = String::from("BLOG_CONTENT");
    let kv = &ctx.env.kv(&kv_namespace);
    let store = match kv {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to retrieve KV store: {}", e);
            std::process::exit(1);
        }
    };

    // TODO: related postsは /related_posts として作成しても良さそうだと思った
    if category_ids.is_empty() {
        let posts = fetch_posts_usecase(store, per_page, offset).await.unwrap();
        let json_string = serde_json::to_string(&posts).unwrap();
        let mut res = Response::ok(json_string)
            .unwrap()
            .with_cors(&cors())
            .unwrap();
        res.headers_mut()
            .set("content-type", "application/json")
            .unwrap();
        return Ok(res);
    };
    let posts = fetch_related_posts_usecase(store, &category_ids)
        .await
        .unwrap();
    let json_string = serde_json::to_string(&posts).unwrap();
    let mut res = Response::ok(json_string)
        .unwrap()
        .with_cors(&cors())
        .unwrap();
    res.headers_mut()
        .set("content-type", "application/json")
        .unwrap();
    Ok(res)
}
