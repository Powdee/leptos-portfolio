use crate::components::ui::layout::Layout;

use leptos::*;
// Leading Engineering Innovation with Vision
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <div class="absolute backdrop-blur z-0 top-0 h-screen w-full bg-ek-dark bg-dot-ek-orange/[0.35] flex items-center justify-center">
            <div class="absolute pointer-events-none inset-0 flex items-center justify-center bg-ek-dark [mask-image:radial-gradient(ellipse_at_center,transparent_20%,#0e0306)]"></div>
        </div>
        <Layout id="home".to_string() aria_label="Hero" class_name="".to_string()>
            <div class="text-center md:text-left">
                <div class="animated-title">
                    <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading  sm:tracking-heading">
                        "Leading"
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-orange animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading  xl:leading-heading tracking-smallheading sm:tracking-heading">
                        "Engineering"
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white animated-title-element font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading relative">
                        "Innovation"
                    </span>
                // <span class="tracking-p absolute text-ek-orange bottom-10 right-0 text-2xl lowercase leading-p">
                // Rust
                // </span>
                </div>
                <br/>
                <div class="animated-title">
                    <div class="animated-title-element">
                        <span class="text-3xl sm:text-6xl lg:text-7xl text-ek-orange font-[400] uppercase leading-p sm:leading-mediump lg:leading-heading xl:leading-heading tracking-p relative mr-4 xs:mr-12">
                            "with"
                        </span>
                        <br class="block md:hidden"/>
                        <span class="text-5xl sm:text-8xl lg:text-9xl xl:text-10xl text-ek-white font-bold uppercase leading-smallheading sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading ">
                            "Vision"
                        </span>
                    </div>
                </div>
            </div>
        </Layout>
    }
}

