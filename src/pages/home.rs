use crate::components::about::About;
use crate::components::hero::Hero;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main class="grid gap-28 lg:gap-56 my-20 md:my-28 px-4 md:px-6 mx-auto max-w-8xl">
            <Hero/>
            <About/>
        </main>
    }
}

