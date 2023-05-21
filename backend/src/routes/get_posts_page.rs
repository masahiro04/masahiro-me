use crate::{log_request, presentation::render::render_posts, usecase::exe::fetch_posts_usecase};
use worker::*;

pub async fn handle_get_pages_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    console_log!("url: {}", req.url()?.to_string());
    let key = req.url()?.to_string();
    console_log!("sentinel1");
    let id: u32 = ctx.param("id").unwrap().parse().unwrap_or(1);
    console_log!("sentinel2");

    let cache = Cache::default();
    console_log!("sentinel4");

    let cache_data = match cache.get(&key, true).await {
        Ok(val) => val,
        Err(e) => {
            console_log!("error: {}", e);
            None
        }
    };
    console_log!("sentinel5");
    if cache_data.is_some() {
        console_log!("sentinel6");
        let val = cache_data.unwrap();
        return Ok(val);
    }
    console_log!("sentinel7");

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
    console_log!("posts: {:?}", posts.len());
    console_log!("id: {}", id);

    let html = render_posts(&posts, id, true);

    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    // TODO: cacheは外部に切り分けた方が良いかも
    resp.headers_mut().set("cache-control", "s-maxage=86400")?;
    cache.put(key, resp.cloned()?).await?;

    Ok(resp)
}
