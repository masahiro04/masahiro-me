use crate::{
    log_request,
    presentation::sample::render_pages,
    usecase::exe::{fetch_posts_usecase, fetch_related_posts_usecase},
};
use std::collections::HashMap;
use url::form_urlencoded;
use worker::*;

pub async fn handle_get_pages_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
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
    let cors = Cors::new()
        .with_origins(vec!["*".to_string()].iter())
        .with_max_age(86400)
        .with_allowed_headers(vec!["*".to_string()])
        .with_methods(vec![Method::Get, Method::Options, Method::Head]);
    // TODO: related postsは /related_posts として作成しても良さそうだと思った
    // if category_ids.is_empty() {
    // };

    let posts = fetch_posts_usecase(&ctx.env, per_page, offset)
        .await
        .unwrap();
    // let json_string = serde_json::to_string(&posts).unwrap();
    // let mut res = Response::ok(json_string).unwrap().with_cors(&cors).unwrap();
    // res.headers_mut()
    //     .set("content-type", "application/json")
    //     .unwrap();
    // return Ok(res);
    // let posts = fetch_related_posts_usecase(&ctx.env, &category_ids)
    //     .await
    //     .unwrap();
    console_log!("posts: {:?}", posts.len());

    let html = render_pages(&posts);

    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    Ok(resp)
}
