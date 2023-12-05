use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Perdiem() -> impl IntoView {
    view! {
        <Layout aria_label="Perdiem" class_name="flex-col">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-9 leading-tighter">
                <span class="font-light">My</span>
                <br/>
                philosophy
            </h1>

            <div class="grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col gap-x-12 lg:gap-y-0 gap-y-12 mt-20 md:mt-40">
                <div class="order-2 lg:order-1 lg:row-span-3 self-center col-span-1"></div>
                <button
                    type="button"
                    class="lg:row-span-1 lg:col-span-2 order-2 w-full lg:mb-12 rounded-full self-end outline-none font-normal text-lg md:text-3xl px-4 py-8 md:py-12 lg:py-16 text-gray-9 bg-gray-1 shadow-sm ring-1 ring-inset ring-gray-9 hover:bg-gray-9 hover:text-gray-1"
                >
                    "Have an idea?"
                </button>
                <p class="lg:col-span-2 order-1 min-w-full lg:order-3 text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9">
                    A team player with a passion for building modern digital solutions. With a strong affinity for functional programming and a natural problem-solving ability.
                </p>
            </div>
        </Layout>
    }
}

