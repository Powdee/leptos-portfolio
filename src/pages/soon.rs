use crate::components::{header::Header, ui::layout::Layout};

use leptos::*;

#[component]
pub fn SoonPage() -> impl IntoView {
    view! {
        <Header/>
        <main class="grid gap-28 lg:gap-64 mt-10 md:mt-28">
            <Layout aria_label="Coming soon" class_name="flex-col">
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 leading-tighter">
                    Building <span class="font-light text-gray-9">blocks</span> <br/> of brilliance
                </h1>
                <p class="min-w-full text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9 mt-10 lg:mt-20">
                    Mark your calendars for <i>
                        <b>20th January 2024</b>
                    </i> , when this digital canvas
                    transforms into a complete showcase of my <i>
                        <b>engineering</b>
                    </i> journey.
                </p>
            </Layout>
        </main>
    }
}

