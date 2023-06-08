use crate::{
    pages::{
        // bindings,
        posts::hook::{post::use_post, related_posts::use_related_posts},
        posts::shared::{categories::Categories, post_body::PostBody, post_item::PostItem},
        shared::back_button::BackButton,
    },
    usecase::exe::fetch_post_usecase,
};
use std::io::Result;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[function_component]
pub fn PostDetail(props: &PostProps) -> HtmlResult {
    let post = use_post(props.slug.clone())?;
    let category_ids = post
        .categories()
        .iter()
        .map(|category| format!("{}", category.id()))
        .collect::<Vec<String>>()
        .join(",");
    let related_posts = use_related_posts(category_ids)?;

    // #[cfg(target_arch = "wasm32")]
    // {
    //     let title = format!("{} | Masahiro's tech note", post.title());
    //     let excerpt = format!("{}", post.excerpt());
    //     let category_names = post
    //         .categories()
    //         .iter()
    //         .map(|category| format!("{}", category.name()))
    //         .collect::<Vec<String>>()
    //         .join(",");
    //     bindings::updateTitle(&title);
    //     bindings::updateMetaByName(String::from("description"), &excerpt);
    //     bindings::updateMetaByName(String::from("keywords"), &category_names);
    //     bindings::updateMetaByName(String::from("twitter:title"), &title);
    //     bindings::updateMetaByName(String::from("twitter:description"), &excerpt);
    //     bindings::updateMetaByName(String::from("twitter:image"), &post.featured_media());
    // }

    Ok(html! {
        <div>
            <BackButton />
            <div class="relative overflow-hidden py-8 bg-white bg-opacity-50 rounded-md shadow-md sm:py-16">
                <div class="relative px-4 sm:px-6.lg:px-8">
                    <div class="mx-auto max-w-prose text-lg">
                        <Categories categories={post.categories().to_vec()} is_link={true} />
                        <h1 class="mb-8">
                            <span class="mt-2 block text-center text-2xl font-bold leading-8 tracking-tight text-gray-900 sm:text-4xl">
                                {post.title()}
                            </span>
                        </h1>
                    </div>
                    <div class="prose prose-base prose-cyan mx-auto text-gray-600 sm:prose-lg">
                        <PostBody content={post.content().to_string()} />
                    </div>
                </div>
            </div>
            {if related_posts.len() != 0 {
                html! {
                    <div class="mt-5 mx-auto space-y-2 sm:space-y-3 sm:mt-10">
                        <div class="text-gray-700 text-xl font-semibold">{"Featured"}</div>
                        <div class="justify-center space-y-2 sm:space-y-3">
                            {for related_posts.iter().map(|post| {
                                html! { <PostItem post={post.clone()} /> }
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

// #[cfg(feature = "ssr")]
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
        r###"<meta name="description" content="{}" >"###,
        description
    ));
    meta.push_str(&format!(
        r###"<meta name="keywords" content="{}" >"###,
        keywords
    ));
    meta.push_str(&format!(
        r###"<meta property="og:title" content="{}" />
        "###,
        title
    ));
    meta.push_str(&format!(
        r###"<meta property="og:description" content="{}" />
        "###,
        description
    ));
    meta.push_str(&format!(
        r###"<meta property="og:site_name" content="Masahiro's tech note" />
        "###,
    ));
    meta.push_str(&format!(
        r###"<meta property="og:image" content="{}" />
        "###,
        featured_media
    ));
    meta.push_str(&format!(
        r###"<meta name="twitter:creator" content="@masa_okubo" />
        "###,
    ));

    meta
}
