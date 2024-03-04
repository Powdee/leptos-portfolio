use crate::components::ui::{button::Button, layout::Layout};

use leptos::*;

#[component]
pub fn Perdiem() -> impl IntoView {
    view! {
        <Layout id="perdiem".to_string() aria_label="Perdiem" class_name="flex-col".to_string()>
            <h2 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl text-ek-white leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                <div class="animated-title">
                    <span class="animated-title-element text-ek-white font-regular uppercase">
                        My
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-ek-white font-bold uppercase">
                        philosophy
                    </span>
                </div>
            </h2>

            <div class="min-h-[350px] md:min-h-[500px] grid lg:grid-rows-2 lg:grid-cols-2 lg:grid-flow-col mt-20 md:mt-40">
                <div class=" flex justify-center items-center order-2 lg:order-1 lg:row-span-3 self-center col-span-1">
                    <h4 class="text-ek-orange text-center text-2xl md:text-4xl">
                        3D animation <br/> coming soon
                    </h4>
                </div>
                <Button
                    href="mailto:contact@erikkurjak.com".to_string()
                    class_name="order-3 self-center".to_string()
                    label="Have an idea?".to_string()
                />
                <p class="lg:col-span-2 order-1 self-center min-w-full lg:order-3 text-xl md:text-2xl lg:text-3xl leading-p lg:leading-largep text-ek-white">
                    I am dedicated to completing projects that contribute to increased revenue. Reliability and a results-driven approach are at the core of my work.
                </p>
            </div>
        </Layout>
    }
}

