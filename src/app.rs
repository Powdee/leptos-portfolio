use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::about::AboutPage;
use crate::pages::case::UseCasePage;
use crate::pages::home::HomePage;
use crate::pages::notfound::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Script src="https://unpkg.com/htmx.org@1.9.8"/>
        <Title text="Erik Kurjak"/>

        <Router>
            // <canvas id="canvas" width="800" height="600" class="absolute" />
            // <Header/>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/*any" view=NotFound/>
                <Route path="/about" view=AboutPage/>
                <Route path="/projects/:id" view=UseCasePage/>
            </Routes>
        </Router>
    }
}

