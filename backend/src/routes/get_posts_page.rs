use crate::{
    log_request,
    presentation::render::render_pages,
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

    let path_segments = url.path_segments().unwrap().collect::<Vec<_>>();
    let mut id = 1;

    if path_segments.len() > 1 {
        let route_id = path_segments[1];
        id = route_id.parse().unwrap_or(1);
        println!("{}", id);
    } else {
        println!("No ID found in the path");
    }

    let per_page = 10;
    let offset = per_page * (id - 1);
    let cors = Cors::new()
        .with_origins(vec!["*".to_string()].iter())
        .with_max_age(86400)
        .with_allowed_headers(vec!["*".to_string()])
        .with_methods(vec![Method::Get, Method::Options, Method::Head]);
    // TODO: related postsは /related_posts として作成しても良さそうだと思った
    // if category_ids.is_empty() {
    // };

    let kv_namespace = String::from("BLOG_CONTENT");
    let kv = &ctx.env.kv(&kv_namespace);
    let store = match kv {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to retrieve KV store: {}", e);
            std::process::exit(1);
        }
    };

    let posts = fetch_posts_usecase(store, &per_page.to_string(), &offset.to_string())
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
    console_log!("id: {}", id);

    let html = render_pages(&posts, id, true);

    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    Ok(resp)
}
