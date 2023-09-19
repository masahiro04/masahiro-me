use super::hooks;
use super::post_components::{categories, post_body, post_item};
use crate::pages::components::back_button;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[function_component(PostDetail)]
pub fn post_detail(props: &PostProps) -> HtmlResult {
    let post = hooks::post::use_post(props.slug.clone())?;
    let category_ids = post
        .categories()
        .iter()
        .map(|category| format!("{}", category.id()))
        .collect::<Vec<String>>()
        .join(",");
    let related_posts = hooks::related_posts::use_related_posts(category_ids)?;

    Ok(html! {
        <div>
            <back_button::BackButton />
            <div class="relative overflow-hidden py-8 bg-white bg-opacity-50 rounded-md shadow-md sm:py-16">
                <div class="relative px-4 sm:px-6.lg:px-8">
                    <div class="mx-auto max-w-prose text-lg">
                        <categories::Categories categories={post.categories().to_vec()} is_link={true} />
                        <h1 class="mb-8">
                            <span class="mt-2 block text-center text-2xl font-bold leading-8 tracking-tight text-gray-900 sm:text-4xl">
                                {post.title()}
                            </span>
                        </h1>
                    </div>
                    <div class="prose prose-base prose-cyan mx-auto text-gray-600 sm:prose-lg break-words">
                        <post_body::PostBody content={post.content().to_string()} />
                    </div>
                </div>
            </div>
            {if !related_posts.is_empty() {
                html! {
                    <div class="mt-5 mx-auto space-y-2 sm:space-y-3 sm:mt-10">
                        <div class="text-gray-700 text-xl font-semibold">{"Featured"}</div>
                        <div class="justify-center space-y-2 sm:space-y-3">
                            {for related_posts.iter().map(|post| {
                                html! { <post_item::PostItem post={post.clone()} /> }
                            })}
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    })
}

pub fn post_meta_tags(
    title: &str,
    description: &str,
    keywords: &str,
    featured_media: &str,
) -> String {
    let mut meta = String::new();
    meta.push_str(&format!(
        r###"<title>{} | Masahiro's tech note</title>"###,
        title
    ));
    meta.push_str(&format!(
        r###"<meta name="description" content="{}">"###,
        description
    ));
    meta.push_str(&format!(
        r###"<meta name="keywords" content="{}">"###,
        keywords
    ));
    meta.push_str(&format!(
        r###"<meta property="og:title" content="{}">
        "###,
        title
    ));
    meta.push_str(&format!(
        r###"<meta property="og:description" content="{}">
        "###,
        description
    ));
    meta.push_str(&format!(
        r###"<meta property="og:site_name" content="Masahiro's tech note">
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:image" content="{}">
        "###,
        featured_media
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:creator" content="@masa_okubo">
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:card" content="summary_large_image">
        "###,
    ));

    meta
}
