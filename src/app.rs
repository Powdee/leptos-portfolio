use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::download_cv::DownloadCVPage;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::use_cases::UseCasesPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Erik Kurjak" formatter=|text| format!("{text}")/>
        <Html lang="en" attr:data-theme="light"/>

        <Router>
            <Routes>
                <Route path="" view=HomePage/>
                <Route path="/*any" view=NotFound/>
                <Route path="/projects/:slug" view=UseCasesPage/>
                <Route path="/resume" view=DownloadCVPage/>
            </Routes>
        </Router>
    }
}

