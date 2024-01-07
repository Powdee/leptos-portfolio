use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Layout aria_label="Hero">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 leading-tighter text-center md:text-left">
                I <span class="font-light text-gray-9">engineer</span> <br/> digital solutions <br/>
                from scratch
            </h1>
        </Layout>
    }
}

