use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectItemProps {
    pub name: String,
    pub technologies: String,
    pub url: String,
}

#[function_component]
pub fn ProjectItem(props: &ProjectItemProps) -> Html {
    html! {
        <div class="group cursor-pointer mb-2">
            <a href={props.url.clone()} target="_blank" rel="noreferrer">
                <div class="break-inside-avoid-column py-3 bg-white rounded-md max-w-full px-6 bg-opacity-60 shadow-sm duration-500 group-hover:shadow-lg group-hover:scale-[1.01] group-hover:bg-opacity-90">
                    <div class="font-semibold text-gray-600 text-sm sm:text-base">
                        {props.name.clone()}
                    </div>
                    <div class="w-full text-gray-500 font-thin whitespace-pre-line text-sm duration-500 group-hover:text-gray-600">
                        {props.technologies.clone()}
                    </div>
                </div>
            </a>
        </div>
    }
}
