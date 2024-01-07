use std::env;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::soon::SoonPage;
use crate::pages::use_cases::UseCasesPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let env =
        env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Erik Kurjak"/>

        <Router>
            <Show when=move || env == "development" fallback=SoonPage>
                // <canvas id="canvas" width="800" height="600" class="absolute" />
                // <Header/>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                    <Route path="/projects/:id" view=UseCasesPage/>
                </Routes>
            </Show>
        </Router>
    }
}

