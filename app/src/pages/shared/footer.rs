use crate::pages::shared::image::Image;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum FooterItemKind {
    GitHub,
    Twitter,
    Menta,
    Email,
}

impl FooterItemKind {
    fn to_str(&self) -> &'static str {
        match self {
            FooterItemKind::GitHub => "github",
            FooterItemKind::Twitter => "twitter",
            FooterItemKind::Menta => "menta",
            FooterItemKind::Email => "email",
        }
    }
    fn to_link(&self) -> &'static str {
        match self {
            FooterItemKind::GitHub => "https://github.com/masahiro04",
            FooterItemKind::Twitter => "https://twitter.com/masa_okubo",
            FooterItemKind::Menta => "https://menta.work/user/20351",
            FooterItemKind::Email => "https://docs.google.com/forms/d/e/1FAIpQLSfXjYNmZf_Db_KqWrM3YPqkBORiVX_FY_mSv7jXhJ6FRz3iJA/viewform?embedded=true",
        }
    }

    fn to_image_src(&self) -> &'static str {
        match self {
            FooterItemKind::GitHub => "github.svg",
            FooterItemKind::Twitter => "twitter.svg",
            FooterItemKind::Menta => "menta.svg",
            FooterItemKind::Email => "email.svg",
        }
    }
}

#[derive(Clone, PartialEq)]
struct Item {
    pub kind: FooterItemKind,
}

#[derive(Properties, Clone, PartialEq)]
struct FooterItemProps {
    item: Item,
}

#[function_component]
fn FooterItem(props: &FooterItemProps) -> Html {
    // let filename = format!("/images/{}.svg", props.item.clone().kind.to_str());
    let alt_text = format!("{}の画像", props.item.clone().kind.to_str());
    html! {
        <div class="relative">
            <Image
                class="w-6 h-6"
                source={props.item.kind.to_image_src()}
                alt={alt_text}
                width=100
                height=100
            />
            <a class="absolute inset-0" href={props.item.kind.to_link()} target="_blank" rel="noreferrer"></a>
        </div>
    }
}

#[function_component]
pub fn Footer() -> Html {
    let items: Vec<Item> = vec![
        Item {
            kind: FooterItemKind::GitHub,
        },
        Item {
            kind: FooterItemKind::Twitter,
        },
        Item {
            kind: FooterItemKind::Menta,
        },
        Item {
            kind: FooterItemKind::Email,
        },
    ];
    html! {
        <footer class="justify-between text-center sm:flex">
            <div class="flex items-center space-x-3 justify-center mb-5 sm:order-last sm:mb-0">
                {items.iter().map(|item| {
                    html!{
                        <FooterItem item={item.clone()} />
                    }
                }).collect::<Html>()}
            </div>
            <div class="text-gray-700 sm:order-first">{"This blog created using Rust. / copyright © 2016-2023 Masahiro Okubo"}</div>
        </footer>
    }
}
