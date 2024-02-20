
use crate::components::ui::card_link::CardLink;
use crate::components::ui::close::Close;
use crate::components::ui::layout::Layout;
use crate::types::project::Project;
use leptos::prelude::*;

use leptos::svg::Svg;
use leptos::*;
use leptos_use::use_element_hover;

// TODO: Remove this function and update get_projects to take a param
// TODO: project_name (optional) and use it here
#[server(GetProjectByName, "/api", "GetJson", "v1/project")]
pub async fn get_project_by_slug(
    project_name: String,
) -> Result<Option<Project>, ServerFnError> {
    use std::fs;
    use crate::types::project::ProjectData;

    let file_path = format!(
        "{}/target/site/resources/projects.json",
        env!("CARGO_MANIFEST_DIR")
    );

    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Failed to read file: {}",
                e
            )))
        }
    };

    let le_json: ProjectData = match serde_json::from_str(&file_content) {
        Ok(json) => json,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Failed to parse JSON: {}",
                e
            )))
        }
    };

    let project = le_json.data.into_iter().find(|p| p.slug == project_name);
    Ok(project)
}

#[island]
pub fn UseCase(slug: String) -> impl IntoView {
    let (project, write_project) = create_signal(None::<Project>);

    create_effect(move |_| {
        let slug = slug.clone();
        spawn_local(async move {
            match get_project_by_slug(slug).await {
                Ok(value) => {
                    write_project(value);
                }
                Err(_) => {}
            }
        });
    });

    let el = create_node_ref::<Svg>();
    let is_hovered = use_element_hover(el);

    view! {
        <Close el=el/>
        <main class={
            let base_class = "flex delay-75 duration-1000 mb-16 ease-out";
            move || {
                if is_hovered.get() {
                    format!("{} {}", base_class, "usecase-in")
                } else {
                    format!("{} {}", base_class, "usecase-out")
                }
            }
        }>
            <Layout id="project".to_string() aria_label="Usecase" class_name="flex-col".to_string()>
                // TODO: Figure out how to display data
                // TODO: Skeleton or loading information for user - perhaps an overkill?
                {move || match project.get() {
                    None => view! {}.into_view(),
                    Some(data) => {
                        let timeline = data.information.timeline.clone();
                        let role = data.information.role.clone();
                        let vec_names: Vec<String> = data
                            .name
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                        view! {
                            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-normal text-gray-9 leading-tighter mb-4 mt-8 md:mb-10 md:mt-20">
                                {vec_names
                                    .iter()
                                    .map(|name| {
                                        view! {
                                            <div class="animated-title">
                                                <span class="animated-title-element text-gray-9 font-bold">
                                                    {name}
                                                </span>
                                            </div>
                                            {' '}
                                        }
                                    })
                                    .collect_view()}

                            </h1>
                            <div class="flex flex-col md:flex-row gap-8 md:gap-10 lg:gap-20 fade-y-trans">
                                <div class="md:w-2/6 flex flex-col gap-8">
                                    <p class="text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                                        {data.description}
                                    </p>

                                    <div class="flex flex-row flex-wrap gap-2 md:gap-4 overflow-x-scroll md:overflow-x-hidden">
                                        <button class="pill pill-cta" role="button">
                                            <a target="_blank" href=data.link.url>
                                                <span class="pill-cta-border"></span>
                                                <span class="pill-cta-ripple">
                                                    <span></span>
                                                </span>
                                                <span class="pill-cta-title">
                                                    <span data-text="visit">visit</span>
                                                </span>
                                            </a>
                                        </button>

                                        {move || {
                                            data.tags
                                                .iter()
                                                .map(|tag| {
                                                    view! {
                                                        <div class="bg-gray-2 rounded-full px-6 py-2">
                                                            <span class="font-medium text-md">{tag}</span>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        }}

                                    </div>
                                </div>
                                <div class="md:w-4/6 flex flex-col">
                                    <Show when=move || data.information.role.is_some()>
                                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9 font-bold">
                                            {role.clone()}
                                        </p>
                                    </Show>
                                    <Show when=move || data.information.timeline.is_some()>
                                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-6 font-light">
                                            <b>Timeline:</b>
                                            {' '}
                                            {timeline.clone()}
                                        </p>
                                    </Show>
                                    <p
                                        inner_html=ammonia::Builder::new()
                                            .clean(&data.information.responsibility.clone())
                                            .to_string()
                                        class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9 mt-2 md:mt-8"
                                    ></p>
                                </div>
                            </div>
                            <div class="features mt-20 md:mt-40">
                                {move || {
                                    data.cards
                                        .iter()
                                        .map(|area| {
                                            view! {
                                                <CardLink class_name=area.to_string()>
                                                    <div class="h-full flex justify-center items-center">
                                                        <h3 class="text-gray-1 text-xs sm:text-lg md:text-2xl">
                                                            screen coming soon
                                                        </h3>
                                                    </div>
                                                </CardLink>
                                            }
                                        })
                                        .collect_view()
                                }}

                            </div>
                        }
                            .into_view()
                    }
                }}

            </Layout>
        </main>
    }
}














































































































































































































