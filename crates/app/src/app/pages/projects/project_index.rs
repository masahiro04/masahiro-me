use yew::prelude::*;
use {
    crate::pages::projects::project_components::{project_header, project_item},
    crate::use_cases::{
        fetch_advisory_projects_usecase, fetch_past_work_projects_usecase,
        fetch_work_projects_usecase,
    },
};

#[function_component(ProjectIndex)]
pub fn project_index() -> Html {
    let works = fetch_work_projects_usecase();
    let past_works = fetch_past_work_projects_usecase();
    let advidories = fetch_advisory_projects_usecase();

    let render_works = || -> Html {
        works
            .into_iter()
            .map(|project| {
                html! {
                    <project_item::ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
                }
            })
            .collect::<Html>()
    };
    let render_advisories = || -> Html {
        advidories
            .into_iter()
            .map(|project| {
                html! {
                    <project_item::ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
                }
            })
            .collect::<Html>()
    };
    let render_past_works = || -> Html {
        past_works
            .into_iter()
            .map(|project| {
                html! {
                    <project_item::ProjectItem name={project.name().to_string()} technologies={project.technologies().to_string()} url={project.url().to_string()} />
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
            <project_header::ProjectHeader />
            {render_section("Currently working on".to_string(), render_works())}
            {render_section("Member of the Board / Advidors".to_string(), render_advisories())}
            {render_section("Past works".to_string(), render_past_works())}
        </>
    }
}

pub fn projects_meta_tags() -> String {
    let title = "Projects | Masahiro's tech note ";
    let description = "現在参加中の案件一覧です。上流から下流まで対応するプロジェクトやアドバイスを行う顧問活動も行っております。";
    let keywords =
        "参加案件, ソフトウェアエンジニア, バックエンド, フロントエンド, TypeScript, Rust";
    let mut meta = String::new();
    meta.push_str(&format!(r#"<title>{}</title>"#, title));
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

    meta.push_str(r#"<meta name="twitter:card" content="summary_large_image">"#);
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
