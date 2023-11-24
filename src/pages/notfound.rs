use leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="flex min-h-full flex-col">
            <main class="mx-auto flex w-full max-w-7xl flex-auto flex-col justify-center px-6 py-24 sm:py-64 lg:px-8">
                <p class="text-base font-semibold leading-8 text-indigo-600">404</p>
                <h1 class="mt-4 text-3xl font-bold tracking-tight text-gray-900 sm:text-5xl">Page not found</h1>
                <p class="mt-6 text-base leading-7 text-gray-600">"Sorry, we couldn’t find the page you’re looking for."</p>
                <div class="mt-10">
                    <a href="#" class="text-sm font-semibold leading-7 text-indigo-600"><span aria-hidden="true">&larr;</span> Back to home</a>
                </div>
            </main>
        </div>
    }
}
