use crate::components::about::About;
use crate::components::features::Features;
use crate::components::header::Header;
use crate::components::hero::Hero;
use crate::components::info::Info;
use crate::components::perdiem::Perdiem;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Header/>
        <main class="grid gap-20 md:gap-28 lg:gap-64 mt-12 md:mt-20 xl:mt-28">
            <Hero/>
            <About/>
            <Features/>
            <Perdiem/>
            <Info/>
        </main>
    }
}

