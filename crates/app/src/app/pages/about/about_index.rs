use crate::pages::components::image::Image;
use yew::prelude::*;

struct RenderItem {
    pub title: String,
    pub elements: Vec<String>,
}

#[function_component(AboutIndex)]
pub fn about_index() -> Html {
    let render_item = |item: RenderItem| -> Html {
        html! {
            <div class="relative">
                <div
                    class="flex justify-center py-3 bg-white rounded-md max-w-full px-6 bg-opacity-60 text-gray-400 shadow-sm space-x-1 text-sm sm:text-base sm:space-x-2"
                >
                    {
                        item.elements
                            .iter()
                            .enumerate()
                            .map(|(index, element)| {
                                html! {
                                    <>
                                        <div class="text-gray-800 text-center">{element}</div>
                                        {if index != item.elements.len() - 1 { html!{
                                            <div class="text-gray-600 text-center last:hidden">{"/"}</div>
                                        } } else { html!{
                                            <></>
                                        } } }
                                    </>
                                }
                            })
                            .collect::<Html>()
                    }

                </div>
                <div
                    class="absolute inset-0 text-sm text-gray-500 -translate-y-6 sm:translate-y-0 sm:left-2 md:left-3 sm:top-0 md:top-1"
                >
                    { format!("{} :", item.title)}
                </div>
            </div>
        }
    };

    let render_items = || -> Html {
        let languages = RenderItem {
            title: "Languages".to_string(),
            elements: vec!["TypeScript".to_string(), "Rust".to_string()],
        };
        let interests = RenderItem {
            title: "Interests".to_string(),
            elements: vec![
                "Rust".to_string(),
                "WASM".to_string(),
                "Infrastructure".to_string(),
            ],
        };
        html! {
            <>
                {render_item(languages)}
                {render_item(interests)}
            </>
        }
    };

    html! {
        <div class="flex justify-center mx-auto mt-10 sm:w-2/3 sm:mt-0">
            <div class="flex flex-col w-full">
                <Image
                    class="object-cover rounded-full mx-auto w-24 h-24"
                    source="kyuri.png"
                    alt=""
                    width={ 100 }
                    height={ 100 }
                />
                <div class="mt-3 space-y-2">
                    <h3 class="text-xl font-semibold text-center text-gray-800">
                        { "Masahiro Okubo" }
                    </h3>
                    <p class="mx-auto text-center text-gray-800">
                        { "I'm a software engineer living in Japan, Nagoya." } </p>
                </div>
                <div class="mt-8 space-y-7 sm:mt-5 sm:space-y-3">
                    {render_items()}
                    <div class="relative group cursor-pointer">
                        <div
                            class="flex justify-center py-3 bg-white rounded-md max-w-full px-6 bg-opacity-60 text-gray-400 shadow-sm space-x-1 text-sm sm:text-base sm:space-x-2"
                        >
                            <div class="text-gray-800 text-center">
                                { "Contact me on Google Form" }
                            </div>
                        </div>
                        <div
                            class="absolute inset-0 text-sm text-gray-500 -translate-y-6 sm:translate-y-0 sm:left-2 md:left-3 sm:top-0 md:top-1"
                        >
                            { "Contact :" }
                        </div>
                        <a
                            class="absolute inset-0"
                            href="https://docs.google.com/forms/d/e/1FAIpQLSfXjYNmZf_Db_KqWrM3YPqkBORiVX_FY_mSv7jXhJ6FRz3iJA/viewform?embedded=true"
                            target="_blank"
                            rel="noreferrer"
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn about_meta_tags() -> String {
    let title = "About me | Masahiro's tech note";
    let description = "ソフトウェアエンジニア、大久保将広のウェブサイトです。現在取り扱っている言語や興味関心ごとなどを記載しております。";
    let keywords =
        "大久保将広, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust";
    let mut meta = String::new();
    meta.push_str(&format!(r###"<title>{}</title>"###, title));
    meta.push_str(&format!(
        r#"<meta name="description" content="{}">"#,
        description
    ));
    meta.push_str(&format!(r#"<meta name="keywords" content="{}">"#, keywords));
    meta.push_str(&format!(
        r#"<meta property="og:title" content="{}">
        "#,
        title
    ));
    meta.push_str(&format!(
        r#"<meta property="og:description" content="{}">
        "#,
        description
    ));
    meta.push_str(
        r#"<meta property="og:site_name" content="Masahiro's tech note">
        "#,
    );
    meta.push_str(
        r#"<meta property="og:image" content="https://assets.masahiro.me/kyuri.png">
        "#,
    );
    meta.push_str(
        r#"<meta name="twitter:creator" content="@masa_okubo">
        "#,
    );
    meta.push_str(
        r#"<meta name="twitter:card" content="summary_large_image">
        "#,
    );
    meta
}
