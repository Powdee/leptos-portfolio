use crate::components::experience::Experience;
use crate::components::skills::Skills;
use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout
            id="about".to_string()
            aria_label="About"
            class_name="flex flex-col gap-10 lg:gap-32".to_string()
        >
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in">
                <div class="lg:col-span-3">
                    <div class="text-sm lg:text-md leading-about text-ek-white uppercase">
                        <span class="uppercase">About</span>
                    </div>

                </div>
                <p class="lg:col-span-4 min-w-full text-xl lg:text-3xl leading-p lg:leading-largep text-ek-white">
                    Technical leader with management skills, actively involved in coding as a core aspect of my engineering responsibilities.
                    As a software engineer, "I've"
                    contributed to the development of cutting-edge software solutions, demonstrating leadership and architecting
                    projects with an emphasis on documentation. <br class="hidden lg:block"/> <br/>
                    Throughout my career, from startups, established firms to digital agencies,
                    "I've"
                    been committed to driving innovation, fostering team collaboration, and enhancing product quality and user experience.
                </p>
            </div>

            <Experience/>
            <Skills/>
        </Layout>
    }
}

