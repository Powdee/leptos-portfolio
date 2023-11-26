use crate::components::layout::Layout;

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Layout aria_label="Hero">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-black leading-tighter">
                I <span class="font-light">Engineer</span> <br/> digital solutions <br/>
                from scratch
            </h1>
        </Layout>
    }
}

