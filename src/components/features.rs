
use std::fs;

use crate::components::ui::assets::{
    ayx_logo::AyxLogo, helpie_logo::HelpieLogo, invaders_logo::InvadersLogo,
    madesense_logo::MadesenseLogo, oms_logo::OmsLogo, splash_logo::SplashLogo,
};
use crate::components::ui::{card::Card, layout::Layout};
use crate::types::project::{Project, ProjectData};

use leptos::*;

#[server(GetProjects, "/api", "GetJson", "v1/projects")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    let file_path = format!("{}/target/site/resources/projects.json", env!("CARGO_MANIFEST_DIR"));
    
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => return Err(ServerFnError::ServerError(format!("Failed to read file: {}", e))),
    };

    let le_json: ProjectData = match serde_json::from_str(&file_content) {
        Ok(json) => json,
        Err(e) => return Err(ServerFnError::ServerError(format!("Failed to parse JSON: {}", e))),
    };

    Ok(le_json.data)
}

#[island]
fn FeaturedCards() -> impl IntoView {

    let (projects, write_projects) = create_signal(Vec::<Project>::new());

    create_effect(move |_| {
        spawn_local(async move {
            match get_projects().await {
                Ok(value) => {
                    write_projects(value);
                }
                Err(_) => {}
            }
        });
    });

    view! {
        <div class="features mt-20 md:mt-40">
            {move || {
                projects
                    .get()
                    .iter()
                    .map(|project| {
                        let project_clone = project.clone();
                        view! {
                            <Card
                                name=project_clone.name.to_string()
                                style=""
                                class_name=project.area.to_string()
                            >
                                <div class="h-full flex justify-center items-center">
                                    {match project_clone.name.as_str() {
                                        "oms" => view! { <OmsLogo/> }.into_view(),
                                        "alteryx" => view! { <AyxLogo/> }.into_view(),
                                        "madesense" => view! { <MadesenseLogo/> }.into_view(),
                                        "splashsports" => view! { <SplashLogo/> }.into_view(),
                                        "invaders" => view! { <InvadersLogo/> }.into_view(),
                                        "helpie" => view! { <HelpieLogo/> }.into_view(),
                                        _ => view! {}.into_view(),
                                    }}

                                </div>
                            </Card>
                        }
                    })
                    .collect_view()
            }}

        </div>
    }
}

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <Layout aria_label="Features" class_name="flex-col">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-9 leading-tighter">
                Featured <br/> <span class="font-light">work</span> experience <br/> and
                <span class="font-light">projects</span>
            </h1>

            <FeaturedCards/>
        </Layout>
    }
}




























































































































































































































































































































































