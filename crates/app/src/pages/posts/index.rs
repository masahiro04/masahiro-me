use crate::pages::posts::hook::posts::use_posts;
use crate::pages::{posts::shared::pagination::Pagination, posts::shared::post_item::PostItem};
use yew::prelude::*;

const PER_PAGE: i32 = 10;

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub page: i32,
}

#[function_component(PostIndex)]
pub fn post_index(props: &HomeProps) -> HtmlResult {
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

pub fn posts_meta_tags() -> String {
    let title = "Masahiro's tech note";
    let description = "ソフトウェアエンジニア、大久保将広のウェブサイトです。現在取り扱っている言語や興味関心ごとなどを記載しております。";
    let keywords =
        "大久保将広, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust";
    let mut meta = String::new();
    meta.push_str(&format!(r###"<title>{}</title>"###, title));
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
        r###"<meta property="og:image" content="https://assets.masahiro.me/kyuri.png">
        "###,
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
