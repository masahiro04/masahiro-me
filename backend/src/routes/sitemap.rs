use crate::{log_request, usecase::exe::fetch_slugs_usecase};
use worker::*;

pub async fn handle_get_sitemap_request(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    log_request(&req);
    let slugs = fetch_slugs_usecase(&ctx.env).await.unwrap();
    let mut sitemap = String::new();
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">");
    sitemap.push_str(
        "<url>
            <loc>https://masahiro.me/pages/1</loc>
            <priority>1.0</priority>
        </url>",
    );
    sitemap.push_str(
        "<url>
            <loc>https://masahiro.me/projects</loc>
            <priority>0.8</priority>
        </url>",
    );
    sitemap.push_str(
        "<url>
            <loc>https://masahiro.me/about</loc>
            <priority>0.8</priority>
        </url>",
    );
    for slug in slugs {
        sitemap.push_str(&format!(
            "<url>
                <loc>https://masahiro.me/posts/{}</loc>
                <priority>0.8</priority>
            </url>",
            slug
        ));
    }
    sitemap.push_str("</urlset>");
    let cors = Cors::new()
        .with_origins(vec!["*".to_string()].iter())
        .with_max_age(86400)
        .with_allowed_headers(vec!["*".to_string()])
        .with_methods(vec![Method::Get, Method::Options, Method::Head]);
    let mut res = Response::ok(sitemap).unwrap().with_cors(&cors).unwrap();
    res.headers_mut()
        .set("content-type", "application/xml")
        .unwrap();
    Ok(res)
}
