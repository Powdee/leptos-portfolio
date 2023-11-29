use crate::components::about::About;
use crate::components::features::Features;
use crate::components::hero::Hero;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="grid gap-28 lg:gap-64 my-20 md:my-28 px-4 md:px-6 mx-auto max-w-8xl 2xl:max-w-10xl">
            <Hero/>
            <About/>
            <Features/>
        </main>
    }
}

