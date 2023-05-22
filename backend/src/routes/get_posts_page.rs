use crate::{log_request, presentation::render::render_posts, usecase::exe::fetch_posts_usecase};
use worker::*;

pub async fn handle_get_pages_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let key = req.url()?.to_string();
    let id: u32 = ctx.param("id").unwrap().parse().unwrap_or(1);
    let cache = Cache::default();
    let cache_data = match cache.get(&key, true).await {
        Ok(val) => val,
        Err(e) => {
            console_log!("error: {}", e);
            None
        }
    };

    if cache_data.is_some() {
        let val = cache_data.unwrap();
        return Ok(val);
    }

    let per_page = 10;
    let mut offset = 0;
    if id != 1 {
        offset = per_page * (id - 1);
    }

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

    let html = render_posts(&posts, id, true);

    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    // TODO: cacheは外部に切り分けた方が良いかも
    resp.headers_mut()
        .set("cache-control", "s-maxage=86400")
        .unwrap();
    cache.put(key, resp.cloned()?).await?;

    Ok(resp)
}
