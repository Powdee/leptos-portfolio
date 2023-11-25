use crate::components::about::About;
use crate::components::header::Header;
use crate::components::hero::Hero;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="bg-white">
            <Header/>
            <main>
                <Hero/>
                <About/>
            </main>
        </div>
    }
}

