use leptos::*;

use crate::components::{header::Header, ui::layout::Layout};

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Header/>
        <main class="grid gap-28 lg:gap-64 mt-10 md:mt-28">
            <h2 class="text-[520px] text-gray-2 absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 font-medium">
                404
            </h2>
            <Layout aria_label="Not Found" class_name="flex-col">
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 lg:leading-tighter">
                    Page <span class="font-light text-gray-9">not</span> <br/> not found
                </h1>
                <p class="text-xl md:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-gray-9 mt-10 lg:mt-20">
                    Sorry, we "couldn’t" find the <b>
                        <i>page</i>
                    </b> " you’re" looking for
                </p>
                <a
                    class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-gray-9 mt-2 lg:mt-4 font-bold"
                    href="/"
                >
                    "Let’s"
                    go home
                </a>
            </Layout>
        </main>
    }
}

