use crate::components::ui::layout::Layout;
use crate::components::ui::me_circle::MeCircle;
use crate::components::ui::me_circle::MeCircleTablet;

use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout aria_label="About">
            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12">
                <div class="order-2 lg:order-1 lg:row-span-3 self-center col-span-1">
                    <div class="w-full h-full lg:w-96 lg:h-96 flex justify-center items-center">
                        <picture class="absolute flex items-center">
                            <source
                                type="image/webp"
                                srcset="https://leptoscv.s3.eu-central-1.amazonaws.com/me.webp"
                            />
                            <img
                                width="400"
                                height="400"
                                loading="lazy"
                                class="rounded-full object-cover w-48 h-48 md:w-72 md:h-72"
                                decoding="async"
                                alt="erik kurjak"
                                src="https://leptoscv.s3.eu-central-1.amazonaws.com/me.jpg"
                            />
                        </picture>
                        <MeCircle/>
                        <MeCircleTablet/>
                    </div>
                </div>
                <p class="lg:col-span-2 order-1 min-w-full lg:order-2 text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                    A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                </p>
                <button
                    type="button"
                    class="lg:row-span-1 lg:col-span-2 order-3 w-full lg:mt-12 rounded-full self-end outline-none font-normal text-lg md:text-3xl px-4 py-8 md:py-12 lg:py-16 text-gray-9 bg-gray-1 shadow-sm ring-1 ring-inset ring-gray-9 hover:bg-gray-9 hover:text-gray-1"
                >
                    "Let's connect"
                </button>
            </div>
        </Layout>
    }
}

