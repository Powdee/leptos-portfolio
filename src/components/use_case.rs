use std::fs;

use crate::components::ui::card_link::CardLink;
use crate::components::ui::layout::Layout;
use crate::types::project::{Project, ProjectData};

use leptos::*;
use web_sys::MouseEvent;

// TODO: Remove this function and update get_projects to take a param 
// TODO: project_name (optional) and use it here
#[server(GetProjectByName, "/api", "GetJson", "v1/project")]
pub async fn get_project_by_name(project_name: String) -> Result<Option<Project>, ServerFnError> {
    let file_path = format!("{}/resources/projects.json", env!("CARGO_MANIFEST_DIR"));

    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(ServerFnError::ServerError(format!("Failed to read file: {}", e))),
    };

    let le_json: ProjectData = match serde_json::from_str(&file_content) {
        Ok(json) => json,
        Err(e) => return Err(ServerFnError::ServerError(format!("Failed to parse JSON: {}", e))),
    };

    let project = le_json.data.into_iter().find(|p| p.name == project_name);
    Ok(project)
}

#[island]
pub fn UseCase(name: String) -> impl IntoView {
    let (project, write_project) = create_signal(None::<Project>);

	create_effect(move |_| {
		let name_clone = name.clone();
		spawn_local(async move {
			match get_project_by_name(name_clone).await {
				Ok(value) => {
					write_project(value);
				}
				Err(_) => {}
			}
		});
	});

    let (is_hovered, set_hovered) = create_signal(false);

    view! {
        <header class="mx-auto max-w-full py-6 px-10 md:py-12 md:px-16">
            <nav class="gap-2 md:flex-row flex-col flex items-center justify-center" aria-label="X">
                <a href="/">
                    <svg
                        on:mouseenter=move |_e: MouseEvent| set_hovered(true)
                        on:mouseleave=move |_e: MouseEvent| set_hovered(false)
                        width="61"
                        height="61"
                        class="cursor-pointer hover:scale-105 ease-out duration-300 close-x"
                        viewBox="0 0 61 61"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <rect
                            width="61"
                            height="61"
                            rx="30.5"
                            class="fill-gray-2 hover:fill-gray-3 ease-out duration-300"
                        ></rect>
                        <g clip-path="url(#clip0_615_114)">
                            <path
                                d="M38.7751 24.0174L36.9825 22.2248L29.8756 29.3317L22.7687 22.2248L20.9761 24.0174L28.083 31.1243L20.9761 38.2312L22.7687 40.0238L29.8756 32.9169L36.9825 40.0238L38.7751 38.2312L31.6682 31.1243L38.7751 24.0174Z"
                                fill="#212529"
                            ></path>
                        </g>
                        <defs>
                            <clipPath id="clip0_615_114">
                                <rect
                                    width="51.7791"
                                    height="51.7791"
                                    fill="white"
                                    transform="translate(4.25586 4.96509)"
                                ></rect>
                            </clipPath>
                        </defs>
                    </svg>
                </a>
            </nav>
        </header>
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
            <Layout aria_label="Usecase" class_name="flex-col">
                // TODO: Figure out how to display data
                // TODO: Skeleton or loading information for user - perhaps an overkill?
                {move || match project.get() {
                    None => view! {}.into_view(),
                    Some(data) => {
                        view! {
                            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-normal text-gray-9 leading-tighter font-regular mb-4 mt-8 md:mb-10 md:mt-20">
                                {data.name}
                            </h1>
                            <div class="flex flex-col md:flex-row gap-8 md:gap-10 lg:gap-20">
                                <div class="flex flex-col gap-8">
                                    <p class="text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                                        {data.description}
                                    </p>

                                    <div class="flex flex-row gap-2 md:gap-4 overflow-x-scroll md:overflow-x-hidden">

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
                                <div class="flex flex-col gap-2 md:gap-8">
                                    <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                                        {data.information.timeline}
                                    </p>
                                    <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                                        {data.information.about}
                                    </p>
                                    <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9">
                                        {data.information.responsibility}
                                    </p>
                                </div>
                            </div>
                            <div class="grid gap-4 md:grid-cols-5 md:grid-rows-7 mt-10 md:mt-20">
                                <CardLink class_name="md:col-span-3 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row">
                                    <span></span>
                                </CardLink>
                                <CardLink class_name="md:col-span-2 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row">
                                    <span></span>
                                </CardLink>
                                <CardLink class_name="md:col-span-2 md:row-span-4 min-h-card_2_row_mobile md:min-h-card_2_row">
                                    <span></span>
                                </CardLink>
                                <CardLink class_name="md:col-span-3 md:row-span-2 min-h-card_2_row_mobile">
                                    <span></span>
                                </CardLink>
                                <CardLink class_name="md:col-span-1 md:row-span-2 bg-gray-8 min-h-card_2_row_mobile">
                                    <span></span>
                                </CardLink>
                                <CardLink class_name="md:col-span-2 md:row-span-2 min-h-card_2_row_mobile">
                                    <span></span>
                                </CardLink>
                            </div>
                        }
                            .into_view()
                    }
                }}

            </Layout>
        </main>
    }
}






































































