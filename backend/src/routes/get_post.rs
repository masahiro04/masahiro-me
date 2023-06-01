use crate::{log_request, usecase::exe::fetch_post_usecase};
use worker::*;

use super::cors::cors;

pub async fn handle_get_post_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let slug = ctx.param("slug").unwrap();

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
    match result {
        Some(post) => {
            let json_string = serde_json::to_string(&post).unwrap();
            let mut res = Response::ok(json_string)
                .unwrap()
                .with_cors(&cors())
                .unwrap();
            res.headers_mut()
                .set("content-type", "application/json")
                .unwrap();
            Ok(res)
        }
        None => Response::error("Not found", 404),
    }
}
