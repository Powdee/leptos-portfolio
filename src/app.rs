use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::about::AboutPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::soon::SoonPage;
use crate::pages::use_cases::UseCasesPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let le_done = false;

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Erik Kurjak"/>

        <Show when=move || le_done fallback=SoonPage>
            <Router>
                // <canvas id="canvas" width="800" height="600" class="absolute" />
                // <Header/>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/projects/:id" view=UseCasesPage/>
                </Routes>
            </Router>
        </Show>
    }
}

