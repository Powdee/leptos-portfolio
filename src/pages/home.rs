use crate::components::header::Header;
use crate::components::hero::Hero;
use crate::components::me::Me;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="bg-white">
            <Header/>
            <main>
                <Hero/>
                <Me/>
            </main>
        </div>
    }
}

