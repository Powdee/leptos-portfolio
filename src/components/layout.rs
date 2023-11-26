use leptos::*;

#[component]
pub fn Layout(children: Children, aria_label: &'static str) -> impl IntoView {
    view! {
        <section
            aria-label=aria_label
            class="relative w-full isolate lg:mx-auto max-w-full lg:mx-0 lg:flex lg:max-w-none lg:items-center"
        >
            {children()}
        </section>
    }
}

