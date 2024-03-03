use crate::components::header::Header;
use crate::components::profile::Profile;

use crate::sections::about::About;
use crate::sections::experience::Experience;
use crate::sections::features::Features;
use crate::sections::hero::Hero;
use crate::sections::info::Info;
use crate::sections::perdiem::Perdiem;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Header/>
        <Profile/>
        <main class="grid gap-20 md:gap-28 lg:gap-64 mt-12 md:mt-20 xl:mt-28">
            <Hero/>
            <About/>
            <Experience/>
            <Features/>
            <Perdiem/>
            <Info/>
        </main>
    }
}

