use crate::pages::shared::{image::Image, link::Link};
use crate::routes::RootRoutes;
use yew::prelude::*;
use yew_router::prelude::use_location;

struct Item {
    name: String,
    active_paths: Vec<String>,
    href: RootRoutes,
    lp_class: String,
    sm_class: String,
}

#[function_component]
pub fn Header() -> Html {
    let location = use_location();

    let items = vec![
        Item {
            name: "Posts".to_string(),
            active_paths: vec!["pages".to_string(), "posts".to_string()],
            href: RootRoutes::PostIndex { page: 1 },
            lp_class: "tracking-wider text-gray-700 text-base".to_string(),
            sm_class: "tracking-wider text-gray-700 text-sm text-center".to_string(),
        },
        Item {
            name: "Projects".to_string(),
            active_paths: vec!["projects".to_string()],
            href: RootRoutes::Projects,
            lp_class: "tracking-wider text-gray-700 text-base".to_string(),
            sm_class: "tracking-wider text-gray-700 text-sm text-center".to_string(),
        },
        Item {
            name: "About".to_string(),
            active_paths: vec!["about".to_string()],
            href: RootRoutes::AboutIndex,
            lp_class: "tracking-wider text-gray-700 text-base".to_string(),
            sm_class: "tracking-wider text-gray-700 text-sm text-center".to_string(),
        },
    ];

    let is_current_path = |paths: Vec<String>| -> bool {
        match &location {
            Some(lo) => {
                let pathname = lo.path();
                let splited_route = pathname.split('/').collect::<Vec<&str>>();
                let contains_any = splited_route
                    .iter()
                    .any(|&s| paths.iter().any(|path| path == s));
                contains_any
            }
            None => false,
        }
    };

    let make_class_string = |class: String, is_current: bool| -> String {
        if is_current {
            format!(
                "{} underline underline-offset-2 decoration-gray-700 decoration-2",
                class,
            )
        } else {
            class
        }
    };

    html! {
    <nav class="py-3 bg-white rounded-md shadow-lg px-7 bg-opacity-60 mb-5 sm:mb-16">
      <div class="flex items-center justify-between">
        <Link href={RootRoutes::PostIndex { page: 1 }} class={"text-2xl font-semibold tracking-wide text-gray-700 whitespace-nowrap"}>
            { "Masahiro's tech note" }
        </Link>
        <div class="items-center hidden sm:flex sm:space-x-8 md:space-x-12">
            {
                items.iter().map(|item| {
                    html! {
                        <Link href={item.href.clone()} class={make_class_string(item.lp_class.clone(), is_current_path(item.active_paths.clone()))}>
                            { item.name.clone() }
                        </Link>
                    }
                }).collect::<Html>()
            }
        </div>
        <div class="relative">
          <Image class="w-6 h-6 flex-none" source="/images/github.svg" />
          <a
            class="absolute inset-0"
            href="https://github.com/masahiro04/masahiro-me"
            target="_blank"
            rel="noreferrer"
          ></a>
        </div>
      </div>
      <div class="grid grid-cols-3 divide-x divide-gray-300 mt-3 sm:hidden">
        {
            items.iter().map(|item| {
                html! {
                    <Link href={item.href.clone()} class={make_class_string(item.sm_class.clone(), is_current_path(item.active_paths.clone()))}>
                        { item.name.clone() }
                    </Link>
                }
            }).collect::<Html>()
        }
      </div>
    </nav>
    }
}
