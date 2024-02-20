use crate::components::{header::Header, ui::layout::Layout};

use leptos::*;

#[component]
pub fn SoonPage() -> impl IntoView {
    view! {
        <Header/>
        <main class="grid gap-28 lg:gap-64 mt-10 md:mt-28">
            <Layout
                id="soon".to_string()
                aria_label="Coming soon"
                class_name="flex-col".to_string()
            >
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 leading-tighter">
                    <div class="animated-title">
                        <span class="animated-title-element text-gray-9">Building</span>
                    </div>
                    {' '}
                    <div class="animated-title">
                        <em class="animated-title-element font-light text-gray-9">blocks</em>
                    </div>
                    <br/>
                    <div class="animated-title">
                        <span class="animated-title-element text-gray-9">of</span>
                    </div>
                    {' '}
                    <div class="animated-title">
                        <span class="animated-title-element text-gray-9">brilliance</span>
                    </div>
                </h1>
                <p class="min-w-full fade-in text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9 mt-10 lg:mt-20">
                    Mark your calendars for <i>
                        <b>31th January 2024</b>
                    </i> , when this digital canvas
                    transforms into a complete showcase of my <i>
                        <b>engineering</b>
                    </i> journey.
                </p>
            </Layout>
        </main>
    }
}

