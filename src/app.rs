use std::env;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::download_cv::DownloadCVPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::soon::SoonPage;
use crate::pages::use_cases::UseCasesPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let env = env::var("RUST_ENV").unwrap_or_else(|_| "production".to_string());

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Erik Kurjak" formatter=|text| format!("{text}")/>
        <Script
            fetchpriority="low"
            defer="defer"
            src="https://unpkg.com/@studio-freight/lenis@1.0.33/dist/lenis.min.js"
        />
        <Html lang="en" attr:data-theme="light"/>
        <Script fetchpriority="low" defer="defer" src="/pkg/init_lenis.js"/>

        <Router>
            <Show when=move || env == "production" fallback=SoonPage>
                // <canvas id="canvas" width="800" height="600" class="absolute"></canvas>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                    <Route path="/projects/:slug" view=UseCasesPage/>
                    <Route path="/resume" view=DownloadCVPage/>
                </Routes>
            </Show>
        </Router>
    }
}

