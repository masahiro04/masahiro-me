use crate::pages::bindings;
use crate::pages::posts::hook::posts::use_posts;
use crate::pages::{posts::shared::pagination::Pagination, posts::shared::post_item::PostItem};
use yew::prelude::*;

const PER_PAGE: i32 = 10;

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub page: i32,
}

// TODO: paginationをクリックしたら、is_loadingな状態にする
#[function_component]
pub fn PostIndex(props: &HomeProps) -> HtmlResult {
    let offset = if props.page == 1 {
        0
    } else {
        PER_PAGE * (props.page - 1)
    };
    let posts = use_posts(offset)?;
    let has_next_page = use_state(|| posts.clone().len() == PER_PAGE as usize);

    let set_has_next_page = has_next_page.clone();
    let posts_len = posts.len();
    use_effect_with_deps(
        move |_| {
            set_has_next_page.set(posts_len == PER_PAGE as usize);
            || ()
        },
        posts_len,
    );

    #[cfg(target_arch = "wasm32")]
    {
        let title = "Masahiro's tech note";
        let excerpt = "名古屋のソフトウェアエンジニア。SaaSやマッチングサービス、AR/VR等の開発を経て現在は独立して名古屋で開発やITコンサルしています。サービス開発の所感や、ハマった際の解決方法を記載しております。";
        let keywords = "ITエンジニア, ITコンサル, IT顧問, システム開発, Rust, wasm";
        let image_url = "/images/kyuri.png";
        bindings::updateTitle(title);
        bindings::updateMetaByName(String::from("description"), excerpt);
        bindings::updateMetaByName(String::from("keywords"), keywords);

        bindings::updateMetaByName(String::from("twitter:title"), title);
        bindings::updateMetaByName(String::from("twitter:description"), excerpt);
        bindings::updateMetaByName(String::from("twitter:image"), image_url);
    }

    Ok(html! {
        <>
            <div class="space-y-2 sm:space-y-3">
                {
                    html! {
                        posts.iter().map(|post| {
                            html! { <PostItem post={post.clone()} /> }
                        }).collect::<Html>()
                    }
                }
            </div>

            <div class="px-6 mx-auto sm:px-10 sm:max-w-screen-md lg:max-w-screen-lg">
                <Pagination is_loading={false} has_next_page={*has_next_page} current_page={props.page} />
            </div>
        </>
    })
}
