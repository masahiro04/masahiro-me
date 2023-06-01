use crate::{log_request, presentation::render::render_projects};
use worker::*;

pub async fn handle_get_projects_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let key = req.url()?.to_string();
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

    let html = render_projects();
    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();

    resp.headers_mut()
        .set("cache-control", "s-maxage=86400")
        .unwrap();
    cache.put(key, resp.cloned()?).await?;
    Ok(resp)
}
