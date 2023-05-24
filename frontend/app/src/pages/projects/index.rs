use crate::{
    pages::projects::shared::{project_header::ProjectHeader, project_item::ProjectItem},
    // pages::shared::metadata::{insert_metadata, MetadataParams},
    usecase::exe::{
        fetch_advisory_projects_usecase, fetch_past_work_projects_usecase,
        fetch_work_projects_usecase,
    },
};
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let works = fetch_work_projects_usecase().unwrap();
    let past_works = fetch_past_work_projects_usecase().unwrap();
    let advidories = fetch_advisory_projects_usecase().unwrap();

    let render_works = || -> Html {
        works
            .into_iter()
            .map(|project| {
                html! {
                    <ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
                }
            })
            .collect::<Html>()
    };
    let render_advisories = || -> Html {
        advidories
            .into_iter()
            .map(|project| {
                html! {
                    <ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
                }
            })
            .collect::<Html>()
    };
    let render_past_works = || -> Html {
        past_works
            .into_iter()
            .map(|project| {
                html! {
                    <ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
                }
            })
            .collect::<Html>()
    };
    fn render_section(title: String, project_nodes: Html) -> Html {
        html! {
            <div class="mb-3">
                <div class="mb-3 font-semibold text-gray-700 text-lg sm:text-xl">
                    {title.clone()}
                </div>
                <div class="gap-x-10 justify-center sm:columns-2">
                    {project_nodes}
                </div>
            </div>
        }
    }

    html! {
        <>
            <ProjectHeader />
            {render_section("Currently working on".to_string(), render_works())}
            {render_section("Advidors".to_string(), render_advisories())}
            {render_section("Past works".to_string(), render_past_works())}
        </>
    }
}
