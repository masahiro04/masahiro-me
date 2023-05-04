use crate::domain::entities::post::Post;
use askama::Template;

#[derive(Template)]
#[template(path = "meta.html")]
struct MetaTemplate<'a> {
    post: &'a Post,
    keywords: &'a str,
}

pub fn render_meta(post: &Post) -> String {
    let keywords = post
        .categories()
        .iter()
        .map(|c| c.name())
        .collect::<Vec<&str>>()
        .join(",");
    let content = MetaTemplate {
        post,
        keywords: &keywords,
    };
    content.render().expect("Error rendering template")
}
