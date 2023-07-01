use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectItemProps {
    pub name: String,
    pub technologies: String,
    pub url: String,
}

#[function_component(ProjectItem)]
pub fn project_item(props: &ProjectItemProps) -> Html {
    let ProjectItemProps {
        name,
        technologies,
        url,
    } = props;
    html! {
        <div class="group cursor-pointer mb-2">
            <a href={url.to_string()} target="_blank" rel="noreferrer">
                <div class="break-inside-avoid-column py-3 bg-white rounded-md max-w-full px-6 bg-opacity-60 shadow-sm duration-500 group-hover:shadow-lg group-hover:scale-[1.01] group-hover:bg-opacity-90">
                    <div class="font-semibold text-gray-600 text-sm sm:text-base">
                        {name}
                    </div>
                    <div class="w-full text-gray-500 font-thin whitespace-pre-line text-sm duration-500 group-hover:text-gray-600">
                        {technologies}
                    </div>
                </div>
            </a>
        </div>
    }
}
