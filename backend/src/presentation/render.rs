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

#[derive(Template)]
#[template(path = "template.html")]
pub struct PageTemplate<'a> {
    content: &'a str,
}
#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[derive(Template)]
#[template(path = "posts.html")]
struct PagesTemplate<'a> {
    current_page: u32,
    has_next_page: bool,
    posts: &'a Vec<Post>,
}

#[derive(Template)]
#[template(path = "post.html")]
struct PostTemplate<'a> {
    post: &'a Post,
    related_posts: &'a Vec<Post>,
}

pub fn render_pages(posts: &Vec<Post>, current_page: u32, has_next_page: bool) -> String {
    let content = PagesTemplate {
        posts,
        current_page,
        has_next_page,
    }
    .render()
    .unwrap();
    let page = PageTemplate { content: &content };
    page.render().expect("Error rendering template")
}

pub fn render_post(post: &Post, related_posts: &Vec<Post>) -> String {
    let content = PostTemplate {
        post,
        related_posts,
    }
    .render()
    .unwrap();
    let page = PageTemplate { content: &content };
    page.render().expect("Error rendering template")
}
pub fn render_projects() -> String {
    let content = ProjectsTemplate {}.render().unwrap();
    let page = PageTemplate { content: &content };
    page.render().expect("Error rendering template")
}
pub fn render_about() -> String {
    let content = AboutTemplate {}.render().unwrap();
    let page = PageTemplate { content: &content };
    page.render().expect("Error rendering template")
}
