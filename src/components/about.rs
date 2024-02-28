use crate::components::ui::{
    layout::Layout,
    me_circle::{MeCircle, MeCircleTablet},
};

use leptos::*;

#[island]
pub fn ScrollWatchImage() -> impl IntoView {
    view! {
        <img
            width="400"
            height="400"
            loading="lazy"
            class="rounded-full object-cover w-[140px] h-[140px] will-change-auto bg-ek-orange"
            decoding="async"
            alt="erik kurjak"
            src="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
        />
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class=" w-full h-full lg:w-56 lg:h-56 hidden lg:flex justify-center items-center fixed right-10 bottom-10 z-10">
            <div class="w-48 h-48 rounded-full bg-ek-orange flex justify-center items-center">
                <picture class="absolute flex items-center">
                    <source
                        type="image/webp"
                        srcset="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
                    />
                    <ScrollWatchImage/>
                </picture>
                <MeCircle/>
                <MeCircleTablet/>
            </div>
        </div>
        <Layout id="about".to_string() aria_label="About" class_name="".to_string()>
            <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12 fade-in">
                <div class="order-2 lg:order-1 col-span-3">
                    <div class="text-sm lg:text-md leading-about text-ek-white uppercase">
                        <span class="uppercase">About</span>
                    </div>

                </div>
                <p class="lg:col-span-4 order-1 min-w-full lg:order-2 text-xl lg:text-3xl leading-largep text-ek-white">
                    Technical leader with management skills, actively involved in coding as a core aspect of my engineering responsibilities.
                    As a software engineer, "I've"
                    contributed to the development of cutting-edge software solutions, demonstrating leadership and architecting
                    projects with an emphasis on documentation. <br/> <br/>
                    Throughout my career, from startups, established firms to digital agencies,
                    "I've"
                    been committed to driving innovation, fostering team collaboration, and enhancing product quality and user experience.
                </p>
            </div>
        </Layout>
    }
}

