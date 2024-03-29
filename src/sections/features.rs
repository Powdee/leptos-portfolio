use crate::components::ui::assets::{
    ayx_logo::AyxLogo, helpie_logo::HelpieLogo, invaders_logo::InvadersLogo,
    madesense_logo::MadesenseLogo, oms_logo::OmsLogo, splash_logo::SplashLogo,
};
use crate::components::ui::{card::Card, layout::Layout};
use crate::types::project::Project;

use leptos::*;

#[server(GetProjects, "/api", "GetJson", "v1/projects")]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use std::fs;
    use crate::types::project::ProjectData;

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

#[component]
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
        <div class="features mt-20 md:mt-40" id="projects">
            {move || {
                projects
                    .get()
                    .iter()
                    .map(|project| {
                        let project_clone = project.clone();
                        view! {
                            <Card
                                name=project_clone.slug.to_string()
                                style=""
                                class_name=project.area.to_string()
                            >
                                <div class="h-full flex justify-center items-center">
                                    {match project_clone.slug.as_str() {
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
        <Layout id="features".to_string() aria_label="Features" class_name="flex-col".to_string()>
            <h2 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                <div class="animated-title">
                    <span class="animated-title-element text-ek-white font-bold uppercase ">
                        Featured
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element font-light text-ek-white uppercase">
                        work
                    </span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-ek-white font-bold uppercase">
                        experience
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-ek-white font-bold uppercase">
                        and
                    </span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element font-light text-ek-white uppercase">
                        projects
                    </span>
                </div>
            </h2>
            <FeaturedCards/>
        </Layout>
    }
}



























































































































































































































































































































































































































































































































































































































