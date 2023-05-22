use crate::console_log;
use crate::domain::entities::post::Post;
use crate::pages::{
    not_found::NotFound,
    posts::shared::{categories::Categories, post_body::PostBody, post_item::PostItem},
    shared::back_button::BackButton,
    shared::metadata::{insert_metadata, MetadataParams},
};
use crate::usecase::exe::{fetch_post_usecase, fetch_related_posts_usecase};
use yew::platform::spawn_local;
use yew::prelude::*;

#[function_component(Loading)]
fn loading() -> Html {
    html! {
        <>
            <div class="relative cursor-pointer duration-500 py-2 bg-white rounded-md shadow-sm px-2 bg-opacity-60 flex items-center w-24 justify-center text-gray-600 text-sm -translate-y-1 hover:shadow-md sm:-translate-y-2">
                <div class="h-4 w-20 sm:w-5 sm:h-5 bg-gray-300 animate-pulse" />
            </div>
            <div class="relative overflow-hidden py-8 bg-white bg-opacity-50 rounded-md shadow-md sm:py-16">
                <div class="relative px-4 sm:px-6 lg:px-8">
                    <div class="mx-auto max-w-prose">
                        <div class="mx-auto w-100 text-center mb-4">
                            <div class="h-8 w-100 bg-gray-300 animate-pulse text-center mb-4" />
                        </div>
                        {for (0..4).map(|_| { html! {
                            <>
                                <div class="h-4 w-1/2 bg-gray-300 animate-pulse mb-4"></div>
                                <div class="h-4 w-3/4 bg-gray-300 animate-pulse mb-4"></div>
                                <div class="h-4 w-3/4 bg-gray-300 animate-pulse mb-4"></div>
                            </>
                        } })}
                    </div>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[function_component]
pub fn PostDetail(props: &PostProps) -> Html {
    let post: UseStateHandle<Option<Post>> = use_state(|| None);
    let related_posts: UseStateHandle<Vec<Post>> = use_state(|| vec![]);
    let is_loading = use_state(|| true);
    let slug = props.slug.clone();

    let title = "".to_string();
    let title = if let Some(post) = &*post {
        &*post.title()
    } else {
        &title
    };

    let content = "".to_string();
    let content = if let Some(post) = &*post {
        post.content()
    } else {
        &content
    };

    let excerpt = "".to_string();
    let excerpt = if let Some(post) = &*post {
        post.excerpt()
    } else {
        &excerpt
    };
    let categories = vec![];
    let categories = if let Some(post) = &*post {
        post.categories()
    } else {
        &categories
    };

    let featured_media = "".to_string();
    let featured_media = if let Some(post) = &*post {
        post.featured_media()
    } else {
        &featured_media
    };

    {
        let set_post = post.clone();
        let set_is_loading = is_loading.clone();
        use_effect_with_deps(
            move |_| {
                let future = async move {
                    match fetch_post_usecase(slug).await {
                        Ok(post) => set_post.set(post),
                        // Err(e) => log::error!("Error: {}", e),
                        Err(e) => console_log!("Error: {}", e),
                    }
                    set_is_loading.set(false)
                };
                spawn_local(future);
                || ()
            },
            props.slug.clone(),
        );
    }

    {
        let set_related_posts = related_posts.clone();
        let set_is_loading = is_loading.clone();
        let category_ids = categories
            .iter()
            .map(|category| format!("{}", category.id()))
            .collect::<Vec<String>>()
            .join(",");

        use_effect_with_deps(
            move |_| {
                let future = async move {
                    match fetch_related_posts_usecase(&category_ids).await {
                        Ok(posts) => set_related_posts.set(posts),
                        // Err(e) => log::error!("Error: {}", e),
                        Err(e) => console_log!("Error: {}", e),
                    }
                    set_is_loading.set(false);
                };
                spawn_local(future);
                || ()
            },
            categories.clone(),
        );
    }

    {
        let category_names = categories
            .iter()
            .map(|category| category.name().to_string())
            .collect::<Vec<String>>()
            .join(",");
        let metadata_title = &format!("{} | Masahiro's tech note", &title);
        let metadata_params = MetadataParams {
            title: Some(metadata_title),
            keywords: Some(&category_names),
            description: Some(&excerpt),
            image_url: Some(&featured_media),
        };
        insert_metadata(metadata_params);
    }

    if *is_loading {
        return html! { <Loading /> };
    }

    if post.is_none() {
        return html! { <NotFound /> };
    }

    html! {
        <div>
            <BackButton />
            <div class="relative overflow-hidden py-8 bg-white bg-opacity-50 rounded-md shadow-md sm:py-16">
                <div class="relative px-4 sm:px-6.lg:px-8">
                    <div class="mx-auto max-w-prose text-lg">
                        <Categories categories={categories.to_vec()} is_link={true} />
                        <h1 class="mb-8">
                            <span class="mt-2 block text-center text-2xl font-bold leading-8 tracking-tight text-gray-900 sm:text-4xl">
                                {title}
                            </span>
                        </h1>
                    </div>
                    <div class="prose prose-base prose-cyan mx-auto text-gray-600 sm:prose-lg">
                        <PostBody content={content.to_string()} />
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
    }
}
