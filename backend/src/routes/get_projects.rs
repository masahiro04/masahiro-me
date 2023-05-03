use crate::{log_request, presentation::sample::render_projects};
use worker::*;

pub async fn handle_get_projects_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let html = render_projects();
    let mut resp = Response::ok(html).unwrap();
    resp.headers_mut().set("content-type", "text/html").unwrap();
    Ok(resp)
}
