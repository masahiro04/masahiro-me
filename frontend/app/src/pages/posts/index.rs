use crate::domain::entities::post::Post;
use crate::pages::{
    posts::shared::loading_post::LoadingPost,
    posts::shared::pagination::Pagination,
    posts::shared::post_item::PostItem,
    shared::metadata::{insert_metadata, MetadataParams},
};
use crate::usecase::exe::*;
use yew::platform::spawn_local;
use yew::prelude::*;

const PER_PAGE: i32 = 10;

#[function_component(Loading)]
fn loading() -> Html {
    html! {
        { for (0..10).map(|_| html! { <LoadingPost /> }) }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub page: i32,
}

// TODO: paginationをクリックしたら、is_loadingな状態にする
#[function_component]
pub fn PostIndex(props: &HomeProps) -> Html {
    let posts = use_state(|| Vec::<Post>::new());
    let is_loading = use_state(|| true);
    let offset = if props.page == 1 {
        0
    } else {
        PER_PAGE * props.page
    };
    let has_next_page = use_state(|| posts.clone().len() == PER_PAGE as usize);
    {
        let set_posts = posts.clone();
        let set_is_loading = is_loading.clone();
        use_effect_with_deps(
            move |_| {
                let future = async move {
                    match fetch_posts_usecase(PER_PAGE, offset).await {
                        Ok(posts) => set_posts.set(posts),
                        Err(e) => log::error!("Error: {}", e),
                    }
                    set_is_loading.set(false)
                };
                spawn_local(future);
                || ()
            },
            props.clone().page,
        );
    }

    let set_has_next_page = has_next_page.clone();
    let posts_len = posts.len();
    use_effect_with_deps(
        move |_| {
            set_has_next_page.set(posts_len == PER_PAGE as usize);
            || ()
        },
        posts_len,
    );

    let metadata_params = MetadataParams {
        title: None,
        keywords: None,
        description: None,
        image_url: None,
    };
    insert_metadata(metadata_params);

    html! {
        <>
            <div class="space-y-2 sm:space-y-3">
                {
                    if *is_loading {
                        html! { <Loading /> }
                    } else {
                        html! {
                            posts.clone().iter().map(|post| {
                                html! { <PostItem post={post.clone()} /> }
                            }).collect::<Html>()
                        }
                    }
                }
            </div>

            <div class="px-6 mx-auto sm:px-10 sm:max-w-screen-md lg:max-w-screen-lg">
                <Pagination is_loading={*is_loading} has_next_page={*has_next_page} current_page={props.page} />
            </div>
        </>
    }
}
