use crate::components::ui::{
    button::Button, layout::Layout, me_circle::MeCircle,
    me_circle::MeCircleTablet,
};

use leptos::*;
use leptos_use::use_window_scroll;

// TODO: Move to util function - used in other places
fn map_y_to_value(y: f64) -> f64 {
    let start_y = 0.0;
    let end_y = 1200.0;
    let start_value = 0.8;
    let end_value = 1.05;

    if y < start_y {
        return start_value;
    }
    if y > end_y {
        return end_value;
    }

    let scale = (y - start_y) / (end_y - start_y);
    start_value + scale * (end_value - start_value)
}

#[island]
pub fn ScrollWatchImage() -> impl IntoView {
    let (_, y) = use_window_scroll();

    view! {
        <img
            width="400"
            height="400"
            style=move || format!("scale: {}", map_y_to_value(y.get()))
            loading="lazy"
            class="rounded-full object-cover w-48 h-48 md:w-64 md:h-64 will-change-auto bg-gray-3"
            decoding="async"
            alt="erik kurjak"
            src="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
        />
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout id="about".to_string() aria_label="About" class_name="".to_string()>
            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12 fade-in">
                <div class="order-2 lg:order-1 lg:row-span-3 self-center col-span-1">
                    <div class="w-full h-full lg:w-96 lg:h-96 flex justify-center items-center">
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
                <p class="lg:col-span-2 order-1 min-w-full lg:order-2 text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                    A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                </p>
                <Button
                    href="mailto:contact@erikkurjak.com".to_string()
                    class_name="order-2 self-end".to_string()
                    label="Get in touch".to_string()
                />
            </div>
        </Layout>
    }
}

