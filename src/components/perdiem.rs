use crate::components::ui::{
    button::Button, layout::Layout, viewport_visiblity::ViewportVisibility,
};

use leptos::*;

#[island]
pub fn Perdiem() -> impl IntoView {
    view! {
        <Layout aria_label="Perdiem" class_name="flex-col".to_string()>
            <ViewportVisibility>
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-9 leading-tighter">
                    <div class="animated-title">
                        <em class="animated-title-element text-gray-9 font-light">My</em>
                    </div>
                    <br/>
                    <div class="animated-title">
                        <span class="animated-title-element text-gray-9">philosophy</span>
                    </div>
                </h1>

                <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-8 mt-20 md:mt-40">
                    <div class="order-2 lg:order-1 lg:row-span-3 self-center col-span-1"></div>
                    <Button label="Have an idea?".to_string()/>
                    <p class="lg:col-span-2 order-1 min-w-full lg:order-3 text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                        A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                    </p>
                </div>
            </ViewportVisibility>
        </Layout>
    }
}

