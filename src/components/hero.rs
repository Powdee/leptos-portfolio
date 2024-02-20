use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <Layout id="home".to_string() aria_label="Hero" class_name="".to_string()>
            <h1 class="text-6xl sm:text-7xl lg:text-8xl xl:text-10xl tracking-tight text-gray-9 leading-tighter text-left">
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9 font-medium uppercase">I</span>
                </div>
                {' '}
                <div class="animated-title">
                    <em class="animated-title-element text-gray-9 font-light">engineer</em>
                </div>
                {' '}
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9 font-medium">digital</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9 font-medium">solutions</span>
                </div>
                {' '}
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9 font-medium">from</span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-gray-9 font-medium">scratch</span>
                </div>
            </h1>
        </Layout>
    }
}

