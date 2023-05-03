use askama::Template;

use crate::domain::entities::post::Post;

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
#[template(path = "pages.html")]
struct PagesTemplate<'a> {
    posts: &'a Vec<Post>,
}

pub fn render_pages(posts: &Vec<Post>) -> String {
    let content = PagesTemplate { posts }.render().unwrap();
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

// TODO: 下記が実装項目
// TODO: projects
// TODO: about
// TODO: pages
// TODO: post

// #[derive(Template)]
// #[template(path = "projects.html")]
// pub struct PageTemplate<'a> {
//     title: &'a str,
//     // css: &'a str,
// }
