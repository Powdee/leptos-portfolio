use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Perdiem() -> impl IntoView {
    view! {
        <Layout aria_label="Perdiem" class_name="flex-col">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-9 leading-tighter">
                <span class="font-light">My</span>
                <br/>
                philosophy
            </h1>
        </Layout>
    }
}

