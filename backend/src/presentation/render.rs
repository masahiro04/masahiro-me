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
    pub title: &'a str,
    pub keywords: &'a str,
    pub description: &'a str,
    pub image_url: &'a str,
    pub content: &'a str,
}
#[derive(Template)]
#[template(path = "projects.html")]
pub struct ProjectsTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate<'a> {
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

pub fn render_posts(posts: &Vec<Post>, current_page: u32, has_next_page: bool) -> String {
    let content = PostsTemplate {
        posts,
        current_page,
        has_next_page,
    }
    .render()
    .unwrap();
    let title = "Masahiro's tech note".to_string();
    let keywords = "ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm".to_string();
    let description = "名古屋のソフトウェアエンジニア。SaaSやマッチングサービス、AR/VR等の開発を経て現在は独立して名古屋で開発やITコンサルしています。サービス開発の所感や、ハマった際の解決方法を記載しております。".to_string();
    let image_url = "https://assets.masahiro.me/kyuri.png".to_string();

    let page = PageTemplate {
        title: &title,
        keywords: &keywords,
        description: &description,
        image_url: &image_url,
        content: &content,
    };
    page.render().expect("Error rendering template")
}

pub fn render_post(post: &Post, related_posts: &Vec<Post>) -> String {
    let content = PostTemplate {
        post,
        related_posts,
    }
    .render()
    .unwrap();

    let keywords = post
        .categories()
        .iter()
        .map(|c| c.name())
        .collect::<Vec<&str>>()
        .join(",");
    let page = PageTemplate {
        title: &format!("{} | Masahiro's tech note", post.title()),
        keywords: &keywords,
        description: post.excerpt(),
        image_url: post.featured_media(),
        content: &content,
    };
    page.render().expect("Error rendering template")
}

pub fn render_projects() -> String {
    let content = ProjectsTemplate {}.render().unwrap();
    let title = "Projects | Masahiro's tech note".to_string();
    let keywords = "ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm".to_string();
    let description = "参加しているProject一覧です。".to_string();
    let image_url = "https://assets.masahiro.me/kyuri.png".to_string();
    let page = PageTemplate {
        title: &title,
        keywords: &keywords,
        description: &description,
        image_url: &image_url,
        content: &content,
    };
    page.render().expect("Error rendering template")
}
pub fn render_about() -> String {
    let content = AboutTemplate {}.render().unwrap();
    let title = "About | Masahiro's tech note".to_string();
    let keywords = "ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm".to_string();
    let description = "名古屋のソフトウェアエンジニア。SaaSやマッチングサービス、AR/VR等の開発を経て現在は独立して名古屋で開発やITコンサルしています。サービス開発の所感や、ハマった際の解決方法を記載しております。".to_string();
    let image_url = "https://assets.masahiro.me/kyuri.png".to_string();
    let page = PageTemplate {
        title: &title,
        keywords: &keywords,
        description: &description,
        image_url: &image_url,
        content: &content,
    };
    page.render().expect("Error rendering template")
}
