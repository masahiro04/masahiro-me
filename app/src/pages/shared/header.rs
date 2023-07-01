use crate::pages::shared::image::Image;
use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::use_location;
use yew_router::prelude::*;

#[derive(Clone)]
struct Item {
    name: String,
    active_paths: Vec<String>,
    href: Route,
    lp_class: String,
    sm_class: String,
}

#[function_component(Header)]
pub fn header() -> Html {
    let location = use_location();
    let items = vec![
        Item {
            name: "Posts".to_string(),
            active_paths: vec!["pages".to_string(), "posts".to_string()],
            href: Route::PostIndex { page: 1 },
            lp_class: "tracking-wider text-gray-700 text-base".to_string(),
            sm_class: "tracking-wider text-gray-700 text-sm text-center".to_string(),
        },
        Item {
            name: "Projects".to_string(),
            active_paths: vec!["projects".to_string()],
            href: Route::Projects,
            lp_class: "tracking-wider text-gray-700 text-base".to_string(),
            sm_class: "tracking-wider text-gray-700 text-sm text-center".to_string(),
        },
        Item {
            name: "About".to_string(),
            active_paths: vec!["about".to_string()],
            href: Route::AboutIndex,
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
                <Link<Route> classes={"text-2xl font-semibold tracking-wide text-gray-700 whitespace-nowrap"} to={Route::PostIndex { page: 1 }} >
                    { "Masahiro's tech note" }
                </Link<Route>>
                <div class="items-center hidden sm:flex sm:space-x-8 md:space-x-12">
                    {
                        items.iter().map(|item| {
                            html! {
                                <Link<Route> classes={classes!(make_class_string(item.lp_class.clone(), is_current_path(item.active_paths.clone())))} to={item.href.clone()}>
                                    { item.name.clone() }
                                </Link<Route>>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="relative">
                    <Image class="w-6 h-6 flex-none" source="github.svg" />
                    <a
                      class="absolute inset-0"
                      href="https://github.com/masahiro04/masahiro-me"
                      target="_blank"
                      rel="noreferrer"
                    />
                </div>
            </div>
            <div class="grid grid-cols-3 divide-x divide-gray-300 mt-3 sm:hidden">
                {items.iter().map(|item| {
                    html! {
                        <Link<Route> classes={classes!(make_class_string(item.sm_class.clone(), is_current_path(item.active_paths.clone())))} to={item.href.clone()}>
                            { item.name.clone() }
                        </Link<Route>>
                    }
                }).collect::<Html>() }
            </div>
        </nav>
    }
}
