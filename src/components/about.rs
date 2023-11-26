use crate::components::layout::Layout;
use crate::components::me_circle::MeCircle;
use crate::components::me_circle::MeCircleTablet;

use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Layout aria_label="About">
            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12">
                <div class="order-2 lg:order-1 lg:row-span-3 self-center col-span-1">
                    <div class="w-full h-full lg:w-96 lg:h-96 flex justify-center items-center">
                        <img
                            class="absolute w-48 h-48 lg:w-80 lg:h-80 bg-cover bg-no-repeat bg-center rounded-full"
                            style="background-image: url(assets/me.jpg)"
                        />
                        <MeCircle/>
                        <MeCircleTablet/>
                    </div>
                </div>
                <p class="lg:col-span-2 order-1 min-w-full lg:order-2 text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-black">
                    A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                </p>
                <button
                    type="button"
                    class="lg:row-span-1 lg:col-span-2 order-3 w-full lg:mt-12 rounded-full self-end font-normal text-lg md:text-2xl bg-white px-4 py-8 lg:py-16 text-black shadow-sm ring-1 ring-inset ring-gray hover:bg-black hover:text-white"
                >
                    "Let's connect"
                </button>
            </div>
        </Layout>
    }
}

