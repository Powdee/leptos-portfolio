use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Layout aria_label="Hero" class_name="".to_string()>
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 leading-tighter text-center md:text-left">
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9">I</span>
                </div>
                {' '}
                <div class="animated-title">
                    <em class="animated-title-element font-light text-gray-9">engineer</em>
                </div>
                {' '}
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9">digital</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9">solutions</span>
                </div>
                {' '}
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9">from</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9">scratch</span>
                </div>
            </h1>
        </Layout>
    }
}

