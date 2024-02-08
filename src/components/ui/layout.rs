use leptos::*;

#[component]
pub fn Layout(
    children: Children,
    aria_label: &'static str,
    class_name: String,
) -> impl IntoView {
    let spacer = " ";

    view! {
        <section
            aria-label=aria_label
            class=class_name.to_owned() + spacer
                + "selection:bg-gray-9 selection:text-gray-1 relative w-full isolate lg:mx-auto lg:mx-0 lg:flex mx-auto max-w-8xl 2xl:max-w-10xl px-4 md:px-6"
        >
            {children()}
        </section>
    }
}

