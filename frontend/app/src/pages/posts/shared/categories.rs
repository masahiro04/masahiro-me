use crate::domain::entities::category::Category;
use crate::pages::shared::link::Link;
use crate::routes::RootRoutes;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
struct CategoryItemProps {
    pub name: String,
}

#[function_component(CategoryItem)]
fn category_item(props: &CategoryItemProps) -> Html {
    html! {
        <div class="align-middle relative group flex">
            <div class="w-3 h-3 rotate-45 left-0 bg-main-300 rounded-sm mt-0.5" />
                <div class="bg-main-300 rounded-r-sm text-xs tracking-wide text-gray-500 -translate-x-1.5 pl-1.5 pr-1.5">
                    {props.name.clone()}
                </div>
            <div class="absolute rounded-full bg-white bg-opacity-80 w-1 h-1 top-1.5 left-1" />
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CategoriesProps {
    pub categories: Vec<Category>,
    pub is_link: bool,
}

#[function_component(Categories)]
pub fn categories(props: &CategoriesProps) -> Html {
    html! {
         <div class="flex space-x-2 items-center pl-1">
             {props.categories.iter().map(|category| {
                 if props.is_link {
                     html! {
                         <Link href={RootRoutes::PostIndex { page: 1 }}>
                             <CategoryItem name={category.name().to_string()} />
                         </Link>
                     }
                 } else {
                     html! {
                         <CategoryItem name={category.name().to_string()} />
                     }
                 }

             }).collect::<Html>() }
         </div>
    }
}