use leptos::*;
use leptos_meta::*;

use crate::components::{header::Header, ui::layout::Layout};

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Erik Kurjak - Page Not Found"/>
        <Header/>
        <div class="absolute backdrop-blur z-0 top-0 h-screen w-full bg-ek-dark bg-dot-ek-orange/[0.35] flex items-center justify-center">
            <div class="absolute pointer-events-none inset-0 flex items-center justify-center bg-ek-dark [mask-image:radial-gradient(ellipse_at_center,transparent_20%,#0e0306)]"></div>
        </div>
        <main class="grid gap-28 lg:gap-64 mt-10 md:mt-28">
            <h2 class="text-[520px] text-ek-orange/[.75] absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 font-medium">
                404
            </h2>
            <Layout
                id="notfound".to_string()
                aria_label="Not Found"
                class_name="flex-col".to_string()
            >
                <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl text-ek-white font-bold uppercase">
                    Page <br/> <span class="font-[400]">not</span> found
                </h1>
                <p class="text-xl md:text-2xl lg:text-3xl text-ek-white mt-10 lg:mt-20">
                    "Sorry, we couldn’t find the page you’re looking for" <br/>
                    <a
                        class="text-md md:text-lg lg:text-xl text-ek-white mt-2 lg:mt-4 font-bold"
                        href="/"
                    >
                        "Let’s"
                        go home
                    </a>
                </p>
            </Layout>
        </main>
    }
}

